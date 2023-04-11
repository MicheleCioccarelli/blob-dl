use super::super::*;

/// Contains all the information needed to download a youtube video [WIP]
#[derive(Debug)]
pub(crate) struct YtVideoConfig<'a> {
    // Ref to the url stored in CliConfig
    url: &'a String,
    /* Deleted:
     *    /// All formats this video can be downloaded in, fetched using `youtube-dl -F url`
     *    available_formats: VideoSpecs,
     * If strange problems arise put this back
     */
    chosen_format: VideoQualityAndFormatPreferences,
    output_path: String,
    media_selected: MediaSelection,
}

// Similar to YTPlaylist's build_command()
impl<'a> YtVideoConfig<'a> {
    pub(crate) fn new(url: &String,
                      chosen_format: VideoQualityAndFormatPreferences,
                      output_path: String,
                      media_selected: MediaSelection)
                      -> YtVideoConfig {
        // Processing available_formats
        YtVideoConfig { url, chosen_format, output_path, media_selected}
    }
    /// Builds a yt-dl command with the needed specifications
    pub(crate) fn build_command(&self) -> process::Command {
        let mut command = process::Command::new("youtube-dl");

        // Setup output directory and naming scheme
        command.arg("-o");
        command.arg(
            {
                let mut path_and_scheme = String::new();

                // Add the user's output path (empty string for current directory)
                path_and_scheme.push_str(self.output_path.as_str());

                // Add the video's title to the file name
                path_and_scheme.push_str("%(title)s");
                path_and_scheme
            });

        // Makes the id live long enough to be used as an arg for command.
        // If it was fetched from the next match arm the temporary &str would not outlive command
        let id = match &self.chosen_format {
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
