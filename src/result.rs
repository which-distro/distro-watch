//! `Error` and `Result` definitions.

use reqwest::Error as ReqWestError;
use thiserror::Error;

/// The Errors that may occur while using `distro-watch`.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] ReqWestError),
    #[error("Error occurred while parsing the HTML document: {0:?}")]
    ParsingHtmlError(Vec<String>),
    #[error("DistroWatch API changed, plz file a issue at: https://github.com/which-distro/distro-watch/issues")]
    DistroWatchApiChanged,
}

/// `distro-watch`'s customized `Result` type.
pub type Result<T> = std::result::Result<T, Error>;
