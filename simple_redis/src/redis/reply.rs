use anyhow::Ok;
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[derive(Debug)]
pub enum Reply {
    Ok,
    Nil,
    Pong,
    String(String),
    Err(String),
    None,
}

impl Reply {
    const CRCF: &[u8] = "\r\n".as_bytes();
    const NIL: &[u8] = "$-1\r\n".as_bytes();
    const OK_REPLY: &[u8] = "+OK\r\n".as_bytes();
    const PONG_REPLY: &[u8] = "+PONG\r\n".as_bytes();

    pub async fn send_reply(&self, conn: &mut TcpStream) -> anyhow::Result<()> {
        match self {
            Reply::Ok => conn.write_all(Reply::OK_REPLY).await?,
            Reply::Nil => conn.write_all(Reply::NIL).await?,
            Reply::Pong => conn.write_all(Reply::PONG_REPLY).await?,
            Reply::String(s) => {
                conn.write_u8(b'+').await?;
                conn.write_all(s.as_bytes()).await?;
                conn.write(Reply::CRCF).await?;
            }
            Reply::Err(e) => {
                conn.write_u8(b'-').await?;
                conn.write_all(e.as_bytes()).await?;
                conn.write(Reply::CRCF).await?;
            }
            Reply::None => (),
        };
        conn.flush().await?;
        Ok(())
    }
}
