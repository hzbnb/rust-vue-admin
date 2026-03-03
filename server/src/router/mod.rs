use axum::{Router, routing::get};

use crate::handle::health;

pub fn init_route() -> Router {
    Router::new().route("/health", get(health))
}
