use blob_dl::parser;
use std::error::Error;
use blob_dl::dispatcher::dispatch;

fn main() -> Result<(), Box<dyn Error>> {
    // yt_playlist =  https://www.youtube.com/playlist?list=PLxKHVMqMZqUTIQiG1xfD3yc6PJiUmdAqX
    // yt_video    =  https://www.youtube.com/watch?v=ishbTyLs6ps&list=PLGup6kBfcU7Le5laEaCLgTKtlDcxMqGxZ&index=106&shuffle=2655
    // sp_track    =  https://open.spotify.com/track/7sgNkxl87zjuZlDgSFu6ax?si=pdQk2GT2Ra-KV0A2d3615w&utm_source=copy-link
    // sp_playlist =  https://open.spotify.com/playlist/4lHVCvT3f56UIgkxgRrPvA?si=pXQYwEHASpCx0y7WSTp60w&utm_source=copy-link
    // sp_album    =  https://open.spotify.com/album/7rbdgYKz1DI4gXMWveqS5T?si=NRVoWbrbTXaOmVC4YpjE7A&utm_source=copy-link
    //
    // let testing_url = String::from("https://open.spotify.com/album/7rbdgYKz1DI4gXMWveqS5T?si=NRVoWbrbTXaOmVC4YpjE7A&utm_source=copy-link");
    //
    // println!("Tested url: {}", testing_url);
    // analyzer::analyze_url(testing_url);

    // Only run this function after errors are handled
    //blob_dl::run(config);

    //let url = Url::parse("https://www.youtube.com/watch?v=u3O9tzqh8Fg&list=PLdDckWvN8lvDYx5kN90JXluqyuoWAApsS&index=3").ok();

    // Processed command line arguments (for now just the playlist url) live here
    let config = parser::parse_config();

    // Ask for more input, Generate a command, Execute ytdl
    dispatch(&config);

    Ok(())

    //todo!("See .error() in Clap for neat error messages!");
}
