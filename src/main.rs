use zero2prod::router::app_routes;

#[tokio::main]
async fn main() {
    let app = app_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
