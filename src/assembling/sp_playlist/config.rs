use crate::assembling;

/// Contains all the information needed to download a spotify playlist [WIP]
#[derive(Debug)]
pub(crate) struct ConfigSpPlaylist {
    url: String,
    download_format: String,
    output_path: String,
    verbose: bool,
}

impl ConfigSpPlaylist {
    pub(crate) fn new(url: String, download_format: String, output_path: String, verbose: bool) -> ConfigSpPlaylist {
        ConfigSpPlaylist { url, download_format, output_path, verbose }
    }
    /// Builds a sp-dl command with the needed specifications
    pub(crate) fn build_command(&self) -> String {

        todo!()
    }
}
