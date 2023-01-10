use crate::analyzer;
use crate::parser;
use crate::assembling;
use execute::Execute;

// Major refactoring incoming

/// Given a command-line configuration calls the right wizard
pub fn dispatch(config: &parser::CliConfig) {

    let download_option = analyzer::analyze_url(config.url());

    // Generate a command according to the user's configuration
    let mut command = match download_option {
        Some(option) => assembling::generate_command(&config.url(), &option),
        None => panic!("Could not understand the url"), // :/
    };

    // Run the command
    let output = command.execute_output().expect("Error executing the command");

    println!("======================================================\nCaptured output: \n{:?}", output);
    // Process -dl output based on CliConfig flags
}
