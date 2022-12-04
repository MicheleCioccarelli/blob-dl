use std::env;
use super::preferences;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn go(should_set_playlist: bool) -> preferences::Preferences {
    println!("Welcome [insert nice text here]");
    let term = Term::buffered_stderr();

    let url: String =
        if should_set_playlist {
            set_url()
        } else {
            String::new()
        };
    let download_format = get_format(&term);
    let output_path = get_output_path(&term);

    println!("Selected format: {download_format}");
    println!("Selected Directory: {output_path}");
    println!("Selected url: {url}");
    preferences::Preferences::new()
}

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

fn set_url() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("url:")
        .interact_text()
        .unwrap()
}