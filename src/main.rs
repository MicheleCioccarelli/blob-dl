use blob_dl::parser;
use blob_dl::dispatcher::dispatch;

fn main() {
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

    //todo!("See .error() in Clap for neat error messages!");
}
