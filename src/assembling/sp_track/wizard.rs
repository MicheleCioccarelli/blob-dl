use clap::ArgMatches;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use super::config;
use crate::assembling;
use url::Url;

/// Returns a ConfigSpTrack object with all the necessary data
/// to start downloading a spotify track
///
/// Takes in the command line arguments list
pub(crate) fn assemble_data(url: &String) -> config::ConfigSpTrack {
    let term = Term::buffered_stderr();

    // Handle errors?

    config::ConfigSpTrack::new(url.clone(),
                                  get_format(&term),
                                  assembling::get_output_path(&term),
    )
}

/// Aks for a download format in a user-friendly way.
///
/// This interface needs to be remade
fn get_format(term: &Term) -> String {
    todo!()
}