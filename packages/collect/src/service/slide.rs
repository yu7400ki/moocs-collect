use crate::domain::{
    models::{PageKey, Slide, SlideContent},
    repository::{AuthenticationRepository, SlideRepository},
    service::SlideService,
};
use crate::error::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct SlideServiceImpl {
    slide_repository: Arc<dyn SlideRepository>,
    auth_repository: Arc<dyn AuthenticationRepository>,
}

impl SlideServiceImpl {
    pub fn new(
        slide_repository: Arc<dyn SlideRepository>,
        auth_repository: Arc<dyn AuthenticationRepository>,
    ) -> Self {
        Self {
            slide_repository,
            auth_repository,
        }
    }
}

#[async_trait]
impl SlideService for SlideServiceImpl {
    async fn get_slides(&self, page_key: &PageKey) -> Result<Vec<Slide>> {
        // Check MOOCs authentication before fetching slides
        if !self.auth_repository.is_logged_in_moocs().await? {
            return Err(crate::error::CollectError::authentication(
                "Not logged into MOOCs system. Please authenticate first.",
            ));
        }

        self.slide_repository.fetch_slides(page_key).await
    }

    async fn get_slide_content(&self, slide: &Slide) -> Result<SlideContent> {
        // Check both MOOCs and Google authentication for slide content
        if !self.auth_repository.is_logged_in_moocs().await? {
            return Err(crate::error::CollectError::authentication(
                "Not logged into MOOCs system. Please authenticate first.",
            ));
        }

        if !self.auth_repository.is_logged_in_google().await? {
            return Err(crate::error::CollectError::authentication(
                "Not logged into Google system. Google authentication is required to access slide content.",
            ));
        }

        self.slide_repository.fetch_slide_content(slide).await
    }
}
