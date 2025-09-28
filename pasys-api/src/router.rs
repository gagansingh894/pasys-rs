use crate::accounts;
use crate::state::{AppState, Config};
use axum::Router;
use std::env;
use std::sync::Arc;

async fn _router() -> Router {
    let config = Config {
        accounts_host: env::var("ACCOUNTS_HOST").expect("ACCOUNTS_HOST must be set"),
    };

    let app_state = AppState::new(config).await;

    Router::new()
        .nest("/api", accounts::router::router())
        .with_state(Arc::new(app_state))
}
