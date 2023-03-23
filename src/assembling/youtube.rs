pub mod yt_playlist;
pub mod yt_video;

use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

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
use crate::DEBUG;

/// Returns the output of <yt-dlp -j url>
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

/// Returns a Vec with every video's format information. json_dump should be the output of `yt-dlp -j <url>`
pub(super) fn parse_formats(json_dump: &str) -> Result<VideoSpecs> {
    //todo videos which require 18 years to see make ugly errors pop up
    // A lost of every video in the playlist's available formats
    let mut all_formats: Vec<VideoFormat> = Vec::new();

    // todo test if this works
    Ok(serde_json::from_str::<VideoSpecs>(json_dump)?)
}

/// Whether the user wants to download video files or audio-only
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum MediaSelection {
    Video,
    VideoOnly,
    Audio,
}

use serde::{Deserialize, Serialize};
use serde_json::Result;

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
    // Measured in MiB. Unavailable on sb* formats
    filesize: Option<u32>,
}

/// All of the formats a particular video can be downloaded in
#[derive(Debug)]
pub struct VideoSpecs {
    formats: Vec<VideoFormat>,
    //available_codes: Vec<u32>,
}
impl VideoSpecs {
    pub(crate) fn new() -> VideoSpecs {
        VideoSpecs {
            formats: vec![],
            //available_codes: vec![],
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
