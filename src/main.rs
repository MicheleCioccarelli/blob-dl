use blob_dl::parser;
use blob_dl::dispatcher::dispatch;
use which::which;
use blob_dl::parser::is_ytdlp_compatible;
use colored::Colorize;

fn main() {
    // Processed command line arguments live here
    let config = parser::parse_config();
    println!("##DEBUG## {:?}", config);

    // tested with yt-dlp 2024.11.18
    if which("yt-dlp").is_ok() {

        // check whether yt-dlp's version is compatible with this version of blob-dl
        match is_ytdlp_compatible() {
            Ok(false) => {
                print!("{}", "WARNING: ".bold().yellow());
                println!("{}", blob_dl::ui_prompts::WRONG_YTDLP_VERSION);
            }
            Err(_) => {
                print!("{}", "WARNING: ".bold().yellow());
                println!("{}", blob_dl::ui_prompts::COMMAND_NOT_SPAWNED);
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
        eprintln!("{}", blob_dl::ui_prompts::YTDLP_NOT_INSTALLED);
    }
}