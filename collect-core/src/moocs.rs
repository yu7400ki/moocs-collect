use std::collections::{HashMap, HashSet};

use base64::{engine::general_purpose, Engine};
use lol_html::{element, HtmlRewriter, Settings};
use regex::Regex;
use reqwest::Client;
use scraper::Html;

use crate::iniad::{check_logged_in_google, check_logged_in_moocs};

#[derive(Debug, Clone)]
pub struct Url {
    pub year: Option<u32>,
    pub course_id: Option<String>,
    pub lecture_id: Option<String>,
    pub page: Option<String>,
}

impl Url {
    pub const BASE_URL: &'static str = "https://moocs.iniad.org";
    pub const COURSE_URL: &'static str = "https://moocs.iniad.org/courses";

    pub fn parse(url: String) -> Self {
        let url = match url.starts_with("http") {
            true => url,
            false => format!("{}{}", Self::BASE_URL, url),
        };
        let url = url.parse::<reqwest::Url>().unwrap();
        let path = url.path_segments().unwrap().collect::<Vec<_>>();
        let year = path.get(1).map(|s| s.parse::<u32>().unwrap());
        let course_id = path.get(2).map(|s| s.to_string());
        let lecture_id = path.get(3).map(|s| s.to_string());
        let page = path.get(4).map(|s| s.to_string());
        Self {
            year,
            course_id,
            lecture_id,
            page,
        }
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

    pub async fn list(client: &Client, year: Option<u32>) -> anyhow::Result<Vec<Self>> {
        check_logged_in_moocs(client).await??;
        let url = match year {
            Some(year) => format!("{}/{}", Url::COURSE_URL, year),
            None => Url::COURSE_URL.to_string(),
        };
        let response = client.get(&url).send().await?;
        let document = Html::parse_document(&response.text().await?);
        let courses = document
            .select(&scraper::Selector::parse(".media").unwrap())
            .map(|course| {
                let name = course
                    .select(&scraper::Selector::parse(".media-body h4.media-heading").unwrap())
                    .next()
                    .and_then(|name| Some(name.text().collect::<String>()))
                    .unwrap()
                    .trim()
                    .to_string();
                let href = course
                    .select(&scraper::Selector::parse("a").unwrap())
                    .next()
                    .and_then(|href| Some(href.value().attr("href").unwrap().to_string()))
                    .unwrap();
                let url = Url::parse(href);
                Course {
                    year: url.year.unwrap(),
                    id: url.course_id.unwrap(),
                    name,
                }
            })
            .collect();
        Ok(courses)
    }

    pub async fn lectures(&self, client: &Client) -> anyhow::Result<Vec<Lecture>> {
        let response = client.get(&self.url()).send().await?;
        let document = Html::parse_document(&response.text().await?);
        let lectures = document
            .select(&scraper::Selector::parse("ul.sidebar-menu li.treeview").unwrap())
            .map(|treeview| {
                let group = treeview
                    .select(&scraper::Selector::parse("span.sidebar-menu-text").unwrap())
                    .next()
                    .and_then(|group| Some(group.text().collect::<String>()))
                    .unwrap()
                    .trim()
                    .to_string();
                let lectures = treeview
                    .select(&scraper::Selector::parse("ul.treeview-menu li").unwrap())
                    .map(|menu| {
                        let anchor = menu
                            .select(&scraper::Selector::parse("a").unwrap())
                            .next()
                            .unwrap();
                        let name = anchor.text().collect::<String>();
                        let href = anchor.value().attr("href").unwrap();
                        let url = Url::parse(href.to_string());
                        Lecture {
                            course: self,
                            id: url.lecture_id.unwrap(),
                            name: name.trim().to_string(),
                            group: group.clone(),
                        }
                    })
                    .collect::<Vec<Lecture>>();
                lectures
            })
            .flatten()
            .collect();
        Ok(lectures)
    }

    pub fn url(&self) -> String {
        format!("{}/{}/{}", Url::COURSE_URL, self.year, self.id)
    }
}

impl ToString for Course {
    fn to_string(&self) -> String {
        format!("{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct Lecture<'a> {
    pub course: &'a Course,
    pub id: String,
    pub name: String,
    pub group: String,
}

impl<'a> Lecture<'a> {
    pub fn url(&self) -> String {
        format!(
            "https://moocs.iniad.org/courses/{}/{}/{}",
            self.course.year, self.course.id, self.id
        )
    }

    pub async fn pages(&self, client: &Client) -> anyhow::Result<Vec<LecturePage>> {
        let response = client.get(&self.url()).send().await?;
        let current_url = response.url().to_string();
        let document = Html::parse_document(&response.text().await?);
        let pagination = document
            .select(&scraper::Selector::parse("ul.pagination li").unwrap())
            .collect::<Vec<_>>();
        let pagination = &pagination[1..pagination.len() - 1];
        let pages = pagination
            .iter()
            .map(|li| {
                let anchor = li
                    .select(&scraper::Selector::parse("a").unwrap())
                    .next()
                    .unwrap();
                let title = anchor.attr("title").unwrap().trim().to_string();
                let href = anchor.value().attr("href").unwrap().to_string();
                let href = match &*href {
                    "#" => current_url.clone(),
                    _ => href,
                };
                let url = Url::parse(href);
                LecturePage {
                    lecture: self,
                    page: url.page.unwrap(),
                    title,
                }
            })
            .collect();
        Ok(pages)
    }
}

impl<'a> ToString for Lecture<'a> {
    fn to_string(&self) -> String {
        format!("{} - {}", self.group, self.name)
    }
}

#[derive(Debug, Clone)]
pub struct LecturePage<'a> {
    pub lecture: &'a Lecture<'a>,
    pub page: String,
    pub title: String,
}

impl<'a> LecturePage<'a> {
    pub fn url(&self) -> String {
        format!(
            "https://moocs.iniad.org/courses/{}/{}/{}/{}",
            self.lecture.course.year, self.lecture.course.id, self.lecture.id, self.page
        )
    }

