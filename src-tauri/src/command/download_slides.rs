use std::path::PathBuf;

use crate::state::{CollectState, SearchState};
use collect::{
    error::CollectError,
    pdf::{self, PdfConversionError, PreProcessor},
    Course, CourseKey, CourseSlug, Lecture, LectureKey, LecturePage, LectureSlug, PageKey,
    PageSlug, Slide, SlideContent, Year,
};
use sqlx::SqlitePool;
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
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
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
    collect_state: State<'_, CollectState>,
    search_state: State<'_, SearchState>,
    db_pool: State<'_, SqlitePool>,
) -> Result<Option<String>, DownloadError> {
    let collect = &collect_state.collect;

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
            .map(|content| preprocessor.preprocess(&collect_state.client, content)),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<_>, PdfConversionError>>()?;

    let saved_paths = save_slides(
        &slides,
        &preprocessed_contents,
        &lecture_dir,
        page_info.display_name(),
    )?;

    persist_downloaded_slides(
        &db_pool,
        &course_info,
        &lecture_info,
        &page_info,
        &slides,
        &saved_paths,
    )
    .await?;

    let search_service = &search_state.0;
    for (idx, content) in contents.iter().enumerate() {
        if let Err(e) = search_service.index_slide_content(content, idx).await {
            log::warn!("Failed to index slide content ({}): {}", idx, e);
        }
    }

    if let Some(first_path) = saved_paths.first() {
        Ok(Some(first_path.to_string_lossy().to_string()))
    } else {
        Ok(None)
    }
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
    slides: &[Slide],
    contents: &[SlideContent],
    lecture_path: P,
    page_title: &str,
) -> Result<Vec<PathBuf>, DownloadError> {
    assert_eq!(slides.len(), contents.len());
    let path = lecture_path.as_ref();
    std::fs::create_dir_all(&path).map_err(DownloadError::Io)?;

    let mut saved_paths = Vec::with_capacity(slides.len());
    for (index, content) in contents.iter().enumerate() {
        let filename = match slides.len() {
            1 => format!("{}.pdf", page_title),
            _ => format!("{} ({}).pdf", page_title, index + 1),
        };
        let mut pdf = pdf::convert(content)?;
        let file_path = path.join(&sanitize_filename(&filename));
        pdf.save(&file_path).map_err(|e| {
            DownloadError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to save PDF to {}: {}", file_path.display(), e),
            ))
        })?;
        saved_paths.push(file_path);
    }

    Ok(saved_paths)
}

async fn persist_downloaded_slides(
    pool: &SqlitePool,
    course: &Course,
    lecture: &Lecture,
    page: &LecturePage,
    slides: &[Slide],
    saved_paths: &[PathBuf],
) -> Result<(), sqlx::Error> {
    debug_assert_eq!(slides.len(), saved_paths.len());

    let mut tx = pool.begin().await?;

    let course_year = course.key.year.value() as i64;
    let course_slug = course.key.slug.value();
    let course_name = course.display_name();
    let course_index = course.index as i64;

    sqlx::query(
        "INSERT INTO courses (year, slug, name, sort_index) VALUES (?, ?, ?, ?) \
         ON CONFLICT(year, slug) DO UPDATE SET name = excluded.name, sort_index = excluded.sort_index, updated_at = unixepoch()",
    )
    .bind(course_year)
    .bind(course_slug)
    .bind(course_name)
    .bind(course_index)
    .execute(&mut *tx)
    .await?;

    let course_id: i64 = sqlx::query_scalar("SELECT id FROM courses WHERE year = ? AND slug = ?")
        .bind(course_year)
        .bind(course_slug)
        .fetch_one(&mut *tx)
        .await?;

    let lecture_slug = lecture.key.slug.value();
    let lecture_name = lecture.display_name();
    let lecture_index = lecture.index as i64;

    sqlx::query(
        "INSERT INTO lectures (course_id, slug, name, sort_index) VALUES (?, ?, ?, ?) \
         ON CONFLICT(course_id, slug) DO UPDATE SET name = excluded.name, sort_index = excluded.sort_index, updated_at = unixepoch()",
    )
    .bind(course_id)
    .bind(lecture_slug)
    .bind(lecture_name)
    .bind(lecture_index)
    .execute(&mut *tx)
    .await?;

    let lecture_id: i64 =
        sqlx::query_scalar("SELECT id FROM lectures WHERE course_id = ? AND slug = ?")
            .bind(course_id)
            .bind(lecture_slug)
            .fetch_one(&mut *tx)
            .await?;

    let page_slug = page.key.slug.value();
    let page_name = page.display_name();
    let page_index = page.index as i64;
    let page_key = page.key.to_string();

    sqlx::query(
        "INSERT INTO pages (lecture_id, slug, name, sort_index, key) VALUES (?, ?, ?, ?, ?) \
         ON CONFLICT(lecture_id, slug) DO UPDATE SET name = excluded.name, sort_index = excluded.sort_index, key = excluded.key, updated_at = unixepoch()",
    )
    .bind(lecture_id)
    .bind(page_slug)
    .bind(page_name)
    .bind(page_index)
    .bind(&page_key)
    .execute(&mut *tx)
    .await?;

    let page_id: i64 = sqlx::query_scalar("SELECT id FROM pages WHERE lecture_id = ? AND slug = ?")
        .bind(lecture_id)
        .bind(page_slug)
        .fetch_one(&mut *tx)
        .await?;

    for (slide, saved_path) in slides.iter().zip(saved_paths) {
        let slide_index = slide.index as i64;
        let pdf_path = saved_path.to_string_lossy();

        sqlx::query(
            "INSERT INTO slides (page_id, idx, url, pdf_path) VALUES (?, ?, ?, ?) \
             ON CONFLICT(page_id, idx) DO UPDATE SET url = excluded.url, pdf_path = excluded.pdf_path, downloaded_at = unixepoch()",
        )
        .bind(page_id)
        .bind(slide_index)
        .bind(&slide.url)
        .bind(pdf_path.as_ref())
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}
