use std::{path::Path, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_rustls::TlsConnector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let root_ca = loadca::load_root_ca(Path::new("loadca/ca/cert.pem"));

    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_ca)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect("localhost:8443").await?;
    println!("connect to:localhost:8443");

    let domain = "localhost".try_into()?;
    let mut tls_stream = connector.connect(domain, stream).await?;

    tls_stream.write(b"Hello from TLS Client").await?;

    let mut buf = [0u8; 1024];
    let n = tls_stream.read(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf[..n]);
    println!("Server Response: {}", response);

    tls_stream.shutdown().await?;

    Ok(())
}
