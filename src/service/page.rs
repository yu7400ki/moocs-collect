use crate::domain::{
    models::{LectureKey, LecturePage, PageKey},
    repository::{AuthenticationRepository, PageRepository},
    service::PageService,
};
use crate::error::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct PageServiceImpl {
    page_repository: Arc<dyn PageRepository>,
    auth_repository: Arc<dyn AuthenticationRepository>,
}

impl PageServiceImpl {
    pub fn new(
        page_repository: Arc<dyn PageRepository>,
        auth_repository: Arc<dyn AuthenticationRepository>,
    ) -> Self {
        Self {
            page_repository,
            auth_repository,
        }
    }
}

#[async_trait]
impl PageService for PageServiceImpl {
    async fn get_pages(&self, lecture_key: &LectureKey) -> Result<Vec<LecturePage>> {
        // Check authentication before fetching pages
        if !self.auth_repository.is_logged_in_moocs().await? {
            return Err(crate::error::CollectError::authentication(
                "Not logged into MOOCs system. Please authenticate first.",
            ));
        }

        self.page_repository.fetch_pages(lecture_key).await
    }

    async fn get_page(&self, page_key: &PageKey) -> Result<LecturePage> {
        // Get all pages for the lecture and find the one matching the key
        let pages = self.get_pages(&page_key.lecture_key).await?;
        pages
            .into_iter()
            .find(|page| page.key == *page_key)
            .ok_or_else(|| {
                crate::error::CollectError::not_found(format!("Page not found: {page_key}"))
            })
    }
}
