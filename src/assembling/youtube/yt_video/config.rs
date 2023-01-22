use super::super::*;

/// Contains all the information needed to download a youtube video [WIP]
#[derive(Debug)]
pub(crate) struct YtVideoConfig<'a> {
    // Ref to the url stored in CliConfig
    url: &'a String,
    /// All formats this video can be downloaded in, fetched using `youtube-dl -F url`
    available_formats: VideoSpecs,
    selected_quality: VideoQualityAndFormatPreferences,
    output_path: String,
}

impl<'a> YtVideoConfig<'a> {
    pub(crate) fn new(url: &String,
                      available_formats: VideoSpecs,
                      selected_quality: VideoQualityAndFormatPreferences,
                      output_path: String)
                      -> YtVideoConfig {
        // Processing available_formats
        YtVideoConfig { url, available_formats, selected_quality, output_path}
    }
    /// Builds a yt-dl command with the needed specifications
    pub(crate) fn build_command(&self) -> std::process::Command {
        todo!()
    }
}
