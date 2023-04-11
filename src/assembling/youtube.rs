pub mod yt_playlist;
pub mod yt_video;

use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;

/// Asks the user whether they want to download video files or audio-only
fn get_media_selection(term: &Term) -> Result<MediaSelection, std::io::Error> {
    let download_formats = &[
        "Video",
        "Audio-only",
    ];

    // Ask the user which format they want the downloaded files to be in
    let media_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to download video files or audio-only?")
        .default(0)
        .items(download_formats)
        .interact_on(term)?;

    match media_selection {
        0 => Ok(MediaSelection::Video),
        1 => Ok(MediaSelection::Audio),
        _ => panic!("Error getting media selection")
    }
}

use spinoff;
use std::process;
// Running yt-dlp -j <...>
use execute::Execute;

/// Returns the output of <yt-dlp -j url>: a JSON dump of all the available format information for a video
fn get_ytdlp_formats(url: &str) -> Result<process::Output, std::io::Error> {
    // Neat animation to entertain the user while the information is being downloaded
    let sp = spinoff::Spinner::new(spinoff::Spinners::Dots10, "Fetching available formats...", spinoff::Color::Cyan);

    let mut command = process::Command::new("yt-dlp");
    // Get a JSON dump of all the available formats related to this url
    command.arg("-j");
    // Continue even if you get errors
    command.arg("-i");
    command.arg(url);
    // Redirect the output to a variable instead of the screen
    command.stdout(process::Stdio::piped());
    let output = command.execute_output();
    sp.stop();

    output
}


/// Serializes the information about the formats available for 1 video
fn serialize_formats(json_dump: &str) -> serde_json::Result<VideoSpecs> {
    // todo videos which require 18 years to see make ugly errors pop up
    // todo test if this works
    serde_json::from_str(json_dump)
}


/// Whether the user wants to download video files or audio-only
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum MediaSelection {
    Video,
    VideoOnly,
    Audio,
}

/// All the information about a particular video format
#[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq)]
pub(crate) struct VideoFormat {
    format_id: String,
    // File extension
    ext: String,
    // Fps count, is null for audio-only formats
    fps: Option<f64>,
    // How many audio channels are available, is null for video-only formats. Unavailable on weird sb* formats
    audio_channels: Option<u32>,
    // Video resolution, is "audio only" for audio-only formats
    resolution: String,
    // Measured in bytes. Unavailable on sb* formats
    filesize: Option<f32>,
}

impl fmt::Display for VideoFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(filesize) = self.filesize {
            let filesize_section = format!(" filesize: {:.2}MB ", filesize * 0.000001);
            if let Some(fps) = self.fps {
                // The format is either video or video-only
                if let Some(audio_channels) = self.audio_channels {
                    // Full video
                    #[cfg(debug_assertions)]
                    return {
                        write!(f, "[[DEBUG code : {}]] {:<6} {:<13} |{:<23}| {} audio channel(s)", self.format_id, self.ext, self.resolution, filesize_section, audio_channels)
                    };
                    #[cfg(not(debug_assertions))]
                    write!(f, "{:<6} {:<13} |{:<23}| {} audio channel(s)", self.ext, self.resolution, filesize_section, audio_channels)
                } else {
                    // Video only
                    #[cfg(debug_assertions)]
                    return {
                        write!(f, "[[DEBUG code : {:<13}]] {:<6} {:<13} |{:<23}| video only", self.format_id, self.ext, self.resolution, filesize_section)
                    };
                    #[cfg(not(debug_assertions))]
                    write!(f, "{:<6} {:<13} |{:<23}| video only", self.ext, self.resolution, filesize_section)
                }
            } else {
                // Audio only
                #[cfg(debug_assertions)]
                return {
                    write!(f, "[[DEBUG code : {}]] {:<6} |{:<20}| {} audio channels", self.format_id, self.ext, filesize_section, self.audio_channels.unwrap())
                };
                #[cfg(not(debug_assertions))]
                write!(f, "{:<6} |{:<20}| {} audio channels", self.ext, filesize_section, self.audio_channels.unwrap()) }
        } else {
            // The format is not video/audio-only/video-only
            write!(f, "[[DEBUG]] Thumbnail format")
        }
    }
}

// A list of all the formats available for a single video
#[derive(Deserialize, Serialize, Debug)]
struct VideoSpecs {
    formats: Vec<VideoFormat>,
}
impl VideoSpecs {
    fn formats(&self) -> &Vec<VideoFormat> {
        &self.formats
    }
}

#[derive(Debug)]
/// What quality and format the user wants a specific video to be downloaded in
pub(crate) enum VideoQualityAndFormatPreferences {
    // Code of the selected format
    UniqueFormat(String),
    BestQuality,
    WorstQuality,
}
