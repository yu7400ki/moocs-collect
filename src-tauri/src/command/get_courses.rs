use std::sync::{Arc, Mutex};

use crate::state::{ClientState, CourseState};
use collect::moocs;
use tauri::State;

#[derive(serde::Serialize)]
pub struct Course {
    pub year: u32,
    pub id: String,
    pub name: String,
}

impl From<moocs::Course> for Course {
    fn from(course: moocs::Course) -> Self {
        Self {
            year: course.year,
            id: course.id,
            name: course.name,
        }
    }
}

#[tauri::command]
pub async fn get_courses(
    client_state: State<'_, ClientState>,
    course_state: State<'_, Mutex<CourseState>>,
) -> Result<Vec<Course>, ()> {
    let client = &client_state.0;
    let courses = moocs::Course::list(client, None).await.map_err(|_| ())?;

    {
        let mut course_state_guard = course_state.lock().map_err(|_| ())?;
        let course_state = &mut *course_state_guard;
        for course in &courses {
            course_state
                .0
                .insert((course.year, course.id.clone()), Arc::new(course.clone()));
        }
    }

    Ok(courses.into_iter().map(Course::from).collect())
}
