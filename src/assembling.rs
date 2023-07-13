pub mod youtube;

use crate::analyzer;
use crate::error::BlobResult;
use crate::assembling::youtube::config;

use dialoguer::{theme::ColorfulTheme, Input, Select};
use dialoguer::console::Term;
use std::env;

/// Asks the user for specific download type specs (output path, download format, ...) and builds
/// a yt-dlp command according to them
///
/// Returns the command along with a DownloadConfig object, which contains all the user-specified preferences
pub(crate) fn generate_command(url: &String, download_option: &analyzer::DownloadOption) -> BlobResult<(std::process::Command, youtube::config::DownloadConfig)> {
    // fixme these nested matches
    // Get preferences from the user, various errors may occur
    let mut playlist_id: usize = 0;
    let unchecked_config = match download_option {
        analyzer::DownloadOption::YtPlaylist                    => youtube::yt_playlist::assemble_data(url),

        analyzer::DownloadOption::YtVideo(id) =>
            {
                playlist_id = *id;
                youtube::yt_video::assemble_data(url, *id)
            },
    };

    match unchecked_config {
        Ok(safe) => {
            let (command, local_config) = safe.build_command(playlist_id);
            Ok((command, local_config))
        }
        Err(err) => Err(err)
    }
}

/// Contains the download options for all videos
/// todo Make this replace YtPlaylistConfig and YtVideoConfig
#[derive(Debug)]
struct GenericConfig {
    chosen_format: youtube::VideoQualityAndFormatPreferences,
    output_path: String,
    media_selected: youtube::MediaSelection,
    include_indexes: bool,
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
