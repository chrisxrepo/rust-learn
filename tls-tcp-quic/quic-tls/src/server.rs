use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
    sync::Arc,
};

use anyhow::Ok;
use quinn::{Endpoint, crypto::rustls::QuicServerConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let certs = loadca::load_certs(Path::new("loadca/ca/cert.pem"));
    let private_key = loadca::load_private_key(Path::new("loadca/ca/key.pem"));

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, private_key)?;

    let mut server_config =
        quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(crypto)?));

    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4433);
    let endpoint = Endpoint::server(server_config, addr)?;
    println!("start quic server: 127.0.0.1:4433");

    while let Some(conn) = endpoint.accept().await {
        let connection = conn.await?;
        println!("New connection from {}", connection.remote_address());

        tokio::spawn(async move {
            handle_connection(connection).await.unwrap();
        });
    }

    println!("Hello, world!");
    Ok(())
}

async fn handle_connection(conn: quinn::Connection) -> anyhow::Result<()> {
    let (mut send_stream, mut recv_stream) = conn.accept_bi().await?;

    loop {
        let mut buf = [0u8; 1024];
        let len = recv_stream.read(&mut buf).await?.unwrap();

        let msg = String::from_utf8_lossy(&buf[..len]);
        println!("Received: {}", msg);

        send_stream.write_all(b"Hello from QUIC server").await?;
    }
}