    async fn slide(client: &Client, url: &str) -> anyhow::Result<Vec<String>> {
        let svg_regex = Regex::new(r#"\\x3csvg.*?\\x3c\\/svg\\x3e"#)?;
        let response = client.get(url).send().await?;
        let body = response.text().await?;
        let svgs = svg_regex
            .find_iter(&body)
            .map(|m| m.as_str().to_string())
            .map(|s| unicode_escape::decode(&*s.replace(r"\/", "/")).unwrap())
            .collect();
        Ok(svgs)
    }

    async fn iframes(&self, client: &Client) -> anyhow::Result<Vec<String>> {
        let embed_url_regex =
            Regex::new(r#"^https://docs.google.com/presentation/d/.*?/embed\?"#).unwrap();
        let response = client.get(&self.url()).send().await?;
        let document = Html::parse_document(&response.text().await?);
        let iframes = document
            .select(&scraper::Selector::parse("iframe").unwrap())
            .map(|iframe| iframe.value().attr("src").unwrap().to_string())
            .filter(|src| embed_url_regex.is_match(src))
            .collect::<Vec<_>>();
        Ok(iframes)
    }

    pub async fn slides(&self, client: &Client) -> anyhow::Result<Vec<Slide>> {
        check_logged_in_google(client).await??;
        let iframes = self.iframes(client).await?;
        let slides = iframes
            .iter()
            .map(|src| Self::slide(client, src))
            .collect::<Vec<_>>();
        let slides = futures::future::join_all(slides)
            .await
            .into_iter()
            .flat_map(|result| result)
            .map(|content| Slide {
                lecture_page: self,
                content,
            })
            .collect::<Vec<_>>();
        Ok(slides)
    }

    pub async fn has_slide(&self, client: &Client) -> anyhow::Result<bool> {
        let iframes = self.iframes(client).await?;
        Ok(iframes.len() > 0)
    }
}

impl<'a> ToString for LecturePage<'a> {
    fn to_string(&self) -> String {
        format!("{}", self.title)
    }
}

#[derive(Debug, Clone)]
pub enum Mime {
    Svg,
    Png,
    Jpeg,
    Gif,
    Webp,
}

impl Into<&'static str> for Mime {
    fn into(self) -> &'static str {
        match self {
            Mime::Svg => "image/svg+xml",
            Mime::Png => "image/png",
            Mime::Jpeg => "image/jpeg",
            Mime::Gif => "image/gif",
            Mime::Webp => "image/webp",
        }
    }
}

impl From<&[u8]> for Mime {
    fn from(bytes: &[u8]) -> Self {
        match bytes {
            [0x89, 0x50, 0x4E, 0x47, ..] => Mime::Png,
            [0xFF, 0xD8, ..] => Mime::Jpeg,
            [0x47, 0x49, 0x46, 0x38, ..] => Mime::Gif,
            [0x52, 0x49, 0x46, 0x46, ..] => Mime::Webp,
            _ => Mime::Svg,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Slide<'a> {
    pub lecture_page: &'a LecturePage<'a>,
    pub content: Vec<String>,
}

impl<'a> Slide<'a> {
    async fn src_to_base64(src: &str, client: &Client) -> anyhow::Result<String> {
        let response = client.get(src).send().await?;
        let bytes = response.bytes().await?;
        let mime = Mime::from(bytes.as_ref());
        let mime: &str = mime.into();
        let base64 = general_purpose::STANDARD.encode(&bytes);
        let base64 = format!("data:{};base64,{}", mime, base64);
        Ok(base64)
    }

    async fn extract_image(body: &str, client: &Client) -> HashMap<String, String> {
        let image_regex = Regex::new(r#"<image\s+(?:[^>]*?\s+)?xlink:href="([^"]*)""#).unwrap();
        let hrefs = image_regex
            .captures_iter(body)
            .map(|captures| captures.get(1).unwrap().as_str())
            .collect::<HashSet<_>>()
            .into_iter()
            .filter(|href| href.starts_with("http") && href.parse::<reqwest::Url>().is_ok())
            .collect::<Vec<_>>();
        let base64 = hrefs
            .iter()
            .map(|src| Self::src_to_base64(src, client))
            .collect::<Vec<_>>();
        let base64 = futures::future::join_all(base64).await;
        let images = hrefs
            .into_iter()
            .zip(base64)
            .filter_map(|(href, base64)| match base64 {
                Ok(base64) => Some((href.to_string(), base64)),
                Err(_) => None,
            })
            .collect();
        images
    }

    fn embed_image_(slide: &String, images: &HashMap<String, String>) -> anyhow::Result<String> {
        let mut output = vec![];

        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![element!("image", |el| {
                    if let Some(src) = el.get_attribute("xlink:href") {
                        if let Some(base64) = images.get(&src) {
                            el.set_attribute("xlink:href", base64)?;
                        }
                    }
                    Ok(())
                })],
                ..Settings::default()
            },
            |c: &[u8]| output.extend_from_slice(c),
        );

        rewriter.write(slide.as_bytes())?;
        rewriter.end()?;

        let slide = String::from_utf8(output)?;
        Ok(slide)
    }

    pub async fn embed_image(&self, client: &Client) -> anyhow::Result<Self> {
        let contents = self.content.join("\n");
        let images = Self::extract_image(&contents, client).await;
        let content = self
            .content
            .iter()
            .map(|slide| Self::embed_image_(slide, &images))
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(Slide {
            lecture_page: self.lecture_page,
            content,
        })
    }

    pub fn iter(&self) -> SlideIter {
        SlideIter {
            iter: self.content.iter(),
        }
    }
}

impl IntoIterator for Slide<'_> {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter()
    }
}

pub struct SlideIter<'a> {
    iter: std::slice::Iter<'a, String>,
}

impl<'a> Iterator for SlideIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
