#[derive(Debug)]
pub enum Error {
    Serde(serde_yaml::Error),
    Io(std::io::Error),
    FileNotFound(String),
    Compile(String),
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Self::Serde(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl Error {
    pub fn file_not_found<T: std::fmt::Display>(message: T) -> Self {
        Self::FileNotFound(message.to_string())
    }

    pub fn compile<T: std::fmt::Display>(message: T) -> Self {
        Self::Compile(message.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Error::Serde(error) => write!(f, "serde error: {}", error),
            Error::Io(error) => write!(f, "io error: {}", error),
            Error::FileNotFound(message) => write!(f, "file not found: {}", message),
            Error::Compile(message) => write!(f, "compile error: {}", message),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
