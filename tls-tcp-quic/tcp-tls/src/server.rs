use std::{path::Path, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tokio_rustls::TlsAcceptor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let certs = loadca::load_certs(Path::new("loadca/ca/cert.pem"));
    let private_key = loadca::load_private_key(Path::new("loadca/ca/key.pem"));

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, private_key)
        .expect("bad certificate/key");

    let acceptor = TlsAcceptor::from(Arc::new(config));
    let listener = TcpListener::bind("localhost:8443").await?;
    println!("start tls bind: localhost:8443");

    loop {
        let (stream, _) = listener.accept().await?;
        let acceptor = acceptor.clone();
        tokio::spawn(async move {
            let mut tls_stream = acceptor.accept(stream).await.unwrap();

            let mut buf = [0 as u8; 1024];
            let len = tls_stream.read(&mut buf).await.unwrap();
            let data = String::from_utf8_lossy(&buf[..len]);
            println!("Received: {}", data);

            tls_stream
                .write_all(b"Hello from TLS Server!")
                .await
                .unwrap();
        });
    }
}
