use crate::domain::models::{LectureKey, LecturePage, PageKey};
use crate::error::Result;
use async_trait::async_trait;

/// Repository trait for page data access
#[async_trait]
pub trait PageRepository: Send + Sync {
    /// Fetch pages for a given lecture
    async fn fetch_pages(&self, lecture_key: &LectureKey) -> Result<Vec<LecturePage>>;

    /// Fetch a specific page by key
    async fn fetch_page(&self, page_key: &PageKey) -> Result<Option<LecturePage>>;
}
