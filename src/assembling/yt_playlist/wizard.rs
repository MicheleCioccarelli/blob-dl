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
pub(crate) fn assemble_data(url: &String) -> Result<config::ConfigYtPlaylist, std::io::Error> {
    let term = Term::buffered_stderr();

    // Whether the user wants to download video files or audio-only
    let media = get_media_selection(&term)?;

    let format = get_format(&term, &media);

    let output_dir = assembling::get_output_path(&term)?;

    let quality = get_quality(&term)?;

    let preference = get_index_preference(&term)?;

    let output_style = get_output_style(&term)?;

    Ok(config::ConfigYtPlaylist::new(url.clone(),
                                  media,
                                  format,
                                  output_dir,
                                  quality,
                                  preference,
                                  output_style))
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
        0 => Ok(assembling::MediaSelection::Video),
        1 => Ok(assembling::MediaSelection::Audio),
        _ => panic!("Error getting media selection")
    }
}

/// Asks the user to specify a download format and quality
///
/// Either best-quality or worst-quality can be selected for the whole playlist, or a format can be picked for each
/// video. If all videos have a format and quality in common, they can be easily applied
fn get_format(term: &Term, media_selected: &MediaSelection) -> String {
    // To download multiple formats -f 22/17/18 chooses the one which is available and most to the left
    todo!()
}

fn get_quality(term: &Term) -> Result<assembling::Quality, std::io::Error> {
    let download_formats = &[
        "Best quality",
        "Worst quality",
    ];

    let quality_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which quality do you want the downloaded files to be in?")
        .default(0)
        .items(download_formats)
        .interact_on(&term)?;

    match quality_selection {
        0 => Ok(assembling::Quality::Bestquality),
        1 => Ok(assembling::Quality::Worstquality),
        _ => panic!("he only options are 0 and 1")
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