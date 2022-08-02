use super::datastore::DataStoreErrorResponse;

#[derive(Debug)]
pub enum Error {
    FileLoadError(String),
    HttpStatusError { code: u16, msg: String },
    ReqwestError(reqwest::Error),
    IOError(std::io::Error),
    SerdeJsonError(serde_json::Error),
    DataStoreError(DataStoreErrorResponse),
    ParseFloatError(std::num::ParseFloatError),
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::ReqwestError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJsonError(e)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Self::ParseFloatError(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::FileLoadError(s) => write!(f, "failed to read file: {}", s),
            Self::HttpStatusError { code, msg } => write!(f, "http {}: {}", code, msg),
            Self::ReqwestError(e) => write!(f, "{:?}", e),
            Self::IOError(e) => write!(f, "{:?}", e),
            Self::SerdeJsonError(e) => write!(f, "{:?}", e),
            Self::DataStoreError(e) => write!(f, "{:?}", e),
            Self::ParseFloatError(e) => write!(f, "{:?}", e),
        }
    }
}
