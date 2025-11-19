use anyhow::{anyhow, Context, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, info};

/// SOCKS5 protocol constants
const SOCKS5_VERSION: u8 = 0x05;
const SOCKS5_NO_AUTH: u8 = 0x00;
const SOCKS5_CMD_CONNECT: u8 = 0x01;
const SOCKS5_ATYP_IPV4: u8 = 0x01;
const SOCKS5_ATYP_DOMAIN: u8 = 0x03;
const SOCKS5_ATYP_IPV6: u8 = 0x04;
const SOCKS5_SUCCESS: u8 = 0x00;
const SOCKS5_GENERAL_FAILURE: u8 = 0x01;

/// Represents a parsed SOCKS5 connection request
#[derive(Debug, Clone)]
pub struct Socks5Request {
    pub target_addr: String,
    pub target_port: u16,
}

impl std::fmt::Display for Socks5Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.target_addr, self.target_port)
    }
}

/// Handle SOCKS5 handshake and return the target address
pub async fn handle_socks5_handshake(
    stream: &mut TcpStream,
    connection_id: u64,
) -> Result<Socks5Request> {
    // Step 1: Read client greeting
    let mut buf = [0u8; 257];
    let n = stream
        .read(&mut buf)
        .await
        .context("Failed to read SOCKS5 greeting")?;

    if n < 2 {
        return Err(anyhow!("Invalid SOCKS5 greeting: too short"));
    }

    let version = buf[0];
    let nmethods = buf[1] as usize;

    if version != SOCKS5_VERSION {
        return Err(anyhow!("Unsupported SOCKS version: {}", version));
    }

    if n < 2 + nmethods {
        return Err(anyhow!("Invalid SOCKS5 greeting: incomplete methods"));
    }

    debug!("[conn:{}] SOCKS5 greeting: {} methods", connection_id, nmethods);

    // Step 2: Respond with no authentication required
    stream
        .write_all(&[SOCKS5_VERSION, SOCKS5_NO_AUTH])
        .await
        .context("Failed to write SOCKS5 auth response")?;
    stream.flush().await.context("Failed to flush stream")?;

    // Step 3: Read connection request
    let n = stream
        .read(&mut buf)
        .await
        .context("Failed to read SOCKS5 request")?;

    if n < 4 {
        return Err(anyhow!("Invalid SOCKS5 request: too short"));
    }

    let version = buf[0];
    let cmd = buf[1];
    let _rsv = buf[2];
    let atyp = buf[3];

    if version != SOCKS5_VERSION {
        return Err(anyhow!("Invalid SOCKS5 version in request: {}", version));
    }

    if cmd != SOCKS5_CMD_CONNECT {
        send_socks5_error(stream, SOCKS5_GENERAL_FAILURE).await?;
        return Err(anyhow!("Unsupported SOCKS5 command: {}", cmd));
    }

    // Step 4: Parse target address
    let (target_addr, target_port) = match atyp {
        SOCKS5_ATYP_IPV4 => {
            if n < 10 {
                return Err(anyhow!("Invalid SOCKS5 IPv4 request"));
            }
            let addr = format!("{}.{}.{}.{}", buf[4], buf[5], buf[6], buf[7]);
            let port = u16::from_be_bytes([buf[8], buf[9]]);
            (addr, port)
        }
        SOCKS5_ATYP_DOMAIN => {
            let domain_len = buf[4] as usize;
            if n < 5 + domain_len + 2 {
                return Err(anyhow!("Invalid SOCKS5 domain request"));
            }
            let domain = String::from_utf8(buf[5..5 + domain_len].to_vec())
                .context("Invalid domain name")?;
            let port = u16::from_be_bytes([buf[5 + domain_len], buf[5 + domain_len + 1]]);
            (domain, port)
        }
        SOCKS5_ATYP_IPV6 => {
            if n < 22 {
                return Err(anyhow!("Invalid SOCKS5 IPv6 request"));
            }
            let addr = format!(
                "{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}",
                buf[4], buf[5], buf[6], buf[7], buf[8], buf[9], buf[10], buf[11],
                buf[12], buf[13], buf[14], buf[15], buf[16], buf[17], buf[18], buf[19]
            );
            let port = u16::from_be_bytes([buf[20], buf[21]]);
            (addr, port)
        }
        _ => {
            send_socks5_error(stream, SOCKS5_GENERAL_FAILURE).await?;
            return Err(anyhow!("Unsupported SOCKS5 address type: {}", atyp));
        }
    };

    info!(
        "[conn:{}] SOCKS5 request: {}:{}",
        connection_id, target_addr, target_port
    );

    // Step 5: Send success response
    // Response format: [version, status, reserved, atyp, bind_addr, bind_port]
    // We'll use 0.0.0.0:0 as the bind address
    let response = [
        SOCKS5_VERSION,
        SOCKS5_SUCCESS,
        0x00, // reserved
        SOCKS5_ATYP_IPV4,
        0, 0, 0, 0, // bind addr: 0.0.0.0
        0, 0, // bind port: 0
    ];

    stream
        .write_all(&response)
        .await
        .context("Failed to write SOCKS5 success response")?;
    stream.flush().await.context("Failed to flush stream")?;

    Ok(Socks5Request {
        target_addr,
        target_port,
    })
}

/// Send a SOCKS5 error response
async fn send_socks5_error(stream: &mut TcpStream, error_code: u8) -> Result<()> {
    let response = [
        SOCKS5_VERSION,
        error_code,
        0x00, // reserved
        SOCKS5_ATYP_IPV4,
        0, 0, 0, 0, // bind addr: 0.0.0.0
        0, 0, // bind port: 0
    ];

    stream
        .write_all(&response)
        .await
        .context("Failed to write SOCKS5 error response")?;
    stream.flush().await.context("Failed to flush stream")?;

    Ok(())
}

