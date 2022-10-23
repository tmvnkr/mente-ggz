use mente_ggz::{configuration::get_configuration, startup::run};
use std::net::{SocketAddr, TcpListener};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));

    let client_addr = SocketAddr::from(([127, 0, 0, 1], configuration.client_port));

    let listener = TcpListener::bind(&addr).unwrap();

    run(listener, client_addr)?.await
}
