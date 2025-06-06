pub mod parser;
mod assembling;
mod analyzer;
mod dispatcher;
mod run;
mod error;

pub mod app;

// Things blob-dl regularly tells the user
pub mod ui_prompts {
    pub const FFMPEG_UNAVAILABLE_WARNING: &str = "It looks like ffmpeg and ffprobe aren't installed, which means that some of blob-dl's features aren't available!\nPlease install them for a fuller experience";

    pub const LONG_ABOUT: &str = "A command line tool used to make downloading youtube videos in various formats easy\nIf you are having problems passing a URL as an argument, try wrapping it in quotes (\"\")!\n\nFor more details check out the github page https://github.com/MicheleCioccarelli/blob-dl\nRecommended yt-dlp version: 2025.03.31";

    pub const SHORT_ABOUT: &str = "A command line tool used to make downloading youtube videos in various formats easy\nIf you are having problems passing a URL as an argument, try wrapping it in quotes (\"\")!\n\nFor more details check out the github page https://github.com/MicheleCioccarelli/blob-dl\nRecommended yt-dlp version: 2025.03.31";

    pub const YTDLP_NOT_INSTALLED: &str = "blob-dl is a wrapper around yt-dlp and cannot function without it.\nPlease install yt-dlp from the official github page: https://github.com/yt-dlp/yt-dlp";

    pub const BEST_QUALITY_PROMPT_PLAYLIST: &str = "Best possible quality for each video";

    pub const BEST_QUALITY_PROMPT_SINGLE_VIDEO: &str = "Best possible quality";

    pub const SMALLEST_QUALITY_PROMPT_PLAYLIST: &str = "Smallest file size for each video";

    pub const SMALLEST_QUALITY_PROMPT_SINGLE_VIDEO: &str = "Smallest file size";

    pub const YT_FORMAT_PROMPT_PLAYLIST: &str = "Choose a format to download every video in (only the formats which are available for all videos are shown)";

    pub const YT_FORMAT_PROMPT_SINGLE_VIDEO: &str = "Choose a format to download the video in";

    pub const CONVERT_FORMAT_PROMPT_VIDEO_PLAYLIST: &str = "Choose a format to recode all the videos to";

    pub const CONVERT_FORMAT_PROMPT_VIDEO_SINGLE_VIDEO: &str = "Choose a format to recode the video to";

    pub const CONVERT_FORMAT_PROMPT_AUDIO: &str = "Choose an audio format to convert the audios to";

    pub const SEE_HELP_PAGE: &str = "Type blob-dl --help for a list of all the available options";

    pub const USAGE_MSG: &str = "Usage: blob-dl [OPTIONS] [URL]";

    pub const ERROR_RETRY_PROMPT: &str = "The following videos weren't downloaded but retrying might help, choose which videos to re-download [space bar to select]";

    pub const UNRECOVERABLE_ERROR_PROMPT: &str = "The following videos could not be downloaded due to unrecoverable errors";

    pub const DEBUG_REPORT_PROMPT: &str = "By default new errors are flagged as unrecoverable, if any recoverable errors are flagged incorrectly please report them to the github page";

    pub const SELECT_ALL: &str = "Select all\n";
    pub const SELECT_NOTHING: &str = "Don't re-download anything\n";
    
    pub const WRONG_YTDLP_VERSION: &str = "It looks like you have a yt-dlp version which may not work with blob-dl as expected: you may not be able to fetch formats from youtube.\n\
    To fix this you can update your yt-dlp installation to the correct version with the command: sudo yt-dlp --update-to 2025.03.31";
    
    pub const COMMAND_NOT_SPAWNED: &str = "An instance of ytdlp (used to check which version of the program you have installed) could not be spawned";
}

// Youtube's error messages
// THESE SHOULD NOT BE MODIFIED, they are supposed to match exactly youtube's error messages
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

    pub const NONEXISTENT_VIDEO: &str = "Incomplete data received";
    
    pub const NONEXISTENT_FORMAT: &str = "Requested format is not available";

    // All copyright error messages begin with this
    pub const VIDEO_UNAVAILABLE: &str = " Video unavailable";
}


