//use clap::{arg, Command, Parser};
//
/*
//pub fn go() {
//    let matches = Command::new("blob-dl REBORN")
//                    .version("0.1")
//                    .author("Michele Cioccarelli")
//                    .about("Will do")
//                    .arg(arg!(--one <VALUE>).required(true))
//                    .get_matches();
//    println!(
//            "one: {:?}",
//    matches.get_one::<String>("first").expect("required")
//    );
//}*/
//
////
//pub fn derive() -> Parser {#[derive(Parser)]
//#[command(author, version, about, long_about = "Long about goes brr")]
//struct Cli {
//        /// Link to the video/playlist you want to download
//        url: String,
//    /// Show the normal output of youtube-dl/spotify-dl instead of a progress bar
//    #[arg(short, long)]
//    verbose: bool,
//
//    #[arg(short, long, required(false))]
//    output_path: String,
//
//    #[arg(short, long, required(false))]
//    format: String,
//}
//    Cli::parse()
//}