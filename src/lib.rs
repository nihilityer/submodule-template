use anyhow::Result;
use nihility_common::{Log, LogConfig};
use tokio_util::sync::CancellationToken;

mod config;
mod customize;
mod thread;
mod utils;

const SUBMODULE_NAME: &str = "submodule-template";

pub async fn start() -> Result<CancellationToken> {
    let cancellation_token = CancellationToken::new();
    Log::init(&vec![LogConfig::default()]).unwrap();
    Ok(cancellation_token)
}
