use crate::domain::models::{PageKey, Slide, SlideContent};
use crate::error::Result;
use async_trait::async_trait;

/// Repository trait for slide data access
#[async_trait]
pub trait SlideRepository: Send + Sync {
    /// Fetch slides for a given page
    async fn fetch_slides(&self, page_key: &PageKey) -> Result<Vec<Slide>>;

    /// Fetch slide content (SVGs) for a given slide
    async fn fetch_slide_content(&self, slide: &Slide) -> Result<SlideContent>;
}
