use super::super::*;

/// Contains all the information needed to download a youtube playlist [WIP]
#[derive(Debug)]
//todo make this pub(crate)
pub struct YtPlaylistConfig<'a> {
    /// Ref to the url stored in CliConfig
    url: &'a String,

    output_path: String,
    /// Whether to include a file's index (in the playlist it is downloaded from) in its name
    include_indexes: bool,
    /// The quality and format the user wants the downloaded files to be in
    /// Maybe put these extra flags in their own struct in the future
    chosen_format: VideoQualityAndFormatPreferences,
    /// Whether the downloaded files have to be audio-only or normal video
    media_selected: MediaSelection,
}

impl<'a> YtPlaylistConfig<'a> {
    pub(crate) fn new (
        url: &String,
        output_path: String,
        include_indexes: bool,
        chosen_format: VideoQualityAndFormatPreferences,
        media_selected: MediaSelection,
    ) -> YtPlaylistConfig {
        YtPlaylistConfig { url, output_path, include_indexes, chosen_format, media_selected }
    }

    /// Builds a yt-dl command with the needed specifications (downloads a playlist)
    pub(crate) fn build_command(&self) -> std::process::Command {
        let mut command = std::process::Command::new("youtube-dl");

        // Continue even when errors are encountered
        command.arg("-i");

        // If the url refers to a video in a playlist, download the whole playlist
        command.arg("--yes-playlist");

        // Setup output directory and naming scheme
        command.arg("-o");
        command.arg(
            {
                let mut path_and_scheme = String::new();

                // Add the user's output path (empty string for current directory)
                path_and_scheme.push_str(self.output_path.as_str());

                // Create a directory named after the playlist
                path_and_scheme.push_str("/%(playlist)s/");

                if self.include_indexes {
                    path_and_scheme.push_str("%(playlist_index)s_");
                }

                // Add the video's title to the file name
                path_and_scheme.push_str("%(title)s");
                path_and_scheme
            });

        // Makes the id live long enough to be used as an arg for command.
        // If it was fetched from the next match arm the temporary &str would not outlive command
        let id = match self.chosen_format {
            VideoQualityAndFormatPreferences::UniqueFormat(id) => id.to_string(),
            _ => String::new(),
        };

        // Quality and format selection
        command.arg("-f");
        match self.media_selected {
            MediaSelection::Video => {
                command.arg(
                    {
                        match self.chosen_format {
                            VideoQualityAndFormatPreferences::BestQuality => "best",
                            VideoQualityAndFormatPreferences::WorstQuality => "worst",
                            VideoQualityAndFormatPreferences::UniqueFormat(_) => id.as_str(),
                        }
                    });
            },

            MediaSelection::Audio => {
                command.arg(
                    {
                        match self.chosen_format {
                            VideoQualityAndFormatPreferences::BestQuality => "bestaudio",
                            VideoQualityAndFormatPreferences::WorstQuality => "worstaudio",
                            VideoQualityAndFormatPreferences::UniqueFormat(_) => id.as_str(),
                        }
                    });
            },
            _ => panic!("Not yet implemented"),
        };

        // Add the playlist's url
        command.arg(self.url);

        command
    }
}