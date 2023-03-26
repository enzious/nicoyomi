# Nicoyomi

![License](https://img.shields.io/github/license/enzious/nicoyomi)
[![Docker](https://img.shields.io/docker/pulls/enzious/nicoyomi)](https://hub.docker.com/r/enzious/nicoyomi)
![Contributors](https://img.shields.io/github/contributors/enzious/nicoyomi)

A web server for browsing and downloading mangas to your Kindle library from your Kindle web browser.

## Starting a Nicoyomi web server

1. Using a shell/command-line, launch the web server with Docker:
```sh
docker run -p 6969:6969 enzious/nicoyomi
```
2. Open a web browser and open Nicoyomi:
```sh
xdg-open http://localhost:6969
```

## Requirements
- Docker
- Internet access

## Supported by the great work of following projects!
- [MangaDex.org](https://mangadex.org)
- [mangadex-downloader](https://github.com/mansuf/mangadex-downloader)
- [Calibre](https://calibre-ebook.com/)

## Todo
- Internal cron job for file clean-up
- Account access
- Display chapter titles and metadata
- Improved navigation and discovery
- Get the Docker image size down (1.5GB! Curse you Qt!)
- Bring the image retrieval and ebook generation into a Rust library
  - Might be hard to generate ebooks
- Include chapter in title or something visible for easy identification in library
- Improve error handling and logging
