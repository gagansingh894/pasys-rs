use crate::accounts;
use crate::config::Config;
use crate::state::AppState;
use axum::Router;
use axum::http::StatusCode;
use axum::routing::get;
use std::sync::Arc;

pub(crate) async fn router(config: Config) -> Router {
    let app_state = AppState::new(config).await;

    Router::new()
        // nest accounts router
        .route("/api", get(healthcheck))
        .nest("/api", accounts::router::router())
        .with_state(Arc::new(app_state))
}

async fn healthcheck() -> StatusCode {
    StatusCode::OK
}
