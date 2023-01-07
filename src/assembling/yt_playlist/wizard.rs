use clap::ArgMatches;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use super::config;
use crate::assembling;
use url::Url;
use crate::assembling::MediaSelection;
use crate::assembling::youtube;
use crate::assembling::yt_playlist::wizard::QualityScope::AllVideos;

/// Returns a ConfigYtPlaylist object with all the necessary data
/// to start downloading a youtube playlist
///
/// Takes in the command line arguments list
pub(crate) fn assemble_data(url: &String) -> Result<config::ConfigYtPlaylist, std::io::Error> {
    let term = Term::buffered_stderr();

    // Whether the user wants to download video files or audio-only
    let media = get_media_selection(&term)?;

    let format = format::get_format(&term, &media)?;

    let output_dir = assembling::get_output_path(&term)?;

    let quality = get_quality(&term)?;

    let preference = get_index_preference(&term)?;

    let output_style = get_output_style(&term)?;

    Ok(config::ConfigYtPlaylist::new(url.clone(),
                                  media,
                                  format,
                                  output_dir,
                                  quality,
                                  preference,
                                  output_style))
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
        .interact_on(&term)?;

    match media_selection {
        0 => Ok(assembling::MediaSelection::Video),
        1 => Ok(assembling::MediaSelection::Audio),
        _ => panic!("Error getting media selection")
    }
}
mod format {
    use super::*;
    use std::io::BufRead;
    use std::process::{Command, Stdio};
    use execute::Execute;
    use crate::assembling::youtube::Format;

    /// Asks the user to specify a download format and quality
    ///
    /// Either best-quality or worst-quality can be selected for the whole playlist, or a format can be picked for each
    /// video. If all videos have a format and quality in common, they can be easily applied
    pub(super) fn get_format(term: &Term, media_selected: &MediaSelection) -> Result<assembling::VideoQualityAndFormatPreferences, std::io::Error> {
        // To download multiple formats -f 22/17/18 chooses the one which is available and most to the left

        // Each element of this vector describes the quality option for a video in the playlist
        let mut all_preferences: Vec<youtube::VideoQualityAndFormatPreferences> = vec![];

        let mut user_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want a single quality setting to be applied to all videos or do you want to choose one for each video?")
            .default(0)
            .items(&["Apply a setting to all videos", "Choose a quality setting for each video"])
            .interact_on(&term)?;
        let quality_scope = match user_selection {
            0 => QualityScope::AllVideos,
            _ => QualityScope::SingleVideo,
        };

        if quality_scope == QualityScope::AllVideos {
            user_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Which quality do you want to apply to all videos?")
                .default(0)
                .items(&["Best available quality for each video", "Worst available quality for each video",
                    "Placeholder for when uniform format selection will have been implemented"])
                .interact_on(&term)?;
        } else if quality_scope == QualityScope::SingleVideo {
            // Ask for a quality selection for each video in the Playlist
        };
        todo!()
    }
    enum QualityScope {
        AllVideos,
        SingleVideo,
    }

    /// A list of all the download formats available for a video
    ///
    /// If a video's format information is unavailable (maybe because it is age-restricted)
    /// and thus youtube-dl cannot fetch any information about it, the Option is None.
    ///
    /// In every other case there is Some
    type VideoFormats = Option<Vec<Format>>;

    /// Returns a Vec with a format object for every video in the playlist
    fn fetch_formats(playlist_url: &String) -> Result<Vec<VideoFormats>, std::io::Error> {
        let command = Command::new("youtube-dl").arg("-F").arg(playlist_url);
        // Store youtube-dl's output
        let output = command.execute_output().unwrap();
        // If youtube-dl printed something to standard error
        if output.stderr.len() > 0 {
            panic!("Youtube-dl gave an error, for every line with an ERROR, Unavailable should be the Format");
        };
        let mut all_videos: Vec<VideoFormats> = vec![];
        let mut video: VideoFormats = vec![];

        for line in output.stdout.as_str()?.split("[download] Downloading video") {
            // Ignore all useless lines
            if line.contains("[") ||
                line.contains("format") ||
                line.contains("video only") {
                continue;
            };
            // TODO Make a video's format None if there is a youtube-dl ERROR
            // TODO See if this exits the function on failure
            video.push(Format::from_command(line)?);
        }

        todo!()
    }
}

fn get_quality(term: &Term) -> Result<assembling::Quality, std::io::Error> {
    let download_formats = &[
        "Best quality",
        "Worst quality",
    ];

    let quality_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which quality do you want the downloaded files to be in?")
        .default(0)
        .items(download_formats)
        .interact_on(&term)?;

    match quality_selection {
        0 => Ok(assembling::Quality::Bestquality),
        1 => Ok(assembling::Quality::Worstquality),
        _ => panic!("he only options are 0 and 1")
    }
}

/// Whether the downloaded files should include their index in the playlist as a part of their name
fn get_index_preference(term: &Term) -> Result<bool, std::io::Error> {
    let download_formats = &[
        "Yes",
        "No",
    ];
    // Ask the user which format they want the downloaded files to be in
    let index_preference = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you a video's index in the playlist to be in its name?")
        .default(0)
        .items(download_formats)
        .interact_on(&term)?;

    match index_preference {
        0 => Ok(true),
        1 => Ok(false),
        _ => panic!("The only options are 0 and 1")
    }
}

fn get_output_style(term: &Term) -> Result<assembling::OutputStyle, std::io::Error> {
    let download_formats = &[
        "Yes",
        "No",
    ];
    // Ask the user which format they want the downloaded files to be in
    let output_style = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which part of youtube-dl's output do you want to see?")
        .default(0)
        .items(download_formats)
        .interact_on(&term)?;

    match output_style {
        0 => Ok(assembling::OutputStyle::RedirectErrors),
        1 => Ok(assembling::OutputStyle::Full),
        _ => panic!("The only options are 0 and 1")
    }
}