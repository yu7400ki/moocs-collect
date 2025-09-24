use crate::domain::{
    models::{Course, CourseKey, Year},
    repository::{AuthenticationRepository, CourseRepository},
    service::CourseService,
};
use crate::error::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct CourseServiceImpl {
    course_repository: Arc<dyn CourseRepository>,
    auth_repository: Arc<dyn AuthenticationRepository>,
}

impl CourseServiceImpl {
    pub fn new(
        course_repository: Arc<dyn CourseRepository>,
        auth_repository: Arc<dyn AuthenticationRepository>,
    ) -> Self {
        Self {
            course_repository,
            auth_repository,
        }
    }
}

#[async_trait]
impl CourseService for CourseServiceImpl {
    async fn get_courses(&self, year: Option<Year>) -> Result<Vec<Course>> {
        // Check authentication before fetching courses
        if !self.auth_repository.is_logged_in_moocs().await? {
            return Err(crate::error::CollectError::authentication(
                "Not logged into MOOCs system. Please authenticate first.",
            ));
        }

        self.course_repository.fetch_course_list(year).await
    }

    async fn get_course(&self, course_key: &CourseKey) -> Result<Course> {
        // Get all courses and find the one matching the key
        let courses = self.get_courses(Some(course_key.year.clone())).await?;
        courses
            .into_iter()
            .find(|course| course.key == *course_key)
            .ok_or_else(|| {
                crate::error::CollectError::not_found(format!("Course not found: {course_key}"))
            })
    }

    async fn get_archive_years(&self) -> Result<Vec<Year>> {
        // Check authentication before fetching archive years
        if !self.auth_repository.is_logged_in_moocs().await? {
            return Err(crate::error::CollectError::authentication(
                "Not logged into MOOCs system. Please authenticate first.",
            ));
        }

        self.course_repository.fetch_archive_years().await
    }
}
