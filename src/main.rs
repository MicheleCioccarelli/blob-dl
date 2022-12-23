use blob_dl::parser;
use blob_dl::analyzer;

/*
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
*/

fn main() {
    // Add error handling

    // Processed command line arguments live here
    let config = parser::parse_config();

    analyzer::analyze_url(String::from("https://www.youtube.com/watch?v=rcWx3uamWAI&t=1386s"));

    // Only run this function after errors are handled
    //blob_dl::run(config);

    //println!("{:?}", config);

    //todo!("See .error() in Clap for neat error messages!");
}

