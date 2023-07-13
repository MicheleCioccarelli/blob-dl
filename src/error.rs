use std::io::Write;
// Import error messages
use crate::blobdl_error_message::*;
use crate::ui_prompts::*;

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
    UrlIndexParsingError,
    SerdeError(serde_json::Error),
    IoError(std::io::Error),
    QueryCouldNotBeParsed,
}

impl BlobdlError {
    // Output an error message according to the error at hand
    pub fn report(&self) {
        eprintln!("\n{}\n", USAGE_MSG);
        print!("{}: ", "ERROR".red());

        let _ = std::io::stdout().flush();

        match self {
            BlobdlError::QueryNotFound => eprintln!("{}", BROKEN_URL_ERR),

            BlobdlError::UnknownUrl=> eprintln!("{}", BROKEN_URL_ERR),

            BlobdlError::UnsupportedWebsite=> eprintln!("{}", UNSUPPORTED_WEBSITE_ERR),

            BlobdlError::DomainNotFound=> eprintln!("{}", BROKEN_URL_ERR),

            // The link appears to be completely broken
            BlobdlError::UrlParsingError=> eprintln!("{}", BROKEN_URL_ERR),

            BlobdlError::UnknownIssue=> eprintln!("{}", UNKNOWN_ISSUE_ERR),

            BlobdlError::MissingArgument=> eprintln!("{}", MISSING_ARGUMENT_ERR),

            BlobdlError::JsonSerializationError=> eprintln!("{}", JSON_SERIALIZATION_ERR),

            BlobdlError::Utf8Error=> eprintln!("{}", UTF8_ERR),

            BlobdlError::SerdeError(err)=> eprintln!("{} {}", SERDE_ERR, err),

            BlobdlError::IoError(err)=> eprintln!("{} {}", IO_ERR, err),

            BlobdlError::QueryCouldNotBeParsed => eprintln!("{}", URL_QUERY_COULD_NOT_BE_PARSED),

            BlobdlError::UrlIndexParsingError => eprintln!("{}", URL_INDEX_PARSING_ERR),
        }
        eprintln!("{}", SEE_HELP_PAGE);
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

// Used in run.rs
/// Stores the information found in yt-dlp's error-lines output
#[derive(Debug)]
pub(crate) struct YtdlpError {
    video_id: String,
    error_msg: String,
}

impl YtdlpError {
    pub fn video_id(&self) -> &String {
        &self.video_id
    }

    pub fn error_msg(&self) -> &String {
        &self.error_msg
    }
}

impl std::fmt::Display for YtdlpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result = format!("{} {}", "yt-video id:", self.video_id);
        result = format!("{}\n   {} {}\n", result, "Reason:", self.error_msg);

        write!(f, "{}", result)
    }
}

impl YtdlpError {
    /// Parses a YtdlpError object from a ytdlp line which contains an error
    pub fn from_error_output(error_line: &str) -> YtdlpError {
        // yt-dlp error line format: ERROR: [...] video_id: reason
        let mut section = error_line.split_whitespace();

        // Skip ERROR:
        section.next().unwrap();
        // Skip [...]
        section.next().unwrap();

        let mut video_id = section.next().unwrap();
        // Delete the trailing ':'
        video_id = &video_id[..video_id.len() - 1];

        // Concatenate together the error message and restore whitespace
        let error_msg = {
            let mut tmp = String::new();
            for word in section {
                tmp = tmp + " " + word;
            }
            tmp
        };

        YtdlpError { video_id: video_id.to_string(), error_msg }
    }
}