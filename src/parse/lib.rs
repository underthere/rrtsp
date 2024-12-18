use std::collections::VecDeque;
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;
use url::Url;
use strum::{Display, EnumString};


#[derive(Error, Debug)]
enum ParseError {
    #[error("parse error!")]
    Error
}

#[derive(Copy, Clone, Debug, Display, EnumString)]
pub enum RtspMethod {
    #[strum(to_string="OPTIONS")]
    OPTIONS,
    #[strum(to_string="DESCRIBE")]
    DESCRIBE,
    #[strum(to_string="SETUP")]
    SETUP,
    #[strum(to_string="PLAY")]
    PLAY,
}

#[derive(Copy, Clone, Debug)]
enum RtspVersion {
    V1_0,
    V2_0,
}

unsafe impl Send for RtspMethod {}
unsafe impl Sync for RtspMethod {}

struct RtspAuthorization {}

struct RtspRequestHeader {
    method: RtspMethod,
    url: Url,
    version: RtspVersion,
}

const RTSP_REQUEST_HEADER_PARTS: usize = 3;
struct RtspRequest {
    // header
    header: RtspRequestHeader,
    cseq: i64,
    ua: String,
    authorization: Option<RtspAuthorization>,
}

impl FromStr for RtspRequestHeader {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != RTSP_REQUEST_HEADER_PARTS {
            todo!()
        }
        todo!()
    }
}

impl FromStr for RtspRequest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines: VecDeque<&str> = s.lines().map(|ss| s.trim()).collect();
        let head_line = lines.pop_front().unwrap();
        Err(ParseError::Error.into())
    }
}

impl Display for RtspRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", 1)
    }
}
