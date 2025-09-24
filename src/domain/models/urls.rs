use super::keys::{CourseKey, CourseSlug, LectureKey, LectureSlug, PageKey, PageSlug, Year};
use crate::error::{CollectError, Result};
use regex::Regex;
use std::str::FromStr;

/// MOOCs URL types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoocsUrl {
    Course { course_key: CourseKey },
    Lecture { lecture_key: LectureKey },
    Page { page_key: PageKey },
}

impl MoocsUrl {
    pub fn parse_moocs_url(url: &str) -> Result<Self> {
        parse_moocs_url(url)
    }

    /// Create a course URL
    pub fn course_url(year: Year, course_slug: CourseSlug) -> Self {
        Self::Course {
            course_key: CourseKey::new(year, course_slug),
        }
    }

    /// Create a lecture URL
    pub fn lecture_url(year: Year, course_slug: CourseSlug, lecture_slug: LectureSlug) -> Self {
        Self::Lecture {
            lecture_key: LectureKey::new(CourseKey::new(year, course_slug), lecture_slug),
        }
    }

    /// Create a page URL
    pub fn page_url(
        year: Year,
        course_slug: CourseSlug,
        lecture_slug: LectureSlug,
        page_slug: PageSlug,
    ) -> Self {
        Self::Page {
            page_key: PageKey::new(
                LectureKey::new(CourseKey::new(year, course_slug), lecture_slug),
                page_slug,
            ),
        }
    }

    /// Get the course key from any URL type
    pub fn course_key(&self) -> &CourseKey {
        match self {
            Self::Course { course_key } => course_key,
            Self::Lecture { lecture_key } => &lecture_key.course_key,
            Self::Page { page_key } => &page_key.lecture_key.course_key,
        }
    }

    /// Get the lecture key if this is a lecture or page URL
    pub fn lecture_key(&self) -> Option<&LectureKey> {
        match self {
            Self::Course { .. } => None,
            Self::Lecture { lecture_key } => Some(lecture_key),
            Self::Page { page_key } => Some(&page_key.lecture_key),
        }
    }

    /// Get the page key if this is a page URL
    pub fn page_key(&self) -> Option<&PageKey> {
        match self {
            Self::Course { .. } | Self::Lecture { .. } => None,
            Self::Page { page_key } => Some(page_key),
        }
    }

    /// Convert to full URL string
    pub fn to_url_string(&self, base_url: &str) -> String {
        let base = base_url.trim_end_matches('/');
        match self {
            Self::Course { course_key } => {
                format!("{}/courses/{}/{}", base, course_key.year, course_key.slug)
            }
            Self::Lecture { lecture_key } => {
                format!(
                    "{}/courses/{}/{}/{}",
                    base,
                    lecture_key.course_key.year,
                    lecture_key.course_key.slug,
                    lecture_key.slug
                )
            }
            Self::Page { page_key } => {
                format!(
                    "{}/courses/{}/{}/{}/{}",
                    base,
                    page_key.lecture_key.course_key.year,
                    page_key.lecture_key.course_key.slug,
                    page_key.lecture_key.slug,
                    page_key.slug
                )
            }
        }
    }
}

impl FromStr for MoocsUrl {
    type Err = CollectError;

    fn from_str(url: &str) -> Result<Self> {
        parse_moocs_url(url)
    }
}

