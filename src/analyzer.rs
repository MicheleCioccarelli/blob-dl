use url::Url;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::error::{BlobdlError, BlobResult};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum DownloadOption {
    /// If the url refers to a video in a playlist and the user only wants to download the single video, YtVideo's value is the video's index in the playlist
    YtVideo(usize),
    YtPlaylist,
}

/// Analyzes the url provided by the user and deduces whether it
/// refers to a youtube video or playlist
pub fn analyze_url(command_line_url: &str) -> BlobResult<DownloadOption> {
    return if let Ok(url) = Url::parse(command_line_url) {
        if let Some(domain_name) = url.domain() {
            // All youtube-related urls have "youtu" in them
            if domain_name.contains("youtu") {
                inspect_yt_url(url)
            } else {
                // The url isn't from youtube
                Err(BlobdlError::UnsupportedWebsite)
            }
        } else {
            Err(BlobdlError::DomainNotFound)
        }
    } else {
        Err(BlobdlError::UrlParsingError)
    }
}

/// Given a youtube url determines whether it refers to a video/playlist
fn inspect_yt_url(yt_url: Url) -> BlobResult<DownloadOption> {
    if let Some(query) = yt_url.query() {
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
                    let index = if let Some(index_location) = query.find("&index=") {
                        let slice = &query[index_location + "&index=".len() ..];

                        if let Some(second_ampersand_location) = slice.find('&') {
                            // There are url parameters after &index=..
                             &slice[..second_ampersand_location]
                        } else {
                            slice
                        }
                    } else {
                        panic!("url has &index= but doesn't provide a numerical index")
                    };

                    if let Ok(parsed) = index.parse() {
                        Ok(DownloadOption::YtVideo(parsed))
                    } else {
                        Err(BlobdlError::UrlIndexParsingError)
                    }
                }

                _ => Ok(DownloadOption::YtPlaylist),
            };
        }
        if yt_url.path().contains("playlist") || query.contains("list"){
            return Ok(DownloadOption::YtPlaylist);
        }

        // This url is probably referring to a video or a short
        return Ok(DownloadOption::YtVideo(0));
    }

    Err(BlobdlError::QueryCouldNotBeParsed)
}