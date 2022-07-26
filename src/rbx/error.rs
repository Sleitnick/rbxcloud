use thiserror::Error;

#[derive(Error, Debug)]
pub enum RbxError {
	#[error("failed to load file: {0}")]
	FileLoadError(String),

	#[error("http {code:?} - {msg:?}")]
	HttpStatusError {
		code: u16,
		msg: String,
	}
}
