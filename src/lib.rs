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

// Error messages

const MISSING_URL_SECTION_ERR: &str = "The url that was provided didn't contain a query, try using a regular youtube url";
const UNKNOWN_URL_ERR: &str = "Unknown url does not yet have a use";
const UNSUPPORTED_WEBSITE_ERR: &str = "Currently blob-dl only supports downloading youtube videos or playlists";
const UNKNOWN_ISSUE_ERR: &str = "Congrats! You ran into an unknown issue, please file a report on blob-dl's github page :)";
// todo fix this message
const MISSING_ARGUMENT_ERR: &str = "A url is required for blob-dl to function [[please rephrase this error]]";
const JSON_SERIALIZATION_ERR: &str = "There was a problem serializing this video's format information";
const UTF8_ERR: &str = "This video's format information contained non-UTF8 characters and broke the parser, best and worst quality should still work!";
const SERDE_ERR: &str = "Serde ran into a problem when serializing this video's format information: ";
const IO_ERR: &str = "There was an IO error: ";
const UNSUPPORTED_FEATURE_ERR: &str = "Currently with blob-dl you can only download videos or playlists, not media such as series";

// Temporary placement in the module system
type BlobResult<T> = Result<T, BlobdlError>;

/// ### The all-encompassing error type used in this project
/// ## Implements From
/// For the Errors std::io::Error and ParseIntError
/// ## Contains
/// Errors for everything that can go wrong in the project
///
/// Useless comments go brr
#[derive(Debug)]
enum BlobdlError {
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
        match self {
            BlobdlError::QueryNotFound => println!("{}", MISSING_URL_SECTION_ERR),

            // todo this error is unused
            BlobdlError::UnknownUrl=> println!("{}", UNKNOWN_URL_ERR),

            BlobdlError::UnsupportedWebsite=> println!("{}", UNKNOWN_URL_ERR),

            BlobdlError::DomainNotFound=> println!("{}", MISSING_URL_SECTION_ERR),

            // The link appears to be completely broken
            BlobdlError::UrlParsingError=> println!("{}", MISSING_URL_SECTION_ERR),

            BlobdlError::UnknownIssue=> println!("{}", UNKNOWN_ISSUE_ERR),

            BlobdlError::MissingArgument=> println!("{}", MISSING_ARGUMENT_ERR),

            BlobdlError::JsonSerializationError=> println!("{}", JSON_SERIALIZATION_ERR),

            BlobdlError::Utf8Error=> println!("{}", UTF8_ERR),

            BlobdlError::SerdeError(err)=> println!("{} {}", SERDE_ERR, err),

            BlobdlError::IoError(err)=> println!("{} {}", IO_ERR, err),

            BlobdlError::UnsupportedFeature=> println!("{}", UNSUPPORTED_FEATURE_ERR),
        }
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

