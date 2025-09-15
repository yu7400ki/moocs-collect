use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HighlightedText {
    pub text: String,
    pub is_highlighted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub page_key: String,
    pub year: u64,
    pub course: String,
    pub lecture: String,
    pub page: String,
    pub content_snippet: String,
    pub highlighted_content: Vec<HighlightedText>,
    pub score: f32,
}

#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub year: Option<u32>,
    pub limit: usize,
    pub courses: Vec<String>,
}

impl SearchOptions {
    pub fn with_year(mut self, year: u32) -> Self {
        self.year = Some(year);
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_courses(mut self, courses: Vec<String>) -> Self {
        self.courses = courses;
        self
    }
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            year: None,
            limit: 50,
            courses: Vec::new(),
        }
    }
}
