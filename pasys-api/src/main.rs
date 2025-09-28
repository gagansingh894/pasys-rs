use std::env;

use common::shutdown;

use crate::config::Config;

mod helpers;
mod router;
mod state;

mod accounts;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set up a subscriber to print logs to stdout
    tracing_subscriber::fmt::init();

    // setup config
    let config = Config {
        port: env::var("PASYS_API_PORT")
            .unwrap_or("9000".into())
            .parse::<u16>()?,
        accounts_host: env::var("ACCOUNTS_HOST").expect("ACCOUNTS_HOST must be set"),
    };

    // setup router
    let router = router::router(config.clone()).await;

    // setup server
    let address = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to create TCP listener ❌");

    // log that the server is running
    tracing::info!(
        "{}",
        format!("Server is running on http://0.0.0.0:{} 🚀 \n", config.port)
    );

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await?;

    Ok(())
}
