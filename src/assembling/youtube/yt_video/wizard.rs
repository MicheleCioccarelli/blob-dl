use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use super::super::*;
use crate::assembling;
use crate::assembling::youtube::yt_video::config::YtVideoConfig;

/// Returns a ConfigYtVideo object with all the necessary data
/// to start downloading a youtube video
///
/// Takes in the command line arguments list
pub(crate) fn assemble_data(url: &String, playlist_id: usize) -> Result<YtVideoConfig, std::io::Error> {
    let term = Term::buffered_stderr();

    // Whether the user wants to download video files or audio-only
    let media_selected = get_media_selection(&term)?;

    let chosen_format = format::get_format(&term, url, &media_selected, playlist_id)?;

    let output_path = assembling::get_output_path(&term)?;

    Ok(YtVideoConfig::new(
        url,
        chosen_format,
        output_path,
        media_selected,
    ))
}

mod format {
    use super::*;

    /// Asks the user to choose a download format and quality between the ones
    /// available for the current video.
    ///
    /// The options are filtered between video, audio-only and video-only
    pub(super) fn get_format(term: &Term, url: &str, media_selected: &MediaSelection, playlist_id: usize)
                             -> Result<VideoQualityAndFormatPreferences, std::io::Error>
    {
        // A list of all the format options that can be picked
        let mut format_options = vec![
            crate::BEST_QUALITY_PROMPT,
            crate::SMALLEST_QUALITY_PROMPT,
            crate::YT_FORMAT_PROMPT,
            crate::CONVERT_FORMAT_PROMPT,
        ];

        // Set up a prompt for the user
        let user_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which format do you want to apply to the video?")
            .default(0)
            .items(&format_options)
            .interact_on(term)?;

        // See individual function documentations for more context
        match user_selection {
            0 => Ok(VideoQualityAndFormatPreferences::BestQuality),
            1 => Ok(VideoQualityAndFormatPreferences::SmallestSize),
            2 => get_format_from_yt(term, url, media_selected, playlist_id),
            _ => convert_to_format(term, media_selected),
        }
    }

    // Presents the user with the formats youtube provides directly for download, without the need for ffmpeg
    fn get_format_from_yt(term: &Term, url: &str, media_selected: &MediaSelection, playlist_id: usize)
                          -> Result<VideoQualityAndFormatPreferences, std::io::Error>
    {
        // Serialize all available formats from the youtube API (through yt-dlp -F)
        let serialized_formats = {
            // Get a JSON dump of all the available formats for the current url
            let ytdl_formats = get_ytdlp_formats(url)?;

            // Serialize the JSON which contains the format information for the current video
            serialize_formats(
                std::str::from_utf8(&ytdl_formats.stdout[..])
                    .expect(crate::JSON_PARSING_ERROR)
                    // If `url` refers to a playlist the JSON has multiple roots, only parse one
                    .lines()
                    // If the requested video isn't the first in a playlist, only parse its information
                    .nth(playlist_id)
                    // Unwrap is safe because playlist_id is non-0 only when there are multiple lines in the json
                    .unwrap()
            ).expect(crate::JSON_SERIALIZATION_ERROR)
        };

        // Ids which the user can pick according to the current media selection
        let mut correct_ids = vec![];
        // Every format which conforms to media_selected will be pushed here
        let mut format_options = vec![];

        // Choose which formats to show to the user
        for format in serialized_formats.formats() {
            // Skip image formats
            if format.vcodec == "none" && format.acodec == "none" {
                continue;
            }
            // Skip audio-only files if the user wants full video
            if *media_selected == MediaSelection::FullVideo && format.resolution == "audio only" {
                continue;
            }
            // Skip video files if the user wants audio-only
            if *media_selected == MediaSelection::AudioOnly && format.resolution != "audio only" {
                continue;
            }
            // Skip video-only files if the user doesn't want video-only
            if *media_selected == MediaSelection::FullVideo && format.acodec == "none" {
                continue;
            }
            // Skip normal video if the user wants video-only
            if *media_selected == MediaSelection::VideoOnly && format.acodec != "none" {
                continue;
            }

            // Add to the list of available formats the current one formatted in a nice way
            format_options.push(format.to_string());
            // Update the list of ids which match what the user wants
            correct_ids.push(format.format_id.clone());
        }

        // Set up a prompt for the user
        let user_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which format do you want to apply to the video?")
            .default(0)
            .items(&format_options)
            .interact_on(term)?;

        // Return the format corresponding to what the user selected, the choices are limited so there shouldn't be out-of-bounds problems
        Ok(VideoQualityAndFormatPreferences::UniqueFormat(correct_ids[user_selection - 2].clone()))
    }
}