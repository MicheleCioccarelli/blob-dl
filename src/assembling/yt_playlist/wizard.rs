use clap::ArgMatches;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use super::config;
use crate::assembling;
use url::Url;
use crate::assembling::MediaSelection;

/// Returns a ConfigYtPlaylist object with all the necessary data
/// to start downloading a youtube playlist
///
/// Takes in the command line arguments list
pub(crate) fn assemble_data(url: &String, verbose: bool) -> config::ConfigYtPlaylist {
    let term = Term::buffered_stderr();

    // Whether the user wants to download video files or audio-only
    let media = get_media_selection(&term);

    let format = get_format(&term, &media);

    let output = assembling::get_output_path(&term);

    let quality = get_quality(&term);

    let preference = get_index_preference(&term);

    config::ConfigYtPlaylist::new(url.clone(),
                                  media,
                                  format,
                                  output,
                                  quality,
                                  preference,
                                  verbose)
}

/// Asks the user whether they want to download video files or audio-only
fn get_media_selection(term: &Term) -> assembling::MediaSelection {
    let download_formats = &[
        "Video",
        "Audio-only",
    ];
    // Ask the user which format they want the downloaded files to be in
    let media_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to download video files or audio-only?")
        .default(0)
        .items(download_formats)
        .interact_on(&term).expect("Error getting the correct format, please retry");

    match media_selection {
        0 => assembling::MediaSelection::Video,
        1 => assembling::MediaSelection::Audio,
        _ => panic!("Error getting media selection")
    }
}

/// Aks for a download format in a user-friendly way.
///
/// This interface needs to be remade
fn get_format(term: &Term, media_selected: &MediaSelection) -> String {
    println!("format picker not yet implemented");
    match media_selected {
        MediaSelection::Video => {
            println!("mp4 = default for videos");
            String::from("mp4")
        }
        MediaSelection::Audio => {
            println!("mp3 = default for audio-only");
            String::from("mp3")
        }
    }
}

fn get_quality(term: &Term) -> assembling::Quality {
    let download_formats = &[
        "Best quality",
        "Worst quality",
    ];

    let quality_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which quality do you want the downloaded files to be in?")
        .default(0)
        .items(download_formats)
        .interact_on(&term).expect("Error getting the correct quality, please retry");

    match quality_selection {
        0 => assembling::Quality::Bestquality,
        1 => assembling::Quality::Worstquality,
        _ => panic!("Error getting quality selection")
    }
}

/// Whether the downloaded files should include their index in the playlist as a part of their name
fn get_index_preference(term: &Term) -> bool {
    let download_formats = &[
        "Yes",
        "No",
    ];
    // Ask the user which format they want the downloaded files to be in
    let index_preference = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you a video's index in the playlist to be in its name?")
        .default(0)
        .items(download_formats)
        .interact_on(&term).expect("Error getting the correct format, please retry");

    match index_preference {
        0 => true,
        1 => false,
        _ => panic!("Error getting media selection")
    }
}