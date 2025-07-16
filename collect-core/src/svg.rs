use base64::{engine::general_purpose, Engine};
use bytes::Bytes;
use lol_html::{element, html_content::ContentType, HtmlRewriter, Settings};
use regex::Regex;
use reqwest::Client;
use std::collections::HashMap;

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

pub struct Config {
    embed_images: bool,
    embed_text: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            embed_images: true,
            embed_text: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Svg {
    pub src: String,
}

impl Svg {
    pub fn new(src: String) -> Self {
        Self { src }
    }

    pub fn extract_image_url(&self) -> Vec<String> {
        let image_regex = Regex::new(r#"<image\s+(?:[^>]*?\s+)?xlink:href="([^"]*)""#).unwrap();
        image_regex
            .captures_iter(&self.src)
            .filter_map(|captures| captures.get(1))
            .map(|capture| capture.as_str().to_string())
            .filter(|href| href.starts_with("http") && href.parse::<reqwest::Url>().is_ok())
            .collect::<Vec<_>>()
    }

    pub async fn fetch_images(&self, client: &Client) -> anyhow::Result<HashMap<String, Bytes>> {
        let image_urls = self.extract_image_url();

        let images =
            futures::future::join_all(image_urls.iter().map(|url| fetch_image(url, client)))
                .await
                .into_iter()
                .collect::<anyhow::Result<Vec<_>>>()?;

        let images = image_urls
            .into_iter()
            .zip(images.into_iter())
            .collect::<HashMap<_, _>>();

        Ok(images)
    }

    pub fn embed_text(&mut self) -> anyhow::Result<()> {
        let mut output = vec![];

        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![element!("g[role='img'][aria-label]", |el| {
                    if let Some(aria_label) = el.get_attribute("aria-label") {
                        el.append(
                            "<text stroke='none' fill='transparent' transform='scale(0.01)'>",
                            ContentType::Html,
                        );
                        el.append(&aria_label, ContentType::Text);
                        el.append("</text>", ContentType::Html);
                    }
                    Ok(())
                })],
                ..Settings::default()
            },
            |c: &[u8]| output.extend_from_slice(c),
        );

        rewriter.write(self.src.as_bytes())?;
        rewriter.end()?;

        let svg = String::from_utf8(output)?;
        self.src = svg;
        Ok(())
    }

    pub fn embed_images(&mut self, images: &HashMap<String, Bytes>) -> anyhow::Result<()> {
        let mut output = vec![];

        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![element!("image", |el| {
                    if let Some(src) = el.get_attribute("xlink:href") {
                        if let Some(bytes) = images.get(&src) {
                            let base64 = encode_base64(bytes.as_ref());
                            el.set_attribute("xlink:href", &base64)?;
                        }
                    }
                    Ok(())
                })],
                ..Settings::default()
            },
            |c: &[u8]| output.extend_from_slice(c),
        );

        rewriter.write(self.src.as_bytes())?;
        rewriter.end()?;

        let svg = String::from_utf8(output)?;
        self.src = svg;
        Ok(())
    }

    pub async fn process(&self, config: &Config, client: &Client) -> anyhow::Result<Self> {
        let mut new = self.clone();

        if config.embed_images {
            let images = self.fetch_images(client).await?;
            new.embed_images(&images)?;
        };

        if config.embed_text {
            new.embed_text()?;
        };

        Ok(new)
    }
}

async fn fetch_image(url: &str, client: &Client) -> anyhow::Result<Bytes> {
    let response = client.get(url).send().await?;
    Ok(response.bytes().await?)
}

fn encode_base64(bytes: &[u8]) -> String {
    let mime = Mime::from(bytes.as_ref());
    let mime: &str = mime.into();
    let base64 = general_purpose::STANDARD.encode(&bytes);
    let base64 = format!("data:{};base64,{}", mime, base64);
    base64
}
