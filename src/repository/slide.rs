use crate::cache::Cache;
use crate::domain::{
    models::{PageKey, ProcessedSvg, Slide, SlideContent, UrlBuilder},
    repository::SlideRepository,
};
use crate::error::Result;
use crate::utils::parse_selector;
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;
use scraper::Html;
use std::sync::Arc;
use std::time::Duration;

pub struct SlideRepositoryImpl {
    client: Arc<Client>,
    url_builder: UrlBuilder,
    slide_cache: Cache<PageKey, Vec<Slide>>,
    slide_content_cache: Cache<String, SlideContent>,
}

impl SlideRepositoryImpl {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            url_builder: UrlBuilder::default(),
            slide_cache: Cache::new(Duration::from_secs(600)), // 10 minutes
            slide_content_cache: Cache::new(Duration::from_secs(1800)), // 30 minutes
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.url_builder = UrlBuilder::new(base_url);
        self
    }

    async fn fetch_page_content(&self, page_key: &PageKey) -> Result<String> {
        let url = self.url_builder.page_url(
            page_key.lecture_key.course_key.year.clone(),
            page_key.lecture_key.course_key.slug.clone(),
            page_key.lecture_key.slug.clone(),
            page_key.slug.clone(),
        );

        let response = self.client.get(&url).send().await.map_err(|e| {
            crate::error::CollectError::network("Failed to fetch page content", Some(e))
        })?;

        let html = response.text().await.map_err(|e| {
            crate::error::CollectError::network("Failed to read response body", Some(e))
        })?;

        Ok(html)
    }

    fn scrape_slides(&self, html: &str, page_key: &PageKey) -> Result<Vec<Slide>> {
        let document = Html::parse_document(html);
        let iframe_selector = parse_selector("iframe")?;

        let gslide_regex = Regex::new(
            r"^https://docs\.google\.com/(a/[^/]+/)?presentation/d/(e/)?.*?/(embed|pubembed)\?",
        )
        .map_err(|e| {
            crate::error::CollectError::parse("Invalid Google Slides regex", Some(e.to_string()))
        })?;

        let mut slides = Vec::new();
        let mut index = 0;

        for iframe in document.select(&iframe_selector) {
            if let Some(src) = iframe.value().attr("src") {
                if gslide_regex.is_match(src) {
                    let slide = Slide::new(src, page_key.clone(), index);
                    slides.push(slide);
                    index += 1;
                }
            }
        }

        Ok(slides)
    }

    async fn fetch_slide_svgs(&self, slide_url: &str) -> Result<Vec<String>> {
        let response = self.client.get(slide_url).send().await.map_err(|e| {
            crate::error::CollectError::network("Failed to fetch slide content", Some(e))
        })?;

        let body = response.text().await.map_err(|e| {
            crate::error::CollectError::network("Failed to read slide response", Some(e))
        })?;

        let svg_regex = Regex::new(r"\\x3csvg.*?\\x3c\\/svg\\x3e").map_err(|e| {
            crate::error::CollectError::parse("Invalid SVG regex", Some(e.to_string()))
        })?;

        let svgs: Vec<String> = svg_regex
            .find_iter(&body)
            .map(|m| m.as_str().to_string())
            .map(|s| self.decode_unicode_escape(&s.replace(r"\/", "/")))
            .collect::<Result<Vec<_>>>()?;

        Ok(svgs)
    }

    fn decode_unicode_escape(&self, input: &str) -> Result<String> {
        unicode_escape::decode(input).map_err(|e| {
            crate::error::CollectError::parse(
                "Failed to decode unicode escape",
                Some(e.to_string()),
            )
        })
    }

    fn process_svg_content(&self, svgs: Vec<String>) -> Vec<ProcessedSvg> {
        svgs.into_iter()
            .enumerate()
            .map(|(index, content)| ProcessedSvg::new(content, index))
            .collect()
    }
}

#[async_trait]
impl SlideRepository for SlideRepositoryImpl {
    async fn fetch_slides(&self, page_key: &PageKey) -> Result<Vec<Slide>> {
        // Check cache first
        if let Some(cached_slides) = self.slide_cache.get(page_key) {
            return Ok(cached_slides);
        }

        // Cache miss - fetch from API
        let html = self.fetch_page_content(page_key).await?;
        let slides = self.scrape_slides(&html, page_key)?;

        // Cache the result
        self.slide_cache.insert(page_key.clone(), slides.clone());

        Ok(slides)
    }

    async fn fetch_slide_content(&self, slide: &Slide) -> Result<SlideContent> {
        let cache_key = format!("{}_{}", slide.page_key, slide.index);

        // Check cache first
        if let Some(cached_content) = self.slide_content_cache.get(&cache_key) {
            return Ok(cached_content);
        }

        // Cache miss - fetch from API
        let svg_strings = self.fetch_slide_svgs(&slide.url).await?;
        let processed_svgs = self.process_svg_content(svg_strings);
        let slide_content = SlideContent::new(slide.page_key.clone(), processed_svgs);

        // Cache the result
        self.slide_content_cache
            .insert(cache_key, slide_content.clone());

        Ok(slide_content)
    }
}
