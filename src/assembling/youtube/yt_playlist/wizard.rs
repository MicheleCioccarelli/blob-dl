// Refactor some of these in the future
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use super::super::*;
use super::config;
use crate::assembling;

use crate::BlobResult;

/// This is a wizard for downloading a youtube playlist
///
/// It asks for:
/// - Video or Audio
/// - Quality/Format
/// - Output path
/// - Index inclusion
///
/// Returns a fully configured YtPlaylistConfig, build_command() can be called
pub(super) fn assemble_data(url: &String) -> BlobResult<config::YtPlaylistConfig> {
    println!("Playlist btw");
    let term = Term::buffered_stderr();

    // Whether the user wants to download video files or audio-only
    let media_selected = get_media_selection(&term)?;

    let chosen_format = format::get_format(&term, url, &media_selected)?;

    let output_path = assembling::get_output_path(&term)?;

    let include_indexes = get_index_preference(&term)?;

    Ok(config::YtPlaylistConfig::new(
        url,
        output_path,
        include_indexes,
        chosen_format,
        media_selected,
    ))
}

mod format {
    /// All of the formats a particular playlist can be downloaded in
    ///
    /// These are divided in videos
    ///
    #[derive(Debug)]
    struct FormatsLibrary {
        videos: Vec<VideoSpecs>,
    }
    impl FormatsLibrary {
        pub fn new() -> Self {
            FormatsLibrary { videos: vec![] }
        }
        pub fn add_video(&mut self, video_formats: VideoSpecs) {
            self.videos.push(video_formats)
        }
        pub fn videos(&self) -> &Vec<VideoSpecs> {
            &self.videos
        }
    }

    use super::*;

    /// Asks the user to choose a download format and quality
    ///
    /// The chosen format will be applied to the entire playlist
    pub(super) fn get_format(term: &Term, url: &str, media_selected: &MediaSelection)
        -> BlobResult<VideoQualityAndFormatPreferences>
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
            .with_prompt("Which quality do you want to apply to all videos?")
            .default(0)
            .items(&format_options)
            .interact_on(term)?;

        match user_selection {
            0 => Ok(VideoQualityAndFormatPreferences::BestQuality),
            1 => Ok(VideoQualityAndFormatPreferences::SmallestSize),
            2 => get_format_from_yt(term, url, media_selected),
            _ => convert_to_format(term, media_selected),
        }
    }

    // Show the user a list of formats common across the whole playlist, picked from those available directly from yt.
    fn get_format_from_yt(term: &Term, url: &str, media_selected: &MediaSelection)
                          -> BlobResult<VideoQualityAndFormatPreferences>
    {
        // Get a list of all the formats available for the playlist
        let ytdl_formats = get_ytdlp_formats(url)?;

        // Filter out formats not available for all the videos
        let (intersections, all_available_formats) = get_common_formats(ytdl_formats)?;

        // Ids which the user can pick according to the current media selection (VideoOnly / AudioOnly / FullVideo)
        let mut correct_ids = vec![];
        // Format options that will be shown to the user
        let mut ui_format_options = vec![];

        // Only look at ids common across the whole playlist
        for id in intersections.iter() {
            // Since we are looking for ids common to all videos just checking the first one is fine
            if let Some(first_video_formats) = all_available_formats.videos().first() {
                for format in first_video_formats.formats() {
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
                    //Skip normal video if the user wants video-only
                    if *media_selected == MediaSelection::VideoOnly && format.acodec != "none" {
                        continue;
                    }

                    if format.format_id == *id {
                        // Add to the list of available formats the current one formatted in a nice way
                        ui_format_options.push(format.to_string());
                        correct_ids.push(id);
                    }
                }
            }
        }

        let user_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which quality do you want to apply to all videos?")
            .default(0)
            .items(&ui_format_options)
            .interact_on(term)?;

        Ok(VideoQualityAndFormatPreferences::UniqueFormat(correct_ids[user_selection].clone()))
    }

    // Finds the formats available for all videos in the playlist and the list of all the available formats
    fn get_common_formats(json_formats: process::Output) -> BlobResult<(Vec<String>, FormatsLibrary)> {
        // A list of videos, which are Vec of formats
        let mut all_available_formats = FormatsLibrary::new();

        // Compute which formats are common across the entire playlist

        let mut intersections: Vec<String> = vec![];
        let mut current_ids: Vec<String> = vec![];

        // Each line in ytdl_formats contains all the format information for 1 video
        for (i, video_formats_json) in std::str::from_utf8(&json_formats.stdout)?
                .lines()
                .enumerate() {
            let serialized_video = serialize_formats(video_formats_json)?;
            // Add the current video's formats to the list of all formats
            all_available_formats.add_video(serialized_video);

            if i == 0 {
                // In the first iteration the intersection is all the ids
                for format in all_available_formats.videos()[i].formats().iter() {
                    intersections.push(format.format_id.clone());
                }
            } else {
                for format in all_available_formats.videos()[i].formats().iter() {
                    current_ids.push(format.format_id.clone());
                }
                intersections = intersection(&intersections, &current_ids);
            }
        }

        Ok((intersections, all_available_formats))
    }
}

/// Returns an owned intersection Vec
fn intersection<'a, T: Eq + Clone>(vec1: &'a Vec<T>, vec2: &'a Vec<T>) -> Vec<T> {
    let mut intersections = vec![];

    for (element_1, element_2) in vec1.iter().zip(vec2.iter()) {
        if element_1 == element_2 {
            // Intersection element found!
            intersections.push(element_1.clone());
        }
    }

    intersections
}

/// Whether the downloaded files should include their index in the playlist as a part of their name
fn get_index_preference(term: &Term) -> BlobResult<bool> {
    let download_formats = &[
        "Yes",
        "No",
    ];

    // Ask the user which format they want the downloaded files to be in
    let index_preference = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want the video's index (from the playlist) to be prefixed in its filename?")
        .default(0)
        .items(download_formats)
        .interact_on(term)?;

    match index_preference {
        0 => Ok(true),
        1 => Ok(false),
        _ => panic!("The only options are 0 and 1")
    }
}
