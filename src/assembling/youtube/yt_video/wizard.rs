use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use super::super::*;
use crate::assembling;
use crate::assembling::youtube::yt_video::config::YtVideoConfig;

/// Returns a ConfigYtVideo object with all the necessary data
/// to start downloading a youtube video
///
/// Takes in the command line arguments list
pub(crate) fn assemble_data(url: &String) -> Result<YtVideoConfig, std::io::Error> {
    let term = Term::buffered_stderr();

    // Whether the user wants to download video files or audio-only
    let media_selected = get_media_selection(&term)?;

    let chosen_format = get_format(&term, url, &media_selected)?;

    let output_path = assembling::get_output_path(&term)?;

    Ok(YtVideoConfig::new(
        url,
        chosen_format,
        output_path,
        media_selected,
    ))
}

/// Asks the user to choose a download format and quality between the ones
/// available for the current video.
///
/// The options are filtered between video, audio-only and video-only
fn get_format(term: &Term, url: &str, media_selected: &MediaSelection)
              -> Result<VideoQualityAndFormatPreferences, std::io::Error>
{
    // Get a JSON dump of all the available formats for the current url
    let ytdl_formats = get_ytdlp_formats(url)?;

    // todo this expect
    // Serialize the JSON which contains the format information for the current video
    let serialized_formats = serialize_formats(std::str::from_utf8(&ytdl_formats.stdout[..]).expect("Error managing JSON formats")).expect("Problem serializing Formats JSON");


    // A list of all the format options that can be picked
    let mut format_options = vec![
        "Best available quality for each video".to_string(),
        "I want the resulting videos to have the smallest size possible".to_string(),
    ];

    // Ids which the user can pick according to the current media selection
    let mut correct_ids = vec![];

    // Choose which formats to show to the user
    for format in serialized_formats.formats() {
        // Skip image formats
        if format.vcodec == "none" && format.acodec == "none" {
            continue;
        }

        // Skip audio-only files if the user wants full video
        if *media_selected == MediaSelection::Video && format.resolution == "audio only" {
            continue;
        }

        // Skip video files if the user wants audio-only
        if *media_selected == MediaSelection::AudioOnly && format.resolution != "audio only" {
            continue;
        }

        // Skip video-only files if the user doesn't want video-only
        if *media_selected == MediaSelection::VideoOnly && format.acodec != "none" {
            continue;
        }
        // Add to the list of available formats the current one formatted in a nice way
        format_options.push(format.to_string());
        // Update the list of ids which match what the user wants
        correct_ids.push(format.format_id.clone());
    }

    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which quality do you want to apply to all videos?")
        .default(0)
        .items(&format_options)
        .interact_on(term)?;

    match user_selection {
        0 => Ok(VideoQualityAndFormatPreferences::BestQuality),
        1 => Ok(VideoQualityAndFormatPreferences::SmallestSize),
        _ => Ok(VideoQualityAndFormatPreferences::UniqueFormat(correct_ids[user_selection - 2].clone()))
    }
}