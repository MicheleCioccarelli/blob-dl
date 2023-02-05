
// todo make this private
pub mod youtube;

use crate::analyzer;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use dialoguer::console::Term;
use std::env;

// TODO: Re-read how to make children modules for better privacy management and make youtube mods derive from a mod with all of their common utils

/// [Rewrite this in the future] Calls the right wizard to generate the required command
pub(crate) fn generate_command(url: &String, download_option: &analyzer::DownloadOption) -> Result<std::process::Command, std::io::Error> {
    match download_option {
        analyzer::DownloadOption::YtPlaylist => Ok(youtube::yt_playlist::wizard::assemble_data(url)?.build_command()),
        analyzer::DownloadOption::YtVideo =>    Ok(youtube::yt_video::wizard::assemble_data(url)?.build_command()),
        //analyzer::DownloadOption::SpTrack =>    Ok(sp_track::wizard::assemble_data(url).build_command()),
        //analyzer::DownloadOption::SpPlaylist => Ok(sp_playlist::wizard::assemble_data(url).build_command()),
        //analyzer::DownloadOption::SpAlbum =>    Ok(sp_album::wizard::assemble_data(url).build_command()),
        _ => panic!()
    }
}

/// Asks for an directory to store downloaded file(s) in
///
/// The current directory can be selected or one can be typed in
fn get_output_path(term: &Term) -> Result<String, std::io::Error> {
    let output_path_options = &[
        "Current directory",
        "Other [specify]",
    ];

    let output_path = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Where do you want the downloaded file(s) to go?")
        .default(0)
        .items(output_path_options)
        .interact_on(&term)?;

    match output_path {
        // Return the current directory
        0 => Ok(env::current_dir()?
            .as_path()
            .display()
            .to_string()),
        // Return a directory typed in by the user
        _ => Ok(Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Output path:")
            .interact_text()?),
    }
}
