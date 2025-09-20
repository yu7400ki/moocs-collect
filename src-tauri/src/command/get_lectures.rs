use crate::state::CollectState;
use collect::{
    error::CollectError, CourseKey, CourseSlug, LectureGroup as DomainLectureGroup, Year,
};
use tauri::State;

#[derive(Debug, thiserror::Error)]
pub enum LectureError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Core library error: {0}")]
    Core(#[from] CollectError),
}

impl serde::Serialize for LectureError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Lecture {
    pub year: u32,
    pub course_slug: String,
    pub slug: String,
    pub name: String,
    pub index: usize,
}

#[derive(serde::Serialize)]
pub struct LectureGroup {
    pub year: u32,
    #[serde(rename = "courseSlug")]
    pub course_slug: String,
    pub name: String,
    pub lectures: Vec<Lecture>,
    pub index: usize,
}

impl From<&collect::Lecture> for Lecture {
    fn from(lecture: &collect::Lecture) -> Self {
        Self {
            year: lecture.key.course_key.year.value(),
            course_slug: lecture.key.course_key.slug.value().to_string(),
            slug: lecture.key.slug.value().to_string(),
            name: lecture.display_name().to_string(),
            index: lecture.index,
        }
    }
}

impl From<&DomainLectureGroup> for LectureGroup {
    fn from(group: &DomainLectureGroup) -> Self {
        Self {
            year: group.course_key.year.value(),
            course_slug: group.course_key.slug.value().to_string(),
            name: group.display_name().to_string(),
            lectures: group.lectures.iter().map(Lecture::from).collect(),
            index: group.index,
        }
    }
}

#[tauri::command]
pub async fn get_lectures(
    year: u32,
    course_slug: String,
    state: State<'_, CollectState>,
) -> Result<Vec<LectureGroup>, LectureError> {
    let collect = &state.collect;

    let year_obj = Year::new(year)
        .map_err(|e| LectureError::InvalidInput(format!("Invalid year {}: {}", year, e)))?;
    let course_slug_obj = CourseSlug::new(course_slug.clone()).map_err(|e| {
        LectureError::InvalidInput(format!("Invalid course slug '{}': {}", course_slug, e))
    })?;
    let course_key = CourseKey::new(year_obj, course_slug_obj);

    let lecture_groups = collect.get_lecture_groups(&course_key).await?;

    Ok(lecture_groups.iter().map(LectureGroup::from).collect())
}
