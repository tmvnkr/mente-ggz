use mente_ggz::{configuration::get_configuration, startup::run};
use std::net::{SocketAddr, TcpListener};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let configuration = get_configuration().expect("Failed to read configuration");

    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));

    let listener = TcpListener::bind(&addr).unwrap();

    run(listener)?.await
}
