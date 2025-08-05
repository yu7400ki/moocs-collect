use crate::cache::Cache;
use crate::domain::{
    models::{Course, CourseBuilder, CourseKey, MoocsUrl, UrlBuilder, Year},
    repository::CourseRepository,
};
use crate::error::Result;
use crate::utils::{extract_element_attribute, extract_text_content, parse_selector};
use async_trait::async_trait;
use reqwest::Client;
use scraper::Html;
use std::sync::Arc;
use std::time::Duration;

pub struct CourseRepositoryImpl {
    client: Arc<Client>,
    url_builder: UrlBuilder,
    course_cache: Cache<String, Vec<Course>>,
}

impl CourseRepositoryImpl {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            url_builder: UrlBuilder::default(),
            course_cache: Cache::new(Duration::from_secs(900)), // 15 minutes
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.url_builder = UrlBuilder::new(base_url);
        self
    }

    async fn fetch_course_page(&self, year: Option<Year>) -> Result<String> {
        let url = self.url_builder.courses_url(year);

        let response = self.client.get(&url).send().await.map_err(|e| {
            crate::error::CollectError::network("Failed to fetch course page", Some(e))
        })?;

        let html = response.text().await.map_err(|e| {
            crate::error::CollectError::network("Failed to read response body", Some(e))
        })?;

        Ok(html)
    }

    fn scrape_courses(&self, html: &str) -> Result<Vec<Course>> {
        let document = Html::parse_document(html);
        let course_selector = parse_selector(".content .media")?;

        let mut courses = Vec::new();
        for (index, course_card) in document.select(&course_selector).enumerate() {
            let course = self
                .extract_course_from_element(&course_card)?
                .with_index(index)
                .build()
                .ok_or_else(|| {
                    crate::error::CollectError::parse("Failed to build course from element", None)
                })?;
            courses.push(course);
        }

        Ok(courses)
    }

    fn extract_course_from_element(&self, element: &scraper::ElementRef) -> Result<CourseBuilder> {
        let name = extract_text_content(element, ".media-body h4.media-heading")?
            .trim()
            .to_string();

        let href = extract_element_attribute(element, "a", "href")?;

        let course_key = self.parse_course_key_from_url(&href)?;

        let course_builder = Course::builder().with_key(course_key).with_name(name);

        Ok(course_builder)
    }

    fn parse_course_key_from_url(&self, url: &str) -> Result<CourseKey> {
        let full_url = if url.starts_with("http") {
            url.to_string()
        } else {
            format!("{}{}", self.url_builder.base_url(), url)
        };

        let moocs_url = MoocsUrl::parse_moocs_url(&full_url)?;
        Ok(moocs_url.course_key().clone())
    }
}

#[async_trait]
impl CourseRepository for CourseRepositoryImpl {
    async fn fetch_course_list(&self, year: Option<Year>) -> Result<Vec<Course>> {
        let cache_key = year
            .as_ref()
            .map(|y| y.to_string())
            .unwrap_or_else(|| "latest".to_string());

        // Check cache first
        if let Some(cached_courses) = self.course_cache.get(&cache_key) {
            return Ok(cached_courses);
        }

        // Cache miss - fetch from API
        let html = self.fetch_course_page(year).await?;
        let courses = self.scrape_courses(&html)?;

        // Cache the result
        self.course_cache.insert(cache_key, courses.clone());

        Ok(courses)
    }

    async fn fetch_course(&self, course_key: &CourseKey) -> Result<Option<Course>> {
        let courses = self
            .fetch_course_list(Some(course_key.year.clone()))
            .await?;
        Ok(courses.into_iter().find(|course| course.key == *course_key))
    }
}
