mod youtube;

use crate::analyzer;
use crate::error::BlobResult;

use dialoguer::{theme::ColorfulTheme, Input, Select};
use dialoguer::console::Term;
use std::env;

/// [Rewrite this in the future] Calls the right wizard to generate the required command
pub(crate) fn generate_command(url: &String, download_option: &analyzer::DownloadOption) -> BlobResult<std::process::Command> {
    match download_option {
        analyzer::DownloadOption::YtPlaylist => Ok(youtube::yt_playlist::wizard::assemble_data(url)?.build_command()),

        analyzer::DownloadOption::YtVideo(playlist_id) => Ok(youtube::yt_video::wizard::assemble_data(url, *playlist_id)?.build_command()),
    }
}

/// Downloads stuff in the future :)
fn re_download(video_id: &str) {

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
        .interact_on(&term)?;

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
