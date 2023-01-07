mod yt_playlist;
mod sp_track;
mod sp_playlist;
mod yt_video;
mod sp_album;

use crate::analyzer;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use dialoguer::console::Term;
use std::env;
use clap::builder::Str;

// TODO: Re-read how to make children modules for better privacy management

/// [Rewrite this in the future] Calls the right wizard to generate the required command
pub(crate) fn generate_command(url: &String, download_option: &analyzer::DownloadOption) -> std::process::Command {
    todo!()
    /*match download_option {
        analyzer::DownloadOption::YtPlaylist => yt_playlist::wizard::assemble_data(url).build_command(),
        analyzer::DownloadOption::YtVideo =>    yt_video::wizard::assemble_data(url).build_command(),
        analyzer::DownloadOption::SpTrack =>    sp_track::wizard::assemble_data(url).build_command(),
        analyzer::DownloadOption::SpPlaylist => sp_playlist::wizard::assemble_data(url).build_command(),
        analyzer::DownloadOption::SpAlbum =>    sp_album::wizard::assemble_data(url).build_command(),
    }*/
}

/// Whether the user wants to download video files or audio-only
#[derive(Debug)]
pub(crate) enum MediaSelection {
    Video,
    Audio,
}

#[derive(Debug)]
// TODO rename this
pub(crate) enum OutputStyle {
    RedirectErrors,
    Full,
}

/// Asks for an directory to store downloaded file(s) in
///
/// The current directory can be selected or one can be typed in
fn get_output_path(term: &Term) -> Result<String, std::io::Error> {
    let output_path_options = &[
        "Current directory",
        "Other [specify]",
    ];

    let output_path = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Where do you want the downloaded file(s) to go?")
        .default(0)
        .items(output_path_options)
        .interact_on(&term)?;

    match output_path {
        // Return the current directory
        0 => Ok(env::current_dir()?
            .as_path()
            .display()
            .to_string()),
        // Return a directory typed in by the user
        _ => Ok(Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Output path:")
            .interact_text()?),
    }
}

mod youtube {
    pub(crate) enum VideoQualityAndFormatPreferences {
        CustomFormat(Format),
        BestQuality,
        WorstQuality,
    }
    /// Stores all information about a format available for a video (file extension, size, resolution, code)
    #[derive(Debug)]
     pub(crate) struct Format {
        code: u32,
        file_extension: String,
        resolution: String,
        // Size of the downloaded file
        size: String,
        audio_only: bool,
        // Whether the current format is the best available for a given video
        is_best: bool,
    }
    impl Format {
        /// # Usa insiemistica (intersezione tra insiemi di id) e mappe per comparare
        /// Returns an Option\<Format\> object when given a line from the output of the command
        /// "youtube-dl -F \<URL\>"
        /// # Contract
        /// This function returns `None` if it encounters any problems, it is up to the caller
        /// to check for them if they want to do error-handling.
        ///
        /// These conditions are listed in the `Returns None` section
        /// # Returns Some(Format)
        /// When `ytdl_output_line` contains information about (audio-only or video) quality and format for a youtube url
        ///
        /// # Returns None
        /// When `ytdl_output_line` isn't about video quality and format
        /// (for example lines starting with \[info\] or \[youtube\])
        ///
        /// When `ytdl_output_line` is about a video-only format
        ///
        /// When `ytdl_output_line` is an error/warning (these lines start with `ERROR` or `WARNING`)
        pub fn from_command(ytdl_output_line: &str) -> Option<Format> {
            // Collect all elements in a line in a single vector
            let table_elements: Vec<&str> = ytdl_output_line.split_whitespace().collect();
            // 8 is the minimum amount of fields in a valid output
            if table_elements.len() < 8 {
                eprintln!("This youtube-dl output line was rejected: {}", ytdl_output_line);
                return None;
            }
            let mut table_elements_iter = table_elements.into_iter();
            /*
            * Example of 3 valid lines with different properties:
            *
            * 18           mp4        640x360    360p  172k , avc1.42001E, 30fps, mp4a.40.2 (44100Hz)
            * 22           mp4        1280x720   720p  468k , avc1.64001F, 30fps, mp4a.40.2 (44100Hz) (best)
            * 140          m4a        audio only tiny  127k , m4a_dash container, mp4a.40.2@127k (44100Hz), 6.54MiB
            *
            * The fields are: code, extension, resolution/audio-only, quality, note, size, ..., (best)
            */

            let code: u32 = table_elements_iter.next()?.parse().ok()?;

            let file_extension = String::from(table_elements_iter.next()?);

            let resolution = String::from(table_elements_iter.next()?);

            // Audio only files' resolution is marked as "audio only", video files have an actual resolution
            let audio_only = if resolution == "audio" {
                // Skip "only"
                table_elements_iter.next();
                true
            } else {
                false
            };

            // Skip the "note" section of ytdl_output_line
            table_elements_iter.next();

            let mut size = String::from(table_elements_iter.next()?);
            if audio_only {
                // Audio-only with DASH note has one more field to be skipped
                if size == "audio" {
                    size = String::from(table_elements_iter.next()?);
                }
            }

            let last_element = table_elements_iter.last()?;

            // The last element of ytdl_output_line tells you whether this line had the best available format
            let is_best = if last_element == "(best)" {
                true
            } else {
                false
            };
            // All information has been parsed
            Some(Format {
                code: code,
                file_extension: file_extension,
                resolution: resolution,
                size: size,
                audio_only: audio_only,
                is_best: is_best,
            })
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
}
