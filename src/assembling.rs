mod yt_playlist;
mod sp_track;
mod sp_playlist;
mod yt_video;

use dialoguer::{theme::ColorfulTheme, Input, Select};
use dialoguer::console::Term;
use std::env;

trait CommandBuilder {
    fn build_command(&self) -> String;
}

// Put this in a new module
/// Calls the right wizard
fn dispatcher() {}

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

/// Asks for a url to download in a user-friendly way
fn get_url() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("url:")
        .interact_text()
        .expect("Undocumented library error")
}