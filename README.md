<p align="center">
    <img alt="Blob-dl Logo" src="../assets/blob-dl-logo-v1.png" height="150">
    <h3 align="center">blob-dl</h3>
  </a>
</p>

# blob-dl
Blob-dl is a command line tool used to download video and audio files from YouTube

Have you ever had to download videos from YouTube? 
The process can be quite a pain because you will have to either spend your time closing pop-ups from a sketchy website or browsing through [yt-dlp](https://github.com/yt-dlp/yt-dlp)'s documentation.


Blob-dl at its core is a very noob-friendly way to use yt-dlp: it asks a series of questions and then generates and executes a command that fits your needs.

See the <a href="Features">Features</a> section for more details on what this program can do


[![asciicast](https://asciinema.org/a/jZUokSc5oDms6vICdNTic1vxh.svg)](https://asciinema.org/a/jZUokSc5oDms6vICdNTic1vxh)


# Installation
Currently, the only ways to install blob-dl are to use the command `cargo install` or to compile the source code yourself 
## Dependencies
blob-dl executes a yt-dlp command, so it cannot function without it.
If you don't already have it, install yt-dlp [here](https://github.com/yt-dlp/yt-dlp#installation).

You should also consider installing yt-dlp's [recommended dependencies](https://github.com/yt-dlp/yt-dlp#dependencies).

# Usage
When you fire up blob-dl it asks you `What kind of file(s) do you want to download?` 

The answer you choose determines which formats you will be able to pick later on: if you answer that you want to download audio-only files, then formats containing video will be hidden.

The second question `Which quality or format do you want to apply to the video?` allows you to choose a specific format, quality, filesize, ...  

The available answers mean these things:

- `Best possible quality` tells yt-dlp to automatically choose the "best" quality, for more information see yt-dlp's [wiki](https://github.com/yt-dlp/yt-dlp#format-selection)

- `Smallest file size` is self-explanatory

- `Choose a format to recode the video to` requires ffmpeg: after the video is downloaded, it is converted to a file format of your choosing

- `Choose a format to download the video in` doesn't require ffmpeg: it shows a list of formats directly available for download from YouTube without needing to convert things


# Features

### Format conversion
Blob-dl was designed to download large song playlists directly as audio files, as a result choosing between downloading audio files, normal video files or video-only files is very easy

### Error tracking

While downloading, blob-dl keeps track of any errors thrown by yt-dlp and reports them at the end, the ones caused which can be resolved by re-trying the download can be easily re-downloaded


# QA
### Who is this for?
This program is intended for anyone who needs to download things from YouTube without having to remember yt-dlp's syntax. Blob-dl can do everything an average user needs

yt-dlp power-users with very specific needs probably won't find this program useful

### Why did I make this?
I was tired of spending hours downloading music videos and converting them to audio, so I wrote this program to make everything way easier



# Notes
This logo was inspired by [@Primer](https://www.youtube.com/c/PrimerLearning)'s [blob plushie](https://store.dftba.com/collections/primer/products/primer-blob-plushie)
