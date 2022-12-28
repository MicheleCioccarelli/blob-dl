use crate::assembling;

/// Contains all the information needed to download a youtube playlist [WIP]
#[derive(Debug)]
pub(crate) struct ConfigSpAlbum {
    url: String,
    download_format: String,
    output_path: String,
}

impl ConfigSpAlbum {
    pub(crate) fn new(url: String, download_format: String, output_path: String) -> ConfigSpAlbum {
        ConfigSpAlbum { url, download_format, output_path }
    }
    /// Builds a yt-dl command with the needed specifications
    pub(crate) fn build_command(&self) -> std::process::Command {
        todo!()
    }
}
