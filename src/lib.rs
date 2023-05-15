pub mod parser;
pub mod assembling;
pub mod analyzer;
pub mod dispatcher;

// Hard-coded error messages
const JSON_PARSING_ERROR: &str = "The JSON including a video's format information contained non-UTF8 characters, please report this issue";

const JSON_SERIALIZATION_ERROR: &str = "Problem serializing the video's formats from JSON";

// Hard-coded prompts
const BEST_QUALITY_PROMPT: &str = "Best possible quality for each video [ffmpeg required]";

const SMALLEST_QUALITY_PROMPT: &str = "Smallest file size for each video";

const YT_FORMAT_PROMPT: &str = "Choose a file format for each video (only youtube-supported formats for this video) [no ffmpeg]";

const CONVERT_FORMAT_PROMPT: &str = "Choose a file format for each video (any format) [ffmpeg required]";

// Temporary placement in the module system
/// ### The all-encompassing error type used in this project
/// ## Implements From
/// For the Errors std::io::Error and ParseIntError
/// ## Contains
/// Errors for everything that can go wrong in the project
///
/// Useless comments go brr
#[derive(Debug)]
enum BlobdlError {
    UnknownUrl,
    UnsupportedWebsite,
    DomainNotFound,
    UrlParsingError,
    UnknownIssue,
    JsonSerializationError,
    IoError(std::io::Error),
    IntParsingError(std::num::ParseIntError),
}

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

impl From<std::num::ParseIntError> for BlobdlError {
    fn from(err: std::num::ParseIntError) -> Self {
        BlobdlError::IntParsingError(err)
    }
}