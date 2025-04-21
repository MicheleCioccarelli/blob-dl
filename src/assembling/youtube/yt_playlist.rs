use crate::assembling::youtube;
use crate::assembling::youtube::*;
use crate::error::BlobResult;
use crate::ui_prompts::*;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use which::which;

/// This is a wizard for downloading a youtube playlist
///
/// It asks for:
/// - Video or Audio
/// - Quality/Format
/// - Output path
/// - Index inclusion
///
/// Returns a fully configured YtPlaylistConfig, build_command() can be called
/// 
/// User config is the information present in a config file. It has user preferences on things like which file format they prefer
/// knowing this blob-dl can avoid asking redundant questions
/// 
pub(crate) fn assemble_data(url: &str, user_config: youtube::config::DownloadConfig) -> BlobResult<config::DownloadConfig> {
    let term = Term::buffered_stderr();

    let media_selected;
    if let Some(media) = user_config.media_selected {
        // if a media selection was already present in the config file, use that
        media_selected = media;
    } else {
        // Whether the user wants to download video files or audio-only
        media_selected = get_media_selection(&term)?;
    }

    let chosen_format;
    if let Some(format) = user_config.chosen_format {
        // With config files it is possible to "force" blob-dl to try to use ffmpeg
        if let VideoQualityAndFormatPreferences::ConvertTo(_) = format {
            // The user wants their files to be converted to another format
            if !which("ffmpeg").is_ok() {
                // The conversion cannot be performed because ffmpeg is not installed
                chosen_format = format::get_format(&term, url, &media_selected)?;
            } else {
                // ffmpeg is installed so what was specified in the config file can be used
                chosen_format = format;
            }
        } else {
            chosen_format = format;
        }
    } else {
        // Config file didn't say anything about file formats
        chosen_format = format::get_format(&term, url, &media_selected)?;
    }

    let output_path;
    // .trim() trims trailing whitespace at the end of the user-specified path (useful is the user is clumsy)
    if let Some(path) = user_config.output_path {
        output_path = path;
    } else {
        output_path = get_output_path(&term)?.trim().to_string();
    }

    let include_indexes;
    if let Some(indexes) = user_config.include_indexes {
        include_indexes = indexes;
    } else {
        include_indexes = get_index_preference(&term)?;
    }
    
    Ok(config::DownloadConfig::new_playlist(
        url,
        output_path,
        include_indexes,
        chosen_format,
        media_selected,
    ))
}

mod format {
    /// All the formats a particular playlist can be downloaded in
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
    use crate::assembling::youtube::VideoSpecs;
    use crate::error::BlobdlError::JsonSerializationError;

    /// Asks the user to choose a download format and quality
    ///
    /// The chosen format will be applied to the entire playlist
    pub(super) fn get_format(term: &Term, url: &str, media_selected: &MediaSelection)
                             -> BlobResult<VideoQualityAndFormatPreferences>
    {

        // A list of all the format options that can be picked
        let mut format_options: Vec<&str> = vec![];
        // Default choices
        format_options.push(BEST_QUALITY_PROMPT_PLAYLIST);
        format_options.push(SMALLEST_QUALITY_PROMPT_PLAYLIST);

        if which("ffmpeg").is_ok() {
            // If ffmpeg is installed in the system
            // Some features are only available with ffmpeg
            match media_selected {
                MediaSelection::AudioOnly => format_options.push(CONVERT_FORMAT_PROMPT_AUDIO),
                _ => format_options.push(CONVERT_FORMAT_PROMPT_VIDEO_PLAYLIST)
            }

            format_options.push(YT_FORMAT_PROMPT_PLAYLIST);

            // Set up a prompt for the user
            let user_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Which quality or format do you want to apply to all videos?")
                .default(0)
                .items(&format_options)
                .interact_on(term)?;
            match user_selection {
                0 => Ok(VideoQualityAndFormatPreferences::BestQuality),
                1 => Ok(VideoQualityAndFormatPreferences::SmallestSize),
                2 => convert_to_format(term, media_selected),
                _ => get_format_from_yt(term, url, media_selected),
            }
        } else {
            println!("{}", FFMPEG_UNAVAILABLE_WARNING);
            // ffmpeg isn't installed, so ffmpeg-exclusive features are unavailable (video remuxing)
            format_options.push(YT_FORMAT_PROMPT_PLAYLIST);

            // Set up a prompt for the user
            let user_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Which quality or format do you want to apply to all videos?")
                .default(0)
                .items(&format_options)
                .interact_on(term)?;
            match user_selection {
                0 => Ok(VideoQualityAndFormatPreferences::BestQuality),
                1 => Ok(VideoQualityAndFormatPreferences::SmallestSize),
                _ => get_format_from_yt(term, url, media_selected),
            }
        }
    }

    // Show the user a list of formats common across the whole playlist, picked from those available directly from yt.
    fn get_format_from_yt(term: &Term, url: &str, media_selected: &MediaSelection)
                          -> BlobResult<VideoQualityAndFormatPreferences>
    {
        // Get a list of all the formats available for the playlist
        let ytdl_formats = get_ytdlp_formats(url)?;

        // If stdout is empty ytdlp had an error and the formats aren't available
        if ytdl_formats.stdout.is_empty() {
            return Err(JsonSerializationError)
        }

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
                    // If format and media_selected are compatible and this is the correct id
                    if check_format(format, media_selected) && format.format_id == *id {
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

    /// All the formats for all the videos in a playlist
    #[derive(Serialize, Deserialize, Debug)]
    struct Playlist {
        #[serde(rename = "entries")]
        videos: Vec<Option<VideoSpecs>>
    }
    
    /// Finds the formats available for all videos in the playlist and the list of all the available formats
    fn get_common_formats(json_formats: process::Output) -> BlobResult<(Vec<String>, FormatsLibrary)> {
        // A list of videos, which are Vec of formats
        let mut all_available_formats = FormatsLibrary::new();

        // Compute which formats are common across the entire playlist
        let mut intersections: Vec<String> = vec![];
        let mut all_ids: Vec<String> = vec![];
        
        let raw_json = std::str::from_utf8(&json_formats.stdout)?;
        
        let all_playlist_formats: serde_json::error::Result<Playlist> = serde_json::from_str(raw_json);

        match all_playlist_formats {
            Ok(all_playlist_formats) => {
                for (i, video) in all_playlist_formats.videos.into_iter().enumerate() {
                    if let Some(video) = video {
                        // Save the format
                        all_available_formats.add_video(video.clone());
                        // video is a Vector of formats
                        if i == 0 {
                            // In the first iteration every format-id belongs in the intersection
                            for format in video.formats {
                                intersections.push(format.format_id.to_string());
                                all_ids.push(format.format_id.to_string());
                            }
                        } else {
                            // For all other videos just add their format to the list of all formats
                            for format in video.formats {
                                all_ids.push(format.format_id.to_string());
                            }
                            // Actually compute the intersection
                            intersections = intersection(&intersections, &all_ids);
                        }
                        
                    }
                }
            },
            Err(err) => return Err(BlobdlError::SerdeError(err))
        }
        
        Ok((intersections, all_available_formats))
    }
}

/// Returns an owned intersection Vec
fn intersection<'a, T: Eq + Clone>(vec1: &'a [T], vec2: &'a [T]) -> Vec<T> {
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
        .with_prompt("Do you want the files to be numbered as in the playlist?")
        .default(0)
        .items(download_formats)
        .interact_on(term)?;

    match index_preference {
        0 => Ok(true),
        _ => Ok(false),
    }
}
