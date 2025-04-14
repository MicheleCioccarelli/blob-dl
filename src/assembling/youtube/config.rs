use crate::assembling::youtube;
use crate::analyzer;
use std::process;
use serde::{Deserialize, Serialize};
use crate::error::{BlobResult, BlobdlError};

/// Contains all the information needed to download a youtube video or playlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    url: Option<String>,
    
    output_path: Option<String>,
    /// Whether to include a file's index (in the playlist it is downloaded from) in its name
    include_indexes: Option<bool>,
    /// The quality and format the user wants the downloaded files to be in
    chosen_format: Option<youtube::VideoQualityAndFormatPreferences>,
    /// Whether the downloaded files have to be audio-only/video-only/normal video
    media_selected: Option<youtube::MediaSelection>,
    /// Whether the link refers to a playlist or a single video
    pub download_target: Option<analyzer::DownloadOption>,
}

impl DownloadConfig {
    // Creates a DownloadConfig with all fields set to None
    pub(crate) fn empty() -> DownloadConfig {
        DownloadConfig {
            url: None,
            output_path: None,
            include_indexes: None,
            chosen_format: None,
            media_selected: None,
            download_target: None,
        }
    }
    
    pub(crate) fn new_playlist (
        url: &str,
        output_path: String,
        include_indexes: bool,
        chosen_format: youtube::VideoQualityAndFormatPreferences,
        media_selected: youtube::MediaSelection,
    )
        -> DownloadConfig
    {
        DownloadConfig { 
            url: Some(url.to_string()), 
            output_path: Some(output_path), 
            include_indexes: Some(include_indexes), 
            chosen_format: Some(chosen_format), 
            media_selected: Some(media_selected),
            download_target: Some(analyzer::DownloadOption::YtPlaylist) }
    }

    pub(crate) fn new_video (
        url: &str,
        chosen_format: youtube::VideoQualityAndFormatPreferences,
        output_path: String,
        media_selected: youtube::MediaSelection,
    )
        -> DownloadConfig
    {
        DownloadConfig { 
            url: Some(url.to_string()), 
            chosen_format: Some(chosen_format), 
            output_path: Some(output_path), 
            media_selected: Some(media_selected),
            include_indexes: Some(false), 
            download_target: Some(analyzer::DownloadOption::YtVideo(0)) }
    }
}

// Command generation
// IMPORTANT WARNING: All of these functions expect every member of DownloadConfig to not be None, or else they will return errors
// The idea is to provide them before getting to this stage.
impl DownloadConfig {
    /// Builds a command according to the current configuration, which is also returned
    ///
    /// This function is meant for the main video-downloading task
    pub(crate) fn build_command(&self) -> BlobResult<(process::Command, DownloadConfig)> {
        if let Some(download_target) = &self.download_target {
            Ok((
                match download_target {
                    analyzer::DownloadOption::YtVideo(_) => self.build_yt_video_command()?,
                    analyzer::DownloadOption::YtPlaylist => self.build_yt_playlist_command()?,
                },
                self.clone()
            ))
        } else {
            Err(BlobdlError::DownloadTargetNotProvided)
        }
    }

    fn build_yt_playlist_command(&self) -> BlobResult<process::Command>{
        let mut command = process::Command::new("yt-dlp");

        // Continue even when errors are encountered
        command.arg("-i");

        // If the url refers to a video in a playlist, download the whole playlist
        command.arg("--yes-playlist");

        // Setup output directory and naming scheme
        self.choose_output_path(&mut command);

        // Makes the id live long enough to be used as an arg for command.
        // If it was fetched from the next match arm the temporary &str would not outlive command
        let id = match &self.chosen_format {
            Some(youtube::VideoQualityAndFormatPreferences::UniqueFormat(id)) => id.to_string(),
            _ => String::new(),
        };

        // Quality and format selection
        self.choose_format(&mut command, id.as_str());

        if let Some(url) = self.url.clone() {

            // Add the playlist's url
            command.arg(url);

            Ok(command)
        } else {
            Err(BlobdlError::UrlNotProvided)
        }
    }
    
    fn build_yt_video_command(&self) -> BlobResult<process::Command> {
        let mut command = process::Command::new("yt-dlp");

        self.choose_output_path(&mut command);

        if let Some(chosen_format) = &self.chosen_format {

            // Makes the id live long enough to be used as an arg for command.
            // If it was fetched from the next match arm the temporary &str would not outlive command
            let id = match chosen_format {
                youtube::VideoQualityAndFormatPreferences::UniqueFormat(id) => id.to_string(),
                _ => String::new(),
            };

            self.choose_format(&mut command, &id);

            command.arg("--no-playlist");

            // If they are available also download subtitles
            command.arg("--embed-subs");
            
            if let Some(url) = self.url.clone() {
                command.arg(url);
            } else {
                return Err(BlobdlError::UrlNotProvided);
            }

            Ok(command)
        } else {
            Err(BlobdlError::FormatPreferenceNotProvided)
        }
    }

