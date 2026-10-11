# 🎬 tvvampire-cli

[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Tokio](https://img.shields.io/badge/Tokio-Async-blue)](https://tokio.rs/)
[![Clap](https://img.shields.io/badge/Clap-v4-green)](https://docs.rs/clap)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Clippy Clean](https://img.shields.io/badge/Clippy-clean-brightgreen)](https://github.com/rust-lang/rust-clippy)

A blazingly fast Rust CLI that finds an episode and plays it with mpv + yt-dlp as a separate audio track.

Built with `anyhow`, `reqwest`, `tokio` and **love for clean error handling**.

### ✨ Features

- 🔍 Scrapes and finds the correct series page
- 📺 Resolves season / episode via Google Drive IDs
- 🎧 Resolves direct audio stream with yt-dlp -g
- 🚀 Plays everything with mpv --audio-file
- ♻️ Automatic retries with exponential backoff
- 🦀 100% Clippy-clean, idiomatic Rust

### 📦 Requirements

You need these binaries in your $PATH:

- `mpv` - https://mpv.io/
- `yt-dlp` - https://github.com/yt-dlp/yt-dlp

```bash
# Arch / Manjaro
sudo pacman -S mpv yt-dlp

# macOS
brew install mpv yt-dlp
```

---

### ⚡️ Installation
```bash
git clone https://github.com/youruser/tvvampire-cli
cd tvvampire-cli
cargo build --release
# binary will be at `./target/release/tvvampire-cli`
```

### 🚀 Usage
```bash
tvvampire-cli <QUERY> <TEMP> <EP> <VIDEO_URL> --help
```

### Arguments:

- QUERY - Series slug (e.g. vampire-diaries)
- TEMP - Season number (>0)
- EP - Episode number (>0)
- VIDEO_URL - Video file / URL to use as video track in mpv

### Example:
```bash
tvvampire-cli "grimm" 2 5 "https://example.com/my-video.mp4"
```
This will:
- Search vampire-diaries on the blog
- Go to season 2
- Extract episode 05 Drive ID
- Resolve direct audio URL with yt-dlp -g
- Run: mpv --audio-file=<direct> <VIDEO_URL>

### 🧠 How it works
```
[search] -> finds series page (regex)
   |
[get_ep] -> finds iframe -> {slug}-{temp}-temporada.html
   |
         -> parses data-src="drive.google.com/file/d/{id}/preview" + Episódio {ep}
   |
         -> builds https://drive.usercontent.google.com/download?id={id}
   |
[resolve_direct_stream] -> yt-dlp -g {download_url} -> direct stream
   |
[mpv_run] -> mpv --user-agent=... --audio-file={direct} {video_url}
```

### 🛠️ Stack

- anyhow + ensure! - ergonomic error handling
- clap - CLI parsing with NonZeroU8 safety
- reqwest + reqwest-middleware + reqwest-retry - resilient HTTP client
- tokio - async runtime
- regex - fast HTML parsing (for this simple case)

### 📁 Project Structure
```
src/main.rs
├── Args (clap)
├── resolve_direct_stream()
├── mpv_run()
├── search()
├── get_ep()
└── main()
```

### ⚖️ Disclaimer

This project is for educational purposes only, demonstrating Rust async, CLI design and process spawning with mpv / yt-dlp. You are responsible for respecting copyright laws in your country.

### 📄 License

MIT - do whatever you want, but no warranty.
