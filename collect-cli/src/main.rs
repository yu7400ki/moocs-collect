mod pdf;

use std::time::Duration;

use clap::Parser;
use collect::{
    iniad::{login_google, login_moocs, Credentials},
    moocs::Course,
};
use dialoguer::{Input, Password, Select};
use indicatif::ProgressBar;
use reqwest::Client;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    path: Option<String>,
    #[arg(long)]
    year: Option<u32>,
    #[arg(long)]
    no_image_embedding: bool,
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let client = create_client()?;
    let credentials = {
        let username: String = Input::new().with_prompt("ユーザ名").interact_text()?;
        let password: String = Password::new().with_prompt("パスワード").interact()?;
        Credentials { username, password }
    };

    let logged_in = {
        let s = Spinner::new();
        s.set_message("ログイン中...");
        let mut logged_in = login_moocs(&client, &credentials).await?;
        logged_in |= login_google(&client, &credentials).await?;
        logged_in
    };
    if !logged_in {
        eprintln!("ログインに失敗しました\nユーザ名とパスワードを確認してください");
        std::process::exit(1);
    }

    let courses = {
        let s = Spinner::new();
        s.set_message("コースを取得中...");
        Course::list(&client, args.year).await?
    };
    let course_selection = Select::new()
        .with_prompt("コースを選択")
        .items(&courses)
        .default(0)
        .max_length(10)
        .interact()?;
    let course = &courses[course_selection];

    let lectures = {
        let s = Spinner::new();
        s.set_message("授業を取得中...");
        course.lectures(&client).await?
    };
    let lecture_selection = Select::new()
        .with_prompt("授業を選択")
        .items(&lectures)
        .default(0)
        .max_length(10)
        .interact()?;
    let lecture = &lectures[lecture_selection];

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
    let page_selection = Select::new()
        .with_prompt("ページを選択")
        .items(&pages)
        .default(0)
        .max_length(10)
        .interact()?;
    let page = &pages[page_selection];

    {
        let s = Spinner::new();
        s.set_message("保存中...");
        let slides = page.slides(&client).await?;
        let mut slide = slides[0].clone();
        slide.embed_image(&client).await?;
        let mut pdf = pdf::convert(&slide)?;
        pdf.save("output.pdf")?;
        println!("output.pdf に保存しました");
    }

    Ok(())
}
