<p align="center">
    <a>
    <img alt="`blob-dl` Logo" src="../assets/blob-dl-logo-v1.png" height="150">
  </a>
</p>

![Crates.io](https://img.shields.io/crates/d/blob-dl?color=%2325BE5D)
![GitHub](https://img.shields.io/badge/license-MIT-blue)
![GitHub issues](https://img.shields.io/github/issues/MicheleCioccarelli/blob-dl)
![Crates.io](https://img.shields.io/crates/v/blob-dl)


<h1 align="center">blob-dl</h1>


`blob-dl` is a command line tool used to download video and audio files from YouTube, it acts as an alternative frontend to [`yt-dlp`](https://github.com/yt-dlp/yt-dlp) and works by asking a series of questions that help it generate and then execute a command that fits your needs.

See the [Features](https://github.com/MicheleCioccarelli/blob-dl#features) section for more details on what `blob-dl` can do

[![asciicast](https://asciinema.org/a/jZUokSc5oDms6vICdNTic1vxh.svg)](https://asciinema.org/a/jZUokSc5oDms6vICdNTic1vxh)


# Installation
The most straightforward way to install `blob-dl` is to use [the binaries](https://github.com/MicheleCioccarelli/blob-dl/releases/tag/v1.0.0)

Alternatively, if you are a Rust programmer you can install `blob-dl` with `cargo`

```
$ cargo install blob-dl
```
## Dependencies
`blob-dl` calls `yt-dlp` directly, so it cannot function without it.

If you don't already have it, install `yt-dlp` following this guide [here](https://github.com/yt-dlp/yt-dlp#installation).

You also need to install `yt-dlp`'s [recommended dependencies](https://github.com/yt-dlp/yt-dlp#dependencies).

# Usage
When you fire up `blob-dl` it asks you `What kind of file(s) do you want to download?` 

The answer you choose determines which formats you will be able to pick later on: if you answer that you want to download audio-only files, then formats containing video will be hidden.

The second question `Which quality or format do you want to apply to the video?` allows you to choose a specific format, quality, filesize, ...  

The available answers mean these things:

- `Best possible quality` tells yt-dlp to automatically choose the `best` quality, for more information see `yt-dlp`'s [wiki](https://github.com/yt-dlp/yt-dlp#format-selection)

- `Smallest file size` uses the format which results in the smallest file size

- `Choose a format to recode the video to` requires ffmpeg: after the video is downloaded, it is converted to a file format of your choosing

- `Choose a format to download the video in` doesn't require ffmpeg: it shows a list of formats directly available for download from YouTube without needing to convert anything


`blob-dl` will ask further questions, but they are self-explanatory

# Features

### Format conversion
`blob-dl` was designed to download large song playlists directly as audio files. Choosing between downloading audio files, normal video files or video-only files is very easy

### Error tracking

While downloading, `blob-dl` keeps track of any errors thrown by yt-dlp and reports them at the end, the ones caused which can be resolved by re-trying the download can be easily re-downloaded


# Q&A
### Who is this for?
This program is intended for anyone who needs to download things from YouTube without having to remember yt-dlp's syntax. `blob-dl` can do everything an average user needs

`yt-dlp` power-users with very specific needs probably won't find this program useful

### Why did I make this?
Have you ever had to download videos from YouTube? 
The process can be quite a pain because you will have to either spend your time closing pop-ups from a sketchy website or browsing through [`yt-dlp`](https://github.com/yt-dlp/yt-dlp)'s documentation.

I was tired of spending hours downloading music videos and converting them to audio, so I wrote this program to make everything way easier



## Notes
This logo was inspired by [@Primer](https://www.youtube.com/c/PrimerLearning)'s [blob plushie](https://store.dftba.com/collections/primer/products/primer-blob-plushie)
