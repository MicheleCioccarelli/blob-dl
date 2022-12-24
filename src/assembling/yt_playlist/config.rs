use crate::assembling;

/// Contains all the information needed to download a youtube playlist
#[derive(Debug)]
pub(crate) struct ConfigYtPlaylist {
    url: String,
    download_format: String,
    output_path: String,
    verbose: bool,
}

impl ConfigYtPlaylist {
    pub(crate) fn new(url: String, download_format: String, output_path: String, verbose: bool) -> ConfigYtPlaylist {
        ConfigYtPlaylist { url, download_format, output_path, verbose }
    }
    /// Builds a yt-dl command with the needed specifications
    pub(crate) fn build_command(&self) -> String {
        String::new()
    }
}
