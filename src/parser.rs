use clap::{Arg, Command, ArgAction, ArgMatches};
use super::assembling;

pub fn parse_config() -> CliConfig {
    let matches = Command::new("blob-dl")
        .version("0.1")
        .author("cioccarellimi@gmail.com")
        .about("A very convenient wrapper")
        .long_about("Long about")
        .arg(Arg::new("URL")
            .next_line_help(true)
            .help("Link to the video/song/playlist you want to download")
            .required(true)
        )
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(ArgAction::SetTrue)
            .help("Display normal youtube-dl/spotify-dl output instead of a progress bar")
        )
        .get_matches();

    CliConfig::from(matches)
}

/// Holds all the information that can be fetched as a command line argument
pub struct CliConfig {
    url: String,
    verbose: bool,
}

impl CliConfig {
    /// Constructs a CliConfig object based on Clap's output
    pub fn from(matches: ArgMatches) -> CliConfig{
        let url = match matches.get_one::<String>("name") {
            Some(url) => url.clone(),
            None => String::new(),
        };
        let verbose = matches.get_flag("verbose");

        CliConfig {
            url,
            verbose,
        }
    }
}