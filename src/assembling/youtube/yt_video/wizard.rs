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

/// Asks the user to choose a download format and quality
fn get_format(term: &Term, url: &str, media_selected: &MediaSelection)
    -> Result<VideoQualityAndFormatPreferences, std::io::Error>
{
    // Get a list of all the formats available for this video
    let ytdl_formats = get_ytdl_formats(url)?;
    // todo this expect
    // All formats available for this video in a Vec
    let available_formats = fetch_formats(String::from_utf8(ytdl_formats.stdout).expect("Fixme"))?;

    // A list of all the format options that can be picked
    let mut format_options = vec![
        "Best available quality for each video".to_string(),
        "Worst available quality for each video".to_string()
    ];

    // Ids which the user can pick according to the current media selection
    let mut correct_ids = vec![];

    // If there were some formats available, analyze them
    if let Some(first_video_formats) = available_formats.first() {
        for format in first_video_formats.available_formats() {
            // Skip audio-only files if the user wants full video
            if *media_selected == MediaSelection::Video && format.resolution == "audio" {
                continue;
            }

            // Skip video files if the user wants audio-only
            if *media_selected == MediaSelection::Audio && format.resolution != "audio" {
                continue;
            }

            // Add to the list of available formats the current one formatted in a nice way
            format_options.push(format.to_frontend());
            correct_ids.push(format.code);
            }
    }

    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which quality do you want to apply to all videos?")
        .default(0)
        .items(&format_options)
        .interact_on(term)?;

    match user_selection {
        0 => Ok(VideoQualityAndFormatPreferences::BestQuality),
        1 => Ok(VideoQualityAndFormatPreferences::WorstQuality),
        _ => Ok(VideoQualityAndFormatPreferences::UniqueFormat(correct_ids[user_selection - 2]))
    }
}