// blob-dl custom error messages
pub mod blobdl_error_message {
    pub const BROKEN_URL_ERR: &str = "The URL you provided wasn't recognized, try using a regular youtube URL";

    pub const UNSUPPORTED_WEBSITE_ERR: &str = "Currently blob-dl only supports downloading youtube videos or playlists, not content from other websites";

    pub const UNKNOWN_ISSUE_ERR: &str = "Congrats! You ran into an unknown issue, please file a report on blob-dl's github page :)";

    pub const MISSING_ARGUMENT_ERR: &str = "You must provide 1 URL";

    pub const JSON_SERIALIZATION_ERR: &str = "There was a problem serializing this video's format information";

    pub const UTF8_ERR: &str = "This video's format information contained non-UTF8 characters and broke the parser, best and smallest quality should still work!";

    pub const SERDE_ERR: &str = "Serde ran into a problem when serializing this video's format information.\nThis most likely happened because the video you are trying to download is private/copyright claimed\nJson-y reason:";

    pub const IO_ERR: &str = "There was an IO error: ";

    pub const URL_QUERY_COULD_NOT_BE_PARSED: &str = "This URL's query could not be parsed, try using a regular youtube URL";

    pub const URL_INDEX_PARSING_ERR: &str = "The video's index in the playlist couldn't be parsed, please report this issue to the github page";
    
    pub const PLAYLIST_URL_ERROR: &str = "The index/id for the video that you want to download in the playlist could not be parsed.\nTo download just this video try using a URL which links directly to it instead of going through a playlist";

    // POST_CONFIG_FILE_ERRORS
    pub const URL_NOT_PROVIDED_ERROR: &str = "You didn't provide a URL for the video you want to download. The issue most likely has to do with a configuration file.\nTo report this error or learn more about config files please visit the GitHub page";

    pub const FORMAT_PREFERENCE_NOT_PROVIDED_ERROR: &str = "You didn't provide a format preference for the video you want to download. The issue most likely has to do with a configuration file.\nTo report this error or learn more about config files please visit the GitHub page";

    pub const OUTPUT_PATH_NOT_PROVIDED_ERROR: &str = "You didn't provide an output path for the video you want to download. The issue most likely has to do with a configuration file.\nTo report this error or learn more about config files please visit the GitHub page";

    pub const INCLUDE_INDEXES_NOT_PROVIDED_ERROR: &str = "You didn't specify whether indexes should be included in the title of the videos you want to download. The issue most likely has to do with a configuration file.\nTo report this error or learn more about config files please visit the GitHub page";

    pub const DOWNLOAD_TARGET_NOT_PROVIDED_ERROR: &str = "There was an issue figuring out the download target (whether your link refers to a single video or playlist).\nTo report this error please visit the GitHub page";

    pub const MEDIA_SELECTION_NOT_PROVIDED_ERROR: &str = "You didn't provide a media selection for the video you want to download (media selection means whether you want to download audio only/video only/full video). The issue most likely has to do with a configuration file.\nTo report this error or learn more about config files please visit the GitHub page";

    pub const CHOSEN_FORMAT_NOT_PROVIDED_ERROR: &str = "You didn't provide a download format for the video you want to download. The issue most likely has to do with a configuration file.\nTo report this error or learn more about config files please visit the GitHub page";

    pub const CONFIG_FILE_NOT_FOUND_ERR: &str = "No valid home directory path could be retrieved from the operating system. (this problem has to do with the default location of your config file)";
    
    pub const FFMPEG_NOT_AVAILABLE_CONFIG_WARNING: &str = "You are using a config file which tells blob-dl to convert the files you download to a specific format. Doing this requires ffmpeg, which is not installed on your system";
    
    pub const JSON_GENERATION_ERR: &str = "yt-dlp didn't generate the json needed to parse formats for your video.\nThis most likely happened because the video you are trying to download is private/copyright claimed\nIf you are running the recommended version of yt-dlp please report this to the GitHub page.";
}
