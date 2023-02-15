use clap::{Arg, Command, ArgMatches};

pub fn parse_config() -> CliConfig {
    let matches = Command::new("blob-dl")
        .version("0.2")
        .author("cioccarellimi@gmail.com")
        .about("A very convenient wrapper")
        .long_about("Long about")
        .arg(Arg::new("SOURCE")
            .next_line_help(true)
            .help("Url to the binary large object you want to download or the path to a url-list file")
            .required(true)
        )
        .get_matches();

    CliConfig::from(matches)
}

/// Holds all the information that can be fetched as a command line argument
#[derive(Debug)]
pub struct CliConfig {
    // Refs to this String are stored in other Config objects
    url: String,
}

impl CliConfig {
    /// Constructs a CliConfig object based on Clap's output
    pub fn from(matches: ArgMatches) -> CliConfig {
        let url = match matches.get_one::<String>("SOURCE") {
            Some(url) => url.clone(),
            None => panic!("SOURCE is a required argument"),
        };
        CliConfig {
            url,
        }
    }
    pub fn url(&self) -> &String {
        &self.url
    }
}