use crate::domain::models::{LectureKey, LecturePage, PageKey};
use crate::error::Result;
use async_trait::async_trait;

/// Page service trait for business logic operations
#[async_trait]
pub trait PageService: Send + Sync {
    /// Get pages for a specific lecture
    async fn get_pages(&self, lecture_key: &LectureKey) -> Result<Vec<LecturePage>>;

    /// Get a specific page by its key
    async fn get_page(&self, page_key: &PageKey) -> Result<LecturePage>;
}
