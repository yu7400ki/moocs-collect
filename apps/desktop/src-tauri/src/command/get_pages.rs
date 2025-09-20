use crate::state::CollectState;
use collect::{
    error::CollectError, CourseKey, CourseSlug, LectureKey, LecturePage as DomainPage, LectureSlug,
    Year,
};
use tauri::State;

#[derive(Debug, thiserror::Error)]
pub enum PageError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Core library error: {0}")]
    Core(#[from] CollectError),
}

impl serde::Serialize for PageError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub year: u32,
    pub course_slug: String,
    pub lecture_slug: String,
    pub slug: String,
    pub name: String,
}

impl From<DomainPage> for Page {
    fn from(page: DomainPage) -> Self {
        Self {
            year: page.key.lecture_key.course_key.year.value(),
            course_slug: page.key.lecture_key.course_key.slug.value().to_string(),
            lecture_slug: page.key.lecture_key.slug.value().to_string(),
            slug: page.key.slug.value().to_string(),
            name: page.display_name().to_string(),
        }
    }
}

#[tauri::command]
pub async fn get_pages(
    year: u32,
    course_slug: String,
    lecture_slug: String,
    state: State<'_, CollectState>,
) -> Result<Vec<Page>, PageError> {
    let collect = &state.collect;

    let year_obj = Year::new(year)
        .map_err(|e| PageError::InvalidInput(format!("Invalid year {}: {}", year, e)))?;
    let course_slug_obj = CourseSlug::new(course_slug.clone()).map_err(|e| {
        PageError::InvalidInput(format!("Invalid course slug '{}': {}", course_slug, e))
    })?;
    let lecture_slug_obj = LectureSlug::new(lecture_slug.clone()).map_err(|e| {
        PageError::InvalidInput(format!("Invalid lecture slug '{}': {}", lecture_slug, e))
    })?;

    let course_key = CourseKey::new(year_obj, course_slug_obj);
    let lecture_key = LectureKey::new(course_key, lecture_slug_obj);

    let pages = collect.get_pages(&lecture_key).await?;

    Ok(pages.into_iter().map(Page::from).collect())
}
