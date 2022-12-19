#[derive(Debug)]
pub struct Config {
    pub url: String,
    pub download_format: String,
    pub output_path: String,
    pub verbose: bool,
}

impl Config {
    pub fn new(url: String, download_format: String, output_path: String, verbose: bool) -> Config {
        Config { url: url, download_format: download_format, output_path: output_path, verbose: verbose}
    }
}
