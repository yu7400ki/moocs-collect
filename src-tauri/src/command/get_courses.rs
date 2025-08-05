use crate::state::CollectState;
use collect::{
    domain::models::{Course as DomainCourse, Year},
    error::CollectError,
};
use tauri::State;

#[derive(Debug, thiserror::Error)]
pub enum CourseError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Core library error: {0}")]
    Core(#[from] CollectError),
}

impl serde::Serialize for CourseError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(serde::Serialize)]
pub struct Course {
    pub year: u32,
    pub slug: String,
    pub name: String,
}

impl From<DomainCourse> for Course {
    fn from(course: DomainCourse) -> Self {
        Self {
            year: course.key.year.value(),
            slug: course.key.slug.value().to_string(),
            name: course.display_name().to_string(),
        }
    }
}

#[tauri::command]
pub async fn get_courses(
    year: Option<u32>,
    state: State<'_, CollectState>,
) -> Result<Vec<Course>, CourseError> {
    let collect = &state.collect;

    let year_param = year
        .map(|y| {
            Year::new(y)
                .map_err(|e| CourseError::InvalidInput(format!("Invalid year {}: {}", y, e)))
        })
        .transpose()?;

    let courses = collect.get_courses(year_param).await?;

    Ok(courses.into_iter().map(Course::from).collect())
}
