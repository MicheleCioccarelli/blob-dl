use crate::assembling;

/// Contains all the information needed to download a youtube video [WIP]
#[derive(Debug)]
pub(crate) struct ConfigYtVideo {
    url: String,
    download_format: String,
    output_path: String,
}

impl ConfigYtVideo {
    pub(crate) fn new(url: String, download_format: String, output_path: String) -> ConfigYtVideo {
        ConfigYtVideo { url, download_format, output_path }
    }
    /// Builds a yt-dl command with the needed specifications
    pub(crate) fn build_command(&self) -> std::process::Command {
        todo!()
    }
}

pub(crate) enum VideoQualityAndFormatPreferences {
    UniqueFormat(Option<VideoFormat>),
    BestQuality,
    WorstQuality,
}

/// Stores all information about a format available for a video (file extension, size, resolution, code)
#[derive(Debug)]
pub(crate) struct VideoFormat {
    code: u32,
    file_extension: String,
    // This is an actual quality like "1280x720" for a video, it is "audio" if the format is a audio-only
    resolution: String,
    // Size of the downloaded file
    size: String,
    // Whether the current format is the best available for a given video
    is_best: bool,
}
impl VideoFormat {
    // # Usa insiemistica (intersezione tra insiemi di id) e mappe per comparare
    /// Returns an Option\<Format\> object when given a line from the output of the command
    /// "youtube-dl -F \<URL\>"
    /// # Returns Some(Format)
    /// When `ytdl_output_line` contains information about
    /// - audio-only quality and format for a youtube url
    /// - video quality and format for a youtube url
    ///
    /// # Returns None
    /// When `ytdl_output_line` is a youtube-dl error (for example when a video is age-restricted
    /// no data about its format is available)
    ///
    /// # Unaccepted input
    /// This function's output is corrupted if `ytdl_output_line`
    /// - isn't about video quality and format
    ///  (for example lines starting with \[info\] or \[youtube\])
    /// - is about a video-only format
    pub fn from_command(ytdl_output_line: &str) -> Option<VideoFormat> {
        if ytdl_output_line.contains("ERROR") {
            return None;
        }
        // Collect all elements in a line in a single vector
        let table_elements: Vec<&str> = ytdl_output_line.split_whitespace().collect();

        // 8 is the minimum amount of fields in a valid output
        if table_elements.len() < 8 {
            eprintln!("This youtube-dl output line was rejected: {}", ytdl_output_line);
            return None;
        }
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

        let code: u32 = table_elements_iter.next()?.parse().ok()?;

        let file_extension = String::from(table_elements_iter.next()?);

        let resolution = String::from(table_elements_iter.next()?);

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

        let mut size = String::from(table_elements_iter.next()?);
        if audio_only {
            // Audio-only with DASH note has one more field to be skipped
            if size == "audio" {
                size = String::from(table_elements_iter.next()?);
            }
        }

        let last_element = table_elements_iter.last()?;

        // The last element of ytdl_output_line tells you whether this line had the best available format
        let is_best = if last_element == "(best)" {
            true
        } else {
            false
        };
        // All information has been parsed
        Some(VideoFormat {
            code: code,
            file_extension: file_extension,
            resolution: resolution,
            size: size,
            is_best: is_best,
        })
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
        self.format == "audio"
    }
}

/// All of the formats a particular video can be downloaded in
#[derive(Debug)]
pub(crate) struct YtVideoFormats {
    available_formats: Vec<VideoQualityAndFormatPreferences>,
}
impl YtVideoFormats {
    pub(crate) fn new() -> YtVideoFormats {
        YtVideoFormats {
            available_formats: vec![]
        }
    }
    pub(crate) fn add_format(&mut self, format: VideoQualityAndFormatPreferences) {
        self.available_formats.push(format);
    }
}
