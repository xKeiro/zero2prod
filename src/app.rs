use std::future::Future;

use axum::{serve::Serve, Router};
use tokio::net::TcpListener;

pub fn run(listener: TcpListener, app: Router) -> Serve<Router, Router> {
    axum::serve(listener, app)
}

pub fn get_listener(address: &str) -> impl Future<Output = Result<TcpListener, std::io::Error>> {
    tokio::net::TcpListener::bind(address.to_string())
}
