use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read Configuration");
    let connection_pool = PgPool::connect(&configuration.database.connecting_string())
        .await
        .expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address).expect("Failed to bind port");
    println!("listening on: {}", &address);
    run(listener, connection_pool)?.await
}
