use crate::domain::models::keys::CourseKey;

/// Course domain model
#[derive(Debug, Clone)]
pub struct Course {
    pub key: CourseKey,
    pub name: String,
    pub index: usize,
}

impl Course {
    pub fn new(key: CourseKey, name: impl Into<String>, index: usize) -> Self {
        Self {
            key,
            name: name.into(),
            index,
        }
    }

    pub fn builder() -> CourseBuilder {
        CourseBuilder::new()
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

/// Course builder
#[derive(Debug, Clone)]
pub struct CourseBuilder {
    key: Option<CourseKey>,
    name: Option<String>,
    index: Option<usize>,
}

impl CourseBuilder {
    pub fn new() -> Self {
        Self {
            key: None,
            name: None,
            index: None,
        }
    }

    pub fn with_key(mut self, key: CourseKey) -> Self {
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

    pub fn build(self) -> Option<Course> {
        Some(Course::new(self.key?, self.name?, self.index?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::keys::{CourseSlug, Year};

    #[test]
    fn test_course_creation() {
        let year = Year::new(2023).unwrap();
        let slug = CourseSlug::new("test-course").unwrap();
        let key = CourseKey::new(year, slug);
        let course = Course::new(key.clone(), "Test Course", 0);

        assert_eq!(course.key, key);
        assert_eq!(course.name, "Test Course");
        assert_eq!(course.index, 0);
    }

    #[test]
    fn test_course_builder() {
        let year = Year::new(2023).unwrap();
        let slug = CourseSlug::new("test").unwrap();
        let key = CourseKey::new(year, slug);

        let course = CourseBuilder::new()
            .with_key(key.clone())
            .with_name("Test Course")
            .with_index(0)
            .build()
            .unwrap();

        assert_eq!(course.key, key);
        assert_eq!(course.name, "Test Course");
        assert_eq!(course.index, 0);
    }

    #[test]
    fn test_display_name_fallback() {
        let year = Year::new(2023).unwrap();
        let slug = CourseSlug::new("test-course").unwrap();
        let key = CourseKey::new(year, slug);

        // Test with non-empty name
        let course_with_name = Course::new(key.clone(), "Actual Course Name", 0);
        assert_eq!(course_with_name.display_name(), "Actual Course Name");

        // Test with empty name - should fallback to slug
        let course_empty_name = Course::new(key.clone(), "", 0);
        assert_eq!(course_empty_name.display_name(), "test-course");
    }
}
