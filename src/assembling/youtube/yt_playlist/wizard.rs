// Refactor some of these in the future
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use super::super::*;
use super::config;
use crate::assembling;

/// This is a wizard for downloading a youtube playlist
///
/// It asks for:
/// - Video or Audio
/// - Quality/Format
/// - Output path
/// - Index inclusion
///
/// Returns a fully configured YtPlaylistConfig, build_command() can be called
pub fn assemble_data(url: &String) -> Result<config::YtPlaylistConfig, std::io::Error> {
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
    /// All of the formats a playlist can be downloaded in
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
        -> Result<VideoQualityAndFormatPreferences, std::io::Error>
    {
        let ytdl_formats = get_ytdlp_formats(url)?;
        // A list of videos which are Vec of formats
        let mut all_available_formats = FormatsLibrary::new();

        // Compute which formats are common across the entire playlist
        let mut intersections: Vec<String> = vec![];
        let mut current_ids: Vec<String> = vec![];

        // Each line in ytdl_formats contains all the format information for 1 video
        for (i, video_formats_json) in std::str::from_utf8(&ytdl_formats.stdout)
                                .expect("The JSON including a video's format information contained non-UTF8 characters, please report this issue")
                                .lines()
                                .enumerate() {

            let serialized_video= serialize_formats(video_formats_json)?;
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

        // Choices displayed to the user
        let mut format_options = vec! [
            "Best available quality for each video".to_string(),
            "Worst available quality for each video".to_string()
        ];

        // Ids which the user can pick according to the current media selection
        let mut correct_ids = vec![];

        // Only look at ids common across the whole playlist
        for id in intersections.iter() {
            // Find which format corresponds to each id
            // common_formats is a Vec of all the formats for the first video.
            // Since we are looking for ids common to all videos just checking the first one is fine
            if let Some(first_video_formats) = all_available_formats.videos().first() {
                for format in first_video_formats.formats() {
                    // Skip audio-only files if the user wants full video
                    if *media_selected == MediaSelection::Video && format.resolution == "audio" {
                        continue;
                    }

                    // Skip video files if the user wants audio-only
                    if *media_selected == MediaSelection::Audio && format.resolution != "audio" {
                        continue;
                    }

                    if format.format_id == *id {
                        // Add to the list of available formats the current one formatted in a nice way
                        format_options.push(format.to_string());
                        // todo !! see if the references in correct_ids are still valid after the loop
                        correct_ids.push(id);
                    }
                }
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
            _ => Ok(VideoQualityAndFormatPreferences::UniqueFormat(correct_ids[user_selection - 2].clone()))
        }
    }
}

/// Returns an owned intersection Vec
fn intersection<'a, T: Eq + Clone> (vec1: &'a Vec<T>, vec2: &'a Vec<T>) -> Vec<T> {
    let mut intersections = vec![];

    for element_1 in vec1.iter() {
        for element_2 in vec2.iter() {
            if element_1 == element_2 {
                // Intersection element found!
                intersections.push(element_1.clone());
            }
        }
    }

    intersections
}

/// Whether the downloaded files should include their index in the playlist as a part of their name
fn get_index_preference(term: &Term) -> Result<bool, std::io::Error> {
    let download_formats = &[
        "Yes",
        "No",
    ];

    /*
    "Do you a video's index in the playlist to be in its name?
e.g. the video \"blob\" is the fourth in the playlist
Option chosen:		Yes		No
Resulting filename 	04_blob		blob"
*/

    // Ask the user which format they want the downloaded files to be in
    let index_preference = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you a video's index in the playlist to be in its name?")
        .default(0)
        .items(download_formats)
        .interact_on(term)?;

    match index_preference {
        0 => Ok(true),
        1 => Ok(false),
        _ => panic!("The only options are 0 and 1")
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_command() -> Result<(), std::io::Error> {
        let test_str = "139          m4a        audio only DASH audio   50k , m4a_dash container, mp4a.40.5 (22050Hz), 2.45MiB";
        let f = VideoFormat::from_command(test_str);
        let expected_format = VideoFormat {
            code: 139,
            file_extension: String::from("m4a"),
            resolution: String::from("audio"),
            size: String::from("50k"),
        };

        assert_eq!(f, expected_format);

        let test_str = "22           mp4        1280x720   720p  468k , avc1.64001F, 30fps, mp4a.40.2 (44100Hz) (best)";
        let f = VideoFormat::from_command(test_str);
        let expected_format = VideoFormat {
            code: 22,
            file_extension: String::from("mp4"),
            resolution: String::from("1280x720"),
            size: String::from("468k"),
        };

        assert_eq!(f, expected_format);
        Ok(())
    }
}*/