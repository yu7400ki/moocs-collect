use crate::domain::{
    models::{CourseKey, Lecture, LectureGroup, LectureKey},
    repository::{AuthenticationRepository, LectureRepository},
    service::LectureService,
};
use crate::error::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct LectureServiceImpl {
    lecture_repository: Arc<dyn LectureRepository>,
    auth_repository: Arc<dyn AuthenticationRepository>,
}

impl LectureServiceImpl {
    pub fn new(
        lecture_repository: Arc<dyn LectureRepository>,
        auth_repository: Arc<dyn AuthenticationRepository>,
    ) -> Self {
        Self {
            lecture_repository,
            auth_repository,
        }
    }
}

#[async_trait]
impl LectureService for LectureServiceImpl {
    async fn get_lecture_groups(&self, course_key: &CourseKey) -> Result<Vec<LectureGroup>> {
        // Check authentication before fetching lectures
        if !self.auth_repository.is_logged_in_moocs().await? {
            return Err(crate::error::CollectError::authentication(
                "Not logged into MOOCs system. Please authenticate first.",
            ));
        }

        self.lecture_repository
            .fetch_lecture_groups(course_key)
            .await
    }

    async fn get_lectures(&self, course_key: &CourseKey) -> Result<Vec<Lecture>> {
        // Check authentication before fetching lectures
        if !self.auth_repository.is_logged_in_moocs().await? {
            return Err(crate::error::CollectError::authentication(
                "Not logged into MOOCs system. Please authenticate first.",
            ));
        }

        Ok(self
            .lecture_repository
            .fetch_lecture_groups(course_key)
            .await?
            .into_iter()
            .flat_map(|group| group.lectures)
            .collect())
    }

    async fn get_lecture(&self, lecture_key: &LectureKey) -> Result<Lecture> {
        // Get all lectures for the course and find the one matching the key
        let lectures = self.get_lectures(&lecture_key.course_key).await?;
        lectures
            .into_iter()
            .find(|lecture| lecture.key == *lecture_key)
            .ok_or_else(|| {
                crate::error::CollectError::not_found(format!("Lecture not found: {}", lecture_key))
            })
    }
}
