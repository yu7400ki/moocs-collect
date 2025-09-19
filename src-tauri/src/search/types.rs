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
    pub facet: String,
    pub content_snippet: String,
    pub highlighted_content: Vec<HighlightedText>,
    pub score: f32,
}

#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub limit: usize,
}

impl SearchOptions {
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self { limit: 50 }
    }
}
