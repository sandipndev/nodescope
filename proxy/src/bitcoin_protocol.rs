use bitcoin::consensus::encode::{self, Decodable};
use bitcoin::p2p::message::{CommandString, RawNetworkMessage};
use bitcoin::p2p::Magic;
use std::fmt;
use std::io::Cursor;

/// Re-export bitcoin types for convenience
pub use bitcoin::Network;

/// Wrapper around rust-bitcoin's RawNetworkMessage for easier logging
#[derive(Debug, Clone)]
pub struct BitcoinMessage {
    pub network: Network,
    pub command: CommandString,
    pub payload_len: usize,
    pub raw_message: RawNetworkMessage,
}

impl fmt::Display for BitcoinMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BitcoinMessage {{ network: {:?}, command: {}, payload_size: {} }}",
            self.network,
            self.command,
            self.payload_len
        )
    }
}

impl BitcoinMessage {
    /// Get a human-readable command name
    pub fn command_name(&self) -> &str {
        self.command.as_ref()
    }

    /// Get a detailed description of the message for logging
    pub fn description(&self) -> String {
        use bitcoin::p2p::message::NetworkMessage;

        match self.raw_message.payload() {
            NetworkMessage::Version(v) => {
                format!(
                    "version: protocol_version={}, services={:?}, user_agent={}",
                    v.version, v.services, v.user_agent
                )
            }
            NetworkMessage::Verack => "verack: handshake complete".to_string(),
            NetworkMessage::Addr(addrs) => format!("addr: {} addresses", addrs.len()),
            NetworkMessage::Inv(inv) => format!("inv: {} inventory items", inv.len()),
            NetworkMessage::GetData(inv) => format!("getdata: {} requests", inv.len()),
            NetworkMessage::GetBlocks(_) => "getblocks: block locator request".to_string(),
            NetworkMessage::GetHeaders(_) => "getheaders: header request".to_string(),
            NetworkMessage::Tx(tx) => format!("tx: txid={}", tx.compute_txid()),
            NetworkMessage::Block(block) => {
                format!("block: hash={}", block.block_hash())
            }
            NetworkMessage::Headers(headers) => {
                format!("headers: {} headers", headers.len())
            }
            NetworkMessage::SendHeaders => "sendheaders: request header announcements".to_string(),
            NetworkMessage::GetAddr => "getaddr: request peer addresses".to_string(),
            NetworkMessage::MemPool => "mempool: request mempool transactions".to_string(),
            NetworkMessage::Ping(nonce) => format!("ping: nonce={}", nonce),
            NetworkMessage::Pong(nonce) => format!("pong: nonce={}", nonce),
            NetworkMessage::NotFound(inv) => format!("notfound: {} items", inv.len()),
            NetworkMessage::GetCFilters(_) => "getcfilters: compact filter request".to_string(),
            NetworkMessage::CFilter(_) => "cfilter: compact filter".to_string(),
            NetworkMessage::GetCFHeaders(_) => {
                "getcfheaders: compact filter headers request".to_string()
            }
            NetworkMessage::CFHeaders(_) => "cfheaders: compact filter headers".to_string(),
            NetworkMessage::GetCFCheckpt(_) => {
                "getcfcheckpt: compact filter checkpoint request".to_string()
            }
            NetworkMessage::CFCheckpt(_) => "cfcheckpt: compact filter checkpoint".to_string(),
            NetworkMessage::SendCmpct(_) => "sendcmpct: compact block relay".to_string(),
            NetworkMessage::CmpctBlock(_) => "cmpctblock: compact block".to_string(),
            NetworkMessage::GetBlockTxn(_) => "getblocktxn: request block transactions".to_string(),
            NetworkMessage::BlockTxn(_) => "blocktxn: block transactions".to_string(),
            NetworkMessage::Alert(_) => "alert: network alert (deprecated)".to_string(),
            NetworkMessage::Reject(_) => "reject: rejection message".to_string(),
            NetworkMessage::FeeFilter(_) => "feefilter: minimum fee filter".to_string(),
            NetworkMessage::WtxidRelay => "wtxidrelay: witness transaction relay".to_string(),
            NetworkMessage::AddrV2(addrs) => format!("addrv2: {} addresses", addrs.len()),
            NetworkMessage::SendAddrV2 => "sendaddrv2: request addrv2 messages".to_string(),
            NetworkMessage::Unknown { command, payload } => {
                format!("unknown: command={}, {} bytes", command, payload.len())
            }
            _ => format!("{}: (other)", self.command_name()),
        }
    }
}

/// Parser that maintains state for streaming Bitcoin message parsing
pub struct MessageParser {
    buffer: Vec<u8>,
    network: Network,
}

impl MessageParser {
    /// Create a new parser for the given network
    pub fn new(network: Network) -> Self {
        Self {
            buffer: Vec::new(),
            network,
        }
    }

    /// Add data to the parser and extract any complete messages
    pub fn push_data(&mut self, data: &[u8]) -> Vec<BitcoinMessage> {
        self.buffer.extend_from_slice(data);

        let mut messages = Vec::new();

        loop {
            if self.buffer.is_empty() {
                break;
            }

            // Try to decode a message
            let mut cursor = Cursor::new(&self.buffer);
            match RawNetworkMessage::consensus_decode(&mut cursor) {
                Ok(raw_message) => {
                    let bytes_read = cursor.position() as usize;
                    
                    // Verify the network magic matches
                    if *raw_message.magic() != Magic::from(self.network) {
                        // Skip one byte and try again (could be noise or wrong network)
                        self.buffer.drain(..1);
                        continue;
                    }

                    let message = BitcoinMessage {
                        network: self.network,
                        command: raw_message.command(),
                        payload_len: bytes_read - 24, // Subtract header size
                        raw_message,
                    };

                    messages.push(message);
                    self.buffer.drain(..bytes_read);
                }
                Err(encode::Error::Io(ref e)) if e.kind() == bitcoin::io::ErrorKind::UnexpectedEof => {
                    // Not enough data yet, wait for more
                    break;
                }
                Err(_) => {
                    // Parse error - could be noise or encrypted data
                    // Skip one byte and try again
                    self.buffer.drain(..1);
                    
                    // If buffer gets too large without a valid message, clear it
                    if self.buffer.len() > 10_000_000 {
                        self.buffer.clear();
                        break;
                    }
                }
            }
        }

        messages
    }

    /// Get the current buffer size (for debugging)
    #[cfg(test)]
    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_parser_incomplete() {
        let mut parser = MessageParser::new(Network::Bitcoin);
        
        // Incomplete data should not produce any messages
        let messages = parser.push_data(&[0xf9, 0xbe, 0xb4, 0xd9]);
        assert_eq!(messages.len(), 0);
        assert!(parser.buffer_len() > 0);
    }
}

