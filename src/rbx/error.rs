//! Error handling.
use crate::rbx::ds_error::DataStoreErrorResponse;

/// `rbxcloud` error.
#[derive(Debug)]
pub enum Error {
    /// An error occurred regarding reading a file from the file system.
    FileLoadError(String),

    /// Failed to infer asset type.
    InferAssetTypeError(String),

    /// A non-OK HTTP status was returned.
    HttpStatusError { code: u16, msg: String },

    /// An error within the `reqwest` module occurred.
    ReqwestError(reqwest::Error),

    /// An IO error occurred.
    IOError(std::io::Error),

    /// A JSON serialization error occurred.
    SerdeJsonError(serde_json::Error),

    /// A DataStore error occurred.
    DataStoreError(DataStoreErrorResponse),

    /// Failed to parse a float.
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
            Self::FileLoadError(s) => write!(f, "failed to read file: {s}"),
            Self::InferAssetTypeError(s) => write!(f, "failed to infer asset type: {s}"),
            Self::HttpStatusError { code, msg } => write!(f, "http {code}: {msg}"),
            Self::ReqwestError(e) => write!(f, "{e:?}"),
            Self::IOError(e) => write!(f, "{e:?}"),
            Self::SerdeJsonError(e) => write!(f, "{e:?}"),
            Self::DataStoreError(e) => write!(f, "{e:?}"),
            Self::ParseFloatError(e) => write!(f, "{e:?}"),
        }
    }
}
