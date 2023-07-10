use clap::{Arg, Command, ArgMatches, ArgAction};

use crate::error::{BlobdlError, BlobResult};

pub fn parse_config() -> BlobResult<CliConfig> {
    let matches = Command::new("blob-dl")
        .version("1.0 tested with yt-dlp 2023.03.04")
        .author("cioccarellimi@gmail.com")
        .about("A very convenient wrapper")
        .long_about("Long about")
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

        Ok(CliConfig {
            url,
            verbosity,
        })
    }

    pub fn url(&self) -> &String {
        &self.url
    }
    pub fn verbosity(&self) -> &Verbosity {
        &self.verbosity
    }
}