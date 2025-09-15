use tauri::State;

use crate::{
    search::{
        types::{SearchOptions, SearchResult},
        SearchError,
    },
    state::SearchState,
};

#[tauri::command]
pub async fn search_slides(
    query: String,
    year_filter: Option<u32>,
    courses_filter: Option<Vec<String>>,
    search_state: State<'_, SearchState>,
) -> Result<Vec<SearchResult>, SearchError> {
    let search_service = &search_state.0;

    let mut search_options = SearchOptions::default().with_limit(50);
    if let Some(year) = year_filter {
        search_options = search_options.with_year(year);
    }
    if let Some(courses) = courses_filter {
        search_options = search_options.with_courses(courses);
    }

    search_service.search_slides(&query, &search_options).await
}
