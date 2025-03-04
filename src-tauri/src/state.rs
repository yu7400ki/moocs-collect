use collect::moocs::{Course, Lecture, LecturePage};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ClientState(pub reqwest::Client);

impl ClientState {
    pub fn new() -> reqwest::Result<Self> {
        let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0")
        .cookie_store(true)
        .build()?;
        Ok(Self(client))
    }
}

#[derive(Default)]
pub struct CourseState(pub HashMap<(u32, String), Arc<Course>>);

#[derive(Default)]
pub struct LectureState(pub HashMap<(u32, String, String), Arc<Lecture>>);

#[derive(Default)]
pub struct PageState(pub HashMap<(u32, String, String, String), Arc<LecturePage>>);
