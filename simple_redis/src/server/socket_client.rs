use std::{net::SocketAddr, sync::Arc};

use anyhow::{Ok, bail};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct SocketClient {
    addr: SocketAddr,
    conn: TcpStream,
    db: Arc<crate::redis::DB>,
}

impl SocketClient {
    pub async fn new(
        addr: std::net::SocketAddr,
        conn: tokio::net::TcpStream,
        db: Arc<crate::redis::DB>,
    ) -> Self {
        println!("new client connect addr:{:?}", addr);
        Self { addr, conn, db }
    }

    pub async fn serve(&mut self) -> anyhow::Result<()> {
        let mut buffer = [0; 1024];

        loop {
            let n = self.conn.read(&mut buffer).await?;
            if n == 0 {
                println!("connect closed: {}", self.addr);
                bail!("EOF")
            }

            let cmd = crate::redis::Cmd::decode(&buffer[0..n])?;
            println!("recv cmd: {:?}", cmd);

            // direct reply
            if let crate::redis::Cmd::DirectReply(reply) = &cmd {
                reply.send_reply(&mut self.conn).await?;
                continue;
            }

            let reply = cmd.do_cmd(&self.db)?;
            reply.send_reply(&mut self.conn).await?;
        }
    }
}
