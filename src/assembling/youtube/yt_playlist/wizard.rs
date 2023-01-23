// Refactor some of these in the future
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use super::super::*;
use super::config;
use crate::assembling;
// todo make this pub(crate)
/// Returns a ConfigYtPlaylist object with all the necessary data
/// to start downloading a youtube playlist
///
/// Takes in the command line arguments list
pub fn assemble_data(url: &String) -> Result<config::YtPlaylistConfig, std::io::Error> {
    let term = Term::buffered_stderr();

    // Whether the user wants to download video files or audio-only
    let media = get_media_selection(&term)?;

    let available_formats = format::get_format(&term, url, &media)?;

    //let output_dir = assembling::get_output_path(&term)?;

    //let preference = get_index_preference(&term)?;

    // Ok(config::ConfigYtPlaylist::new(url.clone(),
    //                               media,
    //                               format,
    //                               output_dir,
    //                               ,
    //                               preference,
    //                               output_style))
    todo!()

}

/// Asks the user whether they want to download video files or audio-only
fn get_media_selection(term: &Term) -> Result<MediaSelection, std::io::Error> {
    let download_formats = &[
        "Video",
        "Audio-only",
    ];
    // Ask the user which format they want the downloaded files to be in
    let media_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to download video files or audio-only?")
        .default(0)
        .items(download_formats)
        .interact_on(term)?;

    match media_selection {
        0 => Ok(MediaSelection::Video),
        1 => Ok(MediaSelection::Audio),
        _ => panic!("Error getting media selection")
    }
}

mod format {
    use super::*;
    // Doodles to entertain the user while file formats are being fetched
    use spinoff::{Spinner, Spinners, Color};
    use std::process::{Command, Stdio};
    // Running youtube-dl -F <...>
    use execute::Execute;
    // Math library for finding the intersection of all available format ids
    use sdset::multi::OpBuilder;
    use sdset::{SetOperation, Set, SetBuf};
    use spinoff::Color::Magenta;

    /// Asks the user to specify a download format and quality
    ///
    /// Either best-quality or worst-quality can be selected for the whole playlist, or a format can be picked for each
    /// video. If all videos have a format and quality in common, they can be easily applied
    // todo change this visibility to pub(super)
    pub fn get_format(term: &Term, url: &str, media_selected: &MediaSelection) -> Result<Vec<VideoSpecs>, std::io::Error> {
        // todo encapsulate into more functions
        // To download multiple formats -f 22/17/18 chooses the one which is available and most to the left
        let sp = Spinner::new(Spinners::Dots10, "Fetching available formats...", Magenta);
        // Fetch all available formats for the playlist
        let mut command = Command::new("youtube-dl");
        command.arg("-F");
        command.arg(url);
        command.stdout(Stdio::piped());
        let output = command.execute_output()?;
        let mut all_available_formats = fetch_formats(String::from_utf8(output.stdout).expect("Fixme"))?;
        sp.stop();

        // Every set is the ids available for a single video
        let mut all_sets: Vec<&Set<u32>> = vec![];

        for mut video in all_available_formats.iter_mut() {
            let current_ids = video.refresh_and_sort_ids();
            all_sets.push(Set::new(&current_ids[..]).expect("Add error handling to format fetching"));
        }

        let op = OpBuilder::from_vec(all_sets).intersection();
        // A list of ids which every video can be downloaded in
        let common_ids: SetBuf<u32> = op.into_set_buf();

        // todo produce a readable thing to ask for format info instead of a pile of ids
        let mut format_options = vec!["Best available quality for each video".to_string(), "Worst available quality for each video".to_string()];
        for id in common_ids {
            // Find which format corresponds to each id
            // common_formats is a Vec of all the formats for the first video.
            // Since we are looking for ids common to all videos just checking the first one is fine
            if let Some(first_video_formats) = all_available_formats.first() {
                for format in first_video_formats.available_formats() {
                    if format.code == id {
                        // Add to the list of available formats the current one formatted in a nice way
                        format_options.push(format.to_frontend());
                    }
                }
            }
        }
        let user_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which quality do you want to apply to all videos?")
            .default(0)
            .items(&format_options)
            .interact_on(&term)?;
        Ok(vec![])
    }

    /// Returns a Vec with every video's format information
    pub(super) fn fetch_formats(output: String) -> Result<Vec<VideoSpecs>, std::io::Error> {
        // A lost of every video in the playlist's available formats
        let mut all_videos: Vec<VideoSpecs> = Vec::new();
        /* A list of all the download formats available for a video, if its format information is unavailable
         * (maybe because it is age-restricted) and thus youtube-dl cannot fetch any information about it,
         * the Option is None. In every other case there is Some
         */
        let mut video = VideoSpecs::new();

        for paragraph in output
            .split("[download] Downloading video") {
            // Create a new video on every iteration because pushing on a Vec requires moving
            let mut video = VideoSpecs::new();
            // The first line is discarded, it tells information about the index of the current video in the playlist
            for line in paragraph.lines().skip(1) {
                // Ignore all irrelevant lines (they violate VideoFormat::from_command()'s contract
                // Each line which doesn't start with a code has to be ignored
                if !line.chars().next().unwrap().is_numeric() ||
                    line.contains("video only") {
                    continue;
                };
                // The line is about a video or audio-only format or is a youtube-dl error
                video.add_format(VideoFormat::from_command(line));
            }
            // Ignore some quirks of string splitting
            if video.is_empty() {
                continue;
            }
            // Add the current video to the "playlist"
            all_videos.push(video);
        };
        Ok(all_videos)
    }
}

/// Whether the downloaded files should include their index in the playlist as a part of their name
fn get_index_preference(term: &Term) -> Result<bool, std::io::Error> {
    let download_formats = &[
        "Yes",
        "No",
    ];
    // Ask the user which format they want the downloaded files to be in
    let index_preference = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you a video's index in the playlist to be in its name?")
        .default(0)
        .items(download_formats)
        .interact_on(&term)?;

    match index_preference {
        0 => Ok(true),
        1 => Ok(false),
        _ => panic!("The only options are 0 and 1")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_command() -> Result<(), std::io::Error> {
        let test_str = "139          m4a        audio only DASH audio   50k , m4a_dash container, mp4a.40.5 (22050Hz), 2.45MiB";
        let f = VideoFormat::from_command(test_str);
        let expected_format = VideoFormat {
            code: 139,
            file_extension: String::from("m4a"),
            resolution: String::from("audio"),
            size: String::from("50k"),
        };
        assert_eq!(f, expected_format);

        let test_str = "22           mp4        1280x720   720p  468k , avc1.64001F, 30fps, mp4a.40.2 (44100Hz) (best)";
        let f = VideoFormat::from_command(test_str);
        let expected_format = VideoFormat {
            code: 22,
            file_extension: String::from("mp4"),
            resolution: String::from("1280x720"),
            size: String::from("468k"),
        };
        assert_eq!(f, expected_format);
        Ok(())
    }
}