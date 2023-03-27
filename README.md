# Nicoyomi

[![License](https://img.shields.io/github/license/enzious/nicoyomi)](https://github.com/enzious/nicoyomi/blob/main/LICENSE)
[![Docker](https://img.shields.io/docker/pulls/enzious/nicoyomi)](https://hub.docker.com/r/enzious/nicoyomi)
[![Contributors](https://img.shields.io/github/contributors/enzious/nicoyomi)](https://github.com/enzious/nicoyomi/graphs/contributors)
[![GitHub Repo stars](https://img.shields.io/github/stars/enzious/nicoyomi?style=social)](https://github.com/enzious/nicoyomi)

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

## Troubleshooting
1. Why doesn't Nicoyomi work on my Kindle when I host it with TSL/SSL?
   - Try using HTTP instead of HTTPS, or use an older TLS/SSL standard. You may have an older Kindle web browser.
2. Why do some mangas and chapters fail to download while others succeed?
   - There are some mangas listed that MangaDex doesn't actually host but instead redirects to a licensees domain. These will fail, I'll have to figure that out when I have time.
3. What is this "none" volume I'm seeing on some mangas?
   - That is a "fake" volume for chapters that do not have a volume. This could be because there just isn't a volume, or the chapter is new and hasn't been grouped into a volume.
4. I have an idea for a feature, where should I send it?
   - Open an [issue](https://github.com/enzious/nicoyomi/issues/new)! This was a very fun 5-day project and I'm still wanting to add more features.
5. Why?
   - Idk, writing Rust is fun and manga on the Kindle is really comfy.

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
- Add volume covers to chapter ebooks
