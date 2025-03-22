use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use bytes::Bytes;
use rayon::prelude::*;
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

use crate::state::{ClientState, PageState};
use crate::store::{ImageCache, ImageCacheEntry, Settings};
use collect::{
    moocs::{self, Lecture, Slide, SlideContent},
    pdf,
};
use tauri::{Manager, State};

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

    let settings = Settings::from(&app);

    let lecture_dir = get_lecture_dir(&settings.download_dir, &page.lecture);

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

    let images =
        futures::future::join_all(contents.iter().map(|content| get_images(content, &app)))
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ())?
            .into_iter()
            .flatten()
            .collect::<HashMap<_, _>>();

    let contents = contents
        .par_iter()
        .map(|content| {
            content
                .embed_text()
                .and_then(|content| content.embed_images(&images))
                .map_err(|_| ())
        })
        .collect::<Result<Vec<_>, _>>()?;

    save_slides(&slide, &contents, &lecture_dir)?;

    Ok(())
}

async fn get_images(
    content: &SlideContent,
    app: &tauri::AppHandle,
) -> Result<HashMap<String, String>, String> {
    let image_urls = content.extract_image_url();
    let mut futures = vec![];
    for url in image_urls {
        let app = app.clone();
        let future = async move {
            let path = get_image_path(&url, &app).await?;
            Ok((url, path))
        };
        futures.push(future);
    }
    let results = futures::future::join_all(futures)
        .await
        .into_iter()
        .collect::<Result<Vec<(String, String)>, String>>()?;
    Ok(results.into_iter().collect())
}

async fn get_image_path(url: &str, app: &tauri::AppHandle) -> Result<String, String> {
    let image_cache = ImageCache::new(app);
    let cache = image_cache
        .get(url)
        .map_err(|_| "failed to get image cache".to_string())?;
    if let Some(entry) = &cache {
        if fs::metadata(&entry.path).is_ok() {
            return Ok(entry.path.clone());
        }
    }
    let client_state = app.state::<ClientState>().inner();
    let client = &client_state.0;
    let bytes = fetch_image(url, client)
        .await
        .map_err(|_| "failed to fetch image".to_string())?;
    let hash = calculate_image_hash(&bytes);
    let ext = guess_extension(&bytes);
    let app_data = app
        .path()
        .app_cache_dir()
        .expect("failed to get app cache dir");
    let dir = app_data.join("images");
    fs::create_dir_all(&dir).expect("failed to create image cache dir");
    let path = dir
        .join(&format!("{}.{}", hash, ext))
        .to_string_lossy()
        .to_string();
    fs::write(&path, &bytes).expect("failed to write image");
    image_cache.insert(ImageCacheEntry::new(url, &path)).ok();
    Ok(path)
}

fn calculate_image_hash(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let hash_result = hasher.finalize();
    URL_SAFE_NO_PAD.encode(hash_result)
}

fn guess_extension(bytes: &[u8]) -> String {
    let kind = infer::get(&bytes);
    let ext = kind
        .and_then(|kind| Some(kind.extension()))
        .unwrap_or("svg");
    let ext = match ext {
        "xml" => "svg",
        _ => ext,
    };
    ext.to_string()
}

async fn fetch_image(url: &str, client: &Client) -> Result<Bytes, ()> {
    let response = client.get(url).send().await.map_err(|_| ())?;
    Ok(response.bytes().await.map_err(|_| ())?)
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
