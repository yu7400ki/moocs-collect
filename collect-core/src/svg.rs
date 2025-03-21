use bytes::Bytes;
use lol_html::{element, html_content::ContentType, HtmlRewriter, Settings};
use regex::Regex;
use reqwest::Client;
use std::collections::HashMap;

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

    pub fn embed_text(&self) -> anyhow::Result<Self> {
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
        Ok(Self::new(svg))
    }

    pub fn embed_images(&self, images: &HashMap<String, String>) -> anyhow::Result<Self> {
        let mut output = vec![];

        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![element!("image", |el| {
                    if let Some(src) = el.get_attribute("xlink:href") {
                        if let Some(embed) = images.get(&src) {
                            el.set_attribute("xlink:href", &embed)?;
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
        Ok(Self::new(svg))
    }
}

async fn fetch_image(url: &str, client: &Client) -> anyhow::Result<Bytes> {
    let response = client.get(url).send().await?;
    Ok(response.bytes().await?)
}
