#[derive(Debug)]
pub enum Error {
    SerdeError(serde_yaml::Error),
    IoError(std::io::Error),
    FileNotFound(String),
    TestBundleNotFound(String),
    CompileError(String, String),
    TimeLimitExceeded(String),
    RuntimeError(String),
    ExportError(String),
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Self::SerdeError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl Error {
    pub fn file_not_found<T: std::fmt::Display>(message: T) -> Self {
        Self::FileNotFound(message.to_string())
    }

    pub fn test_bundle_not_found<T: std::fmt::Display>(message: T) -> Self {
        Self::TestBundleNotFound(message.to_string())
    }

    pub fn compile_error<T: std::fmt::Display, U: std::fmt::Display>(info: T, message: U) -> Self {
        Self::CompileError(info.to_string(), message.to_string())
    }

    pub fn time_limit_exceeded<T: std::fmt::Display>(info: T) -> Self {
        Self::TimeLimitExceeded(info.to_string())
    }

    pub fn runtime_error<T: std::fmt::Display>(info: T) -> Self {
        Self::RuntimeError(info.to_string())
    }

    pub fn export_error<T: std::fmt::Display>(info: T) -> Self {
        Self::ExportError(info.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Error::SerdeError(error) => write!(f, "serde error: {}", error),
            Error::IoError(error) => write!(f, "io error: {}", error),
            Error::FileNotFound(info) => write!(f, "file not found: {}", info),
            Error::TestBundleNotFound(info) => write!(f, "test bundle not found: {}", info),
            Error::CompileError(info, message) => write!(f, "{} compile error: {}", info, message),
            Error::TimeLimitExceeded(info) => write!(f, "{} time limit exceeded", info),
            Error::RuntimeError(info) => write!(f, "{} runtime error", info),
            Error::ExportError(info) => write!(f, "export error: {}", info),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
