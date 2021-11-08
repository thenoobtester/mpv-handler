[English][readme-en] | [简体中文][readme-zh-hans] | [繁体中文][readme-zh-hant]

[readme-en]: https://github.com/akiirui/mpv-handler/blob/main/README.md
[readme-zh-hans]: https://github.com/akiirui/mpv-handler/blob/main/README.zh-Hans.md
[readme-zh-hant]: https://github.com/akiirui/mpv-handler/blob/main/README.zh-Hant.md

# mpv handler

A protocol handler for mpv, written by Rust.

Please use with userscript:

[![play-with-mpv][badges-play-with-mpv]][greasyfork-play-with-mpv]

## Protocol URL

Base URL:

```
mpv://play/BASE64_ENCODED_VIDEO_URL/
```

Optional parameters:

```
cookies     = [ true, false ]
downloader  = [ ytdl ] (default: ytdl)
quality     = [ best, 2160p, 1440p, 1080p, 720p, 480p, 360p ]

c = cookies
d = downloader
q = quality
```

Example:

```
mpv://play/aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj1HZ2tuMmY1ZS1JVQ==/?cookies=true&downloader=ytdl&quality=best

mpv://play/aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj1EcnZ1c29zekJLQQ==/?c=true&d=ytdl&q=best
```

## Installation

### Linux

- Arch Linux

  [![mpv-handler][badges-aur]][download-aur] \
  [![mpv-handler-git][badges-aur-git]][download-aur-git]

#### Manual installation

1. Download [latest/mpv-handler-linux-x64.zip][download-linux]
2. Unzip the archive
3. Copy `mpv-handler` to `$HOME/.local/bin`
4. Copy `mpv-handler.desktop` to `$HOME/.local/share/applications/`
5. Add `$HOME/.local/bin` to your environment variable `PATH` (if it not lists in your `PATH`)
6. Register xdg-mime (thanks for the [linuxuprising][linuxuprising] reminder)

```
$ xdg-mime default mpv-handler.desktop x-scheme-handler/mpv
```

### Windows

Windows users need to install `mpv-handler` manually.

#### Manual installation

1. Download [latest/mpv-handler-windows-x64.zip][download-windows]
2. Unzip the archive to the directory you want (since v0.2.x, not requires to install in the same directory with `mpv` anymore)
3. Run `handler-install.bat` register protocol handler

[badges-aur-git]: https://img.shields.io/aur/version/mpv-handler-git?label=mpv-handler-git&style=for-the-badge
[badges-aur]: https://img.shields.io/aur/version/mpv-handler?label=mpv-handler&style=for-the-badge
[badges-play-with-mpv]: https://img.shields.io/badge/dynamic/json?style=for-the-badge&label=play-with-mpv&prefix=v&query=version&url=https%3A%2F%2Fgreasyfork.org%2Fscripts%2F416271.json
[download-aur-git]: https://aur.archlinux.org/packages/mpv-handler-git/
[download-aur]: https://aur.archlinux.org/packages/mpv-handler/
[download-linux]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-linux-x64.zip
[download-windows]: https://github.com/akiirui/mpv-handler/releases/latest/download/mpv-handler-windows-x64.zip
[greasyfork-play-with-mpv]: https://greasyfork.org/scripts/416271-play-with-mpv
[linuxuprising]: https://www.linuxuprising.com/2021/07/open-youtube-and-more-videos-from-your.html

## Configuration

```toml
mpv = "/usr/bin/mpv"
ytdl = "/usr/bin/yt-dlp"
```
