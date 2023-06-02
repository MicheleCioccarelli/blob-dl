use crate::analyzer;
use crate::parser;
use crate::assembling;

use crate::error::BlobResult;

/// Calls the right wizard according to what the url refers to, then it runs the ytdl-command and handles errors
pub fn dispatch(config: &parser::CliConfig) -> BlobResult<()> {
    // todo test analyzer::analyze_url()
    let download_option = analyzer::analyze_url(config.url());

    // Generate a command according to the user's wishes
    let mut command =assembling::generate_command(config.url(), &download_option?)? ;

    #[cfg(debug_assertions)]
    println!("[DEBUG ytdl command : {:?}]", command);

    // Run the command
    // let output = command.execute_output().expect("Error executing the command");
    run_and_observe(&mut command);
    Ok(())
}

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use dialoguer::console::Term;

/// Executes the yt-dlp command and analyzes its output.
///
/// It filters what to show to the user according to verbosity options
///
/// It records which links fail download and their reason: if trying again can fix the issue it tells the user
fn run_and_observe(command: &mut Command) {
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
    }

    // Present the errors to the user
    if !errors.is_empty() {
        println!("The following videos could not be downloaded, you can select which to retry [spacebar for selection]");
        let term = Term::buffered_stderr();

        // Copy error message strings
        let mut multiselected = Vec::new();

        multiselected.push(String::from("Select all"));

        for error in errors {
            multiselected.push(error.to_string())
        }

        let user_selection = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Which file do you want to re-download [spacebar to select]?")
            .items(&multiselected[..])
            .interact_on(&term).unwrap();

        let mut to_be_redownloaded = Vec::new();

        if !multiselected.is_empty() {
            //if multiselected[0] != "Select all" {
                // Now the strings need to be turned back into YtdlpError because multiselected only contains strings
                // roadmap: print out if reverse parsing works!!!!
                for video in multiselected {
                    let mut it = video.split_whitespace();
                    println!("Should be \"yt-video\": {:?}", it);
                    // Skip "yt-video"
                    it.next().unwrap();
                    println!("Should be \"id:\": {:?}", it);
                    // skip "id:"
                    it.next().unwrap();
                    println!("Should be the actual id: {:?}", it);
                    let video_id = it.next().unwrap();
                    // skip "Reason:"
                    it.next();
                    let reason =
                        {
                            let mut error_msg = String::new();
                            for word in it {
                                error_msg += word;
                                error_msg += " ";
                            }
                            error_msg
                        };

                    println!("Should be the reason: {}", reason);

                    to_be_redownloaded.push(YtdlpError{video_id: video_id.to_string(), error_msg: reason })
                }
            //} else {
                // The user wants to re-download every video
                //todo!()
            //}
        }
    }
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
}

impl std::fmt::Display for YtdlpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result = format!("yt-video id: {}", self.video_id);
        result = format!("{}\n   Reason: {}", result, self.error_msg);

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