/// Parse a MOOCs URL string into a MoocsUrl enum
fn parse_moocs_url(url: &str) -> Result<MoocsUrl> {
    // Remove base URL and normalize
    let path = extract_path_from_url(url)?;

    // Define regex patterns for different URL types
    let course_pattern = Regex::new(r"^/?courses/(\d{4})/([^/]+)/?$")
        .map_err(|e| CollectError::parse("Invalid course URL pattern", Some(e.to_string())))?;

    let lecture_pattern = Regex::new(r"^/?courses/(\d{4})/([^/]+)/([^/]+)/?$")
        .map_err(|e| CollectError::parse("Invalid lecture URL pattern", Some(e.to_string())))?;

    let page_pattern = Regex::new(r"^/?courses/(\d{4})/([^/]+)/([^/]+)/([^/]+)/?$")
        .map_err(|e| CollectError::parse("Invalid page URL pattern", Some(e.to_string())))?;

    // Try to match page URL first (most specific)
    if let Some(captures) = page_pattern.captures(&path) {
        let year = Year::from_str(&captures[1])?;
        let course_slug = CourseSlug::from_str(&captures[2])?;
        let lecture_slug = LectureSlug::from_str(&captures[3])?;
        let page_slug = PageSlug::from_str(&captures[4])?;

        return Ok(MoocsUrl::page_url(
            year,
            course_slug,
            lecture_slug,
            page_slug,
        ));
    }

    // Try to match lecture URL
    if let Some(captures) = lecture_pattern.captures(&path) {
        let year = Year::from_str(&captures[1])?;
        let course_slug = CourseSlug::from_str(&captures[2])?;
        let lecture_slug = LectureSlug::from_str(&captures[3])?;

        return Ok(MoocsUrl::lecture_url(year, course_slug, lecture_slug));
    }

    // Try to match course URL
    if let Some(captures) = course_pattern.captures(&path) {
        let year = Year::from_str(&captures[1])?;
        let course_slug = CourseSlug::from_str(&captures[2])?;

        return Ok(MoocsUrl::course_url(year, course_slug));
    }

    Err(CollectError::parse(
        "Invalid MOOCs URL format",
        Some(url.to_string()),
    ))
}

/// Extract path component from URL
fn extract_path_from_url(url: &str) -> Result<String> {
    if url.starts_with("http://") || url.starts_with("https://") {
        let url_parts: Vec<&str> = url.splitn(4, '/').collect();
        if url_parts.len() >= 4 {
            Ok(format!("/{}", url_parts[3]))
        } else {
            Ok("/".to_string())
        }
    } else {
        Ok(url.to_string())
    }
}

/// URL builder for constructing MOOCs URLs
pub struct UrlBuilder {
    base_url: String,
}

impl UrlBuilder {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn courses_url(&self, year: Option<Year>) -> String {
        match year {
            Some(y) => format!("{}/courses/{}", self.base_url, y.value()),
            None => format!("{}/courses", self.base_url),
        }
    }

    pub fn course_url(&self, year: Year, course_slug: CourseSlug) -> String {
        let url = MoocsUrl::course_url(year, course_slug);
        url.to_url_string(&self.base_url)
    }

    pub fn lecture_url(
        &self,
        year: Year,
        course_slug: CourseSlug,
        lecture_slug: LectureSlug,
    ) -> String {
        let url = MoocsUrl::lecture_url(year, course_slug, lecture_slug);
        url.to_url_string(&self.base_url)
    }

    pub fn page_url(
        &self,
        year: Year,
        course_slug: CourseSlug,
        lecture_slug: LectureSlug,
        page_slug: PageSlug,
    ) -> String {
        let url = MoocsUrl::page_url(year, course_slug, lecture_slug, page_slug);
        url.to_url_string(&self.base_url)
    }
}

