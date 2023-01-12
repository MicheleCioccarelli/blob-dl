use clap::builder::Str;
use crate::assembling;
use crate::assembling::yt_video;

/// Contains all the information needed to download a youtube playlist [WIP]
#[derive(Debug)]
pub(crate) struct YtPlaylistConfig<'a> {
    // Ref to the url stored in CliConfig
    url: &'a String,
    // Each element in the Vec is the quality that a video needs to be downloaded in
    download_format: Vec<yt_video::config::VideoQualityAndFormatPreferences>,
    output_path: String,
    /// Whether to include a file's index (in the playlist it is downloaded from) in its name
    include_indexes: bool,
    output_style: assembling::OutputStyle,
}

impl YtPlaylistConfig {
    pub(crate) fn new (
        url: String,
        media_selected: assembling::MediaSelection,
        download_format: Vec<yt_video::config::VideoQualityAndFormatPreferences>,
        output_path: String,
        include_indexes: bool,
        output_style: assembling::OutputStyle
    ) -> YtPlaylistConfig {
        YtPlaylistConfig { url, media_selected, download_format, output_path, include_indexes, output_style }
    }

    fn output_style(&self) -> &assembling::OutputStyle {
        &self.output_style
    }

    /// Builds a yt-dl command with the needed specifications (downloads a playlist)
    pub(crate) fn build_command(&self) -> std::process::Command {
        panic!("Command building is currently broken")
        // let mut command = std::process::Command::new("youtube-dl");
        //
        // // Continue even when errors are encountered
        // command.arg("-i");
        //
        // // Setup output directory and naming scheme
        // command.arg("-o");
        // command.arg(
        //     {
        //         let mut path_and_scheme = String::new();
        //
        //         // Add the user's output path (empty string for current directory)
        //         path_and_scheme.push_str(self.output_path.as_str());
        //
        //         // Create a directory named after the playlist
        //         path_and_scheme.push_str("/%(playlist)s/");
        //
        //         if self.include_indexes {
        //             path_and_scheme.push_str("%(playlist_index)s_");
        //         }
        //
        //         // Add the video's title to the file name
        //         path_and_scheme.push_str("%(title)s");
        //         // Add the correct file extension
        //         path_and_scheme.push_str(".%(ext)s");
        //         path_and_scheme
        //     });
        //
        // // Quality and format selection
        // match self.media_selected {
        //     assembling::MediaSelection::Video => {
        //         command.arg("-f");
        //         command.arg(
        //             {
        //                 let mut quality_format = match self.quality {
        //                     //Quality::Bestquality => String::from("best"),
        //                     //Quality::Worstquality => String::from("worst"),
        //                 };
        //
        //                 // Add file format
        //                 quality_format.push_str("[ext=");
        //                 quality_format.push_str(self.download_format.as_str());
        //                 quality_format.push_str("]");
        //                 quality_format
        //             });
        //     },
        //     assembling::MediaSelection::Audio => (),
        // };
        //
        // // Add the playlist's url
        // command.arg(&self.url);
        //
        // command
    }
}