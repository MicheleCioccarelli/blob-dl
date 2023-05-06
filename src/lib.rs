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
