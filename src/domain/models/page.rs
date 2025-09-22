use super::keys::PageKey;

/// Lecture page domain model
#[derive(Debug, Clone)]
pub struct LecturePage {
    pub key: PageKey,
    pub name: String,
    pub index: usize,
}

impl LecturePage {
    pub fn new(key: PageKey, name: impl Into<String>, index: usize) -> Self {
        Self {
            key,
            name: name.into(),
            index,
        }
    }

    pub fn builder() -> LecturePageBuilder {
        LecturePageBuilder::new()
    }

    /// Get display name, falling back to slug if name is empty
    pub fn display_name(&self) -> &str {
        if self.name.is_empty() {
            self.key.slug.value()
        } else {
            &self.name
        }
    }
}

/// Lecture page builder
#[derive(Debug, Clone)]
pub struct LecturePageBuilder {
    key: Option<PageKey>,
    name: Option<String>,
    index: Option<usize>,
}

impl Default for LecturePageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LecturePageBuilder {
    pub fn new() -> Self {
        Self {
            key: None,
            name: None,
            index: None,
        }
    }

    pub fn with_key(mut self, key: PageKey) -> Self {
        self.key = Some(key);
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    pub fn build(self) -> Option<LecturePage> {
        Some(LecturePage::new(self.key?, self.name?, self.index?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::keys::{
        CourseKey, CourseSlug, LectureKey, LectureSlug, PageSlug, Year,
    };

    #[test]
    fn test_lecture_page_creation() {
        let lecture_key = LectureKey::new(
            CourseKey::new(
                Year::new(2023).unwrap(),
                CourseSlug::new("course-slug").unwrap(),
            ),
            LectureSlug::new("lecture-slug").unwrap(),
        );
        let page_key = PageKey::new(lecture_key, PageSlug::new("page-slug").unwrap());
        let page = LecturePage::new(page_key, "Test Page", 1);

        assert_eq!(page.name, "Test Page");
        assert_eq!(page.index, 1);
    }

    #[test]
    fn test_page_display_name_fallback() {
        let lecture_key = LectureKey::new(
            CourseKey::new(
                Year::new(2023).unwrap(),
                CourseSlug::new("course-slug").unwrap(),
            ),
            LectureSlug::new("lecture-slug").unwrap(),
        );
        let page_key = PageKey::new(lecture_key, PageSlug::new("page-slug").unwrap());

        // Test with non-empty name
        let page_with_name = LecturePage::new(page_key.clone(), "Actual Page Name", 1);
        assert_eq!(page_with_name.display_name(), "Actual Page Name");

        // Test with empty name - should fallback to slug
        let page_empty_name = LecturePage::new(page_key, "", 1);
        assert_eq!(page_empty_name.display_name(), "page-slug");
    }
}
