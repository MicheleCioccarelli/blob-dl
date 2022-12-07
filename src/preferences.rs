pub struct Preferences {
    url: String,
    download_format: String,
    output_path: String,
}

// Constructors
impl Preferences {
    pub fn new() -> Preferences {
        Preferences { url: String::new(), download_format: String::new(), output_path: String::new()}
    }
    pub fn build(url: String, download_format: String, output_path: String) -> Preferences {
        Preferences { url: url, download_format: download_format, output_path: output_path }
    }
}
/*
// Getters and setters
impl Preferences {
    pub fn url(&self) -> String {
        self.url.clone()
    }
    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }
    pub fn output_path(&self) -> String {
        self.output_path.clone()
    }
    pub fn set_output_path(&mut self, output_path: String) {
        self.output_path = output_path;
    }
    pub fn download_format(&self) -> String {
        self.download_format.clone()
    }
    pub fn set_download_format(&mut self, download_format: String) {
        self.download_format = download_format;
    }
}*/