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
        panic!("Add a quality slider for audio (--audio-quality QUALITY [0, 10]");
        // A list of all the format options that can be picked
        let mut format_options = vec![
            "Best possible quality [ffmpeg required]",
            "Smallest file size",
            "Choose a file format (only youtube-supported formats for this video) [no ffmpeg]",
            "Choose a file format (any format) [ffmpeg required]",
        ];

        let user_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which format do you want to apply to the video?")
            .default(0)
            .items(&format_options)
            .interact_on(term)?;

        match user_selection {
            0 => Ok(VideoQualityAndFormatPreferences::BestQuality),
            1 => Ok(VideoQualityAndFormatPreferences::SmallestSize),
            2 => get_specific_format(term, url, media_selected, playlist_id),
            _ => convert_to_format(term, media_selected),
        }
    }

    // Get a list of all available formats and ask the user to choose one
    fn get_specific_format(term: &Term, url: &str, media_selected: &MediaSelection, playlist_id: usize)
                           -> Result<VideoQualityAndFormatPreferences, std::io::Error>
    {
        let serialized_formats = {
            // Get a JSON dump of all the available formats for the current url
            let ytdl_formats = get_ytdlp_formats(url)?;

            // Serialize the JSON which contains the format information for the current video
            serialize_formats(
                std::str::from_utf8(&ytdl_formats.stdout[..])
                    .expect("The JSON information about this video's formats contained non-UTF8 characters")
                    // If `url` refers to a playlist the JSON has multiple roots, only parse one
                    .lines()
                    // If the requested video isn't the first in a playlist, only parse its information
                    .nth(playlist_id)
                    // Unwrap is safe because playlist_id is non-0 only when there are multiple lines in the json
                    .unwrap()
            ).expect("Problem serializing the video's formats from JSON")
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
            if *media_selected == MediaSelection::Video && format.resolution == "audio only" {
                continue;
            }

            // Skip video files if the user wants audio-only
            if *media_selected == MediaSelection::AudioOnly && format.resolution != "audio only" {
                continue;
            }

            // Skip video-only files if the user doesn't want video-only
            if *media_selected == MediaSelection::Video && format.acodec == "none" {
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

        // Setup the command-line prompt
        let user_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which format do you want to apply to the video?")
            .default(0)
            .items(&format_options)
            .interact_on(term)?;

        match user_selection {
            0 => Ok(VideoQualityAndFormatPreferences::BestQuality),
            _ => Ok(VideoQualityAndFormatPreferences::UniqueFormat(correct_ids[user_selection - 2].clone()))
        }
    }

    // Ask the user what container they want the downloaded file to be recoded to (ytdlp postprocessor) REQUIRES FFMPEG
    fn convert_to_format(term: &Term, media_selected: &MediaSelection)
                         -> Result<VideoQualityAndFormatPreferences, std::io::Error>
    {
        // Available formats for recoding
        let format_options = match *media_selected {
            MediaSelection::AudioOnly => vec!["mp3", "m4a", "wav", "aac", "alac", "flac", "opus", "vorbis"],

            _ => vec!["mp4", "mkv", "mov", "avi", "flv", "gif", "webm", "aac", "aiff",
                      "alac", "flac", "m4a", "mka", "mp3", "ogg", "opus", "vorbis", "wav"],
        };

        // Setting up the prompt
        let user_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which container do you want the final file to be in?")
            .default(0)
            .items(&format_options)
            .interact_on(term)?;

        Ok(VideoQualityAndFormatPreferences::ConvertTo(format_options[user_selection].to_string()))
    }
}