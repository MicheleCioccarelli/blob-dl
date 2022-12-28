use crate::analyzer;
use crate::parser;
use crate::assembling;

use std::process::Command;
use execute::Execute;

/// Given a command-line configuration calls the right wizard
pub fn dispatch(config: &parser::CliConfig) {

    let download_option = analyzer::analyze_url(config.url());

    let mut command = match download_option {
        Some(option) => assembling::generate_command(&config.url(), &option, config.verbose()),
        None => panic!("Could not understand the url"),
    };

    // Run the command
    command.execute_output().expect("Error executing the command");

    // Process -dl output based on CliConfig flags
}