impl Default for UrlBuilder {
    fn default() -> Self {
        Self::new("https://moocs.iniad.org")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moocs_url_creation() {
        let year = Year::new(2023).unwrap();
        let course_slug = CourseSlug::new("test-course").unwrap();
        let lecture_slug = LectureSlug::new("test-lecture").unwrap();
        let page_slug = PageSlug::new("test-page").unwrap();

        let course_url = MoocsUrl::course_url(year.clone(), course_slug.clone());
        let lecture_url =
            MoocsUrl::lecture_url(year.clone(), course_slug.clone(), lecture_slug.clone());
        let page_url = MoocsUrl::page_url(
            year.clone(),
            course_slug.clone(),
            lecture_slug.clone(),
            page_slug.clone(),
        );

        assert!(matches!(course_url, MoocsUrl::Course { .. }));
        assert!(matches!(lecture_url, MoocsUrl::Lecture { .. }));
        assert!(matches!(page_url, MoocsUrl::Page { .. }));
    }

    #[test]
    fn test_url_parsing_with_special_characters() {
        // Test the problematic URL from the error
        let url1 = "https://moocs.iniad.org/courses/2024/COS209/03-05:";
        let parsed1 = parse_moocs_url(url1);
        assert!(
            parsed1.is_ok(),
            "Failed to parse URL with colon: {:?}",
            parsed1.err()
        );

        if let Ok(moocs_url) = parsed1 {
            assert!(matches!(moocs_url, MoocsUrl::Lecture { .. }));
        }

        // Test URLs with various special characters
        let test_cases = vec![
            "https://moocs.iniad.org/courses/2024/course!@#/lecture%20name",
            "https://moocs.iniad.org/courses/2024/course_with_underscores/lecture-with-dashes",
            "https://moocs.iniad.org/courses/2024/course123/lecture456/page789",
        ];

        for url in test_cases {
            let parsed = parse_moocs_url(url);
            assert!(
                parsed.is_ok(),
                "Failed to parse URL: {} - {:?}",
                url,
                parsed.err()
            );
        }
    }

    #[test]
    fn test_url_string_conversion() {
        let year = Year::new(2023).unwrap();
        let course_slug = CourseSlug::new("test-course").unwrap();
        let base_url = "https://moocs.iniad.org";

        let course_url = MoocsUrl::course_url(year, course_slug);
        let url_string = course_url.to_url_string(base_url);

        assert_eq!(
            url_string,
            "https://moocs.iniad.org/courses/2023/test-course"
        );
    }

    #[test]
    fn test_url_parsing() {
        let url = "https://moocs.iniad.org/courses/2023/test-course";
        let parsed = parse_moocs_url(url).unwrap();

        if let MoocsUrl::Course { course_key } = parsed {
            assert_eq!(course_key.year.value(), 2023);
            assert_eq!(course_key.slug.value(), "test-course");
        } else {
            panic!("Expected Course URL");
        }
    }

    #[test]
    fn test_lecture_url_parsing() {
        let url = "https://moocs.iniad.org/courses/2023/test-course/test-lecture";
        let parsed = parse_moocs_url(url).unwrap();

        if let MoocsUrl::Lecture { lecture_key } = parsed {
            assert_eq!(lecture_key.course_key.year.value(), 2023);
            assert_eq!(lecture_key.course_key.slug.value(), "test-course");
            assert_eq!(lecture_key.slug.value(), "test-lecture");
        } else {
            panic!("Expected Lecture URL");
        }
    }

    #[test]
    fn test_page_url_parsing() {
        let url = "https://moocs.iniad.org/courses/2023/test-course/test-lecture/test-page";
        let parsed = parse_moocs_url(url).unwrap();

        if let MoocsUrl::Page { page_key } = parsed {
            assert_eq!(page_key.lecture_key.course_key.year.value(), 2023);
            assert_eq!(page_key.lecture_key.course_key.slug.value(), "test-course");
            assert_eq!(page_key.lecture_key.slug.value(), "test-lecture");
            assert_eq!(page_key.slug.value(), "test-page");
        } else {
            panic!("Expected Page URL");
        }
    }

    #[test]
    fn test_invalid_url_parsing() {
        let invalid_urls = [
            "https://moocs.iniad.org/invalid",
            "https://moocs.iniad.org/courses/invalid/test",
            "https://moocs.iniad.org/courses/2023",
        ];

        for url in &invalid_urls {
            assert!(parse_moocs_url(url).is_err());
        }
    }

    #[test]
    fn test_url_builder() {
        let builder = UrlBuilder::new("https://moocs.iniad.org");
        let year = Year::new(2023).unwrap();
        let course_slug = CourseSlug::new("test").unwrap();

        let url = builder.course_url(year, course_slug);
        assert_eq!(url, "https://moocs.iniad.org/courses/2023/test");
    }

    #[test]
    fn test_extract_path_from_url() {
        assert_eq!(
            extract_path_from_url("https://moocs.iniad.org/courses/2023/test").unwrap(),
            "/courses/2023/test"
        );
        assert_eq!(
            extract_path_from_url("/courses/2023/test").unwrap(),
            "/courses/2023/test"
        );
    }
}
