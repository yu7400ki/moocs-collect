use rayon::prelude::*;
use std::sync::Mutex;

use crate::state::{ClientState, PageState};
use collect::{
    moocs::{self, Lecture, Slide, SlideContent},
    pdf,
};
use tauri::{Manager, State};
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub async fn download_slides(
    app: tauri::AppHandle,
    year: u32,
    course_id: String,
    lecture_id: String,
    page_id: String,
    client_state: State<'_, ClientState>,
    page_state: State<'_, Mutex<PageState>>,
) -> Result<(), ()> {
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

    let download_dir = get_download_dir(&app)?;
    let lecture_dir = get_lecture_dir(&download_dir, &page.lecture);

    let slide = moocs::Slide::list(client, page).await.map_err(|_| ())?;

    let contents = futures::future::join_all(
        slide
            .iter()
            .map(|slide| async move { slide.content(client).await }),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .map_err(|_| ())?;

    let contents =
        futures::future::join_all(contents.iter().map(|content| content.process(client)))
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ())?;

    save_slides(&slide, &contents, &lecture_dir)?;

    Ok(())
}

fn get_download_dir(app: &tauri::AppHandle) -> Result<std::path::PathBuf, ()> {
    let store = app.store("store.json").map_err(|_| ())?;
    store
        .get("settings")
        .and_then(|settings| settings.get("downloadDir").cloned())
        .and_then(|download_dir| download_dir.as_str().map(String::from))
        .or_else(|| {
            let document_dir = app.path().document_dir().ok()?;
            Some(
                document_dir
                    .join("moocs-collect")
                    .to_string_lossy()
                    .to_string(),
            )
        })
        .ok_or(())
        .map(std::path::PathBuf::from)
}

fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

fn get_lecture_dir<P: AsRef<std::path::Path>>(
    download_dir: P,
    lecture: &Lecture,
) -> std::path::PathBuf {
    fn inner(download_dir: &std::path::Path, lecture: &Lecture) -> std::path::PathBuf {
        download_dir
            .join(&sanitize_filename(&lecture.course.year.to_string()))
            .join(&sanitize_filename(&lecture.course.name))
            .join(&sanitize_filename(&format!(
                "{} - {}",
                lecture.group, lecture.name
            )))
    }
    inner(download_dir.as_ref(), lecture)
}

fn save_slides<P: AsRef<std::path::Path>>(
    slides: &Vec<Slide>,
    contents: &Vec<SlideContent>,
    lecture_path: P,
) -> Result<(), ()> {
    assert_eq!(slides.len(), contents.len());
    let path = lecture_path.as_ref();
    slides.par_iter().zip(contents).enumerate().try_for_each(
        |(i, (slide, content))| -> Result<(), ()> {
            let page = &slide.lecture_page.id;
            let title = &slide.lecture_page.title;
            let filename = match slides.len() {
                1 => format!("{} - {}.pdf", page, title),
                _ => format!("{} - {} ({}).pdf", page, title, i + 1),
            };
            let mut pdf = pdf::convert(content).map_err(|_| ())?;
            std::fs::create_dir_all(&path).map_err(|_| ())?;
            let path = path.join(&sanitize_filename(&filename));
            pdf.save(&path).map_err(|_| ())?;
            Ok(())
        },
    )
}
