use clap::{Arg, Command, ArgMatches};

pub fn parse_config() -> CliConfig {
    let matches = Command::new("blob-dl")
        .version("0.1")
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

// Quite overkill implementation below

/// Holds all the information that can be fetched as a command line argument
pub struct CliConfig {
    url: String,
}

impl CliConfig {
    /// Constructs a CliConfig object based on Clap's output
    pub fn from(matches: ArgMatches) -> CliConfig {
        let url = match matches.get_one::<String>("SOURCE") {
            Some(url) => url.clone(),
            None => String::new(),
        };
        CliConfig {
            url,
        }
    }
    pub fn url(&self) -> &String {
        &self.url
    }
}