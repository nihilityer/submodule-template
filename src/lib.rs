use anyhow::Result;
use nihility_common::{Log, LogConfig};
use tokio_util::sync::CancellationToken;
use tracing::info;

mod config;
mod customize;
mod thread;
mod utils;

const SUBMODULE_NAME: &str = "submodule-template";

pub async fn start() -> Result<CancellationToken> {
    let cancellation_token = CancellationToken::new();
    Log::init(&vec![LogConfig::default()]).unwrap();
    info!("init success");
    Ok(cancellation_token)
}
