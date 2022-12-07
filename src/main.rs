use clap::{Arg, Command, ArgAction};
mod preferences;
mod tutorial;
mod parsing;

fn main() {
    let matches = Command::new("blob-dl")
        .version("0.1")
        .author("Michele Cioccarelli")
        .about("A veery convinient wrapper [hi to anyone reading old commits :)]")
        .arg(Arg::new("URL")
            //.next_line_help(true)
            .help("Link to the video/playlist you want to download")
            .required(true)
        )
        .arg(Arg::new("format")
            .short('f')
            .long("format")
            .next_line_help(true)
            .help("Specify the download format [.mp3 for audio, .mp4 for video, ...]")
        )
        .arg(Arg::new("output-path")
            .short('o')
            .long("output-path")
            .help("Where you want the downloaded files to go")
        )
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(ArgAction::SetTrue)
            .help("Display normal youtube-dl/spotify-dl output instead of a progress bar")
        )
        .get_matches();
}