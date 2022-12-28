use crate::assembling;

/// Contains all the information needed to download a spotify track [WIP]
#[derive(Debug)]
pub(crate) struct ConfigSpTrack {
    url: String,
    download_format: String,
    output_path: String,
    verbose: bool,
}

impl ConfigSpTrack {
    pub(crate) fn new(url: String, download_format: String, output_path: String, verbose: bool) -> ConfigSpTrack {
        ConfigSpTrack { url, download_format, output_path, verbose }
    }
    /// Builds a sp-dl command with the needed specifications
    pub(crate) fn build_command(&self) -> std::process::Command {

        todo!()
    }
}
