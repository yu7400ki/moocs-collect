use crate::cache::Cache;
use crate::domain::{
    models::{LectureKey, LecturePage, MoocsUrl, PageKey, UrlBuilder},
    repository::PageRepository,
};
use crate::error::Result;
use crate::utils::{extract_element_attribute, parse_selector};
use async_trait::async_trait;
use reqwest::Client;
use scraper::Html;
use std::sync::Arc;
use std::time::Duration;

pub struct PageRepositoryImpl {
    client: Arc<Client>,
    url_builder: UrlBuilder,
    page_cache: Cache<LectureKey, Vec<LecturePage>>,
}

impl PageRepositoryImpl {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            url_builder: UrlBuilder::default(),
            page_cache: Cache::new(Duration::from_secs(600)), // 10 minutes
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.url_builder = UrlBuilder::new(base_url);
        self
    }

    async fn fetch_lecture_page(&self, lecture_key: &LectureKey) -> Result<(String, String)> {
        let url = self.url_builder.lecture_url(
            lecture_key.course_key.year.clone(),
            lecture_key.course_key.slug.clone(),
            lecture_key.slug.clone(),
        );

        let response = self.client.get(&url).send().await.map_err(|e| {
            crate::error::CollectError::network("Failed to fetch lecture page", Some(e))
        })?;

        let final_url = response.url().to_string();
        let html = response.text().await.map_err(|e| {
            crate::error::CollectError::network("Failed to read response body", Some(e))
        })?;

        Ok((final_url, html))
    }

    fn scrape_pages(
        &self,
        html: &str,
        current_url: &str,
        lecture_key: &LectureKey,
    ) -> Result<Vec<LecturePage>> {
        let document = Html::parse_document(html);
        let pagination_selector = parse_selector("ul.pagination li")?;

        let pagination_items: Vec<_> = document.select(&pagination_selector).collect();

        if pagination_items.len() <= 2 {
            return Ok(vec![]);
        }

        let current_page_key = self.parse_page_key_from_url(current_url, lecture_key)?;
        let mut pages = Vec::new();

        for (index, li) in pagination_items[1..pagination_items.len() - 1]
            .iter()
            .enumerate()
        {
            let page = self.extract_page_from_element(li, lecture_key, &current_page_key, index)?;
            pages.push(page);
        }

        Ok(pages)
    }

    fn extract_page_from_element(
        &self,
        element: &scraper::ElementRef,
        lecture_key: &LectureKey,
        current_page_key: &PageKey,
        index: usize,
    ) -> Result<LecturePage> {
        let title = extract_element_attribute(element, "a", "title")?;
        let href = extract_element_attribute(element, "a", "href")?;

        let page_key = if href == "#" {
            current_page_key.clone()
        } else {
            self.parse_page_key_from_url(&href, lecture_key)?
        };

        let page = LecturePage::new(page_key, title, index);
        Ok(page)
    }

    fn parse_page_key_from_url(&self, url: &str, lecture_key: &LectureKey) -> Result<PageKey> {
        let full_url = if url.starts_with("http") {
            url.to_string()
        } else {
            format!("{}{}", self.url_builder.base_url(), url)
        };

        let moocs_url = MoocsUrl::parse_moocs_url(&full_url)?;
        match moocs_url {
            MoocsUrl::Page { page_key } => {
                if page_key.lecture_key == *lecture_key {
                    Ok(page_key)
                } else {
                    Err(crate::error::CollectError::parse(
                        "Page lecture key mismatch",
                        Some(format!(
                            "Expected: {}, Found: {}",
                            lecture_key, page_key.lecture_key
                        )),
                    ))
                }
            }
            _ => Err(crate::error::CollectError::parse(
                "URL is not a page URL",
                Some(url.to_string()),
            )),
        }
    }
}

#[async_trait]
impl PageRepository for PageRepositoryImpl {
    async fn fetch_pages(&self, lecture_key: &LectureKey) -> Result<Vec<LecturePage>> {
        // Check cache first
        if let Some(cached_pages) = self.page_cache.get(lecture_key) {
            return Ok(cached_pages);
        }

        // Cache miss - fetch from API
        let (current_url, html) = self.fetch_lecture_page(lecture_key).await?;
        let pages = self.scrape_pages(&html, &current_url, lecture_key)?;

        // Cache the result
        self.page_cache.insert(lecture_key.clone(), pages.clone());

        Ok(pages)
    }

    async fn fetch_page(&self, page_key: &PageKey) -> Result<Option<LecturePage>> {
        let pages = self.fetch_pages(&page_key.lecture_key).await?;
        Ok(pages.into_iter().find(|page| page.key == *page_key))
    }
}
