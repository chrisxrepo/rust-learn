use std::sync::Arc;

pub struct SocketServer {
    listener: tokio::net::TcpListener,
}

impl SocketServer {
    pub async fn new(addr: &str) -> anyhow::Result<SocketServer> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        Ok(SocketServer { listener })
    }

    pub async fn serve(&self, db: Arc<crate::redis::DB>) -> anyhow::Result<()> {
        loop {
            let (stream, addr) = self.listener.accept().await?;

            let mut client =
                crate::server::socket_client::SocketClient::new(addr, stream, db.clone()).await;

            tokio::spawn(async move {
                match client.serve().await {
                    Err(e) => println!("client spawn exist:{}", e),
                    Ok(()) => (),
                };
            });
        }
    }
}
