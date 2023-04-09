pub mod yt_playlist;
pub mod yt_video;

use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use serde_json;

/// Returns an intersection Vec
fn intersection<'a, T: Eq> (vec1: &'a Vec<T>, vec2: &'a Vec<T>) -> Vec<&'a T> {
    let mut intersections = vec![];

    for element_1 in vec1.iter() {
        for element_2 in vec2.iter() {
            if element_1 == element_2 {
                // Intersection element found!
                intersections.push(element_1);
            }
        }
    }

    intersections
}

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
fn serialize_formats(json_dump: &str) -> serde_json::Result<Vec<VideoFormat>> {
    // todo videos which require 18 years to see make ugly errors pop up
    // todo test if this works
    Ok(serde_json::from_str(json_dump)?)
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
    // Measured in MiB. Unavailable on sb* formats
    filesize: Option<u32>,
}

/// All of the formats a particular video can be downloaded in
#[derive(Deserialize, Serialize, Debug)]
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
    /*
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
    }*/
}

#[derive(Debug)]
/// What quality and format the user wants a specific video to be downloaded in
pub(crate) enum VideoQualityAndFormatPreferences {
    // Code of the selected format
    UniqueFormat(u32),
    BestQuality,
    WorstQuality,
}
