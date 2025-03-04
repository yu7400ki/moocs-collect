use std::sync::{Arc, Mutex};

use crate::state::{ClientState, LectureState, PageState};
use collect::moocs;
use tauri::State;

#[derive(serde::Serialize)]
pub struct Page {
    pub year: u32,
    pub course_id: String,
    pub lecture_id: String,
    pub id: String,
    pub title: String,
}

impl From<moocs::LecturePage> for Page {
    fn from(page: moocs::LecturePage) -> Self {
        Self {
            year: page.lecture.course.year,
            course_id: page.lecture.course.id.clone(),
            lecture_id: page.lecture.id.clone(),
            id: page.id,
            title: page.title,
        }
    }
}

#[tauri::command]
pub async fn get_pages(
    year: u32,
    course_id: String,
    lecture_id: String,
    client_state: State<'_, ClientState>,
    lecture_state: State<'_, Mutex<LectureState>>,
    page_state: State<'_, Mutex<PageState>>,
) -> Result<Vec<Page>, ()> {
    let client = &client_state.0;
    let lecture = {
        let lecture_state_guard = lecture_state.lock().map_err(|_| ())?;
        let lecture_state = &*lecture_state_guard;
        lecture_state
            .0
            .get(&(year, course_id.clone(), lecture_id.clone()))
            .cloned()
            .ok_or(())?
    };
    let pages = moocs::LecturePage::list(client, lecture)
        .await
        .map_err(|_| ())?;

    {
        let mut page_state_guard = page_state.lock().map_err(|_| ())?;
        let page_state = &mut *page_state_guard;
        for page in &pages {
            page_state.0.insert(
                (year, course_id.clone(), lecture_id.clone(), page.id.clone()),
                Arc::new(page.clone()),
            );
        }
    }

    Ok(pages.into_iter().map(Page::from).collect())
}
