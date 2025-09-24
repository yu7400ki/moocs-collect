use crate::error::{CollectError, Result};
use std::fmt;
use std::str::FromStr;

/// Year value object
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Year(u32);

impl Year {
    pub fn new(year: u32) -> Result<Self> {
        if year < 2017 {
            return Err(CollectError::parse(
                "Invalid year",
                Some("Year must be 2017 or later".to_string()),
            ));
        }
        Ok(Self(year))
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Year {
    type Err = CollectError;

    fn from_str(s: &str) -> Result<Self> {
        let year = s
            .parse::<u32>()
            .map_err(|_| CollectError::parse("Invalid year format", Some(s.to_string())))?;
        Self::new(year)
    }
}

/// Course slug value object
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CourseSlug(String);

impl CourseSlug {
    pub fn new(slug: impl Into<String>) -> Result<Self> {
        let slug = slug.into();
        if slug.is_empty() {
            return Err(CollectError::parse("Course slug cannot be empty", None));
        }
        // Allow any non-empty string as course slug
        Ok(Self(slug))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CourseSlug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CourseSlug {
    type Err = CollectError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

/// Lecture slug value object
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LectureSlug(String);

impl LectureSlug {
    pub fn new(slug: impl Into<String>) -> Result<Self> {
        let slug = slug.into();
        if slug.is_empty() {
            return Err(CollectError::parse("Lecture slug cannot be empty", None));
        }
        Ok(Self(slug))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LectureSlug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for LectureSlug {
    type Err = CollectError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

/// Page slug value object
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PageSlug(String);

impl PageSlug {
    pub fn new(slug: impl Into<String>) -> Result<Self> {
        let slug = slug.into();
        if slug.is_empty() {
            return Err(CollectError::parse("Page slug cannot be empty", None));
        }
        Ok(Self(slug))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PageSlug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PageSlug {
    type Err = CollectError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

/// Composite course key (year + slug ensures uniqueness)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CourseKey {
    pub year: Year,
    pub slug: CourseSlug,
}

impl CourseKey {
    pub fn new(year: Year, slug: CourseSlug) -> Self {
        Self { year, slug }
    }
}

impl fmt::Display for CourseKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.year, self.slug)
    }
}

/// Composite lecture key (course_key + slug ensures uniqueness)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LectureKey {
    pub course_key: CourseKey,
    pub slug: LectureSlug,
}

impl LectureKey {
    pub fn new(course_key: CourseKey, slug: LectureSlug) -> Self {
        Self { course_key, slug }
    }
}

impl fmt::Display for LectureKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.course_key, self.slug)
    }
}

/// Composite page key (lecture_key + slug ensures uniqueness)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PageKey {
    pub lecture_key: LectureKey,
    pub slug: PageSlug,
}

impl PageKey {
    pub fn new(lecture_key: LectureKey, slug: PageSlug) -> Self {
        Self { lecture_key, slug }
    }
}

impl fmt::Display for PageKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.lecture_key, self.slug)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year_validation() {
        assert!(Year::new(2023).is_ok());
        assert!(Year::new(1999).is_err());
    }

    #[test]
    fn test_course_slug_validation() {
        assert!(CourseSlug::new("valid-course_123").is_ok());
        assert!(CourseSlug::new("course:with:colons").is_ok());
        assert!(CourseSlug::new("invalid course!").is_ok()); // Now allows any non-empty string
        assert!(CourseSlug::new("").is_err());
    }

    #[test]
    fn test_key_display() {
        let year = Year::new(2023).unwrap();
        let course_slug = CourseSlug::new("test-course").unwrap();
        let course_key = CourseKey::new(year, course_slug);

        assert_eq!(course_key.to_string(), "2023/test-course");
    }

    #[test]
    fn test_composite_keys() {
        let year = Year::new(2023).unwrap();
        let course_slug = CourseSlug::new("course").unwrap();
        let lecture_slug = LectureSlug::new("lecture").unwrap();
        let page_slug = PageSlug::new("page").unwrap();

        let course_key = CourseKey::new(year, course_slug);
        let lecture_key = LectureKey::new(course_key, lecture_slug);
        let page_key = PageKey::new(lecture_key, page_slug);

        assert_eq!(page_key.to_string(), "2023/course/lecture/page");
    }

    #[test]
    fn test_slug_from_str() {
        let slug: CourseSlug = "test-course".parse().unwrap();
        assert_eq!(slug.value(), "test-course");

        let year: Year = "2023".parse().unwrap();
        assert_eq!(year.value(), 2023);
    }
}
