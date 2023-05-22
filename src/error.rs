// Import error messages
use crate::error_message::*;

use colored::Colorize;

pub type BlobResult<T> = Result<T, BlobdlError>;

/// ### The all-encompassing error type used in this project
/// ## Implements From
/// For the Errors std::io::Error and ParseIntError
/// ## Contains
/// Errors for everything that can go wrong in the project
///
/// Useless comments go brr
#[derive(Debug)]
pub enum BlobdlError {
    QueryNotFound,
    UnknownUrl,
    UnsupportedWebsite,
    DomainNotFound,
    UrlParsingError,
    UnknownIssue,
    MissingArgument,
    JsonSerializationError,
    Utf8Error,
    //tmp
    SerdeError(serde_json::Error),
    IoError(std::io::Error),
    UnsupportedFeature,
}

impl BlobdlError {
    // Output an error message according to the error at hand
    pub fn report(&self) {
        println!("\n{}\n", crate::USAGE_MSG);
        print!("{}: ", "error".bold().red());
        match self {
            BlobdlError::QueryNotFound => println!("{}", BROKEN_URL_ERR),

            // todo this error is unused
            BlobdlError::UnknownUrl=> println!("{}", BROKEN_URL_ERR),

            BlobdlError::UnsupportedWebsite=> println!("{}", UNSUPPORTED_WEBSITE_ERR),

            BlobdlError::DomainNotFound=> println!("{}", BROKEN_URL_ERR),

            // The link appears to be completely broken
            BlobdlError::UrlParsingError=> println!("{}", BROKEN_URL_ERR),

            BlobdlError::UnknownIssue=> println!("{}", UNKNOWN_ISSUE_ERR),

            BlobdlError::MissingArgument=> println!("{}", MISSING_ARGUMENT_ERR),

            BlobdlError::JsonSerializationError=> println!("{}", JSON_SERIALIZATION_ERR),

            BlobdlError::Utf8Error=> println!("{}", UTF8_ERR),

            BlobdlError::SerdeError(err)=> println!("{} {}", SERDE_ERR, err),

            BlobdlError::IoError(err)=> println!("{} {}", IO_ERR, err),

            BlobdlError::UnsupportedFeature=> println!("{}", UNSUPPORTED_FEATURE_ERR),
        }
        println!("{}", crate::SEE_HELP_PAGE);
    }
}


// Implementing conversions and boilerplate

impl std::fmt::Display for BlobdlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Hi :) I am the BlobdlError default message, I shouldn't show up, if you see me please report me to the github page")
    }
}
impl std::error::Error for BlobdlError {}

impl From<std::io::Error> for BlobdlError {
    fn from(err: std::io::Error) -> Self {
        BlobdlError::IoError(err)
    }
}

impl From<std::str::Utf8Error> for BlobdlError {
    fn from(_: std::str::Utf8Error) -> Self {
        BlobdlError::Utf8Error
    }
}
