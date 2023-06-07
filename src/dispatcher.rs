use std::collections::HashMap;

use crate::analyzer;
use crate::parser;
use crate::assembling;
use crate::error::BlobResult;

/// Calls the right wizard according to what the url refers to, then it runs the ytdl-command and handles errors
pub fn dispatch(config: &parser::CliConfig) -> BlobResult<()> {
    // todo test analyzer::analyze_url()
    let download_option = analyzer::analyze_url(config.url());

    // Generate a command according to the user's wishes
    let mut command = assembling::generate_command(config.url(), &download_option?)? ;

    #[cfg(debug_assertions)]
    println!("[DEBUG ytdl command : {:?}]", command);

    // Run the command
    run_and_observe(&mut command.0, &command.1);

    Ok(())
}

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use dialoguer::console::Term;
use crate::assembling::youtube::config;

/// Executes the yt-dlp command and analyzes its output.
///
/// It filters what to show to the user according to verbosity options
///
/// It records which links fail download and their reason: if trying again can fix the issue it tells the user
fn run_and_observe(command: &mut Command, config: &config::DownloadConfig) {
    // Run the command and record any errors
    if let Some(errors) = run_command(command) {
        // Some videos could not be downloaded

        // Ask the user which videos they want to try to re-download
        let user_selection = ask_for_redownload(&errors);

        // The list of commands that have to be re-run in case of errors
        let mut to_be_downloaded = Vec::new();

        if !user_selection.is_empty() {
            if user_selection[0] == 0 {
                // The user wants to re-download all the videos
                for video_to_re_download in &errors {
                    // Re-download every video while keeping the current command configuration (quality, naming preference, ...)
                    to_be_downloaded.push(config.build_command_for_video(video_to_re_download.video_id()));
                }
            } else if user_selection[0] == 1 {
                    // The user doesn't want to re-download anything
            } else {
                // Only re-download the selected videos
                for i in user_selection {
                    // Skip 0 and 1 because they are hard-coded options (select all or nothing)
                    if i == 0 || i == 1 {
                        continue;
                    }

                    // There is a 1:1 correspondence between the number in user_selection and
                    // the index of the video it refers to in errors todo test this extensively
                    to_be_downloaded.push(config.build_command_for_video(errors[i - 2].video_id.as_str()));
                }
            }
        }
        for mut com in to_be_downloaded {
            run_command(&mut com);
        }
    } else {
        // The command ran without any errors!
        #[cfg(debug_assertions)]
        println!("The command ran without any errors!! :)");
    }
}

/// A list of all the documented youtube error messages and whether they are recoverable.
///
/// If an error is recoverable, the user is presented with the choice of downloading it again
fn init_error_msg_lut() -> HashMap<&'static str, bool> {
    HashMap::from([
        (crate::PRIVATE_VIDEO, false)
    ])
}

/// Runs the command and displays the output to the console.
///
/// If yt-dlp runs into any errors, they are returned in a vector of Ytdlp errors (parsed Strings)
fn run_command(command: &mut Command) -> Option<Vec<YtdlpError>> {
    // Run the command and capture its output
    let mut youtube_dl = command.stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start yt-dlp process");
    // fixme this expect ^

    let stdout = BufReader::new(youtube_dl.stdout.take().unwrap());
    let stderr = BufReader::new(youtube_dl.stderr.take().unwrap());

    // All the errors produced by yt-dlp
    let mut errors: Vec<YtdlpError> = vec![];

    // Print to the console what youtube-dl is doing and update merged
    for line in stdout.lines().chain(stderr.lines()) {
        // fixme handle this Result
        let line = line.unwrap();

        // todo when outputting things color error lines red
        // Currently verbosity options are ignored
        println!("{line}");

        if line.contains("ERROR:") {
            errors.push(extract_error_info(&line));
        }
    };

    if errors.is_empty() {
        None
    } else {
        Some(errors)
    }
}

/// Shows the user which videos could not be downloaded and returns which have to be re-downloaded based on what the user wants
fn ask_for_redownload(errors: &Vec<YtdlpError>) -> Vec<usize> {
    println!("The following videos could not be downloaded, you can select which to retry [spacebar for selection]");
    let term = Term::buffered_stderr();

    // Convert errors to string form, so they can be displayed on the terminal
    let mut multiselected = Vec::new();

    // todo these hard-coded strings
    multiselected.push(String::from("Select all\n"));
    multiselected.push(String::from("Don't re-download anything\n"));

    for error in errors {
        multiselected.push(error.to_string())
    }

    let user_selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which file do you want to re-download [spacebar to select]?")
        .items(&multiselected[..])
        .interact_on(&term).unwrap();
    user_selection
}


#[derive(Debug)]
struct YtdlpError {
    video_id: String,
    error_msg: String,
}

impl YtdlpError {
    // todo refactor
    // Construct a YtdlpError object from the string created by this stuct's Display implementation
    fn from_string(string: &str) -> YtdlpError {
        let mut it = string.split_whitespace();
        // skip "yt-video"
        it.next().unwrap();
        // skip "id:"
        it.next().unwrap();
        let video_id = it.next().unwrap();
        // skip "Reason:"
        it.next().unwrap();

        let mut error_msg = String::new();
        for word in it {
            error_msg += word;
            error_msg += " ";
        }

        YtdlpError {
            video_id: String::from(video_id),
            error_msg
        }
    }

    pub fn video_id(&self) -> &String {
        &self.video_id
    }
}

impl std::fmt::Display for YtdlpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result = format!("{} {}", "yt-video id:", self.video_id);
        result = format!("{}\n   {} {}\n", result, "Reason:", self.error_msg);

        write!(f, "{}", result)
    }
}

/// Expects a line which actually contains an error
fn extract_error_info(error_line: &str) -> YtdlpError {
    // yt-dlp error line format: ERROR: [...] video_id: reason
    let mut section = error_line.split_whitespace();

    // Skip ERROR:
    section.next().unwrap();
    // Skip [...]
    section.next().unwrap();

    let mut video_id = section.next().unwrap();
    // Delete the trailing ':'
    video_id = &video_id[..video_id.len() - 1];

    // Concatenate together the error message and restore whitespace
    let error_msg = {
        let mut tmp = String::new();
        for word in section {
            tmp = tmp + " " + word;
        }
        tmp
    };

    YtdlpError {video_id: video_id.to_string(), error_msg}
}
