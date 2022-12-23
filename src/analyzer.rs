/// All of the supported sources
pub enum DownloadOption {
    YtVideo,
    YtPlaylist,
    SpSong,
    SpPlaylist,
}

/// Analyzes the url provided by the user and deduces whether it
/// refers to a spotify song/url or a youtube video/url
pub fn analyze_url(url: String) -> Option<DownloadOption> {
    None
}
