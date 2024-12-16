pub mod yt_playlist;
pub mod yt_video;
pub mod config;

use crate::error::{BlobdlError, BlobResult};
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select, Input};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{env, fmt};
use colored::Colorize;

// Functions used both in yt_video.rs and yt_playlist.rs
/// Asks the user whether they want to download video files or audio-only
fn get_media_selection(term: &Term) -> Result<MediaSelection, std::io::Error> {
    let download_formats = &[
        "Normal Video",
        "Audio-only",
        "Video-only"
    ];

    // Ask the user which format they want the downloaded files to be in
    let media_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What kind of file(s) do you want to download?")
        .default(0)
        .items(download_formats)
        .interact_on(term)?;

    match media_selection {
        0 => Ok(MediaSelection::FullVideo),
        1 => Ok(MediaSelection::AudioOnly),
        _ => Ok(MediaSelection::VideoOnly),
    }
}

/// Asks for an directory to store downloaded file(s) in
///
/// The current directory can be selected or one can be typed in
fn get_output_path(term: &Term) -> BlobResult<String> {
    let output_path_options = &[
        "Current directory",
        "Other [specify]",
    ];

    let output_path = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Where do you want the downloaded file(s) to be saved?")
        .default(0)
        .items(output_path_options)
        .interact_on(term)?;

    match output_path {
        // Return the current directory
        0 => Ok(env::current_dir()?
            .as_path()
            .display()
            .to_string()),

        // Return a directory typed in by the user
        _ => Ok(Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Output path:")
            .interact_text()?),
    }
}


use spinoff;
use std::process;
// Running yt-dlp -j <...>
use execute::Execute;

/// Returns the output of <yt-dlp -j url>: a JSON dump of all the available format information for a video
fn get_ytdlp_formats(url: &str) -> Result<process::Output, std::io::Error> {
    // Neat animation to entertain the user while the information is being downloaded
    let mut sp = spinoff::Spinner::new(spinoff::spinners::Dots10, "Fetching available formats...", spinoff::Color::Cyan);

    let mut command = process::Command::new("yt-dlp");
    // Get a JSON dump of all the available formats related to this url
    command.arg("-J");
    // Continue even if you get errors
    command.arg("-i");
    command.arg(url);

    // Redirect the output to a variable instead of to the screen
    command.stdout(process::Stdio::piped());
    // Don't show errors and warnings
    command.stderr(process::Stdio::piped());
    let output = command.execute_output();

    // Stop the ui spinner
    sp.success("Formats downloaded successfully".bold().to_string().as_str());

    output
}

/// Ask the user what format they want the downloaded file to be recoded to (yt-dlp postprocessor) REQUIRES FFMPEG
fn convert_to_format(term: &Term, media_selected: &MediaSelection)
                     -> BlobResult<VideoQualityAndFormatPreferences>
{
    // Available formats for recoding
    let format_options = match *media_selected {
        // Only show audio-only formats
        MediaSelection::AudioOnly => vec!["mp3", "m4a", "wav", "aac", "alac", "flac", "opus", "vorbis"],
        // Only show formats which aren't audio-only
        MediaSelection::VideoOnly => vec!["mp4", "mkv", "mov", "avi", "flv", "gif", "webm", "aiff", "mka", "ogg"],
        // Show all the available formats
        MediaSelection::FullVideo => vec!["mp4", "mkv", "mov", "avi", "flv", "gif", "webm", "aac", "aiff",
                                          "alac", "flac", "m4a", "mka", "mp3", "ogg", "opus", "vorbis", "wav"],
    };

    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which container do you want the final file to be in?")
        .default(0)
        .items(&format_options)
        .interact_on(term)?;

    Ok(VideoQualityAndFormatPreferences::ConvertTo(format_options[user_selection].to_string()))
}

#[derive(Deserialize, Serialize, Debug)]
struct format {
    format_id: String,
}

/// Serializes the information about all the formats available for 1 video
fn serialize_formats(json_dump: &str) -> BlobResult<VideoSpecs> {
    let result = serde_json::from_str(json_dump);
    match result {
        Ok(cool) => Ok(cool),
        Err(err) => Err(BlobdlError::SerdeError(err))
    }
}

