use blob_dl::parser;
use blob_dl::dispatcher::dispatch;
use which::which;

fn main() {
    // Processed command line arguments live here
    let config = parser::parse_config();

    // tested with yt-dlp 2023.07.06
    if which("yt-dlp").is_ok() {
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