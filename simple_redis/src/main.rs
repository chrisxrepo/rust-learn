use std::sync::Arc;

use anyhow::Ok;

mod redis;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let socker_server = server::SocketServer::new("127.0.0.1:7000").await?;
    socker_server
        .serve(Arc::new(crate::redis::DB::new()))
        .await?;

    Ok(())
}
