use crate::analyzer;
use crate::parser;
use crate::assembling;
use crate::error::BlobResult;
use crate::run;

/// Calls the right wizard according to what the url refers to, then it runs the ytdl-command and handles errors
pub fn dispatch(config: &parser::CliConfig) -> BlobResult<()> {
    // todo test analyzer::analyze_url()
    let download_option = analyzer::analyze_url(config.url());

    // Generate a command according to the user's wishes
    let mut command_and_config = assembling::generate_command(config.url(), &download_option?)? ;

    if config.show_command() {
        println!("Command generated by blob-dl: {:?}", command_and_config.0);
    }

    // Run the command
    run::run_and_observe(&mut command_and_config.0, &command_and_config.1, config.verbosity());

    Ok(())
}

