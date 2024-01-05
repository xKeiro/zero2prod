use axum::{routing::get, Router};

use crate::routes::health_check::health_check;

pub fn app_routes() -> Router {
    Router::new().route("/health_check", get(health_check))
}
