use collect::moocs::Course;
use std::collections::HashMap;

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
pub struct CourseState(pub HashMap<(u32, String), Course>);
