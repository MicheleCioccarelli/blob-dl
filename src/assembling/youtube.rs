pub mod yt_playlist;
pub mod yt_video;

// Structs and enums used in both child modules

/// Whether the user wants to download video files or audio-only
#[derive(Debug, Eq, PartialEq)]
//todo make this pub(crate)
pub enum MediaSelection {
    Video,
    Audio,
}

/// Stores all information about a format available for a video (file extension, size, resolution, code)
#[derive(Debug, PartialOrd, PartialEq)]
pub(crate) struct VideoFormat {
    code: u32,
    file_extension: String,
    // This is an actual quality like "1280x720" for a video, it is "audio" if the format is a audio-only
    resolution: String,
    // Size of the downloaded file
    size: String,
}

impl VideoFormat {
    /// Returns an Option\<Format\> object when given a valid ine from the output of the command
    /// "youtube-dl -F \<URL\>"
    ///
    /// Before using this function make sure that unaccepted input is handled by the caller
    ///
    /// This function is intended to be used in a wizard
    /// # Accepted input
    /// When `ytdl_output_line` contains information about
    /// - audio-only quality and format for a youtube url
    /// - video quality and format for a youtube url
    ///
    /// # Unaccepted input
    /// This function's output is corrupted if `ytdl_output_line`
    /// - isn't about video quality and format
    ///  (for example lines starting with \[info\] or \[youtube\])
    /// - is about a video-only format
    /// - is a youtube-dl error/warning
    /// - is a youtube-dl message (e.g. "This video may be inappropriate for some users")
    pub fn from_command(ytdl_output_line: &str) -> VideoFormat {
        // Collect all elements in a line in a single vector
        let table_elements: Vec<&str> = ytdl_output_line.split_whitespace().collect();

        let mut err_msg = String::from("VideoFormat::from_command()'s contract was violated\nGuilty line:\n");
        err_msg.push_str(ytdl_output_line);

        let mut table_elements_iter = table_elements.into_iter();
        /*
        * Example of 3 valid lines with different properties:
        *
        * 18           mp4        640x360    360p  172k , avc1.42001E, 30fps, mp4a.40.2 (44100Hz)
        * 22           mp4        1280x720   720p  468k , avc1.64001F, 30fps, mp4a.40.2 (44100Hz) (best)
        * 140          m4a        audio only tiny  127k , m4a_dash container, mp4a.40.2@127k (44100Hz), 6.54MiB
        *
        * The fields are: code, extension, resolution/audio-only, quality, note, size, ..., (best)
        */

        let code: u32 = table_elements_iter.next().expect(&err_msg)
            .parse().expect(&err_msg);

        let file_extension = String::from(table_elements_iter.next().expect(&err_msg));

        let resolution = String::from(table_elements_iter.next().expect(&err_msg));

        // Audio only files' resolution is marked as "audio only", video files have an actual resolution
        let audio_only = if resolution == "audio" {
            // Skip "only"
            table_elements_iter.next();
            true
        } else {
            false
        };

        // Skip the "note" section of ytdl_output_line
        table_elements_iter.next();

        let mut size = String::from(table_elements_iter.next().expect(&err_msg));
        if audio_only {
            // Audio-only with DASH note has one more field to be skipped
            if size == "audio" {
                size = String::from(table_elements_iter.next().expect(&err_msg));
            }
        }
        // All information has been parsed
        VideoFormat {
            code,
            file_extension,
            resolution,
            size,
        }
    }

    fn code(&self) -> u32 {
        self.code
    }
    fn file_extension(&self) -> &String {
        &self.file_extension
    }
    fn resolution(&self) -> &String {
        &self.resolution
    }
    fn is_audio_only(&self) -> bool {
        self.file_extension == "audio"
    }

    /// Returns a String containing all format information which can be displayed to someone picking formats
    pub(crate) fn to_frontend(&self) -> String {
        // todo get a better name
        format!("{}-{} (size: {}) [DEBUG ID: {}]", self.file_extension, self.resolution, self.size, self.code)
    }
}

/// All of the formats a particular video can be downloaded in
#[derive(Debug)]
pub struct VideoSpecs {
    available_formats: Vec<VideoFormat>,
    available_codes: Vec<u32>,
}
impl VideoSpecs {
    pub(crate) fn new() -> VideoSpecs {
        VideoSpecs {
            available_formats: vec![],
            available_codes: vec![],
        }
    }
    /// Updates the struct's internal list of ids and returns a ref to it
    ///
    /// It also sorts the list of ids
    pub(crate) fn refresh_and_sort_ids(&mut self) -> &Vec<u32> {
        self.available_codes = self.available_formats.iter().map(|format| format.code()).collect();
        self.available_codes.sort();
        &self.available_codes
    }
    pub(crate) fn add_format(&mut self, format: VideoFormat) {
        self.available_formats.push(format);
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.available_formats.is_empty()
    }
    pub(crate) fn available_formats(&self) -> &Vec<VideoFormat>{
        &self.available_formats
    }
}

// todo make pub(crate)
#[derive(Debug)]
/// What quality and format the user wants a specific video to be downloaded in
pub enum VideoQualityAndFormatPreferences {
    // Code of the selected format
    UniqueFormat(u32),
    BestQuality,
    WorstQuality,
}
