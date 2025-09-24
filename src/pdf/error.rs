use thiserror::Error;

#[derive(Error, Debug)]
pub enum PdfConversionError {
    #[error("SVG parsing failed: {0}")]
    SvgParsing(String),
    #[error("PDF generation failed: {0}")]
    PdfGeneration(String),
    #[error("Font loading failed")]
    FontLoading,
    #[error("Document merge failed: {0}")]
    DocumentMerge(String),
    #[error("Image processing failed: {0}")]
    ImageProcessing(#[from] reqwest::Error),
    #[error("HTML rewriting failed: {0}")]
    HtmlRewriting(#[from] lol_html::errors::RewritingError),
    #[error("UTF-8 conversion failed: {0}")]
    Utf8Conversion(#[from] std::string::FromUtf8Error),
    #[error("PDF document loading failed: {0}")]
    PdfLoading(#[from] lopdf::Error),
}

#[derive(Error, Debug)]
pub enum ImageConvertError {
    #[error("Unsupported image format")]
    UnsupportedFormat,
}
