use blob_dl::parser;
use std::error::Error;
use blob_dl::dispatcher::dispatch;

fn main() -> Result<(), Box<dyn Error>> {

    // Processed command line arguments (for now just the playlist url) live here
    let config = parser::parse_config();

    // Ask for more input, Generate a command, Execute ytdl
    if let Err(E) = dispatch(&config) {
        // If there is an error, handle it

    }

    Ok(())

    //todo!("See .error() in Clap for neat error messages!");
}
