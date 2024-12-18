mod parse;

use std::str::FromStr;
use tokio;
use anyhow::{Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};


#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", parse::lib::RtspMethod::OPTIONS.to_string());
    let m =  parse::lib::RtspMethod::from_str("SETUP")?;
    println!("{}", m);
    return Ok(());

    let mut conn = tokio::net::TcpStream::connect("192.168.12.223:554").await?;
    let options_body = "OPTIONS rtsp://192.168.12.223:554 RTSP/1.0\r\nCSeq: 1\r\nUser-Agent: rrtsp-client\r\n\r\n";
    conn.write(options_body.as_ref()).await?;
    let mut read_buf: [u8; 2048] = [0; 2048];
    conn.read(&mut read_buf).await?;
    println!("{}", std::str::from_utf8(&read_buf).unwrap());
    Ok(())
}
