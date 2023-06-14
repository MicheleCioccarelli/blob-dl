use url::Url;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::error::{BlobdlError, BlobResult};

/// All of the supported sources
#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum DownloadOption {
    /// If the url refers to a video in a playlist and the user only wants to download the video,
    ///
    /// YtVideo's value is the video's index in the playlist
    YtVideo(usize),
    YtPlaylist,
}

/// Analyzes the url provided by the user and deduces whether it
/// refers to a spotify song/url or a youtube video/url
///
/// Returns Some(DownloadOption) if it recognized the url
///
/// Returns None if the url isn't supported by blob-dl
///
pub fn analyze_url(command_line_url: &str) -> BlobResult<DownloadOption> {
    // .ok() converts from Result to Option
    return if let Ok(url) = Url::parse(command_line_url) {
        if let Some(domain_name) = url.domain() {
            // All youtube-related urls have "youtu" in them
            if domain_name.contains("youtu") {
                inspect_yt_url(url)
            } else {
                // The provided url wasn't for youtube
                Err(BlobdlError::UnsupportedWebsite)
            }
        } else {
            // Url domain could not be found
            Err(BlobdlError::DomainNotFound)
        }
    } else {
        Err(BlobdlError::UrlParsingError)
    }
}

/// Given a youtube url determines whether it refers to a video/playlist/something unsupported
fn inspect_yt_url(yt_url: Url) -> BlobResult<DownloadOption> {
    if let Some(query) = yt_url.query() {
        // If the url's query exists, continue
        if query.contains("&index=") {
            // This video is part of a youtube playlist
            let term = Term::buffered_stderr();

            // Ask the user whether they want to download the whole playlist or just the video
            let user_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("The url refers to a video in a playlist, which do you want to download?")
                .default(0)
                .items(&["Only the video", "The whole playlist"])
                .interact_on(&term)?;

            return match user_selection {
                0 => {
                    // "&index="'s existence was checked in the previous if statement
                    let index = &query[query.find("&index=").unwrap() + "&index=".len()..query.len()];
                    // todo take a look at this expect
                    Ok(DownloadOption::YtVideo(index.parse().expect("This link has an unknown issue, please report it")))
                }
                _ => Ok(DownloadOption::YtPlaylist),
            };
        }
    }
    // fixme sto bordello
    if yt_url.path().contains("playlist") {
        return Ok(DownloadOption::YtPlaylist);
    } else if let Some(res) = yt_url.query() {
        if res.contains("list") {
            return Ok(DownloadOption::YtPlaylist);
        }
    }
    return Ok(DownloadOption::YtVideo(0));

    // The url doesn't refer to a youtube video/playlist (maybe a user, etc)
    println!("Youtube url not recognized as a video/playlist");
    Err(BlobdlError::UnsupportedFeature)
}