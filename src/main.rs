use blob_dl::parser;
/*
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
*/

fn main() {
    // Add error handling

    // Processed command line arguments live here
    let config = parser::parse_config();

    // Only run this function after errors are handled
    //blob_dl::run(config);

    //println!("{:?}", config);

    //todo!("See .error() in Clap for neat error messages!");
}

