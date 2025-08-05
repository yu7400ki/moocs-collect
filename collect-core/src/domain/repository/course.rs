use crate::domain::models::{Course, CourseKey, Year};
use crate::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CourseRepository: Send + Sync {
    /// Fetch list of courses for a given year
    async fn fetch_course_list(&self, year: Option<Year>) -> Result<Vec<Course>>;

    /// Fetch course details by key
    async fn fetch_course(&self, course_key: &CourseKey) -> Result<Option<Course>>;
}
