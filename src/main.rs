use zero2prod::{
    app::{get_listener, run},
    router::app_routes,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = get_listener("0.0.0.0:3000")
        .await
        .expect("Failed to bind address");
    let app = app_routes();
    run(listener, app).await
}
