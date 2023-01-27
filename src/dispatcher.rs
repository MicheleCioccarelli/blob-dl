use crate::analyzer;
use crate::parser;
use crate::assembling;
use execute::Execute;

/// Given a command-line configuration calls the right wizard
pub fn dispatch(config: &parser::CliConfig) -> Result<(), std::io::Error> {
    // todo test analyzer::analyze_url()
    let download_option = analyzer::analyze_url(config.url());

    // Generate a command according to the user's wishes
    let mut command = match download_option {
        Some(option) => assembling::generate_command(config.url(), &option)?,
        // todo exit gracefully
        None => panic!("Could not understand the url"), // :/
    };

    println!("[DEBUG ytdl command : {:?}]", command);

    // Run the command
    let output = command.execute_output().expect("Error executing the command");
    Ok(())
}
