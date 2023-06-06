use crate::assembling::youtube;
use crate::analyzer;
use std::process;

/// Contains all the information needed to download a youtube playlist [WIP]
#[derive(Debug, Clone)]
pub struct DownloadConfig {
    /// Ref to the url stored in CliConfig
    url: String,

    output_path: String,
    /// Whether to include a file's index (in the playlist it is downloaded from) in its name
    include_indexes: bool,
    /// The quality and format the user wants the downloaded files to be in
    /// Maybe put these extra flags in their own struct in the future
    chosen_format: youtube::VideoQualityAndFormatPreferences,
    /// Whether the downloaded files have to be audio-only or normal video
    media_selected: youtube::MediaSelection,

    /// Whether the link refers to a playlist or a single video
    download_target: analyzer::DownloadOption,
}

// Constructors
impl DownloadConfig {
    pub(crate) fn new_playlist (
        url: &str,
        output_path: String,
        include_indexes: bool,
        chosen_format: youtube::VideoQualityAndFormatPreferences,
        media_selected: youtube::MediaSelection,
    )
        -> DownloadConfig
    {
        DownloadConfig { url: url.to_string(), output_path, include_indexes, chosen_format, media_selected,
            download_target: analyzer::DownloadOption::YtPlaylist }
    }

    pub(crate) fn new_video (
        url: &str,
        chosen_format: youtube::VideoQualityAndFormatPreferences,
        output_path: String,
        media_selected: youtube::MediaSelection,
    )
        -> DownloadConfig
    {
        DownloadConfig { url: url.to_string(), chosen_format, output_path, media_selected,
            include_indexes: false, download_target: analyzer::DownloadOption::YtVideo(0) }
    }
}

// Command generation
impl DownloadConfig {
    /// Builds a command according to the current configuration, which is also returned
    ///
    /// This function is meant for the main video-downloading task
    pub(crate) fn build_command(&self) -> (process::Command, DownloadConfig) {
        (
            match self.download_target {
                analyzer::DownloadOption::YtVideo(_) => self.build_yt_video_command(),
                analyzer::DownloadOption::YtPlaylist => self.build_yt_playlist_command(),
            },

            self.clone()
        )
    }

    fn build_yt_playlist_command(&self) -> process::Command {
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
            youtube::VideoQualityAndFormatPreferences::UniqueFormat(id) => id.to_string(),
            _ => String::new(),
        };

        // Quality and format selection
        self.choose_format(&mut command, id.as_str());

        // Add the playlist's url
        command.arg(self.url.clone());

        command
    }

    fn build_yt_video_command(&self) -> process::Command {
        let mut command = process::Command::new("yt-dlp");

        // Setup output directory and naming scheme
        self.choose_output_path(&mut command);

        // Makes the id live long enough to be used as an arg for command.
        // If it was fetched from the next match arm the temporary &str would not outlive command
        let id = match &self.chosen_format {
            youtube::VideoQualityAndFormatPreferences::UniqueFormat(id) => id.to_string(),
            _ => String::new(),
        };

        // Quality and format selection
        self.choose_format(&mut command, &id);

        // Add the playlist's url
        command.arg(self.url.clone());

        command
    }

    /// Downloads a new video while keeping the current preferences.
    ///
    /// This function is meant to be used to re-download videos which failed because of issues like bad internet
    pub fn build_command_for_video(&self, video_id: &str) -> process::Command {
        let mut command = process::Command::new("yt-dlp");

        // Setup output directory and naming scheme
        self.choose_output_path(&mut command);

        // Makes the id live long enough to be used as an arg for command.
        // If it was fetched from the next match arm the temporary &str would not outlive command
        let id = match &self.chosen_format {
            youtube::VideoQualityAndFormatPreferences::UniqueFormat(id) => id.to_string(),
            _ => String::new(),
        };

        // Quality and format selection
        self.choose_format(&mut command, id.as_str());

        // Add the video's id
        command.arg(video_id);

        command
    }

    fn choose_output_path(&self, command: &mut process::Command) {
        command.arg("-o");
        command.arg(
            {
                let mut path_and_scheme = String::new();
                // Add the user's output path (empty string for current directory)
                path_and_scheme.push_str(self.output_path.as_str());

                if self.download_target == analyzer::DownloadOption::YtPlaylist {
                    // Create a directory named after the playlist
                    #[cfg(target_os = "windows")]
                    path_and_scheme.push_str("\\%(playlist)s\\");

                    #[cfg(not(target_os = "windows"))]
                    path_and_scheme.push_str("/%(playlist)s/");

                    if self.include_indexes {
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
    }

    fn choose_format(&self, command: &mut process::Command, format_id: &str) {
        match self.media_selected {
            youtube::MediaSelection::FullVideo => {
                match &self.chosen_format {
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
            }

            youtube::MediaSelection::AudioOnly => {
                match &self.chosen_format {
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
                match &self.chosen_format {
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
            }
        };
    }
}