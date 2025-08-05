use rayon::prelude::*;

use crate::state::CollectState;
use collect::{
    domain::models::{
        CourseKey, CourseSlug, LectureKey, LectureSlug, PageKey, PageSlug, Slide, SlideContent,
        Year,
    },
    error::CollectError,
    pdf::{self, PdfConversionError, PreProcessor},
};
use tauri::{Manager, State};
use tauri_plugin_store::StoreExt;

#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Core library error: {0}")]
    Core(#[from] CollectError),
    #[error("PDF conversion error: {0}")]
    PdfConversion(#[from] PdfConversionError),
    #[error("Store error: {0}")]
    Store(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Path error: {0}")]
    Path(String),
}

impl serde::Serialize for DownloadError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[tauri::command]
pub async fn download_slides(
    app: tauri::AppHandle,
    year: u32,
    course_slug: String,
    lecture_slug: String,
    page_slug: String,
    state: State<'_, CollectState>,
) -> Result<(), DownloadError> {
    let collect = &state.collect;

    // Build the page key with better error messages
    let year_obj = Year::new(year)
        .map_err(|e| DownloadError::InvalidInput(format!("Invalid year {}: {}", year, e)))?;
    let course_slug_obj = CourseSlug::new(course_slug.clone()).map_err(|e| {
        DownloadError::InvalidInput(format!("Invalid course slug '{}': {}", course_slug, e))
    })?;
    let lecture_slug_obj = LectureSlug::new(lecture_slug.clone()).map_err(|e| {
        DownloadError::InvalidInput(format!("Invalid lecture slug '{}': {}", lecture_slug, e))
    })?;
    let page_slug_obj = PageSlug::new(page_slug.clone()).map_err(|e| {
        DownloadError::InvalidInput(format!("Invalid page slug '{}': {}", page_slug, e))
    })?;

    let course_key = CourseKey::new(year_obj, course_slug_obj);
    let lecture_key = LectureKey::new(course_key, lecture_slug_obj);
    let page_key = PageKey::new(lecture_key, page_slug_obj);

    let download_dir = get_download_dir(&app)?;

    // Get page info for directory structure
    let page_info = collect.get_page_info(&page_key).await?;
    let lecture_info = collect.get_lecture_info(&page_info.key.lecture_key).await?;
    let course_info = collect
        .get_course_info(&lecture_info.key.course_key)
        .await?;

    let lecture_dir = get_lecture_dir_from_info(
        &download_dir,
        course_info.display_name(),
        lecture_info.display_name(),
        year,
    );

    let slides = collect.get_slides(&page_key).await?;

    let contents =
        futures::future::join_all(slides.iter().map(|slide| collect.get_slide_content(slide)))
            .await
            .into_iter()
            .collect::<Result<Vec<_>, CollectError>>()?;

    // Preprocess slide contents using shared HTTP client
    // This embeds images and text into SVGs for better PDF generation
    let preprocessor = PreProcessor::default();
    let preprocessed_contents = futures::future::join_all(
        contents
            .iter()
            .map(|content| preprocessor.preprocess(&state.client, content)),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<_>, PdfConversionError>>()?;

    save_slides(
        &slides,
        &preprocessed_contents,
        &lecture_dir,
        page_info.display_name(),
    )?;

    Ok(())
}

fn get_download_dir(app: &tauri::AppHandle) -> Result<std::path::PathBuf, DownloadError> {
    let store = app
        .store("store.json")
        .map_err(|e| DownloadError::Store(format!("Failed to access store: {}", e)))?;

    let download_dir = store
        .get("settings")
        .and_then(|settings| settings.get("downloadDir").cloned())
        .and_then(|download_dir| download_dir.as_str().map(String::from))
        .or_else(|| {
            app.path().document_dir().ok().map(|document_dir| {
                document_dir
                    .join("moocs-collect")
                    .to_string_lossy()
                    .to_string()
            })
        })
        .ok_or_else(|| DownloadError::Path("Could not determine download directory".to_string()))?;

    Ok(std::path::PathBuf::from(download_dir))
}

fn sanitize_filename(s: &str) -> String {
    #[cfg(windows)]
    const INVALID_CHARS: [char; 9] = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];

    #[cfg(unix)]
    const INVALID_CHARS: [char; 2] = ['/', '\0'];

    s.chars()
        .map(|c| if INVALID_CHARS.contains(&c) { '_' } else { c })
        .collect()
}

fn get_lecture_dir_from_info<P: AsRef<std::path::Path>>(
    download_dir: P,
    course_name: &str,
    lecture_name: &str,
    year: u32,
) -> std::path::PathBuf {
    download_dir
        .as_ref()
        .join(&sanitize_filename(&year.to_string()))
        .join(&sanitize_filename(course_name))
        .join(&sanitize_filename(lecture_name))
}

fn save_slides<P: AsRef<std::path::Path>>(
    slides: &Vec<Slide>,
    contents: &Vec<SlideContent>,
    lecture_path: P,
    page_title: &str,
) -> Result<(), DownloadError> {
    assert_eq!(slides.len(), contents.len());
    let path = lecture_path.as_ref();
    std::fs::create_dir_all(&path).map_err(|e| DownloadError::Io(e))?;

    slides.par_iter().zip(contents).enumerate().try_for_each(
        |(i, (_slide, content))| -> Result<(), DownloadError> {
            let filename = match slides.len() {
                1 => format!("{}.pdf", page_title),
                _ => format!("{} ({}).pdf", page_title, i + 1),
            };
            let mut pdf = pdf::convert(content)?;
            let file_path = path.join(&sanitize_filename(&filename));
            pdf.save(&file_path).map_err(|e| {
                DownloadError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to save PDF to {}: {}", file_path.display(), e),
                ))
            })?;
            Ok(())
        },
    )
}
