use clap::{Arg, Command, ArgAction};
use super::assembling;
use super::config;

/// Fetches command line arguments and assembles them in
pub fn parse_config() -> config::Config {
    let matches = Command::new("blob-dl")
        .version("0.1")
        .author("Michele Cioccarelli, cioccarellimi@gmail.com")
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

    assembling::assemble_data(matches)
}
