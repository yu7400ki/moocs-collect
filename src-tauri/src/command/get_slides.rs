use std::sync::Mutex;

use crate::state::{ClientState, PageState};
use collect::moocs;
use tauri::State;

#[derive(serde::Serialize)]
pub struct SlideContent {
    pub content: Vec<String>,
}

impl From<moocs::SlideContent> for SlideContent {
    fn from(content: moocs::SlideContent) -> Self {
        Self {
            content: content.content.into_iter().map(|c| c.src).collect(),
        }
    }
}

#[tauri::command]
pub async fn get_slides(
    year: u32,
    course_id: String,
    lecture_id: String,
    page_id: String,
    client_state: State<'_, ClientState>,
    page_state: State<'_, Mutex<PageState>>,
) -> Result<Vec<SlideContent>, ()> {
    let client = &client_state.0;
    let page = {
        let page_state_guard = page_state.lock().map_err(|_| ())?;
        let page_state = &*page_state_guard;
        page_state
            .0
            .get(&(year, course_id.clone(), lecture_id.clone(), page_id.clone()))
            .cloned()
            .ok_or(())?
    };

    let slide = moocs::Slide::list(client, page).await.map_err(|_| ())?;

    let contents = futures::future::join_all(
        slide
            .into_iter()
            .map(|slide| async move { slide.content(client).await }),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .map_err(|_| ())?;

    Ok(contents.into_iter().map(SlideContent::from).collect())
}
