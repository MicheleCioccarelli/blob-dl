use blob_dl::parser;
use blob_dl::analyzer;

fn main() {
    // yt_playlist =  https://www.youtube.com/playlist?list=PLxKHVMqMZqUTIQiG1xfD3yc6PJiUmdAqX
    // yt_video    =  https://www.youtube.com/watch?v=ishbTyLs6ps&list=PLGup6kBfcU7Le5laEaCLgTKtlDcxMqGxZ&index=106&shuffle=2655
    // sp_track    =  https://open.spotify.com/track/7sgNkxl87zjuZlDgSFu6ax?si=pdQk2GT2Ra-KV0A2d3615w&utm_source=copy-link
    // sp_playlist =  https://open.spotify.com/playlist/4lHVCvT3f56UIgkxgRrPvA?si=pXQYwEHASpCx0y7WSTp60w&utm_source=copy-link
    // sp_album    =  https://open.spotify.com/album/7rbdgYKz1DI4gXMWveqS5T?si=NRVoWbrbTXaOmVC4YpjE7A&utm_source=copy-link

    let testing_url = String::from("https://open.spotify.com/album/7rbdgYKz1DI4gXMWveqS5T?si=NRVoWbrbTXaOmVC4YpjE7A&utm_source=copy-link");
    // Processed command line arguments live here
    //let config = parser::parse_config();

    println!("Tested url: {}", testing_url);
    analyzer::analyze_url(testing_url);

    // Only run this function after errors are handled
    //blob_dl::run(config);

    //println!("{:?}", config);

    //todo!("See .error() in Clap for neat error messages!");
}
