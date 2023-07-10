use blob_dl::parser;
use blob_dl::dispatcher::dispatch;
use which::which;

fn main() {

    // tested with yt-dlp 2023.03.04
    if which("yt-dlp").is_ok() {
        // Processed command line arguments (for now just the playlist url) live here
        let config = parser::parse_config();

        // If there was an error parsing the command-line arguments, handle it
        if let Err(err) = config {
            err.report();
            return;
        }

        // Ask for more input, Generate a command, Execute ytdl todo make this prettier
        if let Err(err) = dispatch(&config.unwrap()) {
            // If there is an error, handle it
            err.report();
        }
    } else {
        // ytdlp is not installed!
        println!("{}", blob_dl::YTDLP_NOT_INSTALLED);
    }

    //todo!("See .error() in Clap for neat error messages!");
}
