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
        YtVideoConfig { url, chosen_format, output_path, media_selected }
    }
    /// Builds a yt-dl command with the needed specifications
    pub(crate) fn build_command(&self) -> process::Command {
        let mut command = process::Command::new("youtube-dl");

        // Setup output directory and naming scheme
        self.choose_output_path(&mut command);

        // Makes the id live long enough to be used as an arg for command.
        // If it was fetched from the next match arm the temporary &str would not outlive command
        let id = match &self.chosen_format {
            VideoQualityAndFormatPreferences::UniqueFormat(id) => id.to_string(),
            _ => String::new(),
        };

        // Quality and format selection
        self.choose_format(&mut command, &id);

        // Add the playlist's url
        command.arg(self.url);

        command
    }


    fn choose_output_path(&self, command: &mut process::Command) {
        command.arg("-o");
        command.arg(
            {
                let mut path_and_scheme = String::new();

                // Add the user's output path (empty string for current directory)
                path_and_scheme.push_str(self.output_path.as_str());

                // Add the video's title to the file name
                path_and_scheme.push_str("(title)s.%(ext)s");
                path_and_scheme
            });
    }

    fn choose_format(&self, command: &mut process::Command, id: &str) {
        match self.media_selected {
            MediaSelection::FullVideo => {
                match &self.chosen_format {
                    VideoQualityAndFormatPreferences::BestQuality => {}

                    VideoQualityAndFormatPreferences::SmallestSize => {
                        command.arg("-S").arg("+size,+br");
                    }

                    VideoQualityAndFormatPreferences::UniqueFormat(_) => {
                        command.arg("-f").arg(id);
                    }
                    VideoQualityAndFormatPreferences::ConvertTo(f) => {
                        command.arg("--recode-video").arg(f.as_str());
                    }
                }
            }

            MediaSelection::AudioOnly => {
                match &self.chosen_format {
                    VideoQualityAndFormatPreferences::BestQuality => {
                        command.arg("-f").arg("bestaudio");
                    }

                    VideoQualityAndFormatPreferences::SmallestSize => {
                        command.arg("-f").arg("worstaudio");
                    }

                    VideoQualityAndFormatPreferences::UniqueFormat(_) => {
                        command.arg("-f").arg(id);
                    }
                    VideoQualityAndFormatPreferences::ConvertTo(f) => {
                        command.arg("-x").arg("--audio-format").arg(f.as_str());
                    }
                }
            }

            MediaSelection::VideoOnly => {
                match &self.chosen_format {
                    VideoQualityAndFormatPreferences::BestQuality => {
                        command.arg("-f").arg("bestvideo");
                    }

                    VideoQualityAndFormatPreferences::SmallestSize => {
                        command.arg("-f").arg("worstvideo");
                    }

                    VideoQualityAndFormatPreferences::UniqueFormat(_) => {
                        command.arg("-f").arg(id);
                    }
                    VideoQualityAndFormatPreferences::ConvertTo(f) => {
                        command.arg("--recode-video").arg(f.as_str());
                    }
                }
            }
        };
    }
}
