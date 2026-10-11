use anyhow::{Context, Result, ensure};
use regex::Regex;
use reqwest_middleware::ClientWithMiddleware;
use std::collections::HashMap;
use std::num::NonZeroU8;
use std::process::Command;

/// Resolve a direct stream URL using yt-dlp -g
/// This is used to get the real audio URL from a Google Drive download link
pub fn resolve_direct_stream(url: &str) -> Result<String> {
    ensure!(url.starts_with("http"), "Invalid URL!");

    // Run yt-dlp to get the direct URL (-g = --get-url)
    let output = Command::new("yt-dlp").args(["-g", url]).output()?;
    ensure!(
        output.status.success(),
        "Failed to run yt-dlp -g on audio URL! Error: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // yt-dlp returns the URL with a newline, so we trim it
    let direct_url = String::from_utf8(output.stdout)?.trim().to_owned();
    ensure!(!direct_url.is_empty(), "yt-dlp returned an empty URL!");

    Ok(direct_url)
}

/// Run mpv with a separate audio file (resolved via yt-dlp) and a video URL
pub fn mpv_run(audio_url: &str, vid_url: &str) -> Result<()> {
    ensure!(audio_url.starts_with("http"), "Invalid audio URL!");

    println!("🔎 Resolving audio stream via yt-dlp...");
    let resolved_audio_url = resolve_direct_stream(audio_url)?;

    // Spoof a common user-agent to avoid 403s
    let user_agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

    // Build and run mpv. Using status() instead of output() so mpv can show its window
    let status = Command::new("mpv")
        .arg(format!("--user-agent={user_agent}"))
        .arg(format!("--audio-file={}", resolved_audio_url))
        .arg(vid_url)
        .status()?;

    ensure!(status.success(), "mpv exited with an error.");

    Ok(())
}

/// Search for a series page on the blog
pub async fn search(client: &ClientWithMiddleware, query: &str) -> Result<String> {
    // Build a regex that looks for href="...{query}....html"
    let patt = format!(
        r#"href=['"]([^'"]*{}[^'"]*\.html)['"]"#,
        query.replace(" ", "-")
    );
    let rgx = Regex::new(&patt)?;
    let query = query.replace(" ", "+");

    let html = client
        .get(format!("https://tvvampire.blogspot.com/search?q={}", query))
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    // First capture group is the URL
    let url = &rgx
        .captures(&html)
        .context("Failed to find a valid URL to the search!")?;
    let url = &url[1];
    Ok(url.to_string())
}

/// Get the Google Drive download link for a specific episode
pub async fn get_ep(
    client: &ClientWithMiddleware,
    temp: NonZeroU8,
    ep: NonZeroU8,
    query: &str,
    url: &str,
) -> Result<String> {
    // Find the iframe that contains the series page
    let patt = format!(r#"iframe src=['"]([^'"]*{}[^'"]*\.html)['"]"#, query);
    let rgx = Regex::new(&patt)?;
    let html = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let url = &rgx
        .captures(&html)
        .context("Failed to find a valid URL fot the ep!")?;
    let url = &url[1];

    // Build season URL: e.g..../show.html ->.../show-1-temporada.html
    let url = url
        .strip_suffix(".html")
        .context("Failed to strip suffix!")?;
    let url = format!("{}-{}-temporada.html", url, temp);
    // Regex to extract drive id and episode number from the season page
    let rgx = Regex::new(
        r#"data-src="https://drive\.google\.com/file/d/(?P<id>[\w-]+)/preview\s*"[\s\S]*?value="\s*Episódio\s*(?P<ep>\d+)"#,
    )?;
    let html = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    // Map ep number -> drive id
    let mut map = HashMap::new();
    for cap in rgx.captures_iter(&html) {
        map.insert(cap["ep"].to_string(), cap["id"].to_string());
    }
    ensure!(!map.is_empty(), "No episodes found in the provided HTML.");

    // Format with leading zero because the map keys are like "01", "02"
    let url_dl = format!(
        r#"https://drive.usercontent.google.com/download?id={}&export=download"#,
        map.get(&format!("{:02}", ep))
            .context("Failed to get ID for the given episode!")?
    );

    Ok(url_dl)
}
