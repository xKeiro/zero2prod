use axum::{
    routing::{get, post},
    Router,
};

use crate::routes::{health_check::health_check, subscriptions::subscribe};

pub fn app_routes() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}