/// Checks if format has conflicts with media_selected (like a video only format and an audio-only media_selection
///
/// Returns true format and media_selected are compatible
fn check_format(format: &VideoFormat, media_selected: &MediaSelection) -> bool {
    // Skip image and weird formats (examples of strange formats ids: 233, 234, sb2, sb1, sb0)
    if format.filesize.is_none() {
        return false;
    }
    // Skip audio-only files if the user wants full video
    if *media_selected == MediaSelection::FullVideo && format.resolution == "audio only" {
        return false;
    }
    // Skip video files if the user wants audio-only
    if *media_selected == MediaSelection::AudioOnly && format.resolution != "audio only" {
        return false;
    }
    if let Some(acodec) = &format.acodec {
        // Skip video-only files if the user doesn't want video-only
        if *media_selected == MediaSelection::FullVideo && acodec == "none" {
            return false;
        }
        //Skip normal video if the user wants video-only
        if *media_selected == MediaSelection::VideoOnly && acodec != "none" {
            return false;
        }
    }
    true
}

// Common enums and structs
/// Whether the user wants to download video files or audio-only
#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum MediaSelection {
    FullVideo,
    VideoOnly,
    AudioOnly,
}

/// All the information about a particular video format
#[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq, Clone)]
struct VideoFormat {
    format_id: String,
    // File extension
    ext: String,
    // Fps count, is null for audio-only formats
    fps: Option<f64>,
    // How many audio channels are available, is null for video-only formats. Unavailable on weird sb* formats
    audio_channels: Option<u64>,
    // Video resolution, is "audio only" for audio-only formats
    resolution: String,
    // Measured in MB. Unavailable on sb* formats
    filesize: Option<u64>,
    // Video codec, can be "none"
    vcodec: String,
    // Audio codec, can be "none" or straight up not exist (like in mp4 audio-only formats)
    acodec: Option<String>,
    // Codec container
    container: Option<String>,
    // Total average bitrate
    tbr: Option<f64>,
    // When filesize is null, this may be available
    filesize_approx: Option<u64>,
}

// A list of all the formats available for a single video
#[derive(Deserialize, Serialize, Debug, Clone)]
struct VideoSpecs {
    formats: Vec<VideoFormat>,
}

/// What quality and format the user wants a specific video to be downloaded in
#[derive(Debug, Clone)]
pub(crate) enum VideoQualityAndFormatPreferences {
    // Code of the selected format
    UniqueFormat(String),
    // Recode the downloaded file to this format (post-processor)
    ConvertTo(String),
    BestQuality,
    SmallestSize,
}

impl fmt::Display for VideoFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result;

        if let Some(tbr) = self.tbr {
            // Skip picture formats
            // Add container
            result = format!("{:<6} ", self.ext);

            if self.resolution != "audio only" {
                result = format!("{}| {:<13} ", result, self.resolution);
            }

            // This isn't a picture format so unwrap() is safe
            let filesize = self.filesize.unwrap_or(0);

            // filesize is converted from bytes to MB
            let filesize_section = format!("| filesize: {:<.2}MB", filesize as f32 * 0.000001);
            result = format!("{}{:<24}", result, filesize_section);

            // If available, add audio channels
            if let Some(ch) = self.audio_channels {
                result = format!("{}| {} audio ch ", result, ch);
            }

            result = format!("{}| tbr: {:<8.2} ", result, tbr);

            if self.vcodec != "none" {
                result = format!("{}| vcodec: {:<13} ", result, self.vcodec);
            }

            if let Some(acodec) = &self.acodec {
                if acodec != "none" {
                    result = format!("{}| acodec: {:<13} ", result, acodec);
                }
            }

            #[cfg(debug_assertions)]
            return {
                result = format!("[[DEBUG code: {:<3}]] {} ", self.format_id, result);
                write!(f, "{}", result)
            };

            #[cfg(not(debug_assertions))]
            write!(f, "{}", result)
        } else {
            write!(f, "I shouldn't show up because I am a picture format")
        }
    }
}

impl VideoSpecs {
    fn formats(&self) -> &Vec<VideoFormat> {
        &self.formats
    }
}
