use crate::domain::models::{Course, CourseKey, Year};
use crate::error::Result;
use async_trait::async_trait;

/// Course service trait for business logic operations
#[async_trait]
pub trait CourseService: Send + Sync {
    /// Get list of courses for a specific year
    async fn get_courses(&self, year: Option<Year>) -> Result<Vec<Course>>;

    /// Get a specific course by its key
    async fn get_course(&self, course_key: &CourseKey) -> Result<Course>;

    /// Get list of available archive years
    async fn get_archive_years(&self) -> Result<Vec<Year>>;
}
