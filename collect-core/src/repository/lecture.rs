use crate::cache::Cache;
use crate::domain::{
    models::{
        CourseKey, Lecture, LectureBuilder, LectureGroup, LectureGroupBuilder, LectureKey,
        MoocsUrl, UrlBuilder,
    },
    repository::LectureRepository,
};
use crate::error::Result;
use crate::utils::{extract_element_attribute, extract_text_content, parse_selector};
use async_trait::async_trait;
use reqwest::Client;
use scraper::Html;
use std::sync::Arc;
use std::time::Duration;

pub struct LectureRepositoryImpl {
    client: Arc<Client>,
    url_builder: UrlBuilder,
    lecture_group_cache: Cache<CourseKey, Vec<LectureGroup>>,
}

impl LectureRepositoryImpl {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            url_builder: UrlBuilder::default(),
            lecture_group_cache: Cache::new(Duration::from_secs(900)), // 15 minutes
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.url_builder = UrlBuilder::new(base_url);
        self
    }

    async fn fetch_course_page(&self, course_key: &CourseKey) -> Result<String> {
        let url = self
            .url_builder
            .course_url(course_key.year.clone(), course_key.slug.clone());

        let response = self.client.get(&url).send().await.map_err(|e| {
            crate::error::CollectError::network("Failed to fetch course page", Some(e))
        })?;

        let html = response.text().await.map_err(|e| {
            crate::error::CollectError::network("Failed to read response body", Some(e))
        })?;

        Ok(html)
    }

    fn scrape_lecture_groups(
        &self,
        html: &str,
        course_key: &CourseKey,
    ) -> Result<Vec<LectureGroup>> {
        let document = Html::parse_document(html);
        let treeview_selector = parse_selector("ul.sidebar-menu li.treeview")?;

        let mut lecture_groups = Vec::new();

        for (i, treeview) in document.select(&treeview_selector).enumerate() {
            let group_name = self.extract_group_name(&treeview)?;
            let lectures = self.extract_lecture_items(&treeview, course_key)?;

            let lecture_group = LectureGroupBuilder::new()
                .with_course_key(course_key.clone())
                .with_name(group_name)
                .with_lectures(lectures)
                .with_index(i)
                .build()
                .ok_or_else(|| {
                    crate::error::CollectError::parse("Failed to build lecture group", None)
                })?;

            lecture_groups.push(lecture_group);
        }

        Ok(lecture_groups)
    }

    fn extract_group_name(&self, treeview: &scraper::ElementRef) -> Result<String> {
        let group = extract_text_content(treeview, "span.sidebar-menu-text")?
            .trim()
            .to_string();
        Ok(group)
    }

    fn extract_lecture_items(
        &self,
        treeview: &scraper::ElementRef,
        course_key: &CourseKey,
    ) -> Result<Vec<Lecture>> {
        let menu_selector = parse_selector("ul.treeview-menu li")?;
        let mut lectures = Vec::new();

        for (i, menu_item) in treeview.select(&menu_selector).enumerate() {
            let lecture = self.extract_lecture_from_element(&menu_item, course_key, i)?;
            lectures.push(lecture);
        }

        Ok(lectures)
    }

    fn extract_lecture_from_element(
        &self,
        element: &scraper::ElementRef,
        course_key: &CourseKey,
        index: usize,
    ) -> Result<Lecture> {
        let name = extract_text_content(element, "a")?.trim().to_string();

        let href = extract_element_attribute(element, "a", "href")?;
        let lecture_key = self.parse_lecture_key_from_url(&href, course_key)?;

        let lecture = LectureBuilder::new()
            .with_key(lecture_key)
            .with_name(name)
            .with_index(index)
            .build()
            .ok_or_else(|| crate::error::CollectError::parse("Failed to build lecture", None))?;

        Ok(lecture)
    }

    fn parse_lecture_key_from_url(&self, url: &str, course_key: &CourseKey) -> Result<LectureKey> {
        let full_url = if url.starts_with("http") {
            url.to_string()
        } else {
            format!("{}{}", self.url_builder.base_url(), url)
        };

        let moocs_url = MoocsUrl::parse_moocs_url(&full_url)?;
        match moocs_url {
            MoocsUrl::Lecture { lecture_key } => {
                if lecture_key.course_key == *course_key {
                    Ok(lecture_key)
                } else {
                    Err(crate::error::CollectError::parse(
                        "Lecture course key mismatch",
                        Some(format!(
                            "Expected: {}, Found: {}",
                            course_key, lecture_key.course_key
                        )),
                    ))
                }
            }
            _ => Err(crate::error::CollectError::parse(
                "URL is not a lecture URL",
                Some(url.to_string()),
            )),
        }
    }
}

#[async_trait]
impl LectureRepository for LectureRepositoryImpl {
    async fn fetch_lecture_groups(&self, course_key: &CourseKey) -> Result<Vec<LectureGroup>> {
        // Check cache first
        if let Some(cached_groups) = self.lecture_group_cache.get(course_key) {
            return Ok(cached_groups);
        }

        // Cache miss - fetch from API
        let html = self.fetch_course_page(course_key).await?;
        let lecture_groups = self.scrape_lecture_groups(&html, course_key)?;

        // Cache the result
        self.lecture_group_cache
            .insert(course_key.clone(), lecture_groups.clone());

        Ok(lecture_groups)
    }

    async fn fetch_lecture(&self, lecture_key: &LectureKey) -> Result<Option<Lecture>> {
        let lectures = self
            .fetch_lecture_groups(&lecture_key.course_key)
            .await?
            .into_iter()
            .flat_map(|group| group.lectures)
            .collect::<Vec<Lecture>>();
        Ok(lectures
            .into_iter()
            .find(|lecture| lecture.key == *lecture_key))
    }
}
