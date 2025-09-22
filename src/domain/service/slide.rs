use crate::{
    domain::models::{PageKey, Slide, SlideContent},
    error::Result,
};
use async_trait::async_trait;
use futures::future::try_join_all;

/// Slide service trait for business logic operations
#[async_trait]
pub trait SlideService: Send + Sync {
    /// Get slides for a specific page
    async fn get_slides(&self, page_key: &PageKey) -> Result<Vec<Slide>>;

    /// Get slide content (SVG data) for a specific slide
    async fn get_slide_content(&self, slide: &Slide) -> Result<SlideContent>;

    async fn get_slides_content(&self, slides: &[Slide]) -> Result<Vec<SlideContent>> {
        let futures = slides.iter().map(|slide| self.get_slide_content(slide));
        try_join_all(futures).await
    }
}
