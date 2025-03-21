use keyring::Entry;
use rayon::prelude::*;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
    time::Duration,
};

use base64::{engine::general_purpose, Engine};
use clap::Parser;
use collect::{
    iniad::{login_google, login_moocs, Credentials},
    moocs::{Course, LecturePage, Slide, SlideContent},
    pdf,
};
use dialoguer::{console::Style, Input, Password, Select};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    path: Option<PathBuf>,
    #[arg(long)]
    year: Option<u32>,
}

fn create_client() -> reqwest::Result<Client> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0")
        .cookie_store(true)
        .build()?;
    Ok(client)
}

struct Spinner {
    spinner: ProgressBar,
}

impl Spinner {
    fn new() -> Self {
        let spinner = ProgressBar::new_spinner();
        spinner.enable_steady_tick(Duration::from_millis(100));
        Self { spinner }
    }

    fn set_message(&self, msg: &'static str) {
        self.spinner.set_message(msg);
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.spinner.finish_and_clear();
    }
}

fn ignore_invalid_char(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

fn slide_dir(slide: &Slide) -> String {
    format!(
        "{}/{} - {}",
        ignore_invalid_char(&slide.lecture_page.lecture.course.name),
        ignore_invalid_char(&slide.lecture_page.lecture.group),
        ignore_invalid_char(&slide.lecture_page.lecture.name)
    )
}

fn save_slides<P: AsRef<Path>>(
    slides: &Vec<Slide>,
    contents: &Vec<SlideContent>,
    path: P,
) -> anyhow::Result<()> {
    assert_eq!(slides.len(), contents.len());
    let path = path.as_ref();
    slides.par_iter().zip(contents).enumerate().try_for_each(
        |(i, (slide, content))| -> Result<(), anyhow::Error> {
            let dir = slide_dir(&slide);
            let page = ignore_invalid_char(&slide.lecture_page.id);
            let title = ignore_invalid_char(&slide.lecture_page.title);
            let filename = match slides.len() {
                1 => format!("{} - {}.pdf", page, title),
                _ => format!("{} - {} ({}).pdf", page, title, i + 1),
            };
            let mut pdf = pdf::convert(content)?;
            let path = path.join(&dir);
            create_dir_all(&path)?;
            let path = path.join(&filename);
            pdf.save(&path)?;
            Ok(())
        },
    )
}

async fn save_slides_from_pages<P: AsRef<Path> + Sync>(
    client: &Client,
    pages: &Vec<LecturePage>,
    path: P,
) -> anyhow::Result<()> {
    let slides = futures::future::join_all(pages.iter().map(|page| page.slides(client)))
        .await
        .into_iter()
        .collect::<anyhow::Result<Vec<_>>>()?;
    let slide_contents = slides
        .iter()
        .map(|slides| futures::future::join_all(slides.iter().map(|slide| slide.content(client))));
    let slide_contents = futures::future::join_all(slide_contents)
        .await
        .into_iter()
        .map(|slides| slides.into_iter().collect::<anyhow::Result<Vec<_>>>())
        .collect::<anyhow::Result<Vec<_>>>()?;
    let slide_contents = slide_contents
        .iter()
        .map(|slides| {
            futures::future::join_all(slides.into_iter().map(|slide| process_slide(slide, client)))
        })
        .collect::<Vec<_>>();
    let slide_contents = futures::future::join_all(slide_contents)
        .await
        .into_iter()
        .map(|slides| slides.into_iter().collect::<anyhow::Result<Vec<_>>>())
        .collect::<anyhow::Result<Vec<_>>>()?;
    slides.par_iter().zip(&slide_contents).try_for_each(
        |(slides, contents)| -> Result<(), anyhow::Error> { save_slides(slides, contents, &path) },
    )?;
    Ok(())
}

async fn process_slide(slide: &SlideContent, client: &Client) -> anyhow::Result<SlideContent> {
    let images = slide.fetch_images(client).await?;
    let images = images
        .into_iter()
        .map(|(url, bytes)| (url, encode_base64(&bytes)))
        .collect();
    let slide = slide.embed_text()?.embed_images(&images)?;
    Ok(slide)
}

pub fn encode_base64(bytes: &[u8]) -> String {
    let kind = infer::get(&bytes);
    let mime = kind
        .and_then(|kind| Some(kind.mime_type()))
        .unwrap_or("image/svg+xml");
    let mime = match mime {
        "text/xml" => "image/svg+xml",
        _ => mime,
    };
    let base64 = general_purpose::STANDARD.encode(&bytes);
    let base64 = format!("data:{};base64,{}", mime, base64);
    base64
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let client = create_client()?;
    let username: String = Input::new().with_prompt("ユーザー名").interact_text()?;
    let entry = Entry::new("me.yu7400ki.moocs-collect", &username)?;
    let password: String = match entry.get_password() {
        Ok(password) => password,
        Err(_) => {
            let password: String = Password::new().with_prompt("パスワード").interact()?;
            entry.set_password(&password).ok();
            password
        }
    };
    let credentials = Credentials { username, password };

    let logged_in = {
        let s = Spinner::new();
        s.set_message("ログイン中...");
        let mut logged_in = login_moocs(&client, &credentials).await?;
        logged_in |= login_google(&client, &credentials).await?;
        logged_in
    };
    if !logged_in {
        eprintln!("ログインに失敗しました\nユーザー名とパスワードを確認してください");
        entry.delete_credential().ok();
        std::process::exit(1);
    }

    let path = args.path.unwrap_or_else(|| PathBuf::from("."));
    let underline = Style::new().underlined();
    let progress_template =
        ProgressStyle::with_template("{percent:>3}% {bar:40} {pos:>2}/{len:2} {msg}").unwrap();

    let courses = {
        let s = Spinner::new();
        s.set_message("科目を取得中...");
        Course::list(&client, args.year).await?
    };
    let mut course_items = courses
        .iter()
        .map(|course| course.to_string())
        .collect::<Vec<_>>();
    course_items.insert(0, underline.apply_to("全ての科目").to_string());
    let course_selection = Select::new()
        .with_prompt("科目を選択")
        .items(&course_items)
        .default(0)
        .max_length(10)
        .interact()?;

    if course_selection == 0 {
        for course in courses.iter() {
            let lectures = course.lectures(&client).await?;
            let bar = ProgressBar::new(lectures.len() as u64);
            bar.set_style(progress_template.clone());
            for lecture in lectures.iter() {
                bar.set_message(format!("{}", lecture.name));
                let pages = lecture.pages(&client).await?;
                save_slides_from_pages(&client, &pages, &path).await?;
                bar.inc(1);
            }
            bar.finish();
        }
        return Ok(());
    }

    let course = &courses[course_selection - 1];
    let lectures = {
        let s = Spinner::new();
        s.set_message("授業を取得中...");
        course.lectures(&client).await?
    };
    let mut lecture_items = lectures
        .iter()
        .map(|lecture| lecture.to_string())
        .collect::<Vec<_>>();
    lecture_items.insert(0, underline.apply_to("全ての授業").to_string());
    let lecture_selection = Select::new()
        .with_prompt("授業を選択")
        .items(&lecture_items)
        .default(0)
        .max_length(10)
        .interact()?;

    if lecture_selection == 0 {
        let bar = ProgressBar::new(lectures.len() as u64);
        bar.set_style(progress_template.clone());
        for lecture in lectures.iter() {
            bar.set_message(format!("{}", lecture.name));
            let pages = lecture.pages(&client).await?;
            save_slides_from_pages(&client, &pages, &path).await?;
            bar.inc(1);
        }
        bar.finish();
        return Ok(());
    }

    let lecture = &lectures[lecture_selection - 1];

    let pages = {
        let s = Spinner::new();
        s.set_message("ページを取得中...");
        let pages = lecture.pages(&client).await?;
        let mut pages_has_slide = vec![];
        for page in pages {
            let has_slide = page.has_slide(&client).await?;
            if has_slide {
                pages_has_slide.push(page);
            }
        }
        pages_has_slide
    };
    let mut pages_items = pages
        .iter()
        .map(|page| page.to_string())
        .collect::<Vec<_>>();
    pages_items.insert(0, underline.apply_to("全てのページ").to_string());
    let page_selection = Select::new()
        .with_prompt("ページを選択")
        .items(&pages_items)
        .default(0)
        .max_length(10)
        .interact()?;

    if page_selection == 0 {
        let s = Spinner::new();
        s.set_message("保存中...");
        save_slides_from_pages(&client, &pages, &path).await?;
        return Ok(());
    }

    let page = &pages[page_selection - 1];

    let s = Spinner::new();
    s.set_message("保存中...");
    let slides = page.slides(&client).await?;
    let content = futures::future::join_all(slides.iter().map(|slide| slide.content(&client)))
        .await
        .into_iter()
        .collect::<anyhow::Result<Vec<_>>>()?;
    let content =
        futures::future::join_all(content.iter().map(|slide| process_slide(slide, &client)))
            .await
            .into_iter()
            .collect::<anyhow::Result<Vec<_>>>()?;
    save_slides(&slides, &content, &path)?;

    Ok(())
}
