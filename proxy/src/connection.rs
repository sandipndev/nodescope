use crate::bitcoin_protocol::{BitcoinMessage, MessageParser, Network};
use anyhow::Context;
use app::NodeScopeApp;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, info, warn, error};

/// Represents a direction of traffic flow
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Inbound,  // Client -> Bitcoin Core
    Outbound, // Bitcoin Core -> Client
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Inbound => write!(f, "→"),
            Direction::Outbound => write!(f, "←"),
        }
    }
}

/// Statistics for a connection
#[derive(Debug, Clone, Default)]
pub struct ConnectionStats {
    pub bytes_inbound: u64,
    pub bytes_outbound: u64,
    pub messages_inbound: u64,
    pub messages_outbound: u64,
}

/// Handles a single proxy connection between a client and Bitcoin Core
pub struct ConnectionHandler {
    connection_id: u64,
    client_addr: String,
    target_addr: String,
    network: Network,
    stats: Arc<tokio::sync::Mutex<ConnectionStats>>,
    app: NodeScopeApp,
}

impl ConnectionHandler {
    pub fn new(
        connection_id: u64,
        client_addr: String,
        target_addr: String,
        network: Network,
        app: NodeScopeApp,
    ) -> Self {
        Self {
            connection_id,
            client_addr,
            target_addr,
            network,
            stats: Arc::new(tokio::sync::Mutex::new(ConnectionStats::default())),
            app,
        }
    }

    /// Handle the proxied connection
    pub async fn handle(
        self,
        mut client: TcpStream,
        mut target: TcpStream,
    ) -> anyhow::Result<()> {
        info!(
            "[conn:{}] Established: {} <-> {}",
            self.connection_id, self.client_addr, self.target_addr
        );

        // Record connection in database
        if let Err(e) = self.app.record_connection(
            self.connection_id,
            &self.client_addr,
            &self.target_addr,
        ).await {
            error!("[conn:{}] Failed to record connection in database: {}", self.connection_id, e);
        }

        // Split both streams into read/write halves
        let (client_read, client_write) = client.split();
        let (target_read, target_write) = target.split();

        // Create bidirectional forwarding tasks
        let inbound = self.forward_traffic(
            client_read,
            target_write,
            Direction::Inbound,
        );

        let outbound = self.forward_traffic(
            target_read,
            client_write,
            Direction::Outbound,
        );

        // Wait for either direction to close or error
        tokio::select! {
            result = inbound => {
                if let Err(e) = result {
                    warn!("[conn:{}] Inbound error: {}", self.connection_id, e);
                }
            }
            result = outbound => {
                if let Err(e) = result {
                    warn!("[conn:{}] Outbound error: {}", self.connection_id, e);
                }
            }
        }

        // Log final statistics
        let stats = self.stats.lock().await;
        info!(
            "[conn:{}] Closed: {} bytes in ({} msgs), {} bytes out ({} msgs)",
            self.connection_id,
            stats.bytes_inbound,
            stats.messages_inbound,
            stats.bytes_outbound,
            stats.messages_outbound
        );

        // Record disconnection in database
        if let Err(e) = self.app.record_disconnection(
            self.connection_id,
            stats.bytes_inbound,
            stats.bytes_outbound,
        ).await {
            error!("[conn:{}] Failed to record disconnection in database: {}", self.connection_id, e);
        }

        Ok(())
    }

    /// Forward traffic in one direction while parsing Bitcoin messages
    async fn forward_traffic<R, W>(
        &self,
        mut reader: R,
        mut writer: W,
        direction: Direction,
    ) -> anyhow::Result<()>
    where
        R: AsyncReadExt + Unpin,
        W: AsyncWriteExt + Unpin,
    {
        let mut parser = MessageParser::new(self.network);
        let mut buffer = vec![0u8; 8192];

        loop {
            // Read data from source
            let n = reader
                .read(&mut buffer)
                .await
                .context("Failed to read from stream")?;

            if n == 0 {
                debug!("[conn:{}] {} EOF", self.connection_id, direction);
                break;
            }

            let data = &buffer[..n];

            // Update statistics
            {
                let mut stats = self.stats.lock().await;
                match direction {
                    Direction::Inbound => stats.bytes_inbound += n as u64,
                    Direction::Outbound => stats.bytes_outbound += n as u64,
                }
            }

            // Try to parse Bitcoin messages
            let messages = parser.push_data(data);
            for msg in messages {
                self.log_message(&msg, direction).await;
            }

            // Forward the data unchanged
            writer
                .write_all(data)
                .await
                .context("Failed to write to stream")?;
            writer.flush().await.context("Failed to flush stream")?;
        }

        Ok(())
    }

    /// Log a parsed Bitcoin message
    async fn log_message(&self, msg: &BitcoinMessage, direction: Direction) {
        let mut stats = self.stats.lock().await;
        match direction {
            Direction::Inbound => stats.messages_inbound += 1,
            Direction::Outbound => stats.messages_outbound += 1,
        }

        // Determine source and destination peers based on direction
        let (source_peer, destination_peer) = match direction {
            Direction::Inbound => (self.client_addr.clone(), self.target_addr.clone()),
            Direction::Outbound => (self.target_addr.clone(), self.client_addr.clone()),
        };

        let direction_str = match direction {
            Direction::Inbound => "inbound",
            Direction::Outbound => "outbound",
        };

        info!(
            "[conn:{}] {} {} (from {} to {})",
            self.connection_id, direction, msg.description(), source_peer, destination_peer
        );

        // Record message in database
        if let Err(e) = self.app.record_message(
            self.connection_id,
            direction_str,
            &source_peer,
            &destination_peer,
            msg.message_type(),
            msg.payload_len as u64,
            &msg.description(),
        ).await {
            warn!("[conn:{}] Failed to record message in database: {}", self.connection_id, e);
        }
    }
}

