mod error;
mod mime;

use error::ImageConvertError;
pub use error::PdfConversionError;

use crate::domain::models::{ProcessedSvg, SlideContent};
use base64::{engine::general_purpose, Engine};
use bytes::Bytes;
use lol_html::{element, html_content::ContentType, HtmlRewriter, Settings};
use lopdf::{dictionary, Document, Object};
use mime::Mime;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use svg2pdf::{
    to_pdf,
    usvg::{Options, Tree},
    ConversionOptions, PageOptions,
};

/// PDF変換設定
#[derive(Debug, Clone)]
pub struct PreProcessConfig {
    pub embed_images: bool,
    pub embed_text: bool,
}

impl Default for PreProcessConfig {
    fn default() -> Self {
        Self {
            embed_images: true,
            embed_text: true,
        }
    }
}

/// 統合PDFコンバーター
#[derive(Default)]
pub struct PreProcessor {
    config: PreProcessConfig,
}

impl PreProcessor {
    /// 新しいコンバーターを作成
    pub fn new(config: PreProcessConfig) -> Self {
        Self { config }
    }
    /// SVGの前処理
    pub async fn preprocess(
        &self,
        client: &reqwest::Client,
        slide_content: &SlideContent,
    ) -> Result<SlideContent, PdfConversionError> {
        let futures = slide_content.svgs.iter().map(|svg| async move {
            let mut content = svg.content.clone();

            if self.config.embed_images {
                content = self.embed_images_in_svg(client, content).await?;
            }

            if self.config.embed_text {
                content = self.embed_text_in_svg(content)?;
            }

            Ok(content)
        });

        futures::future::try_join_all(futures)
            .await
            .map(|contents| {
                slide_content
                    .svgs
                    .iter()
                    .zip(contents)
                    .map(|(svg, content)| ProcessedSvg {
                        content,
                        index: svg.index,
                    })
                    .collect()
            })
            .map(|svgs| SlideContent {
                svgs,
                ..slide_content.clone()
            })
    }

    /// SVG内の画像を埋め込み
    async fn embed_images_in_svg(
        &self,
        client: &reqwest::Client,
        svg_content: String,
    ) -> Result<String, PdfConversionError> {
        let image_urls = self.extract_image_urls(&svg_content);
        let images = self.fetch_images(client, &image_urls).await?;

        let mut output = vec![];
        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![element!("image", |el| {
                    if let Some(src) = el.get_attribute("xlink:href") {
                        if let Some(bytes) = images.get(&src) {
                            if let Ok(base64) = self.encode_base64(bytes.as_ref()) {
                                el.set_attribute("xlink:href", &base64)?;
                            }
                        }
                    }
                    Ok(())
                })],
                ..Settings::default()
            },
            |c: &[u8]| output.extend_from_slice(c),
        );

        rewriter.write(svg_content.as_bytes())?;
        rewriter.end()?;

        Ok(String::from_utf8(output)?)
    }

    /// SVG内にテキストを埋め込み
    fn embed_text_in_svg(&self, svg_content: String) -> Result<String, PdfConversionError> {
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

        rewriter.write(svg_content.as_bytes())?;
        rewriter.end()?;

        Ok(String::from_utf8(output)?)
    }

    /// SVGから画像URLを抽出
    fn extract_image_urls(&self, svg_content: &str) -> Vec<String> {
        let image_regex = Regex::new(r#"<image\s+(?:[^>]*?\s+)?xlink:href="([^"]*)""#).unwrap();
        image_regex
            .captures_iter(svg_content)
            .filter_map(|captures| captures.get(1))
            .map(|capture| capture.as_str().to_string())
            .filter(|href| href.starts_with("http") && href.parse::<reqwest::Url>().is_ok())
            .collect()
    }

    /// 画像を取得
    async fn fetch_images(
        &self,
        client: &reqwest::Client,
        urls: &[String],
    ) -> Result<HashMap<String, Bytes>, PdfConversionError> {
        let futures = urls.iter().map(|url| async move {
            let bytes = self.fetch_image(client, url).await?;
            Ok((url.clone(), bytes))
        });

        futures::future::try_join_all(futures)
            .await
            .map(|results| results.into_iter().collect())
    }

    /// 単一画像を取得
    async fn fetch_image(
        &self,
        client: &reqwest::Client,
        url: &str,
    ) -> Result<Bytes, PdfConversionError> {
        let response = client.get(url).send().await?;
        Ok(response.bytes().await?)
    }

    /// Base64エンコード
    fn encode_base64(&self, bytes: &[u8]) -> Result<String, ImageConvertError> {
        let mime = Mime::try_from(bytes)?;
        let mime: &str = mime.into();
        let base64 = general_purpose::STANDARD.encode(bytes);
        Ok(format!("data:{};base64,{}", mime, base64))
    }
}

/// SlideContentをPDFに変換（メイン機能）
pub fn convert(slide: &SlideContent) -> Result<Document, PdfConversionError> {
    let options = Options::default();

    let documents = slide
        .svgs
        .par_iter()
        .map(|svg_content| {
            let conversion_options = ConversionOptions::default();
            let page_options = PageOptions::default();
            let tree = Tree::from_str(&svg_content.content, &options)
                .map_err(|e| PdfConversionError::SvgParsing(e.to_string()))?;
            let pdf = to_pdf(&tree, conversion_options, page_options)
                .map_err(|e| PdfConversionError::PdfGeneration(e.to_string()))?;
            Document::load_mem(&pdf).map_err(PdfConversionError::PdfLoading)
        })
        .collect::<Result<Vec<_>, _>>()?;

    merge_documents(documents)
}

/// 複数PDFドキュメントをマージ
fn merge_documents(documents: Vec<Document>) -> Result<Document, PdfConversionError> {
    let mut merged = Document::with_version("1.5");
    let mut document_pages = vec![];

    for mut document in documents {
        document.renumber_objects_with(merged.max_id + 1);
        merged.max_id = document.max_id;

        for (_, object_id) in document.get_pages() {
            let reference = Object::Reference(object_id);
            document_pages.push(reference);
        }

        merged.objects.extend(document.objects);
    }

    let count = document_pages.len() as u32;
    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => document_pages,
        "Count" => count,
    };
    let pages_id = merged.new_object_id();
    merged.objects.insert(pages_id, Object::Dictionary(pages));

    let catalog_id = merged.new_object_id();
    let catalog = dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    };

    merged
        .objects
        .insert(catalog_id, Object::Dictionary(catalog));
    merged.trailer.set("Root", catalog_id);

    Ok(merged)
}
