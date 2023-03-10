use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

use zerozero::configuration::get_configuration;
use zerozero::startup::run;
use zerozero::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zerozero".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(&configuration.database.connect_string().expose_secret())
            .await
            .expect("Failed to connect to postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await?;
    Ok(())
}
