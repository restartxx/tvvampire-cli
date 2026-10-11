mod cli;
mod parser;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use parser::{get_ep, mpv_run, search};
use regex::escape;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Args::parse();

    // Retry policy with exponential backoff for transient errors
    let retry_pol = ExponentialBackoff::builder().build_with_max_retries(10);
    let client = ClientBuilder::new(Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_pol))
        .build();

    // Escape user input for safe use inside regex
    let safe_query = escape(&cli.query).to_lowercase();

    let url = search(&client, &safe_query).await?;
    let audio_url = get_ep(
        &client,
        cli.temp,
        cli.ep,
        &safe_query.replace(" ", "-"),
        &url,
    )
    .await?;

    mpv_run(&audio_url, &cli.video_url)?;

    Ok(())
}
