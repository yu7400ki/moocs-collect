use sqlx::Row;
use tauri::State;

use crate::state::DbState;

#[derive(Debug, thiserror::Error)]
pub enum RecordedCoursesError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl serde::Serialize for RecordedCoursesError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordedCourse {
    pub year: u32,
    pub slug: String,
    pub name: String,
    pub sort_index: i64,
}

#[tauri::command]
pub async fn get_recorded_courses(
    db_state: State<'_, DbState>,
) -> Result<Vec<RecordedCourse>, RecordedCoursesError> {
    let db_pool = db_state.0.read().await;
    let rows = sqlx::query(
        "SELECT year, slug, name, sort_index FROM courses ORDER BY year DESC, sort_index ASC",
    )
    .fetch_all(&*db_pool)
    .await?;

    let courses = rows
        .into_iter()
        .map(|row| RecordedCourse {
            year: row.get::<i64, _>("year") as u32,
            slug: row.get::<String, _>("slug"),
            name: row.get::<String, _>("name"),
            sort_index: row.get::<i64, _>("sort_index"),
        })
        .collect();

    Ok(courses)
}