    /// Downloads a new video while keeping the current preferences.
    ///
    /// This function is meant to be used to re-download videos which failed because of issues like bad internet
    pub fn build_command_for_video(&self, video_id: &str) -> BlobResult<process::Command> {
        let mut command = process::Command::new("yt-dlp");

        self.choose_output_path(&mut command);

        if let Some(chosen_format) = &self.chosen_format {

            // Makes the id live long enough to be used as an arg for command.
            // If it was fetched from the next match arm the temporary &str would not outlive command
            let id = match chosen_format {
                youtube::VideoQualityAndFormatPreferences::UniqueFormat(id) => id.to_string(),
                _ => String::new(),
            };

            self.choose_format(&mut command, id.as_str());

            command.arg("--no-playlist");

            command.arg(video_id);

            Ok(command)
        } else {
            Err(BlobdlError::FormatPreferenceNotProvided)
        }
    }

    // funzione un po' schifosa
    fn choose_output_path(&self, command: &mut process::Command) -> BlobResult<()> {
        if let Some(output_path) = &self.output_path {
            if let Some(download_target) = &self.download_target {
                if let Some(include_indexes) = self.include_indexes {
                    command.arg("-o");
                    command.arg(
                        {
                            let mut path_and_scheme = String::new();
                            // Add the user's output path (empty string for current directory)
                            path_and_scheme.push_str(output_path);

                            if *download_target == analyzer::DownloadOption::YtPlaylist {
                                // Create a directory named after the playlist
                                #[cfg(target_os = "windows")]
                                path_and_scheme.push_str("\\%(playlist)s\\");

                                #[cfg(not(target_os = "windows"))]
                                path_and_scheme.push_str("/%(playlist)s/");

                                if include_indexes {
                                    path_and_scheme.push_str("%(playlist_index)s_");
                                };
                                path_and_scheme.push_str("%(title)s");
                            } else {
                                // Downloading a yt_video
                                #[cfg(target_os = "windows")]
                                path_and_scheme.push_str("\\%(title)s.%(ext)s");

                                #[cfg(not(target_os = "windows"))]
                                path_and_scheme.push_str("/%(title)s.%(ext)s");
                            }

                            path_and_scheme
                        }
                    );
                    Ok(())

                } else {
                    Err(BlobdlError::IncludeIndexesNotProvided)
                }
            } else {
                Err(BlobdlError::DownloadTargetNotProvided)
            }
        } else {
            Err(BlobdlError::OutputPathNotProvided)
        }
    }

    fn choose_format(&self, command: &mut process::Command, format_id: &str) -> BlobResult<()> {
        if let Some(media_selected) = &self.media_selected {
            if let Some(chosen_format) = &self.chosen_format {
                match media_selected {
                    youtube::MediaSelection::FullVideo => {
                        match chosen_format {
                            youtube::VideoQualityAndFormatPreferences::BestQuality => {}

                            youtube::VideoQualityAndFormatPreferences::SmallestSize => {
                                command.arg("-S").arg("+size,+br");
                            }

                            youtube::VideoQualityAndFormatPreferences::UniqueFormat(_) => {
                                command.arg("-f").arg(format_id);
                            }
                            youtube::VideoQualityAndFormatPreferences::ConvertTo(f) => {
                                command.arg("--recode-video").arg(f.as_str());
                            }
                        }
                        // If they are available also download subtitles
                        command.arg("--embed-subs");
                    }

                    youtube::MediaSelection::AudioOnly => {
                        match chosen_format {
                            youtube::VideoQualityAndFormatPreferences::BestQuality => {
                                command.arg("-f").arg("bestaudio");
                            }

                            youtube::VideoQualityAndFormatPreferences::SmallestSize => {
                                command.arg("-f").arg("worstaudio");
                            }

                            youtube::VideoQualityAndFormatPreferences::UniqueFormat(_) => {
                                command.arg("-f").arg(format_id);
                            }
                            youtube::VideoQualityAndFormatPreferences::ConvertTo(f) => {
                                command.arg("-x").arg("--audio-format").arg(f.as_str());
                            }
                        }
                    }

                    youtube::MediaSelection::VideoOnly => {
                        match chosen_format {
                            youtube::VideoQualityAndFormatPreferences::BestQuality => {
                                command.arg("-f").arg("bestvideo");
                            }

                            youtube::VideoQualityAndFormatPreferences::SmallestSize => {
                                command.arg("-f").arg("worstvideo");
                            }

                            youtube::VideoQualityAndFormatPreferences::UniqueFormat(_) => {
                                command.arg("-f").arg(format_id);
                            }
                            youtube::VideoQualityAndFormatPreferences::ConvertTo(f) => {
                                command.arg("--recode-video").arg(f.as_str());
                            }
                        }
                        // If they are available also download subtitles
                        command.arg("--embed-subs");
                    }
                };
            } else {
                return Err(BlobdlError::ChosenFormatNotProvided);
            }
        } else {
            return Err(BlobdlError::MediaSelectedNotProvided);
        }
        Ok(())
    }
}
