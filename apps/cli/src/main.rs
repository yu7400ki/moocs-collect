use clap::Parser;
use collect::{
    error::CollectError, pdf, Collect, Credentials, LecturePage, PageKey, SlideContent, Year,
};
use dialoguer::{console::Style, Input, Password, Select};
use indicatif::{ProgressBar, ProgressStyle};
use keyring::Entry;
use rayon::prelude::*;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
    sync::LazyLock,
    time::Duration,
};

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0")
        .cookie_store(true)
        .build().expect("Failed to create HTTP client")
});

// Helper struct to hold slide information needed for file naming
struct PageInfo {
    course_name: String,
    lecture_group: String,
    lecture_name: String,
    page_slug: String,
    page_title: String,
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    path: Option<PathBuf>,
    #[arg(long)]
    year: Option<u32>,
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

fn sanitize_filename(s: &str) -> String {
    #[cfg(windows)]
    const INVALID_CHARS: [char; 9] = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];

    #[cfg(unix)]
    const INVALID_CHARS: [char; 2] = ['/', '\0'];

    s.chars()
        .map(|c| if INVALID_CHARS.contains(&c) { '_' } else { c })
        .collect()
}

async fn get_page_info(collect: &Collect, page_key: &PageKey) -> anyhow::Result<PageInfo> {
    // With caching, these calls should be very fast as data is already cached
    let page = collect.get_page_info(page_key).await?;
    let lecture = collect.get_lecture_info(&page.key.lecture_key).await?;
    let course = collect.get_course_info(&lecture.key.course_key).await?;

    // Optimized: Use the known course key to get lecture groups, then find the specific group
    let lecture_group = match collect.get_lecture_groups(&lecture.key.course_key).await {
        Ok(groups) => groups
            .iter()
            .find(|group| group.course_key == lecture.key.course_key)
            .map(|group| group.display_name().to_string())
            .unwrap_or_else(|| lecture.display_name().to_string()),
        Err(_) => lecture.display_name().to_string(), // Fallback to lecture name
    };

    Ok(PageInfo {
        course_name: course.display_name().to_string(),
        lecture_group,
        lecture_name: lecture.display_name().to_string(),
        page_slug: page.key.slug.value().to_string(),
        page_title: page.display_name().to_string(),
    })
}

fn slide_dir_from_info(info: &PageInfo) -> String {
    format!(
        "{}/{} - {}",
        sanitize_filename(&info.course_name),
        sanitize_filename(&info.lecture_group),
        sanitize_filename(&info.lecture_name)
    )
}

async fn save_slides<P: AsRef<Path> + Sync>(
    collect: &Collect,
    slide_contents: &[SlideContent],
    path: P,
) -> anyhow::Result<()> {
    if slide_contents.is_empty() {
        return Ok(());
    }

    let path = path.as_ref();

    let page_info = get_page_info(collect, &slide_contents[0].page_key).await?;
    let dir = slide_dir_from_info(&page_info);
    let path = path.join(&dir);
    create_dir_all(&path)?;

    let preprocessor = pdf::PreProcessor::default();

    let slide_contents = slide_contents
        .iter()
        .map(|content| preprocessor.preprocess(&CLIENT, content))
        .collect::<Vec<_>>();
    let slide_contents = futures::future::join_all(slide_contents)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    slide_contents.par_iter().enumerate().try_for_each(
        |(i, content)| -> Result<(), anyhow::Error> {
            let filename = match slide_contents.len() {
                1 => format!(
                    "{} - {}.pdf",
                    sanitize_filename(&page_info.page_slug),
                    sanitize_filename(&page_info.page_title)
                ),
                _ => format!(
                    "{} - {} ({}).pdf",
                    sanitize_filename(&page_info.page_slug),
                    sanitize_filename(&page_info.page_title),
                    i + 1
                ),
            };

            let mut pdf = pdf::convert(content)?;
            let file_path = path.join(&filename);
            pdf.save(&file_path)?;
            Ok(())
        },
    )
}

