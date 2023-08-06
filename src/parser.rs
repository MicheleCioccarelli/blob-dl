use clap::{Arg, Command, ArgMatches, ArgAction};

use crate::ui_prompts::*;
use crate::error::{BlobdlError, BlobResult};

pub fn parse_config() -> BlobResult<CliConfig> {
    let matches = Command::new("blob-dl")
        .version("0.1.0")
        .author("cioccarellimi@gmail.com")
        .about(SHORT_ABOUT)
        .long_about(LONG_ABOUT)
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Show all the output produced by yt-dlp")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Silence all output except for the final error summary")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show-command")
                .help("Print to the console the command generated by blob-dl")
                .long("show-command")
                .short('s')
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("URL")
            .help("Link to the youtube video/playlist that you want to download")
        )
        .get_matches();

    CliConfig::from(matches)
}

/// The 3 possible verbosity options for this program
#[derive(Debug)]
pub enum Verbosity {
    Verbose,
    Default,
    Quiet,
}

/// Holds all the information that can be fetched as a command line argument
#[derive(Debug)]
pub struct CliConfig {
    // Refs to this String are stored in other Config objects
    url: String,
    verbosity: Verbosity,
    // Whether to print to the console the final command which is the run by yt-dlp
    show_command: bool,
}

impl CliConfig {
    /// Constructs a CliConfig object based on Clap's output
    pub fn from(matches: ArgMatches) -> BlobResult<CliConfig> {

        let url = match matches.get_one::<String>("URL") {
            Some(url) => url.clone(),
            None => return Err(BlobdlError::MissingArgument),
        };

        let verbosity = {
            if matches.get_flag("quiet") {
                Verbosity::Quiet
            }
            else if matches.get_flag("verbose") {
                Verbosity::Verbose
            }
            else {
                Verbosity::Default
            }
        };
        let show_command = matches.get_flag("show-command");

        Ok(CliConfig {
            url,
            verbosity,
            show_command,
        })
    }

    pub fn url(&self) -> &String {
        &self.url
    }
    pub fn verbosity(&self) -> &Verbosity {
        &self.verbosity
    }
    pub fn show_command(&self) -> bool {
        self.show_command
    }
}