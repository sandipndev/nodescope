mod db;

pub use db::{ConnectionStats, Database, Message, PeerConnection};

use anyhow::Result;
use std::path::Path;

/// Main application structure that holds the database and application state
#[derive(Clone)]
pub struct NodeScopeApp {
    db: Database,
}

impl NodeScopeApp {
    /// Create a new NodeScopeApp instance with the database at the specified path
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let db = Database::new(db_path).await?;
        Ok(Self { db })
    }

    /// Get a reference to the database
    pub fn db(&self) -> &Database {
        &self.db
    }

    /// Record a new peer connection
    pub async fn record_connection(
        &self,
        connection_id: u64,
        client_addr: &str,
        target_addr: &str,
    ) -> Result<i64> {
        self.db.record_connection(connection_id, client_addr, target_addr).await
    }

    /// Record a peer disconnection with statistics
    pub async fn record_disconnection(
        &self,
        connection_id: u64,
        bytes_inbound: u64,
        bytes_outbound: u64,
    ) -> Result<()> {
        self.db.record_disconnection(
            connection_id,
            bytes_inbound,
            bytes_outbound,
        ).await
    }

    /// Get all peer connections
    pub async fn get_all_connections(&self) -> Result<Vec<PeerConnection>> {
        self.db.get_all_connections().await
    }

    /// Get active peer connections
    pub async fn get_active_connections(&self) -> Result<Vec<PeerConnection>> {
        self.db.get_active_connections().await
    }

    /// Get connection statistics
    pub async fn get_connection_stats(&self) -> Result<ConnectionStats> {
        self.db.get_connection_stats().await
    }

    /// Record an individual message
    pub async fn record_message(
        &self,
        connection_id: u64,
        direction: &str,
        source_peer: &str,
        destination_peer: &str,
        message_type: &str,
        payload_size: u64,
        description: &str,
    ) -> Result<i64> {
        self.db.record_message(
            connection_id,
            direction,
            source_peer,
            destination_peer,
            message_type,
            payload_size,
            description,
        ).await
    }

    /// Get all messages for a specific connection
    pub async fn get_messages_by_connection(&self, connection_id: u64) -> Result<Vec<Message>> {
        self.db.get_messages_by_connection(connection_id).await
    }

    /// Get recent messages across all connections
    pub async fn get_recent_messages(&self, limit: i64) -> Result<Vec<Message>> {
        self.db.get_recent_messages(limit).await
    }

    /// Get messages by peer address
    pub async fn get_messages_by_peer(&self, peer_addr: &str) -> Result<Vec<Message>> {
        self.db.get_messages_by_peer(peer_addr).await
    }
}
