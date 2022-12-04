pub struct Preferences {
    url: String,
    output_path: String,
    download_format: String,
}

impl Preferences {
    pub fn new() -> Preferences {
        Preferences { url: String::new(), output_path: String::new(), download_format: String::new() }
    }
    pub fn url(&self) -> String {
        self.url.clone()
    }
    pub fn output_path(&self) -> String {
        self.output_path.clone()
    }
    pub fn download_format(&self) -> String {
        self.download_format.clone()
    }
}