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

const ERROR_RETRY_PROMPT: &str = "The following videos weren't downloaded but retrying might help, choose which videos to re-download [spacebar to select]";

const UNRECOVERABLE_ERROR_PROMPT: &str = "The following videos could not be downloaded due to unrecoverable errors";

const DEBUG_REPORT_PROMPT: &str = "By default new errors are flagged as recoverable, if any unrecoverable errors are flagged incorrectly please report them to the github page :)";

const SELECT_ALL: &str = "Select all\n";
const SELECT_NOTHING: &str = "Don't re-download anything\n";

// Youtube's error messages
const PRIVATE_VIDEO: &str = " Private video. Sign in if you've been granted access to this video";

const NONEXISTENT_PLAYLIST: &str = " YouTube said: The playlist does not exist.";

const HOMEPAGE_REDIRECT: &str = " The channel/playlist does not exist and the URL redirected to youtube.com home page";

const NETWORK_FAIL: &str = " Unable to download API page: <urlopen error [Errno -3] Temporary failure in name resolution> (caused by URLError(gaierror(-3, 'Temporary failure in name resolution')))";

const VIOLENT_VIDEO: &str = " This video has been removed for violating YouTube's policy on violent or graphic content";

const REMOVED_VIDEO: &str = " Video unavailable. This video has been removed by the uploader";

const VIDEO_NOT_FOUND: &str = " not found, unable to continue";

const YTDLP_GAVE_UP: &str = " error: HTTP Error 403: Forbidden. Giving up after 10 retries";

// All copyright error messages begin with this
const VIDEO_UNAVAILABLE: &str = " Video unavailable";

// blob-dl custom error messages
mod error_message {
    pub const BROKEN_URL_ERR: &str = "The url provided wasn't recognized, try using a regular youtube url";

    pub const UNSUPPORTED_WEBSITE_ERR: &str = "Currently blob-dl only supports downloading youtube videos or playlists";

    pub const UNKNOWN_ISSUE_ERR: &str = "Congrats! You ran into an unknown issue, please file a report on blob-dl's github page :)";

    // fixme fix this message
    pub const MISSING_ARGUMENT_ERR: &str = "A url is required for blob-dl to function [[please rephrase this error]]";

    pub const JSON_SERIALIZATION_ERR: &str = "There was a problem serializing this video's format information";

    pub const UTF8_ERR: &str = "This video's format information contained non-UTF8 characters and broke the parser, best and worst quality should still work!";

    pub const SERDE_ERR: &str = "Serde ran into a problem when serializing this video's format information: ";

    pub const IO_ERR: &str = "There was an IO error: ";

    pub const UNSUPPORTED_FEATURE_ERR: &str = "Currently with blob-dl you can only download videos or playlists, not media such as series";
}