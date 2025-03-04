use std::sync::{Arc, Mutex};

use crate::state::{ClientState, CourseState, LectureState};
use collect::moocs;
use tauri::State;

#[derive(serde::Serialize)]
pub struct Lecture {
    pub year: u32,
    pub course_id: String,
    pub id: String,
    pub name: String,
    pub group: String,
}

impl<'a> From<moocs::Lecture> for Lecture {
    fn from(lecture: moocs::Lecture) -> Self {
        Self {
            year: lecture.course.year,
            course_id: lecture.course.id.clone(),
            id: lecture.id,
            name: lecture.name,
            group: lecture.group,
        }
    }
}

#[tauri::command]
pub async fn get_lectures(
    year: u32,
    course_id: String,
    client_state: State<'_, ClientState>,
    course_state: State<'_, Mutex<CourseState>>,
    lecture_state: State<'_, Mutex<LectureState>>,
) -> Result<Vec<Lecture>, ()> {
    let client = &client_state.0;
    let course = {
        let course_state_guard = course_state.lock().map_err(|_| ())?;
        let course_state = &*course_state_guard;
        course_state
            .0
            .get(&(year, course_id.clone()))
            .cloned()
            .ok_or(())?
    };
    let lectures = moocs::Lecture::list(client, course).await.map_err(|_| ())?;

    {
        let mut lecture_state_guard = lecture_state.lock().map_err(|_| ())?;
        let lecture_state = &mut *lecture_state_guard;
        for lecture in &lectures {
            lecture_state.0.insert(
                (year, course_id.clone(), lecture.id.clone()),
                Arc::new(lecture.clone()),
            );
        }
    }

    Ok(lectures.into_iter().map(Lecture::from).collect())
}
