use crate::assembling;

/// Contains all the information needed to download a spotify playlist [WIP]
#[derive(Debug)]
pub(crate) struct ConfigSpPlaylist {
    url: String,
    download_format: String,
    output_path: String,
}

impl ConfigSpPlaylist {
    pub(crate) fn new(url: String, download_format: String, output_path: String) -> ConfigSpPlaylist {
        ConfigSpPlaylist { url, download_format, output_path }
    }
    /// Builds a sp-dl command with the needed specifications
    pub(crate) fn build_command(&self) -> std::process::Command {

        todo!()
    }
}
