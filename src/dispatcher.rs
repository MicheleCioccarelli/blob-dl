use crate::analyzer;
use crate::parser;
use crate::assembling;

/// Given a DownloadOption calls the right wizard and output processing method
pub fn dispatch(config: parser::CliConfig, download_option: analyzer::DownloadOption) {

    // Get a command from the assembler
    let command = assembling::generate_command(config.url(), download_option, config.verbose());

    // Process -dl output based on CliConfig flags
}
