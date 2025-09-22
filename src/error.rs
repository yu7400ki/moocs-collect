/// Main error type for the collect-core library
#[derive(Debug, thiserror::Error)]
pub enum CollectError {
    #[error("Network error: {message}")]
    Network {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Authentication failed: {reason}")]
    Authentication { reason: String },

    #[error("Parse error: {message}{}", context.as_ref().map(|c| format!(" ({})", c)).unwrap_or_default())]
    Parse {
        message: String,
        context: Option<String>,
    },

    #[error("Not found: {message}")]
    NotFound { message: String },

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("Timeout error: operation timed out after {duration_ms}ms")]
    Timeout { duration_ms: u64 },

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("IO error: {message}")]
    Io {
        message: String,
        #[source]
        source: Option<std::io::Error>,
    },

    #[error("PDF generation error: {message}")]
    PdfGeneration { message: String },

    #[error("SVG processing error: {message}")]
    SvgProcessing { message: String },
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, CollectError>;

impl CollectError {
    pub fn network<E>(message: impl Into<String>, source: Option<E>) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Network {
            message: message.into(),
            source: source.map(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        }
    }

    pub fn authentication(reason: impl Into<String>) -> Self {
        Self::Authentication {
            reason: reason.into(),
        }
    }

    pub fn parse(message: impl Into<String>, context: Option<String>) -> Self {
        Self::Parse {
            message: message.into(),
            context,
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound {
            message: message.into(),
        }
    }

    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    pub fn timeout(duration_ms: u64) -> Self {
        Self::Timeout { duration_ms }
    }

    pub fn rate_limit() -> Self {
        Self::RateLimit
    }

    pub fn io<E>(message: impl Into<String>, source: Option<E>) -> Self
    where
        E: Into<std::io::Error>,
    {
        Self::Io {
            message: message.into(),
            source: source.map(Into::into),
        }
    }

    pub fn pdf_generation(message: impl Into<String>) -> Self {
        Self::PdfGeneration {
            message: message.into(),
        }
    }

    pub fn svg_processing(message: impl Into<String>) -> Self {
        Self::SvgProcessing {
            message: message.into(),
        }
    }
}

// Error conversions from external libraries
impl From<reqwest::Error> for CollectError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            CollectError::Timeout { duration_ms: 30000 }
        } else if error.is_connect() {
            CollectError::Network {
                message: "Connection failed".to_string(),
                source: Some(Box::new(error)),
            }
        } else if error.is_request() {
            CollectError::Network {
                message: "Request failed".to_string(),
                source: Some(Box::new(error)),
            }
        } else {
            CollectError::Network {
                message: error.to_string(),
                source: Some(Box::new(error)),
            }
        }
    }
}

impl From<std::io::Error> for CollectError {
    fn from(error: std::io::Error) -> Self {
        CollectError::Io {
            message: error.to_string(),
            source: Some(error),
        }
    }
}

impl From<lopdf::Error> for CollectError {
    fn from(error: lopdf::Error) -> Self {
        CollectError::PdfGeneration {
            message: format!("PDF error: {}", error),
        }
    }
}

impl From<regex::Error> for CollectError {
    fn from(error: regex::Error) -> Self {
        CollectError::Parse {
            message: format!("Regex error: {}", error),
            context: None,
        }
    }
}
