use zero2prod::{
    app::{get_listener, run},
    configuration::get_configuration,
    router::app_routes,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = get_listener(&address)
        .await
        .expect("Failed to bind address");
    let app = app_routes();
    run(listener, app).await
}
