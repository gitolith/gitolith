use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
	#[error("IO error")]
	IoError(#[from] std::io::Error),
	#[cfg(feature = "hyper-http")]
	#[error("Hyper HTTP error")]
	HyperHttpError(String),
	#[error("unknown error")]
	Unknown,
}

#[cfg(feature = "hyper-http")]
impl From<hyper::error::Error> for AppError {
	fn from(error: hyper::Error) -> Self {
		AppError::HyperHttpError(error.to_string())
	}
}

#[cfg(feature = "hyper-http")]
impl From<hyper::http::Error> for AppError {
	fn from(error: hyper::http::Error) -> Self {
		AppError::HyperHttpError(error.to_string())
	}
}

pub type Result<T> = std::result::Result<T, AppError>;
