<p align="center">
    <img alt="Blob-dl Logo" src="../assets/blob-dl-logo-v1.png" height="150">
    <h3 align="center">blob-dl</h3>
  </a>
</p>

# blob-dl
blob-dl is a command line tool used to download videos and audio files from youtube (and different websites in the future!)

essentialy it is a very noob-friendly way to use yt-dlp: it asks a series of questions which help generate and execute the correct command.

# Installation
Currently the only ways to install blob-dl are to use the command `cargo install` or to compile the source code yourself (make this paragraph better plz)

## Dependencies
blob-dl is a noob-friendly way to use yt-dlp, so it cannot function without it.
If you don't already have it, install yt-dlp [here](https://github.com/yt-dlp/yt-dlp#installation).

You should also consider installing yt-dlp's [recommended dependencies](https://github.com/yt-dlp/yt-dlp#dependencies).

# Usage
When you fire up blob-dl it asks you `What kind of file(s) do you want to download?` which answer you choose determines which formats you will be able to pick later on, for example if you say you want to download audio-only files, formats containing video will be hidden.

The second question `Which quality or format do you want to apply to the video?` allowa you to choose a specific formaat, quality, filesize, ...    The available answers mean these things:

`Best possible quality` telle yt-dlp to automatically choose the "best" quality, for more information see yt-dlp's [wiki](https://github.com/yt-dlp/yt-dlp#format-selection)

`Smallest file size` is self explanatory

`Choose a format to recode the video to` requires ffmpeg: after the video is downloaded, it is converted to a file format of your choosing

`Choose a format to download the video in` doesn't require ffmpeg: it shows a list of formats directly available for download from youtube without needing to convert things

The other questions blob-dl asks don't need an explanation

This logo was inspired by [@Primer](https://www.youtube.com/c/PrimerLearning)'s [blob plushie](https://store.dftba.com/collections/primer/products/primer-blob-plushie)

Recommended dependencies: ffmpeg, ffprobe

Needed for: merging the best video-audio files && post-processing
