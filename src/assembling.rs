mod yt_playlist;
mod sp_track;
mod sp_playlist;
mod yt_video;
mod sp_album;

use crate::analyzer;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use dialoguer::console::Term;
use std::env;

/// [Rewrite this in the future] Calls the right wizard to generate the required command
pub(crate) fn generate_command(url: &String, download_option: analyzer::DownloadOption, verbose: bool) -> String {
    match download_option {
        analyzer::DownloadOption::YtPlaylist => yt_playlist::wizard::assemble_data(url, verbose).build_command(),
        analyzer::DownloadOption::YtVideo =>    yt_video::wizard::assemble_data(url, verbose).build_command(),
        analyzer::DownloadOption::SpTrack =>    sp_track::wizard::assemble_data(url, verbose).build_command(),
        analyzer::DownloadOption::SpPlaylist => sp_playlist::wizard::assemble_data(url, verbose).build_command(),
        analyzer::DownloadOption::SpAlbum =>    sp_album::wizard::assemble_data(url, verbose).build_command(),
    }
}

// Helper functions common to all wizards

/// Asks for an directory to store downloaded file(s) in
///
/// The current directory can be selected or one can be typed in
fn get_output_path(term: &Term) -> String {
    let output_path_options = &[
        "Current directory",
        "Other [specify]",
    ];

    let output_path = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Where do you want the downloaded file(s) to go?")
        .items(output_path_options)
        .interact_on(&term)
        .expect("Error getting path selection, please retry");

    match output_path {
        // Return the current directory
        0 => env::current_dir().expect("Problem getting the current directory")
            .as_path()
            .display()
            .to_string(),
        // Return a directory typed in by the user
        _ => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Output path:")
            .interact_text()
            .expect("Error getting path selection, please retry"),
    }
}
