mod utils;

pub mod cache;
pub mod domain;
pub mod error;
pub mod pdf;
pub mod repository;
pub mod service;

use crate::domain::{
    models::{
        Course, CourseKey, Credentials, Lecture, LectureGroup, LectureKey, LecturePage, PageKey,
        Slide, SlideContent, Year,
    },
    service::{AuthenticationService, CourseService, LectureService, PageService, SlideService},
};
use crate::error::Result;
use crate::repository::{
    auth::AuthenticationRepositoryImpl, course::CourseRepositoryImpl,
    lecture::LectureRepositoryImpl, page::PageRepositoryImpl, slide::SlideRepositoryImpl,
};
use crate::service::{
    AuthenticationServiceImpl, CourseServiceImpl, LectureServiceImpl, PageServiceImpl,
    SlideServiceImpl,
};
use reqwest::Client;
use std::sync::Arc;

pub struct Collect {
    course_service: Arc<dyn CourseService>,
    lecture_service: Arc<dyn LectureService>,
    page_service: Arc<dyn PageService>,
    slide_service: Arc<dyn SlideService>,
    auth_service: Arc<dyn AuthenticationService>,
}

impl Collect {
    pub fn new(http_client: Client) -> Self {
        let client = Arc::new(http_client);

        // Create repository instances
        let auth_repository = Arc::new(AuthenticationRepositoryImpl::new(client.clone()));
        let course_repository = Arc::new(CourseRepositoryImpl::new(client.clone()));
        let lecture_repository = Arc::new(LectureRepositoryImpl::new(client.clone()));
        let page_repository = Arc::new(PageRepositoryImpl::new(client.clone()));
        let slide_repository = Arc::new(SlideRepositoryImpl::new(client.clone()));

        // Create service instances
        let auth_service = Arc::new(AuthenticationServiceImpl::new(auth_repository.clone()));
        let course_service = Arc::new(CourseServiceImpl::new(
            course_repository,
            auth_repository.clone(),
        ));
        let lecture_service = Arc::new(LectureServiceImpl::new(
            lecture_repository,
            auth_repository.clone(),
        ));
        let page_service = Arc::new(PageServiceImpl::new(
            page_repository,
            auth_repository.clone(),
        ));
        let slide_service = Arc::new(SlideServiceImpl::new(slide_repository, auth_repository));
        Collect {
            course_service,
            lecture_service,
            page_service,
            slide_service,
            auth_service,
        }
    }

    // Authentication methods
    pub async fn login_moocs(&self, credentials: &Credentials) -> Result<()> {
        self.auth_service.login_moocs(credentials).await
    }

    pub async fn login_google(&self, credentials: &Credentials) -> Result<()> {
        self.auth_service.login_google(credentials).await
    }

    pub async fn authenticate(&self, credentials: &Credentials) -> Result<()> {
        self.auth_service.login_moocs(credentials).await?;
        self.auth_service.login_google(credentials).await?;
        Ok(())
    }

    pub async fn is_authenticated(&self) -> Result<AuthStatus> {
        let moocs_authenticated = self.auth_service.is_logged_in_moocs().await?;
        let google_authenticated = self.auth_service.is_logged_in_google().await?;

        Ok(AuthStatus {
            moocs_authenticated,
            google_authenticated,
        })
    }

    // Course operations
    pub async fn get_courses(&self, year: Option<Year>) -> Result<Vec<Course>> {
        self.course_service.get_courses(year).await
    }

    pub async fn get_archive_years(&self) -> Result<Vec<Year>> {
        self.course_service.get_archive_years().await
    }

    // Lecture operations
    pub async fn get_lecture_groups(&self, course_key: &CourseKey) -> Result<Vec<LectureGroup>> {
        self.lecture_service.get_lecture_groups(course_key).await
    }

    pub async fn get_lectures(&self, course_key: &CourseKey) -> Result<Vec<Lecture>> {
        self.lecture_service.get_lectures(course_key).await
    }

    // Page operations
    pub async fn get_pages(&self, lecture_key: &LectureKey) -> Result<Vec<LecturePage>> {
        self.page_service.get_pages(lecture_key).await
    }

    // Slide operations
    pub async fn get_slides(&self, page_key: &PageKey) -> Result<Vec<Slide>> {
        self.slide_service.get_slides(page_key).await
    }

    pub async fn get_slide_content(&self, slide: &Slide) -> Result<SlideContent> {
        self.slide_service.get_slide_content(slide).await
    }

    // Helper methods for CLI
    pub async fn get_page_info(&self, page_key: &PageKey) -> Result<LecturePage> {
        self.page_service.get_page(page_key).await
    }

    pub async fn get_lecture_info(&self, lecture_key: &LectureKey) -> Result<Lecture> {
        self.lecture_service.get_lecture(lecture_key).await
    }

    pub async fn get_course_info(&self, course_key: &CourseKey) -> Result<Course> {
        self.course_service.get_course(course_key).await
    }
}

impl Default for Collect {
    fn default() -> Self {
        Collect::new(Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0")
        .cookie_store(true)
        .build().expect("Failed to create HTTP client"))
    }
}

pub struct AuthStatus {
    pub moocs_authenticated: bool,
    pub google_authenticated: bool,
}
