use std::{fmt::format, sync::Arc};

use anyhow::{Ok, bail};

#[derive(Debug)]
pub enum Cmd<'a> {
    Set { key: &'a str, value: &'a str },
    Get { key: &'a str },

    DirectReply(crate::redis::Reply),
}

impl<'a> Cmd<'a> {
    const EMPTY_BYTES: &'static str = "";

    pub fn decode(bs: &'a [u8]) -> anyhow::Result<Self> {
        let chunks = Self::parse_cmd_chunks(bs)?;

        match chunks[0].to_uppercase().as_str() {
            "GET" => {
                if chunks.len() < 2 {
                    return Cmd::err_number_args(chunks[0]);
                }
                Ok(Cmd::Get { key: chunks[1] })
            }

            "SET" => {
                if chunks.len() < 3 {
                    return Cmd::err_number_args(chunks[0]);
                }

                Ok(Cmd::Set {
                    key: chunks[1],
                    value: chunks[2],
                })
            }

            "PING" => Ok(Cmd::DirectReply(crate::redis::Reply::Pong)),
            "COMMAND" => Ok(Cmd::DirectReply(crate::redis::Reply::Ok)),

            _ => Ok(Cmd::DirectReply(crate::redis::Reply::Err(format!(
                "Not support command:{}",
                chunks[0].to_lowercase().as_str()
            )))),
        }
    }

    pub fn do_cmd(&self, db: &crate::redis::DB) -> anyhow::Result<crate::redis::Reply> {
        match self {
            Cmd::Set { key, value } => db.set_key_value(key.to_string(), value.to_string()),
            Cmd::Get { key } => db.get_key_value(key.to_string()),
            Cmd::DirectReply(_) => Ok(crate::redis::Reply::None),
        }
    }

    fn parse_cmd_chunks(bs: &[u8]) -> anyhow::Result<Vec<&str>> {
        let chunks = Self::split_crlf(&bs);
        if chunks[0][0] != b'*' {
            bail!("Invalid Redis bytes:{}", String::from_utf8(bs.to_vec())?)
        }
        let fields_len: usize = String::from_utf8(chunks[0][1..].to_vec())?.parse()?;

        let mut fields: Vec<&str> = Vec::new();
        let mut value_flag = false;
        let mut value_size: i32 = 0;
        for chunk in chunks[1..].to_vec() {
            if value_flag {
                if chunk.len() as i32 != value_size {
                    bail!("Invalid Redis bytes:{}", String::from_utf8(bs.to_vec())?)
                }
                fields.push(std::str::from_utf8(chunk)?);
                value_flag = false;
                continue;
            }

            if chunk[0] != b'$' {
                bail!("Invalid Redis bytes:{}", String::from_utf8(bs.to_vec())?)
            }

            value_size = String::from_utf8(chunk[1..].to_vec())?.parse()?;
            if value_size < 0 {
                fields.push(Self::EMPTY_BYTES);
            } else {
                value_flag = true;
            }
        }

        if fields.len() != fields_len {
            bail!("Invalid Redis bytes:{}", String::from_utf8(bs.to_vec())?)
        }
        Ok(fields)
    }

    fn split_crlf(data: &[u8]) -> Vec<&[u8]> {
        let finder = memchr::memmem::Finder::new(b"\r\n");
        let mut chunks = Vec::new();
        let mut start = 0;

        while let Some(pos) = finder.find(&data[start..]) {
            let end = start + pos;
            chunks.push(&data[start..end]);
            start = end + 2;
        }

        if data.len() > start {
            chunks.push(&data[start..]);
        }
        chunks
    }

    fn err_number_args(cmd: &str) -> anyhow::Result<Cmd> {
        let err_str = format!(
            "ERR wrong number of arguments for '{}' command",
            cmd.to_lowercase().as_str()
        );
        Ok(Cmd::DirectReply(crate::redis::Reply::Err(err_str)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_bytes() {
        let bs = "*3\r\n:1\r\n:2\r\n:3\r\n".as_bytes();
        let chunks = Cmd::split_crlf(&bs);
        println!("len: {}", chunks.len());

        assert_eq!(chunks.len(), 4);
        assert_eq!(chunks[0], b"*3");
        assert_eq!(chunks[1], b":1");
        assert_eq!(chunks[2], b":2");
        assert_eq!(chunks[3], b":3");
    }

    #[test]
    fn test_parse_cmd_chunks() {
        let bs = "*2\r\n$4\r\nLLEN\r\n$6\r\nmylist\r\n".as_bytes();
        let chunks = Cmd::parse_cmd_chunks(bs).unwrap();
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0], "LLEN");
        assert_eq!(chunks[1], "mylist");
    }
}
