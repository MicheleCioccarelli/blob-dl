use crate::assembling;

/// Contains all the information needed to download a youtube playlist [WIP]
#[derive(Debug)]
pub(crate) struct ConfigSpAlbum {
    url: String,
    download_format: String,
    output_path: String,
    verbose: bool,
}

impl ConfigSpAlbum {
    pub(crate) fn new(url: String, download_format: String, output_path: String, verbose: bool) -> ConfigSpAlbum {
        ConfigSpAlbum { url, download_format, output_path, verbose }
    }
    /// Builds a yt-dl command with the needed specifications
    pub(crate) fn build_command(&self) -> std::process::Command {
        todo!()
    }
}
