use crate::accounts::handlers::{create_account, get_account};
use crate::state::AppState;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/account", post(create_account))
        .route("/account/{id}", get(get_account))
}
