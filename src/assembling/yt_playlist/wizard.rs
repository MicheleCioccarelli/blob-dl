// Refactor some of these in the future
use clap::ArgMatches;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use crate::assembling;
use url::Url;
use crate::assembling::MediaSelection;
use crate::assembling::yt_playlist::config;
use crate::assembling::yt_video;

/// Returns a ConfigYtPlaylist object with all the necessary data
/// to start downloading a youtube playlist
///
/// Takes in the command line arguments list
pub(crate) fn assemble_data(url: &String) -> Result<config::ConfigYtPlaylist, std::io::Error> {
    let term = Term::buffered_stderr();

    // Whether the user wants to download video files or audio-only
    let media = get_media_selection(&term)?;

    let available_formats = format::get_format(&term, url)?;

    let output_dir = assembling::get_output_path(&term)?;

    // let quality = get_quality(&term);

    let preference = get_index_preference(&term)?;

    let output_style = get_output_style(&term)?;

    // Ok(config::ConfigYtPlaylist::new(url.clone(),
    //                               media,
    //                               format,
    //                               output_dir,
    //                               ,
    //                               preference,
    //                               output_style))
    todo!()
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
        0 => Ok(MediaSelection::Video),
        1 => Ok(MediaSelection::Audio),
        _ => panic!("Error getting media selection")
    }
}

mod format {
    use super::*;
    use spinoff::{Spinner, Spinners, Color};
    use std::process::{Command, Stdio};
    use execute::Execute;

    /// Asks the user to specify a download format and quality
    ///
    /// Either best-quality or worst-quality can be selected for the whole playlist, or a format can be picked for each
    /// video. If all videos have a format and quality in common, they can be easily applied
    pub(super) fn get_format(term: &Term, url: &String) -> Result<Vec<VideoFormats>, std::io::Error> {
        // To download multiple formats -f 22/17/18 chooses the one which is available and most to the left

        // Each element of this vector describes the quality option for a video in the playlist
        let mut all_preferences: Vec<VideoQualityAndFormatPreferences> = vec![];

        let mut user_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want a single quality setting to be applied to all videos or do you want to choose one for each video?")
            .default(0)
            .items(&["Apply a setting to all videos", "Choose a quality setting for each video"])
            .interact_on(&term)?;
        let quality_scope = match user_selection {
            0 => QualityScope::AllVideos,
            _ => QualityScope::SingleVideo,
        };

        // The user wants to choose a format to apply to all videos
        if quality_scope == QualityScope::AllVideos {
            let available_formats = fetch_formats(url)?;
            // Find formats common between all videos

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

    #[derive(PartialEq)]
    enum QualityScope {
        AllVideos,
        SingleVideo,
    }

    use crate::assembling::yt_video::config::{VideoFormat, VideoFormats, VideoQualityAndFormatPreferences};

    /// Returns a Vec with every video's format information
    fn fetch_formats(playlist_url: &String) -> Result<Vec<VideoFormats>, std::io::Error> {
        let mut command = Command::new("youtube-dl");
        command.arg("-F");
        command.arg(playlist_url);
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        // Store youtube-dl's full output and let the user know that something is going on
        let spinner = Spinner::new(Spinners::Dots12, "Fetching format info...", Color::Red);
        let output = command.execute_output()?;
        spinner.stop();
        // A lost of every video in the playlist's available formats
        let mut all_videos: Vec<VideoFormats> = Vec::new();
        /* A list of all the download formats available for a video, if its format information is unavailable
         * (maybe because it is age-restricted) and thus youtube-dl cannot fetch any information about it,
         * the Option is None. In every other case there is Some
         */
        let mut video = VideoFormats::new();

        for paragraph in String::from_utf8(output.stdout)
                                        .unwrap()
                                        .as_str()
                                        .split("[download] Downloading video") {
            // Create a new video on every iteration because pushing on a Vec requires moving
            let mut video = VideoFormats::new();
            // The first line is discarded, it tells information about the index of the current video in the playlist
            for line in paragraph.lines().skip(1) {
                // Ignore all irrelevant lines (they violate VideoFormat::from_command()'s contract
                if line.contains("[") ||
                    line.contains("format") ||
                    line.contains("video only") {
                    continue;
                };
                // The line is about a video or audio-only format or is a youtube-dl error
                video.add_format(VideoQualityAndFormatPreferences::UniqueFormat(VideoFormat::from_command(line)));
            }
            // Ignore some quirks of string splitting
            if video.is_empty() {
                continue;
            }
            // Add the current video to the "playlist"
            all_videos.push(video);
        };
        println!("Videos: {:#?}", all_videos);
        Ok(all_videos)
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