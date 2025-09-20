use crate::domain::models::{CourseKey, Lecture, LectureGroup, LectureKey};
use crate::error::Result;
use async_trait::async_trait;

/// Lecture service trait for business logic operations
#[async_trait]
pub trait LectureService: Send + Sync {
    /// Get lecture groups for a specific course
    async fn get_lecture_groups(&self, course_key: &CourseKey) -> Result<Vec<LectureGroup>>;

    /// Get lectures for a specific course (flattened from groups)
    async fn get_lectures(&self, course_key: &CourseKey) -> Result<Vec<Lecture>>;

    /// Get a specific lecture by its key
    async fn get_lecture(&self, lecture_key: &LectureKey) -> Result<Lecture>;
}
