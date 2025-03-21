use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use bytes::Bytes;
use regex::Regex;
use reqwest::Client;
use scraper::Html;

use crate::{
    iniad::{check_logged_in_google, check_logged_in_moocs},
    svg::Svg,
    utils::{extract_element_attribute, extract_text_content},
};

#[derive(Debug, Clone)]
pub struct Url {
    pub year: Option<u32>,
    pub course_id: Option<String>,
    pub lecture_id: Option<String>,
    pub page_id: Option<String>,
}

impl Url {
    pub const BASE_URL: &'static str = "https://moocs.iniad.org";
    pub const COURSE_URL: &'static str = "https://moocs.iniad.org/courses";

    pub fn is_course(&self) -> bool {
        self.course_id.is_some() && self.lecture_id.is_none() && self.page_id.is_none()
    }

    pub fn is_lecture(&self) -> bool {
        self.course_id.is_some() && self.lecture_id.is_some() && self.page_id.is_none()
    }

    pub fn is_page(&self) -> bool {
        self.course_id.is_some() && self.lecture_id.is_some() && self.page_id.is_some()
    }
}

impl TryFrom<&str> for Url {
    type Error = anyhow::Error;

    fn try_from(url: &str) -> Result<Self, Self::Error> {
        let url = match url.starts_with("http") {
            true => url.to_string(),
            false => format!("{}{}", Self::BASE_URL, url),
        };
        let url = url.parse::<reqwest::Url>()?;
        let path = url
            .path_segments()
            .ok_or(anyhow::anyhow!("Invalid URL"))?
            .collect::<Vec<_>>();
        let year = path.get(1).map(|s| s.parse::<u32>()).transpose()?;
        let course_id = path.get(2).map(|s| s.to_string());
        let lecture_id = path.get(3).map(|s| s.to_string());
        let page_id = path.get(4).map(|s| s.to_string());
        Ok(Self {
            year,
            course_id,
            lecture_id,
            page_id,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Course {
    pub year: u32,
    pub id: String,
    pub name: String,
}

impl Course {
    pub fn new(year: u32, id: &str, name: &str) -> Self {
        Self {
            year,
            id: id.to_string(),
            name: name.to_string(),
        }
    }

    pub async fn fetch_page(client: &Client, year: Option<u32>) -> anyhow::Result<String> {
        let url = match year {
            Some(year) => format!("{}/{}", Url::COURSE_URL, year),
            None => Url::COURSE_URL.to_string(),
        };
        let response = client.get(&url).send().await?;
        let html = response.text().await?;
        Ok(html)
    }

    pub fn scrape_page(html: &str) -> Vec<(String, Url)> {
        let document = Html::parse_document(html);
        let courses = document
            // course_cards
            .select(&scraper::Selector::parse(".content .media").unwrap())
            .map(|course_card| -> anyhow::Result<(String, Url)> {
                let name = extract_text_content(&course_card, ".media-body h4.media-heading")?
                    .trim()
                    .to_string();
                let href = extract_element_attribute(&course_card, "a", "href")?;
                let url = Url::try_from(href.as_str())?;
                if !url.is_course() {
                    return Err(anyhow::anyhow!("Invalid URL"));
                }
                Ok((name, url))
            })
            // filter out errors
            .filter_map(|result| match result {
                Ok(course) => Some(course),
                Err(_) => None,
            })
            .collect::<Vec<_>>();
        courses
    }

    pub async fn list(client: &Client, year: Option<u32>) -> anyhow::Result<Vec<Self>> {
        if !check_logged_in_moocs(client).await? {
            return Err(anyhow::anyhow!("Not logged in"));
        }
        let html = Self::fetch_page(client, year).await?;
        let courses = Self::scrape_page(&html)
            .into_iter()
            .map(|(name, url)| {
                let year = url.year.unwrap();
                let id = url.course_id.unwrap();
                Self::new(year, &id, &name)
            })
            .collect();
        Ok(courses)
    }

    pub async fn lectures(&self, client: &Client) -> anyhow::Result<Vec<Lecture>> {
        let course = Arc::new(self.clone());
        let lectures = Lecture::list(client, course).await?;
        Ok(lectures)
    }
}

impl ToString for Course {
    fn to_string(&self) -> String {
        format!("{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct Lecture {
    pub course: Arc<Course>,
    pub id: String,
    pub name: String,
    pub group: String,
}

impl Lecture {
    pub async fn fetch_page(client: &Client, year: u32, course_id: &str) -> anyhow::Result<String> {
        let url = format!("{}/{}/{}", Url::COURSE_URL, year, course_id);
        let response = client.get(&url).send().await?;
        let html = response.text().await?;
        Ok(html)
    }

    pub fn scrape_page(html: &str) -> Vec<(String, String, Url)> {
        let document = Html::parse_document(html);
        let lectures = document
            .select(&scraper::Selector::parse("ul.sidebar-menu li.treeview").unwrap())
            .map(|treeview| {
                let group = extract_text_content(&treeview, "span.sidebar-menu-text")
                    .unwrap()
                    .trim()
                    .to_string();
                treeview
                    .select(&scraper::Selector::parse("ul.treeview-menu li").unwrap())
                    .map(|menu| -> anyhow::Result<(String, String, Url)> {
                        let name = extract_text_content(&menu, "a")?;
                        let href = extract_element_attribute(&menu, "a", "href")?;
                        let url = Url::try_from(href.as_str())?;
                        if !url.is_lecture() {
                            return Err(anyhow::anyhow!("Invalid URL"));
                        }
                        Ok((name, group.clone(), url))
                    })
                    .filter_map(|result| match result {
                        Ok(lecture) => Some(lecture),
                        Err(_) => None,
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
        lectures
    }

    pub async fn list(client: &Client, course: Arc<Course>) -> anyhow::Result<Vec<Self>> {
        let html = Self::fetch_page(client, course.year, &course.id).await?;
        let lectures = Self::scrape_page(&html)
            .into_iter()
            .map(|(name, group, url)| {
                let id = url.lecture_id.unwrap();
                Lecture {
                    course: Arc::clone(&course),
                    id,
                    name,
                    group,
                }
            })
            .collect();
        Ok(lectures)
    }

    pub async fn pages(&self, client: &Client) -> anyhow::Result<Vec<LecturePage>> {
        let lecture = Arc::new(self.clone());
        let pages = LecturePage::list(client, lecture).await?;
        Ok(pages)
    }
}

impl ToString for Lecture {
    fn to_string(&self) -> String {
        format!("{} - {}", self.group, self.name)
    }
}

#[derive(Debug, Clone)]
pub struct LecturePage {
    pub lecture: Arc<Lecture>,
    pub id: String,
    pub title: String,
}

impl LecturePage {
    pub async fn fetch_page(
        client: &Client,
        year: u32,
        course_id: &str,
        lecture_id: &str,
    ) -> anyhow::Result<(String, String)> {
        let url = format!("{}/{}/{}/{}", Url::COURSE_URL, year, course_id, lecture_id);
        let response = client.get(&url).send().await?;
        let url = response.url().to_string();
        let html = response.text().await?;
        Ok((url, html))
    }

    pub fn scrape_page(html: &str) -> Vec<(String, Option<Url>)> {
        let document = Html::parse_document(html);
        let pagination = document
            .select(&scraper::Selector::parse("ul.pagination li").unwrap())
            .collect::<Vec<_>>();
        let pagination = if pagination.len() > 2 {
            &pagination[1..pagination.len() - 1]
        } else {
            return vec![];
        };
        let pages = pagination
            .iter()
            .map(|li| -> anyhow::Result<(String, Option<Url>)> {
                let title = extract_element_attribute(li, "a", "title")?;
                let href = extract_element_attribute(li, "a", "href")?;
                let url = match &*href {
                    "#" => None,
                    _ => Some(Url::try_from(href.as_str())?),
                };
                Ok((title, url))
            })
            .filter_map(|result| match result {
                Ok(page) => Some(page),
                Err(_) => None,
            })
            .collect::<Vec<_>>();
        pages
    }

    pub async fn list(client: &Client, lecture: Arc<Lecture>) -> anyhow::Result<Vec<Self>> {
        let (current_url, html) =
            Self::fetch_page(client, lecture.course.year, &lecture.course.id, &lecture.id).await?;
        let current_url = Url::try_from(current_url.as_str())?;
        let pages = Self::scrape_page(&html)
            .into_iter()
            .map(|(title, page)| -> anyhow::Result<LecturePage> {
                let url = page.unwrap_or_else(|| current_url.clone());
                if !url.is_page() {
                    return Err(anyhow::anyhow!("Invalid URL"));
                }
                Ok(LecturePage {
                    lecture: Arc::clone(&lecture),
                    id: url.page_id.unwrap(),
                    title,
                })
            })
            .filter_map(|result| match result {
                Ok(page) => Some(page),
                Err(_) => None,
            })
            .collect();
        Ok(pages)
    }

    pub async fn slides(&self, client: &Client) -> anyhow::Result<Vec<Slide>> {
        let lecture_page = Arc::new(self.clone());
        let slides = Slide::list(client, lecture_page).await?;
        Ok(slides)
    }

    pub async fn has_slide(&self, client: &Client) -> anyhow::Result<bool> {
        let slides = self.slides(client).await?;
        Ok(slides.len() > 0)
    }
}

impl ToString for LecturePage {
    fn to_string(&self) -> String {
        format!("{}", self.title)
    }
}

#[derive(Debug, Clone)]
pub struct Slide {
    pub lecture_page: Arc<LecturePage>,
    pub slide_url: String,
}

impl Slide {
    pub async fn fetch_page(
        client: &Client,
        year: u32,
        course_id: &str,
        lecture_id: &str,
        page_id: &str,
    ) -> anyhow::Result<String> {
        let url = format!(
            "{}/{}/{}/{}/{}",
            Url::COURSE_URL,
            year,
            course_id,
            lecture_id,
            page_id
        );
        let response = client.get(&url).send().await?;
        let html = response.text().await?;
        Ok(html)
    }

    pub fn scrape_page(html: &str) -> Vec<String> {
        let document = Html::parse_document(html);
        let gslide_regex =
            Regex::new(r#"^https://docs.google.com/presentation/d/.*?/embed\?"#).unwrap();
        let slides = document
            .select(&scraper::Selector::parse("iframe").unwrap())
            .filter_map(|iframe| iframe.value().attr("src"))
            .filter(|src| gslide_regex.is_match(src))
            .map(|src| src.to_string())
            .collect::<Vec<_>>();
        slides
    }

    pub async fn list(
        client: &Client,
        lecture_page: Arc<LecturePage>,
    ) -> anyhow::Result<Vec<Self>> {
        let html = Self::fetch_page(
            client,
            lecture_page.lecture.course.year,
            &lecture_page.lecture.course.id,
            &lecture_page.lecture.id,
            &lecture_page.id,
        )
        .await?;
        let slide_urls = Self::scrape_page(&html);
        let slides = slide_urls
            .into_iter()
            .map(|slide_url| Slide {
                lecture_page: Arc::clone(&lecture_page),
                slide_url,
            })
            .collect();
        Ok(slides)
    }

    pub async fn content(&self, client: &Client) -> anyhow::Result<SlideContent> {
        let slide = Arc::new(self.clone());
        let content = SlideContent::fetch_content(client, &self.slide_url).await?;
        Ok(SlideContent { slide, content })
    }
}

#[derive(Debug, Clone)]
pub struct SlideContent {
    pub slide: Arc<Slide>,
    pub content: Vec<Svg>,
}

impl SlideContent {
    pub async fn fetch_content(client: &Client, slide_url: &str) -> anyhow::Result<Vec<Svg>> {
        if !check_logged_in_google(client).await? {
            return Err(anyhow::anyhow!("Not logged in"));
        }
        let svg_regex = Regex::new(r#"\\x3csvg.*?\\x3c\\/svg\\x3e"#)?;
        let response = client.get(slide_url).send().await?;
        let body = response.text().await?;
        let svgs = svg_regex
            .find_iter(&body)
            .map(|m| m.as_str().to_string())
            .map(|s| unicode_escape::decode(&*s.replace(r"\/", "/")).unwrap())
            .map(|s| Svg::new(s))
            .collect();
        Ok(svgs)
    }

    pub fn extract_image_url(&self) -> Vec<String> {
        self.content
            .iter()
            .map(|svg| svg.extract_image_url())
            .flatten()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    pub async fn fetch_images(&self, client: &Client) -> anyhow::Result<HashMap<String, Bytes>> {
        let futures = self.content.iter().map(|svg| svg.fetch_images(client));
        let results = futures::future::join_all(futures).await;
        let images = results.into_iter().collect::<anyhow::Result<Vec<_>>>()?;
        let images = images
            .into_iter()
            .map(|image| image.into_iter())
            .flatten()
            .collect();
        Ok(images)
    }

    pub fn embed_text(&self) -> anyhow::Result<Self> {
        let content = self
            .content
            .iter()
            .map(|svg| svg.embed_text())
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(Self {
            slide: Arc::clone(&self.slide),
            content,
        })
    }

    pub fn embed_images(&self, images: &HashMap<String, String>) -> anyhow::Result<Self> {
        let content = self
            .content
            .iter()
            .map(|svg| svg.embed_images(images))
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(Self {
            slide: Arc::clone(&self.slide),
            content,
        })
    }
}
