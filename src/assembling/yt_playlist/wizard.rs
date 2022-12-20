use clap::ArgMatches;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use super::config;
use crate::assembling;

/// Returns a ConfigYtPlaylist object with all the necessary data
/// to start downloading a youtube playlist
///
/// Takes in the command line arguments list
pub fn assemble_data(matches: ArgMatches) -> config::ConfigYtPlaylist {
    let term = Term::buffered_stderr();

    let url: String = match matches.get_one::<String>("URL") {
        Some(_url) => _url.to_owned(),
        // This shouldn't happen as URL is a required argument
        None => assembling::get_url(),
    };
    let download_format: String = match matches.get_one::<String>("format") {
        Some(_format) => _format.to_owned(),
        // Ask for a format using a tutorial
        None => get_format(&term),
    };
    let output_path: String = match matches.get_one::<String>("output-path") {
        Some(_output_path) => _output_path.to_owned(),
        // Ask for an output path using a tutorial
        None => assembling::get_output_path(&term),
    };

    config::ConfigYtPlaylist::new(url, download_format, output_path, matches.get_flag("verbose"))
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