use crate::assembling;

/// Contains all the information needed to download a youtube playlist
#[derive(Debug)]
pub struct ConfigYtPlaylist {
    url: String,
    download_format: String,
    output_path: String,
    verbose: bool,
}

impl ConfigYtPlaylist {
    pub fn new(url: String, download_format: String, output_path: String, verbose: bool) -> ConfigYtPlaylist {
        ConfigYtPlaylist { url, download_format, output_path, verbose }
    }
}

impl assembling::CommandBuilder for ConfigYtPlaylist {
    /// Builds a yt-dl command with the needed specifications
    fn build_command(&self) -> String {
        String::new();
    }
}