async fn save_slides_from_pages<P: AsRef<Path> + Sync>(
    collect: &Collect,
    pages: &[LecturePage],
    path: P,
) -> anyhow::Result<()> {
    let slides = futures::future::join_all(pages.iter().map(|page| collect.get_slides(&page.key)))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|slides| !slides.is_empty())
        .collect::<Vec<_>>();

    let slide_contents = slides.iter().map(|slides| {
        futures::future::join_all(slides.iter().map(|slide| collect.get_slide_content(slide)))
    });

    let contents = futures::future::join_all(slide_contents)
        .await
        .into_iter()
        .map(|slides| slides.into_iter().collect::<Result<Vec<_>, _>>())
        .collect::<Result<Vec<_>, _>>()?;

    futures::future::join_all(
        contents
            .iter()
            .map(|contents| save_slides(collect, contents, &path)),
    )
    .await
    .into_iter()
    .collect::<Result<(), _>>()?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let collect = Collect::new(CLIENT.clone());

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

    let result = {
        let s = Spinner::new();
        s.set_message("ログイン中...");
        collect.authenticate(&credentials).await
    };

    match result {
        Err(CollectError::Authentication { reason: _ }) => {
            eprintln!("ログインに失敗しました\nユーザー名とパスワードを確認してください");
            entry.delete_credential().ok();
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("ログインに失敗しました\n不明なエラーが発生しました");
            entry.delete_credential().ok();
            std::process::exit(1);
        }
        Ok(_) => {}
    }

    let path = args.path.unwrap_or_else(|| PathBuf::from("."));
    let underline = Style::new().underlined();
    let progress_template =
        ProgressStyle::with_template("{percent:>3}% {bar:40} {pos:>2}/{len:2} {msg}").unwrap();

    let courses = {
        let s = Spinner::new();
        s.set_message("科目を取得中...");
        let year = args.year.map(Year::new).transpose()?;
        collect.get_courses(year).await?
    };

    let mut course_items = courses
        .iter()
        .map(|course| course.display_name().to_string())
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
            let lectures = collect.get_lectures(&course.key).await?;
            let bar = ProgressBar::new(lectures.len() as u64);
            bar.set_style(progress_template.clone());
            for lecture in lectures.iter() {
                bar.set_message(lecture.display_name().to_string());
                let pages = collect.get_pages(&lecture.key).await?;
                save_slides_from_pages(&collect, &pages, &path).await?;
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
        collect.get_lectures(&course.key).await?
    };

    let mut lecture_items = lectures
        .iter()
        .map(|lecture| lecture.display_name().to_string())
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
            bar.set_message(lecture.display_name().to_string());
            let pages = collect.get_pages(&lecture.key).await?;
            save_slides_from_pages(&collect, &pages, &path).await?;
            bar.inc(1);
        }
        bar.finish();
        return Ok(());
    }

    let lecture = &lectures[lecture_selection - 1];

    let pages = {
        let s = Spinner::new();
        s.set_message("ページを取得中...");
        let mut pages_has_slide = vec![];
        let all_pages = collect.get_pages(&lecture.key).await?;
        for page in all_pages {
            let slides = collect.get_slides(&page.key).await?;
            if !slides.is_empty() {
                pages_has_slide.push(page);
            }
        }
        pages_has_slide
    };

    let mut pages_items = pages
        .iter()
        .map(|page| page.display_name().to_string())
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
        save_slides_from_pages(&collect, &pages, &path).await?;
        return Ok(());
    }

    let page = &pages[page_selection - 1];

    let s = Spinner::new();
    s.set_message("保存中...");
    let slides = collect.get_slides(&page.key).await?;
    let content =
        futures::future::join_all(slides.iter().map(|slide| collect.get_slide_content(slide)))
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

    save_slides(&collect, &content, &path).await?;

    Ok(())
}
