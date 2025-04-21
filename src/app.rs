use colored::Colorize;
use crate::{parser, ui_prompts};
use crate::dispatcher::dispatch;
use which::which;

/// Handles most of the running logic behind blob-dl
/// 
/// First it checks whether yt-dlp is installed
/// 
/// Then it launches functions to parse command-line arguments and passes them to dispatcher()
pub fn run() {
    // Processed command line arguments live here
    let config = parser::parse_config();
    #[cfg(debug_assertions)]
    println!("##DEBUG## {:?}", config);

    // tested with yt-dlp 2025.03.31
    if which("yt-dlp").is_ok() {
        // check whether yt-dlp's version is compatible with this version of blob-dl
        match parser::is_ytdlp_compatible() {
            Ok(false) => {
                // This was annoying, might add this back in a future update
                print!("{}", "WARNING: ".bold().yellow());
                println!("{}", ui_prompts::WRONG_YTDLP_VERSION);
            }
            Err(_) => {
                print!("{}", "WARNING: ".bold().yellow());
                println!("{}", ui_prompts::COMMAND_NOT_SPAWNED);
            },
            _ => {}
        }
        
        match config {
            Ok(config) => {
                // Ask for more input > Generate a command > Execute yt-dlp
                if let Err(err) = dispatch(&config) {
                    // Tell the user about the error
                    err.report();
                }
            }
            Err(err) => {
                err.report();
            }
        }
    } else {
        // ytdlp is not installed!
        eprintln!("{}", crate::ui_prompts::YTDLP_NOT_INSTALLED);
    }
}