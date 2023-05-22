pub mod parser;
pub mod assembling;
pub mod analyzer;
pub mod dispatcher;
mod error;

// Hard-coded prompts
const BEST_QUALITY_PROMPT: &str = "Best possible quality for each video [ffmpeg required]";

const SMALLEST_QUALITY_PROMPT: &str = "Smallest file size for each video";

const YT_FORMAT_PROMPT: &str = "Choose a file format for each video (only youtube-supported formats for this video) [no ffmpeg]";

const CONVERT_FORMAT_PROMPT: &str = "Choose a file format for each video (any format) [ffmpeg required]";

const SEE_HELP_PAGE: &str = "Type blob-dl --help for a list of all the available options";

const USAGE_MSG: &str = "When the commands are complete make me a usage message plz :)";

mod error_message {
    pub const BROKEN_URL_ERR: &str = "The url provided wasn't recognized, try using a regular youtube url";

    pub const UNSUPPORTED_WEBSITE_ERR: &str = "Currently blob-dl only supports downloading youtube videos or playlists";

    pub const UNKNOWN_ISSUE_ERR: &str = "Congrats! You ran into an unknown issue, please file a report on blob-dl's github page :)";

    // todo fix this message
    pub const MISSING_ARGUMENT_ERR: &str = "A url is required for blob-dl to function [[please rephrase this error]]";

    pub const JSON_SERIALIZATION_ERR: &str = "There was a problem serializing this video's format information";

    pub const UTF8_ERR: &str = "This video's format information contained non-UTF8 characters and broke the parser, best and worst quality should still work!";

    pub const SERDE_ERR: &str = "Serde ran into a problem when serializing this video's format information: ";

    pub const IO_ERR: &str = "There was an IO error: ";

    pub const UNSUPPORTED_FEATURE_ERR: &str = "Currently with blob-dl you can only download videos or playlists, not media such as series";
}