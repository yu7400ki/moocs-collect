use crate::domain::models::keys::PageKey;

/// Slide domain model
#[derive(Debug, Clone)]
pub struct Slide {
    pub url: String,
    pub page_key: PageKey,
    pub index: usize,
}

impl Slide {
    pub fn new(url: impl Into<String>, page_key: PageKey, index: usize) -> Self {
        Self {
            url: url.into(),
            page_key,
            index,
        }
    }

    pub fn builder() -> SlideBuilder {
        SlideBuilder::new()
    }
}

/// Slide builder
#[derive(Debug, Clone)]
pub struct SlideBuilder {
    url: Option<String>,
    page_key: Option<PageKey>,
    index: Option<usize>,
}

impl SlideBuilder {
    pub fn new() -> Self {
        Self {
            url: None,
            page_key: None,
            index: None,
        }
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn with_page_key(mut self, page_key: PageKey) -> Self {
        self.page_key = Some(page_key);
        self
    }

    pub fn with_index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    pub fn build(self) -> Option<Slide> {
        Some(Slide::new(self.url?, self.page_key?, self.index?))
    }
}

/// Processed SVG data
#[derive(Debug, Clone)]
pub struct ProcessedSvg {
    pub content: String,
    pub index: usize,
}

impl ProcessedSvg {
    pub fn new(content: impl Into<String>, index: usize) -> Self {
        Self {
            content: content.into(),
            index,
        }
    }
}

/// Slide content domain model
#[derive(Debug, Clone)]
pub struct SlideContent {
    pub page_key: PageKey,
    pub svgs: Vec<ProcessedSvg>,
}

impl SlideContent {
    pub fn new(page_key: PageKey, svgs: Vec<ProcessedSvg>) -> Self {
        Self { page_key, svgs }
    }

    pub fn is_empty(&self) -> bool {
        self.svgs.is_empty()
    }

    pub fn slide_count(&self) -> usize {
        self.svgs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::keys::{
        CourseKey, CourseSlug, LectureKey, LectureSlug, PageSlug, Year,
    };

    #[test]
    fn test_slide_content() {
        let year = Year::new(2023).unwrap();
        let course_slug = CourseSlug::new("course").unwrap();
        let lecture_slug = LectureSlug::new("lecture").unwrap();
        let page_slug = PageSlug::new("page").unwrap();

        let course_key = CourseKey::new(year, course_slug);
        let lecture_key = LectureKey::new(course_key, lecture_slug);
        let page_key = PageKey::new(lecture_key, page_slug);

        let svg = ProcessedSvg::new("content", 0);
        let content = SlideContent::new(page_key, vec![svg]);

        assert!(!content.is_empty());
        assert_eq!(content.slide_count(), 1);
    }
}
