pub mod youtube;

use crate::analyzer;
use crate::error::BlobResult;

/// Asks the user for specific download preferences (output path, download format, ...) and builds
/// a yt-dlp command according to them
///
/// User config is the information present in a config file. It has user preferences on things like which file format they prefer
/// knowing this blob-dl can avoid asking redundant questions
/// 
/// Returns the command along with a DownloadConfig object, which contains all the user-specified preferences
pub(crate) fn generate_command(url: &str, download_option: &analyzer::DownloadOption, user_config: youtube::config::DownloadConfig) -> BlobResult<(std::process::Command, youtube::config::DownloadConfig)> {
    // Get preferences from the user, various errors may occur
    let unchecked_config = match download_option {
        analyzer::DownloadOption::YtPlaylist => youtube::yt_playlist::assemble_data(url, user_config),

        analyzer::DownloadOption::YtVideo(id) => youtube::yt_video::assemble_data(url, *id, user_config)
    };

    match unchecked_config {
        Ok(safe) => {
            // Everything went smoothly, now generate a yt-dlp command
            let (command, local_config) = safe.build_command()?;
            Ok((command, local_config))
        }
        // Propagate the errors
        Err(err) => Err(err)
    }
}