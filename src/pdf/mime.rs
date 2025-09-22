use super::error::ImageConvertError;

#[derive(Debug, Clone)]
pub enum Mime {
    Svg,
    Png,
    Jpeg,
    Gif,
    Webp,
    Bmp,
    Avif,
}

impl From<Mime> for &'static str {
    fn from(val: Mime) -> Self {
        match val {
            Mime::Svg => "image/svg+xml",
            Mime::Png => "image/png",
            Mime::Jpeg => "image/jpeg",
            Mime::Gif => "image/gif",
            Mime::Webp => "image/webp",
            Mime::Bmp => "image/bmp",
            Mime::Avif => "image/avif",
        }
    }
}

impl TryFrom<&'static str> for Mime {
    type Error = ImageConvertError;

    fn try_from(mime_str: &'static str) -> Result<Self, Self::Error> {
        match mime_str {
            "image/svg+xml" => Ok(Mime::Svg),
            "image/png" => Ok(Mime::Png),
            "image/jpeg" => Ok(Mime::Jpeg),
            "image/gif" => Ok(Mime::Gif),
            "image/webp" => Ok(Mime::Webp),
            "image/bmp" => Ok(Mime::Bmp),
            "image/avif" => Ok(Mime::Avif),
            _ => Err(ImageConvertError::UnsupportedFormat),
        }
    }
}

impl TryFrom<&[u8]> for Mime {
    type Error = ImageConvertError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        // まずSVGかどうかをチェック
        if Mime::is_svg(bytes) {
            return Ok(Mime::Svg);
        }

        // SVGでなければinferクレートに判定を委譲
        match infer::get(bytes) {
            Some(info) => info.mime_type().try_into(),
            None => Err(ImageConvertError::UnsupportedFormat),
        }
    }
}

impl Mime {
    /// SVGかどうかを判定（パブリック関数）
    pub fn is_svg(bytes: &[u8]) -> bool {
        let content = std::str::from_utf8(bytes).unwrap_or("");
        let trimmed = content.trim_start();

        // XMLヘッダー + SVGルート要素、または直接SVGタグ
        (trimmed.starts_with("<?xml") && content.contains("<svg")) || trimmed.starts_with("<svg")
    }
}
