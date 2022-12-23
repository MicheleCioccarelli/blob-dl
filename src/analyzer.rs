use url::{Url, Host, Position};

/// All of the supported sources
pub enum DownloadOption {
    YtVideo,
    YtPlaylist,
    SpTrack,
    SpPlaylist,
}

/// Analyzes the url provided by the user and deduces whether it
/// refers to a spotify song/url or a youtube video/url
pub fn analyze_url(command_line_url: String) -> Option<DownloadOption> {
    // .ok() converts from a Result to an Option
    let url = Url::parse(&command_line_url).ok();

    println!("Time to print debug stuff");
    println!("host_str = {:?}", &url?.host_str());
    if url?.host_str()?.contains("youtu") {

    }


    if command_line_url.contains("www.youtube.com") {
        if command_line_url.contains("playlist") {
            return Some(DownloadOption::YtPlaylist)
        }
        return Some(DownloadOption::YtVideo)
    }

    if command_line_url.contains("spotify.com") {
        if command_line_url.contains("track") {
            return Some(DownloadOption::SpTrack)
        }
        if command_line_url.contains("playlist") {
            return Some(DownloadOption::SpPlaylist)
        }
    }

    None
}
