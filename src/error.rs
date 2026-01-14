/// Errors that can occur during template processing
#[derive(Debug)]
pub enum TemplateError {
    Io(std::io::Error),
    JsonParse(serde_json::Error),
    TeraError(tera::Error),
    InvalidArgument(String),
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TemplateError::Io(e) => write!(f, "IO error: {}", e),
            TemplateError::JsonParse(e) => write!(f, "JSON parsing error: {}", e),
            TemplateError::TeraError(e) => write!(f, "Template error: {}", e),
            TemplateError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
        }
    }
}

impl std::error::Error for TemplateError {}

impl From<std::io::Error> for TemplateError {
    fn from(err: std::io::Error) -> Self {
        TemplateError::Io(err)
    }
}

impl From<serde_json::Error> for TemplateError {
    fn from(err: serde_json::Error) -> Self {
        TemplateError::JsonParse(err)
    }
}

impl From<tera::Error> for TemplateError {
    fn from(err: tera::Error) -> Self {
        TemplateError::TeraError(err)
    }
}

pub type Result<T> = std::result::Result<T, TemplateError>;
