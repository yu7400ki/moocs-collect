use sqlx::{Row, SqlitePool};
use tauri::State;

use crate::search::{
    types::{SearchOptions, SearchResult},
    SearchError,
};
use crate::state::SearchState;

#[derive(Debug, thiserror::Error)]
pub enum SearchSlidesError {
    #[error(transparent)]
    Search(#[from] SearchError),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl serde::Serialize for SearchSlidesError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SlideSearchEntry {
    pub search_result: SearchResult,
    pub year: u32,
    pub course_name: String,
    pub lecture_name: String,
    pub page_name: String,
    pub download_path: String,
}

#[tauri::command]
pub async fn search_slides(
    query: String,
    filters: Vec<String>,
    search_state: State<'_, SearchState>,
    db_pool: State<'_, SqlitePool>,
) -> Result<Vec<SlideSearchEntry>, SearchSlidesError> {
    let search_service = &search_state.0;

    let search_options = SearchOptions::default()
        .with_limit(50)
        .with_facet_filters(filters);

    let results = search_service
        .search_slides(&query, &search_options)
        .await?;

    enrich_results(&*db_pool, results).await
}

async fn enrich_results(
    pool: &SqlitePool,
    results: Vec<SearchResult>,
) -> Result<Vec<SlideSearchEntry>, SearchSlidesError> {
    let mut enriched = Vec::with_capacity(results.len());

    for result in results {
        let SearchResult {
            page_key: key,
            facet,
            content_snippet,
            highlighted_content,
            score,
        } = result;

        let (fallback_year, fallback_course, fallback_lecture, fallback_page, slide_idx) =
            split_facet_path(&facet);

        let row = sqlx::query(
            r#"
            SELECT
                courses.year AS course_year,
                courses.name AS course_name,
                lectures.name AS lecture_name,
                pages.name AS page_name,
                slides.pdf_path AS pdf_path
            FROM pages
            INNER JOIN lectures ON lectures.id = pages.lecture_id
            INNER JOIN courses ON courses.id = lectures.course_id
            LEFT JOIN slides ON slides.page_id = pages.id
            WHERE pages.key = ? AND slides.idx = ?
            "#,
        )
        .bind(&key)
        .bind(slide_idx)
        .fetch_one(pool)
        .await?;

        let mut year = fallback_year.unwrap_or_default();
        let mut course_name = fallback_course.unwrap_or("").to_string();
        let mut lecture_name = fallback_lecture.unwrap_or("").to_string();
        let mut page_name = fallback_page.unwrap_or("").to_string();

        if let Some(db_course) = row.try_get::<Option<String>, _>("course_name")? {
            if !db_course.is_empty() {
                course_name = db_course;
            }
        }
        if let Some(db_lecture) = row.try_get::<Option<String>, _>("lecture_name")? {
            if !db_lecture.is_empty() {
                lecture_name = db_lecture;
            }
        }
        if let Some(db_page) = row.try_get::<Option<String>, _>("page_name")? {
            if !db_page.is_empty() {
                page_name = db_page;
            }
        }
        if let Some(db_year) = row.try_get::<Option<i64>, _>("course_year")? {
            if db_year >= 0 {
                year = db_year as u32;
            }
        }

        let download_path = row
            .try_get::<Option<String>, _>("pdf_path")?
            .unwrap_or_default();

        enriched.push(SlideSearchEntry {
            search_result: SearchResult {
                page_key: key,
                facet,
                content_snippet,
                highlighted_content,
                score,
            },
            year,
            course_name,
            lecture_name,
            page_name,
            download_path,
        });
    }

    Ok(enriched)
}

fn split_facet_path(
    facet: &str,
) -> (
    Option<u32>,
    Option<&str>,
    Option<&str>,
    Option<&str>,
    Option<u32>,
) {
    let trimmed = facet.trim_start_matches('/');
    let mut parts = trimmed.split('/');

    let year = parts.next().and_then(|part| part.parse::<u32>().ok());
    let course = parts.next();
    let lecture = parts.next();
    let page = parts.next();
    let index = parts.next().and_then(|part| part.parse::<u32>().ok());

    (year, course, lecture, page, index)
}
