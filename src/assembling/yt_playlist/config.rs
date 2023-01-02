use std::ops::Add;
use clap::builder::Str;
use crate::assembling;
use crate::assembling::Quality;

/// Contains all the information needed to download a youtube playlist [WIP]
#[derive(Debug)]
pub(crate) struct ConfigYtPlaylist {
    url: String,
    media_selected: assembling::MediaSelection,
    download_format: String,
    output_path: String,
    quality: assembling::Quality,
    /// Whether to include a file's index (in the playlist it is downloaded from) in its name
    include_indexes: bool,
    output_style: assembling::OutputStyle,
}

impl ConfigYtPlaylist {
    pub(crate) fn new (
        url: String,
        media_selected: assembling::MediaSelection,
        download_format: String,
        output_path: String,
        quality: assembling::Quality,
        include_indexes: bool,
        output_style: assembling::OutputStyle
    ) -> ConfigYtPlaylist {
        ConfigYtPlaylist { url, media_selected, download_format, output_path, quality, include_indexes, output_style }
    }

    fn output_style(&self) -> &assembling::OutputStyle {
        &self.output_style
    }

    /// Builds a yt-dl command with the needed specifications (downloads a playlist)
    pub(crate) fn build_command(&self) -> std::process::Command {
        let mut command = std::process::Command::new("youtube-dl");

        // Continue even when errors are encountered
        command.arg("-i");

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
                // Add the correct file extension
                path_and_scheme.push_str(".%(ext)s");
                path_and_scheme
            });

        // Quality and format selection
        match self.media_selected {
            assembling::MediaSelection::Video => {
                command.arg("-f");
                command.arg(
                    {
                        let mut quality_format = match self.quality {
                            Quality::Bestquality => String::from("best"),
                            Quality::Worstquality => String::from("worst"),
                        };

                        // Add file format
                        quality_format.push_str("[ext=");
                        quality_format.push_str(self.download_format.as_str());
                        quality_format.push_str("]");
                        quality_format
                    });
            },
            assembling::MediaSelection::Audio => (),
        };

        // Add the playlist's url
        command.arg(&self.url);

        command
    }
}

/// Stores all information about a given video's format and quality options
#[derive(Debug)]
pub(crate) struct Format {
    code: u32,
    file_extension: String,
    resolution: String,
    audio_only: bool,
}

impl Format {
    /// # TODO: Fix error handling
    /// # Usa insiemistica (intersezione tra insiemi di id) e mappe per comparare
    /// Returns an Option\<Format\> object when given a line from the output of the command
    /// "youtube-dl -F \<URL\>"
    /// # Returns Some(Format)
    /// When `ytdl_output_line` contains information about (audio-only or video) quality and format for a youtube url
    ///
    /// # Returns None
    /// If `ytdl_output_line` isn't about video quality and format
    /// (for example lines starting with \[info\] or \[youtube\])
    ///
    /// or when `ytdl_output_line` is about a video-only format
    pub(crate) fn from(ytdl_output_line: &str) -> Option<Format> {
        // Skip lines without useful format information
        if ytdl_output_line.contains("[") ||
            ytdl_output_line.contains("format") ||
            ytdl_output_line.contains("video only") ||
            ytdl_output_line.contains("ERROR") {
            return None;
        };
        let table_elements: Vec<&str> = line.split_whitespace().collect();
        let code = table_elements[0].parse().expect("Problem parsing id");
        let extension = String::from(table_elements[1]);
        let mut resolution = String::new();

        // Audio only files' resolution is marked as "audio only", video files have an actual resolution
        let audio_only =  if table_elements[2] == "audio" {
            true
        } else {
            resolution = String::from(table_elements[2]);
            false
        };
        Some(Format { code: code, file_extension: extension, resolution: resolution, audio_only: audio_only })
    }

    fn code(&self) -> u32 {
        self.code
    }
    fn file_extension(&self) -> &String {
        &self.file_extension
    }
    fn resolution(&self) -> &String {
        &self.resolution
    }
    fn audio_only(&self) -> bool {
        self.audio_only
    }
}