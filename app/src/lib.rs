mod db;

pub use db::{ConnectionStats, Database, PeerConnection};

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
        messages_inbound: u64,
        messages_outbound: u64,
    ) -> Result<()> {
        self.db.record_disconnection(
            connection_id,
            bytes_inbound,
            bytes_outbound,
            messages_inbound,
            messages_outbound,
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
}
