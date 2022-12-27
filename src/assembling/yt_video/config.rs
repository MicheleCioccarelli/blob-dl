use crate::assembling;

/// Contains all the information needed to download a youtube video [WIP]
#[derive(Debug)]
pub(crate) struct ConfigYtVideo {
    url: String,
    download_format: String,
    output_path: String,
    verbose: bool,
}

impl ConfigYtVideo {
    pub(crate) fn new(url: String, download_format: String, output_path: String, verbose: bool) -> ConfigYtVideo {
        ConfigYtVideo { url, download_format, output_path, verbose }
    }
    /// Builds a yt-dl command with the needed specifications
    pub(crate) fn build_command(&self) -> String {
        todo!()
    }
}
