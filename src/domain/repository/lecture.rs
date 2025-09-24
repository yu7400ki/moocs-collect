use crate::domain::models::{CourseKey, Lecture, LectureGroup, LectureKey};
use crate::error::Result;
use async_trait::async_trait;

/// Repository trait for lecture data access
#[async_trait]
pub trait LectureRepository: Send + Sync {
    /// Fetch lecture groups for a given course
    async fn fetch_lecture_groups(&self, course_key: &CourseKey) -> Result<Vec<LectureGroup>>;

    /// Fetch a specific lecture by key
    async fn fetch_lecture(&self, lecture_key: &LectureKey) -> Result<Option<Lecture>>;
}
