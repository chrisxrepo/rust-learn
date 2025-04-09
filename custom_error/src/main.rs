//#[derive(Debug)]
// enum CustomError {
//     IOError(std::io::Error),
//     Utf8Error(std::str::Utf8Error),
//     ParseIntError(std::num::ParseIntError),
// }

// impl std::fmt::Display for CustomError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match &self {
//             CustomError::IOError(e) => e.fmt(f),
//             CustomError::Utf8Error(e) => e.fmt(f),
//             CustomError::ParseIntError(e) => e.fmt(f),
//         }
//     }
// }

// impl std::error::Error for CustomError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match &self {
//             CustomError::IOError(e) => Some(e),
//             CustomError::Utf8Error(e) => Some(e),
//             CustomError::ParseIntError(e) => Some(e),
//         }
//     }
// }

// impl From<std::io::Error> for CustomError {
//     fn from(e: std::io::Error) -> Self {
//         CustomError::IOError(e)
//     }
// }

// impl From<std::str::Utf8Error> for CustomError {
//     fn from(e: std::str::Utf8Error) -> Self {
//         CustomError::Utf8Error(e)
//     }
// }
// impl From<std::num::ParseIntError> for CustomError {
//     fn from(e: std::num::ParseIntError) -> Self {
//         CustomError::ParseIntError(e)
//     }
// }

use anyhow::{Chain, Context, bail};

fn main() -> Result<(), anyhow::Error> {
    // let o = Some(100);
    // let v = o.ok_or(0);
    //let v = anyhow::anyhow!("Missing attribute");

    let v = read_file("data.txt")?;
    let x = to_utf8(v.as_bytes())?;
    let u = to_u32(x)?;
    println!("{:?}", u);
    Ok(())
}

fn read_file(path: &str) -> anyhow::Result<String> {
    //   let v = std::fs::read_to_string(path)?;
    let v = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file from {}", path))?;
    //.map_err(|e| anyhow::anyhow!("read_to_string error:{:?}", e))?;
    Ok(v)
}

fn to_utf8(v: &[u8]) -> Result<&str, anyhow::Error> {
    let v = std::str::from_utf8(v)?;
    Ok(v)
}

fn to_u32(v: &str) -> Result<u32, anyhow::Error> {
    let v = v.parse()?;
    Ok(v)
}
