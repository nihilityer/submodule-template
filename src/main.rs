use tokio::{
    select, signal,
};
use tracing::error;

use submodule_template::start;

#[tokio::main]
async fn main() {
    match start().await {
        Ok(cancellation_token) => {
            select! {
                _ = signal::ctrl_c() => {
                    cancellation_token.cancel();
                },
                _ = cancellation_token.cancelled() => {}
            }
        }
        Err(e) => {
            println!("Application Run Fail: {}", e);
        }
    }
    println!("press any key to exit");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}
