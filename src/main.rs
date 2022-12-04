use clap::Parser;
mod preferences;
mod tutorial;

#[derive(Parser)]
#[command(author, version, about, long_about = "Long about goes brr")]
// #[command(next_line_help = true)]
struct Cli {
    /// Link to the video/playlist you want to download
    url: String,
}

fn main() {
    let cli = Cli::parse();

    tutorial::go(true);

    println!("url: {}", cli.url);

    // Interactive tutorial
}