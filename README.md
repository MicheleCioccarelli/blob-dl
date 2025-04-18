<p align="center">
    <a>
    <img alt="`blob-dl` Logo" src="../assets/blob-dl-logo-v1.png" height="150">
  </a>
</p>

<div align="center">
    
![Crates.io](https://img.shields.io/crates/d/blob-dl?color=%2325BE5D)
![GitHub](https://img.shields.io/badge/license-MIT-blue)
![GitHub issues](https://img.shields.io/github/issues/MicheleCioccarelli/blob-dl)
![Crates.io](https://img.shields.io/crates/v/blob-dl)
    
</div>

<h1 align="center">blob-dl</h1>


`blob-dl` is a command line tool used to download video and audio files from YouTube. It acts as an interface to [`yt-dlp`](https://github.com/yt-dlp/yt-dlp) and works by asking a series of questions that make it generate and then execute a `yt-dlp` command that fits your needs.

The idea behind this program is to remove all the tedious work of researching which flags you need to pass to `yt-dlp` to make it do what you want.
When you use blob-dl you only need to know the url of what you want to download, and it'll figure out the rest.

- See the [Features](https://github.com/MicheleCioccarelli/blob-dl#features) section for more details on what `blob-dl` can do

[![asciicast](https://asciinema.org/a/jZUokSc5oDms6vICdNTic1vxh.svg)](https://asciinema.org/a/jZUokSc5oDms6vICdNTic1vxh)


# Installation
The most straightforward way to install `blob-dl` is to use [the binaries](https://github.com/MicheleCioccarelli/blob-dl/releases/)

Alternatively, if you are a Rust programmer you can install `blob-dl` with `cargo`

```
$ cargo install blob-dl
```
## Dependencies
`blob-dl` depends on `yt-dlp`, you can install it by following the official [guide](https://github.com/yt-dlp/yt-dlp#installation).

You should also install `yt-dlp`'s [recommended dependencies](https://github.com/yt-dlp/yt-dlp#dependencies) to access all of `blob-dl`'s features (namely `ffmpeg` and `ffprobe`).

# Usage
To use `blob-dl` you just have to pass it the url of the video or playlist that you want to download, the program will understand by itself what the link refers to and ask you questions accordingly.

The first one is `What kind of file(s) do you want to download?`

The answer you choose determines which download formats you can pick later on: For example, if you answer that you want to download audio-only files, then formats containing video will be hidden. In this readme, statements about downloading `video`s also apply to audio-only downloads

The second question `Which quality or format do you want to apply to the video?` allows you to choose a specific format, quality, filesize, ...  

The available answers mean these things:

- `Best possible quality` tells yt-dlp to automatically choose the `best` quality, for more information see `yt-dlp`'s [wiki](https://github.com/yt-dlp/yt-dlp#format-selection)

- `Smallest file size` uses the format which results in the smallest file size

- `Choose a format to recode the video to` is only available if ffmpeg is installed: After the video is downloaded, it can be converted to a file format of your choosing

- `Choose a format to download the video in` doesn't require ffmpeg: it shows a list of formats directly available for download from YouTube without needing to convert anything, but the choice is rather limited


`blob-dl` will also ask other questions, but they are self-explanatory

# Features

### Format conversion
`blob-dl` was designed to download large song playlists directly as audio files. Choosing between downloading audio files, normal video files or video-only files is very easy

### Playlist Download
With `blob-dl` you can download whole playlists in one go, you can also choose a single file format to apply to all videos

### Error tracking

While downloading, `blob-dl` keeps track of any errors thrown by yt-dlp and reports them at the end, the ones caused which can be resolved by re-trying the download can be easily re-downloaded

## Conifguration files
If you find yourself downloading videos using the same settings often and always answering the same questions has
started to annoy you it's time to use a config file!

It makes blob-dl already know what you want so it can avoid asking too many questions.

### How to create a config file
Creating a new config file is a straight-forward process: just use the `-g` flag and blob-dl will generate a config file with your answers in its default location
```
$ blob-dl -g "youtube url"
```
Note that using `-g` multiple times will overwrite old config files: there will only be one at a time in the default location



####  Default Config File Location

|    OS     |                                Config Path                                |
|:---------:|:-------------------------------------------------------------------------:|
| **Linux** | `~/.config/blob-dl/config.json` or <br/> `$XDG_CONFIG_HOME/blob-dl/config.json`|
| **macOS** |            `~/Library/Application Support/blob-dl/config.json`            |
| **Windows** |                      `%APPDATA%\blob-dl\config.json`                      |

#### Notes:
- On **Linux**, if `$XDG_CONFIG_HOME` is not set, it defaults to `~/.config`.
- On **macOS**, `~/Library` is typically hidden. Use `Cmd + Shift + .` in Finder to reveal hidden folders.
- On **Windows**, `%APPDATA%` usually resolves to `C:\Users\YourName\AppData\Roaming`.

### Usage

Use `-c` or `--use-config` to use the config file present in blob-dl's default location
```
$ blob-dl -c "youtube url"
```
If you've moved your config file somewhere else you should use `-l "filepath"`
```
$ blob-dl -l "/Users/YourName/Desktop/config.json" "youtube url"
```

### How to edit your config file
A blob-dl config file looks something like this:


filename: `config.json`
```
{
    "url": null,
    "output_path": "/Users/YourName/Desktop",
    "include_indexes": false,
    "chosen_format": "BestQuality",
    "media_selected": "FullVideo",
    "download_target": "YtPlaylist"
}
```
Each of these fields can be set to "null". If that is the case blob-dl will ask you a question related to what you've left out

`url` is ignored by blob-dl, so you can leave this always null

`output_path` is where the files you are downloading will end up, it should be path

`include_indexes` is a boolean value: when you are downloading a playlist you can have a video's position in the playlist as a part of its filename (e.g. 1_firstvideo, 2_secondvideo, ... )

`chosen_format` is what format you want your files to be in. It has a few options: 

|        Option         |                                                                                                         What it does                                                                                                         |
|:---------------------:|:----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------:|
|    **BestQuality**    |                                                                           blob-dl will download your video/audio in the highest quality available                                                                            |
|   **SmallestSize**    |                                                                         blob-dl will download your video/audio using the smallest filesize available                                                                         |
| **ConvertTo(format)** |                                                            After downloading your video/audio blob-dl will use ffmpeg to convert it to a format of your choosing                                                             |
| **UniqueFormat(id)**  | This is not supposed to be edited by end users: each possible format has a numerical id. The problem is that it is unlikely for a specific format to be available for multiple videos, making it a bad fit for a config file |

Syntax for using ConvertTo(format):
```
  "chosen_format": {
    "ConvertTo": "mp4"
  },
```
This feature supports all the formats that ffmpeg does: `mp4, mkv, mov, avi, flv, gif, webm, aac, aiff, alac, flac, m4a, mka, mp3, ogg, opus, vorbis, wav`

`media_selection` refers to whether you want to download a normal video, audio only or video only.
It expects a string and the available options are: `FullVideo` `AudioOnly` `VideoOnly` 

`download_target` is whether you are downloading a single video or a full playlist.
It expects a string, the options are `YtPlaylist` (which should be used in most circumstances, even when downloading a normal video) and  `YtVideo(index)` which is only needed when you are downloading a single video from a playlist, needing to specify its index in it.


# ADD FEATURE THAT TELLS YOU WHERE SHIT IS

# Q&A
### Who is this for?
This program is intended for anyone who wants to download things from YouTube without having to remember yt-dlp's syntax. `blob-dl` can do everything an average user needs but with less hassle

`yt-dlp` power users with advanced needs probably won't find this program useful.

### Why did I make this?
Have you ever had to download videos from YouTube? 
The process can be quite a pain because you will have to either spend your time closing pop-ups from a sketchy website or browsing through [`yt-dlp`](https://github.com/yt-dlp/yt-dlp)'s documentation.

I was tired of spending hours downloading music videos and converting them to audio, so I wrote this program to make everything way easier

## Notes
This logo was inspired by [@Primer](https://www.youtube.com/c/PrimerLearning)'s [blob plushie](https://store.dftba.com/collections/primer/products/primer-blob-plushie)
