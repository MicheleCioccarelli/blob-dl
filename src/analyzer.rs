use url::{Url, Host, Position};

/// All of the supported sources
pub enum DownloadOption {
    YtVideo,
    YtPlaylist,
    SpTrack,
    SpPlaylist,
    SpAlbum,
}

/// Analyzes the url provided by the user and deduces whether it
/// refers to a spotify song/url or a youtube video/url
///
/// Returns Some(DownloadOption) if it recognized the url
///
/// Returns None if the url isn't supported by blob-dl
///
pub fn analyze_url(command_line_url: &String) -> Option<DownloadOption> {
    // .ok() converts from a Result to an Option
    let url = Url::parse(&command_line_url).ok();

    if let Some(matched_url) = url {
        if let Some(domain_name) = matched_url.domain() {
            // All youtube-related urls have "youtu" in them
            if domain_name.contains("youtu") {
                return inspect_yt_url(matched_url);
            } else if domain_name.contains("spotify") {
                return inspect_sp_url(matched_url);
            } else {
                // The provided url wasn't for spotify or youtube
                println!("Not a supported website");
            }
        } else {
            // Url domain could not be found
            println!("No domain");
        }
    } else {
        // Url was not properly parsed
        println!("No url");
    }

    None
}

/// Given a youtube url determines whether it refers to a video/playlist/something unsupported
fn inspect_yt_url(yt_url: Url) -> Option<DownloadOption> {
    if yt_url.path().contains("playlist") {
        return Some(DownloadOption::YtPlaylist);
    }
    else if yt_url.path().contains("watch") ||
            yt_url.path().contains("/v/")   ||
            yt_url.path() == ""
    {
        return Some(DownloadOption::YtVideo);
    }
    // The url doesn't refer to a youtube video/playlist (maybe a user, etc)
    println!("Youtube url not recognized as a video/playlist");
    None
}

/// Given a spotify url determines whether it refers to a track/playlist/album/something unsupported
fn inspect_sp_url(sp_url: Url) -> Option<DownloadOption> {
    if sp_url.path().contains("track") {
        return Some(DownloadOption::SpTrack);
    } else if sp_url.path().contains("playlist") {
        return Some(DownloadOption::SpPlaylist);
    } else if sp_url.path().contains("album") {
        return Some(DownloadOption::SpAlbum);
    }

    None
}
