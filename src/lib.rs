pub mod parser;
pub mod assembling;
pub mod analyzer;
pub mod dispatcher;
mod run;
mod error;

// Things blob-dl regularly tells the user
pub mod ui_prompts {
    pub const FFMPEG_UNAVAILABLE_WARNING: &str = "It looks like ffmpeg and ffprobe aren't installed, which means that some of blob-dl's features aren't available!\nPlease install them for a fuller experience";

    pub const LONG_ABOUT: &str = "A command line tool used to make downloading youtube videos in various formats easy\nIf you are having problems passing a URL as an argument, try wrapping it in quotes (\"\")!";

    pub const SHORT_ABOUT: &str = "A command line tool used to make downloading youtube videos in various formats easy\nIf you are having problems passing a URL as an argument, try wrapping it in quotes (\"\")!";

    pub const YTDLP_NOT_INSTALLED: &str = "blob-dl is a wrapper around yt-dlp and cannot function without it.\nPlease install yt-dlp from the official github page: https://github.com/yt-dlp/yt-dlp";

    pub const BEST_QUALITY_PROMPT_PLAYLIST: &str = "Best possible quality for each video";

    pub const BEST_QUALITY_PROMPT_SINGLE_VIDEO: &str = "Best possible quality";

    pub const SMALLEST_QUALITY_PROMPT_PLAYLIST: &str = "Smallest file size for each video";

    pub const SMALLEST_QUALITY_PROMPT_SINGLE_VIDEO: &str = "Smallest file size";

    pub const YT_FORMAT_PROMPT_PLAYLIST: &str = "Choose a format to download to every video in (only formats available for all videos are shown)";

    pub const YT_FORMAT_PROMPT_SINGLE_VIDEO: &str = "Choose a format to download the video in";

    pub const CONVERT_FORMAT_PROMPT_VIDEO_PLAYLIST: &str = "Choose a format to recode all the videos to";

    pub const CONVERT_FORMAT_PROMPT_VIDEO_SINGLE_VIDEO: &str = "Choose a format to recode the video to";

    pub const CONVERT_FORMAT_PROMPT_AUDIO: &str = "Choose an audio format to convert the audios to";

    pub const SEE_HELP_PAGE: &str = "Type blob-dl --help for a list of all the available options";

    pub const USAGE_MSG: &str = "Usage: blob-dl [OPTIONS] [URL]";

    pub const ERROR_RETRY_PROMPT: &str = "The following videos weren't downloaded but retrying might help, choose which videos to re-download [spacebar to select]";

    pub const UNRECOVERABLE_ERROR_PROMPT: &str = "The following videos could not be downloaded due to unrecoverable errors";

    pub const DEBUG_REPORT_PROMPT: &str = "By default new errors are flagged as recoverable, if any unrecoverable errors are flagged incorrectly please report them to the github page";

    pub const SELECT_ALL: &str = "Select all\n";
    pub const SELECT_NOTHING: &str = "Don't re-download anything\n";
}

// Youtube's error messages
mod youtube_error_message {
    pub const PRIVATE_VIDEO: &str = " Private video. Sign in if you've been granted access to this video";

    pub const NONEXISTENT_PLAYLIST: &str = " YouTube said: The playlist does not exist.";

    pub const HOMEPAGE_REDIRECT: &str = " The channel/playlist does not exist and the URL redirected to youtube.com home page";

    pub const NETWORK_FAIL: &str = " Unable to download API page: <urlopen error [Errno -3] Temporary failure in name resolution> (caused by URLError(gaierror(-3, 'Temporary failure in name resolution')))";

    pub const VIOLENT_VIDEO: &str = " This video has been removed for violating YouTube's policy on violent or graphic content";

    pub const REMOVED_VIDEO: &str = " Video unavailable. This video has been removed by the uploader";

    pub const VIDEO_NOT_FOUND: &str = " not found, unable to continue";

    pub const YTDLP_GAVE_UP: &str = " error: HTTP Error 403: Forbidden. Giving up after 10 retries";

    pub const NO_API_PAGE: &str = " Unable to download API page: HTTP Error 404: Not Found (caused by <HTTPError 404: 'Not Found'>); please report this issue on https://github.com/yt-dlp/yt-dlp/issues?q= , filling out the appropriate issue template. Confirm you are on the latest version using yt-dlp -U";

    pub const ENCODER_STREAM_ERROR: &str = " Postprocessing: Error selecting an encoder for stream 0:1";

    // All copyright error messages begin with this
    pub const VIDEO_UNAVAILABLE: &str = " Video unavailable";
}
// blob-dl custom error messages
mod blobdl_error_message {
    pub const BROKEN_URL_ERR: &str = "The url provided wasn't recognized, try using a regular youtube url";

    pub const UNSUPPORTED_WEBSITE_ERR: &str = "Currently blob-dl only supports downloading youtube videos or playlists, not content from other websites";

    pub const UNKNOWN_ISSUE_ERR: &str = "Congrats! You ran into an unknown issue, please file a report on blob-dl's github page :)";

    pub const MISSING_ARGUMENT_ERR: &str = "You must provide 1 URL";

    pub const JSON_SERIALIZATION_ERR: &str = "There was a problem serializing this video's format information";

    pub const UTF8_ERR: &str = "This video's format information contained non-UTF8 characters and broke the parser, best and worst quality should still work!";

    pub const SERDE_ERR: &str = "Serde ran into a problem when serializing this video's format information: ";

    pub const IO_ERR: &str = "There was an IO error: ";

    pub const URL_QUERY_COULD_NOT_BE_PARSED: &str = "This url's query could not be parsed, try using a regular youtube url";

    pub const URL_INDEX_PARSING_ERR: &str = "The video's index in the playlist couldn't be parsed, please report this issue to the github page";
}
