use clap::ArgMatches;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use super::config;
use crate::assembling;

/// Returns a ConfigYtPlaylist object with all the necessary data
/// to start downloading a youtube playlist
///
/// Takes in the command line arguments list
pub(crate) fn assemble_data(url: &String, verbose: bool) -> config::ConfigYtPlaylist {
    let term = Term::buffered_stderr();

    config::ConfigYtPlaylist::new(url: url,
                                  download_format: get_format(&term),
                                  output_path: assembling::get_output_path(&term),
                                  verbose)
}

/// Aks for a download format in a user-friendly way.
///
/// This interface needs to be remade
fn get_format(term: &Term) -> String {
    let download_formats = &[
        "Audio [mp3]",
        "Video [mp4]",
        "Other [specify]",
    ];
    // Ask the user which format they want the downloaded files to be in
    let format = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which format do you want the downloaded file(s) to be in?")
        .items(download_formats)
        .interact_on(&term).expect("Error getting the correct format, please retry");

    match format {
        0 => String::from("mp3"),
        1 => String::from("mp4"),
        // The user wants to provide a custom download format
        _ => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Format:")
            .interact_text()
            .expect("Undocumented library error"),
    }
}