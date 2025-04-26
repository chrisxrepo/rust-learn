use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
    sync::Arc,
};

use anyhow::{Ok, bail};
use quinn::{Endpoint, crypto::rustls::QuicClientConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let root_ca = loadca::load_root_ca(Path::new("loadca/ca/cert.pem"));

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let crypto = rustls::ClientConfig::builder()
        .with_root_certificates(root_ca)
        .with_no_client_auth();

    let mut client_config = quinn::ClientConfig::new(Arc::new(QuicClientConfig::try_from(crypto)?));
    //  let transport_config = Arc::get_mut(&mut client_config.transport).unwrap();
    // transport_config.max_concurrent_uni_streams(0_u8.into());
    let mut trasport_cfg = quinn::TransportConfig::default();
    client_config.transport_config(Arc::new(trasport_cfg));

    let mut endpoint = Endpoint::client("[::]:0".parse()?)?;
    endpoint.set_default_client_config(client_config.clone());

    // let conn = endpoint.connect(addr, "localhost")?.await?;
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4433);
    let conn = endpoint
        .connect_with(client_config, addr, "localhost")?
        .await?;
    println!("connect quic server: 127.0.0.1:4433");

    let (mut send_stream, mut recv_stream) = conn.open_bi().await?;

    //   let mut send_stream = conn.open_uni().await?;
    send_stream.write(b"Hello from TLS Client").await?;

    //    let mut recv_stream = conn.accept_uni().await?;
    let mut buf = [0u8; 1024];
    let n = recv_stream.read(&mut buf).await?.unwrap();

    let response = String::from_utf8_lossy(&buf[..n]);
    println!("Server Response: {}", response);

    Ok(())
}
