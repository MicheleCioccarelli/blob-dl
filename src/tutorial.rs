use std::env;
use super::preferences;

// Potential BS
use clap::ArgMatches;

use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn go(matches: ArgMatches) -> preferences::Preferences {
    println!("Welcome to the Tutorial! [insert nice text here]");
    let term = Term::buffered_stderr();

    let url: String = match matches.get_one::<String>("URL") {
        Some(_url) => _url.to_owned(),
        // This shouldn't happen as URL is a required argument
        None => get_url(),
    };

    let download_format: String = match matches.get_one::<String>("format") {
        Some(_format) => _format.to_owned(),
        // Ask for a format using a tutorial
        None => get_format(&term),
    };

    let output_path: String = match matches.get_one::<String>("output-path") {
        Some(_o) => _o.to_owned(),
        // Ask for an output path using a tutorial
        None => get_output_path(&term),
    };

    preferences::Preferences::build(url, download_format, output_path)
}

/// No file was provided as a command line argument, this function asks for it in a user-friendly way.
///
/// The user can select .mp3, .mp4 or specify one
fn get_format(term: &Term) -> String {
    let download_formats = &[
        "Audio [.mp3]",
        "Video [.mp4]",
        "Other [specify]",
        ];
    // Ask the user which format they want the downloaded files to be in
    let format = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Which format do you want to use?")
                        .items(download_formats)
                        .interact_on(&term)
                        .unwrap();
    match format {
        0 => String::from("mp3"),
        1 => String::from("mp4"),
        // The user wants to provide a custom download format
        _ => Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Format:")
                .interact_text()
                .unwrap(),
    }
}

/// No output path was provided as a command line argument, this function asks for it in a user-friendly way.
///
/// The user can select the current directory or specify one
fn get_output_path(term: &Term) -> String {
    let output_path_options = &[
        "Current directory",
        "Other [specify]",
        ];

    let output_path = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Where do you want the downloaded file(s) to go?")
                        .items(output_path_options)
                        .interact_on(&term)
                        .unwrap();

    match output_path {
        // Return the current directory
        0 => env::current_dir()
                .expect("Problem getting current directory")
                .as_path()
                .display()
                .to_string(),
        // Return a directory typed in by the user
        _ => Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Output path:")
                .interact_text()
                .unwrap(),
    }
}
/// No url was provided as a command line argument, this function asks for it in a user-friendly way
fn get_url() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("url:")
        .interact_text()
        .unwrap()
}