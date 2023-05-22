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
    // todo this expect

    let stdout = BufReader::new(youtube_dl.stdout.take().unwrap());
    let stderr = BufReader::new(youtube_dl.stderr.take().unwrap());

    // Full output of youtube-dl (stdout + stderr)
    let mut merged = String::new();
    // All the line numbers where youtube-dl threw an error
    let mut errors = vec![];

    // Print to the console what youtube-dl is doing and update merged
    for (err_line_number, line) in stdout.lines().chain(stderr.lines()).enumerate() {
        // todo handle this Result
        let line = line.unwrap();

        merged.push_str(line.as_str());
        merged.push('\n');

        println!("{}", line);

        if line.contains("ERROR") {
            errors.push(err_line_number);
        }
    }

    #[cfg(debug_assertions)]
    {
        println!("Errors captured: ");
        println!("{:?}", errors);
    }
}