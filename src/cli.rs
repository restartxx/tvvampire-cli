use clap::Parser;
use std::num::NonZeroU8;

/// Simple CLI to find and play episodes using mpv + yt-dlp
#[derive(Parser, Debug)]
#[command(author, version, about = "Finds an episode link and plays it with mpv")]
pub struct Args {
    /// Series slug to search for (e.g. vampire-diaries)
    #[arg(help = "Series name/slug used for search and iframe matching")]
    pub query: String,

    /// Season number (must be > 0)
    #[arg(help = "Season number")]
    pub temp: NonZeroU8,

    /// Episode number (must be > 0)
    #[arg(help = "Episode number")]
    pub ep: NonZeroU8,

    /// Video URL to play in mpv (will be used as the main video)
    #[arg(help = "Direct video URL or path to play as video track in mpv")]
    pub video_url: String,
}
