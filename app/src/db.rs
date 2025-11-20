use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::Row;
use std::path::Path;
use tracing::{info, debug};

/// Database handle for managing peer connection data
#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

/// Represents a peer connection event
#[derive(Debug, Clone)]
pub struct PeerConnection {
    pub id: i64,
    pub connection_id: u64,
    pub client_addr: String,
    pub target_addr: String,
    pub connected_at: DateTime<Utc>,
    pub disconnected_at: Option<DateTime<Utc>>,
    pub bytes_inbound: Option<u64>,
    pub bytes_outbound: Option<u64>,
    pub messages_inbound: Option<u64>,
    pub messages_outbound: Option<u64>,
}

impl Database {
    /// Initialize a new database connection
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let db_path_str = db_path.as_ref().to_string_lossy().to_string();
        
        info!("Initializing database at: {}", db_path_str);

        let options = SqliteConnectOptions::new()
            .filename(&db_path_str)
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .context("Failed to connect to database")?;

        let db = Self { pool };
        db.run_migrations().await?;

        info!("Database initialized successfully");
        Ok(db)
    }

    /// Run database migrations
    async fn run_migrations(&self) -> Result<()> {
        debug!("Running database migrations");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS peer_connections (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                connection_id INTEGER NOT NULL,
                client_addr TEXT NOT NULL,
                target_addr TEXT NOT NULL,
                connected_at TEXT NOT NULL,
                disconnected_at TEXT,
                bytes_inbound INTEGER,
                bytes_outbound INTEGER,
                messages_inbound INTEGER,
                messages_outbound INTEGER
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create peer_connections table")?;

        // Create indexes for common queries
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_connection_id 
            ON peer_connections(connection_id)
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create connection_id index")?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_connected_at 
            ON peer_connections(connected_at)
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create connected_at index")?;

        debug!("Database migrations completed");
        Ok(())
    }

    /// Record a new peer connection
    pub async fn record_connection(
        &self,
        connection_id: u64,
        client_addr: &str,
        target_addr: &str,
    ) -> Result<i64> {
        let connected_at = Utc::now();
        
        let result = sqlx::query(
            r#"
            INSERT INTO peer_connections 
            (connection_id, client_addr, target_addr, connected_at)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(connection_id as i64)
        .bind(client_addr)
        .bind(target_addr)
        .bind(connected_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .context("Failed to record connection")?;

        let id = result.last_insert_rowid();
        debug!(
            "Recorded connection: id={}, connection_id={}, target={}",
            id, connection_id, target_addr
        );

        Ok(id)
    }

    /// Record disconnection and connection statistics
    pub async fn record_disconnection(
        &self,
        connection_id: u64,
        bytes_inbound: u64,
        bytes_outbound: u64,
        messages_inbound: u64,
        messages_outbound: u64,
    ) -> Result<()> {
        let disconnected_at = Utc::now();

        sqlx::query(
            r#"
            UPDATE peer_connections 
            SET disconnected_at = ?,
                bytes_inbound = ?,
                bytes_outbound = ?,
                messages_inbound = ?,
                messages_outbound = ?
            WHERE connection_id = ?
            AND disconnected_at IS NULL
            "#,
        )
        .bind(disconnected_at.to_rfc3339())
        .bind(bytes_inbound as i64)
        .bind(bytes_outbound as i64)
        .bind(messages_inbound as i64)
        .bind(messages_outbound as i64)
        .bind(connection_id as i64)
        .execute(&self.pool)
        .await
        .context("Failed to record disconnection")?;

        debug!(
            "Recorded disconnection: connection_id={}, bytes_in={}, bytes_out={}",
            connection_id, bytes_inbound, bytes_outbound
        );

        Ok(())
    }

    /// Get all peer connections
    pub async fn get_all_connections(&self) -> Result<Vec<PeerConnection>> {
        let rows = sqlx::query(
            r#"
            SELECT id, connection_id, client_addr, target_addr, 
                   connected_at, disconnected_at,
                   bytes_inbound, bytes_outbound,
                   messages_inbound, messages_outbound
            FROM peer_connections
            ORDER BY connected_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch connections")?;

        let connections = rows
            .into_iter()
            .map(|row| {
                let connected_at_str: String = row.get("connected_at");
                let disconnected_at_str: Option<String> = row.get("disconnected_at");

                PeerConnection {
                    id: row.get("id"),
                    connection_id: row.get::<i64, _>("connection_id") as u64,
                    client_addr: row.get("client_addr"),
                    target_addr: row.get("target_addr"),
                    connected_at: DateTime::parse_from_rfc3339(&connected_at_str)
                        .unwrap()
                        .with_timezone(&Utc),
                    disconnected_at: disconnected_at_str.and_then(|s| {
                        DateTime::parse_from_rfc3339(&s)
                            .ok()
                            .map(|dt| dt.with_timezone(&Utc))
                    }),
                    bytes_inbound: row.get::<Option<i64>, _>("bytes_inbound").map(|v| v as u64),
                    bytes_outbound: row.get::<Option<i64>, _>("bytes_outbound").map(|v| v as u64),
                    messages_inbound: row.get::<Option<i64>, _>("messages_inbound").map(|v| v as u64),
                    messages_outbound: row.get::<Option<i64>, _>("messages_outbound").map(|v| v as u64),
                }
            })
            .collect();

        Ok(connections)
    }

    /// Get active connections (not yet disconnected)
    pub async fn get_active_connections(&self) -> Result<Vec<PeerConnection>> {
        let rows = sqlx::query(
            r#"
            SELECT id, connection_id, client_addr, target_addr, 
                   connected_at, disconnected_at,
                   bytes_inbound, bytes_outbound,
                   messages_inbound, messages_outbound
            FROM peer_connections
            WHERE disconnected_at IS NULL
            ORDER BY connected_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch active connections")?;

        let connections = rows
            .into_iter()
            .map(|row| {
                let connected_at_str: String = row.get("connected_at");

                PeerConnection {
                    id: row.get("id"),
                    connection_id: row.get::<i64, _>("connection_id") as u64,
                    client_addr: row.get("client_addr"),
                    target_addr: row.get("target_addr"),
                    connected_at: DateTime::parse_from_rfc3339(&connected_at_str)
                        .unwrap()
                        .with_timezone(&Utc),
                    disconnected_at: None,
                    bytes_inbound: None,
                    bytes_outbound: None,
                    messages_inbound: None,
                    messages_outbound: None,
                }
            })
            .collect();

        Ok(connections)
    }

    /// Get connection statistics
    pub async fn get_connection_stats(&self) -> Result<ConnectionStats> {
        let row = sqlx::query(
            r#"
            SELECT 
                COUNT(*) as total_connections,
                COUNT(CASE WHEN disconnected_at IS NULL THEN 1 END) as active_connections,
                COALESCE(SUM(bytes_inbound), 0) as total_bytes_inbound,
                COALESCE(SUM(bytes_outbound), 0) as total_bytes_outbound,
                COALESCE(SUM(messages_inbound), 0) as total_messages_inbound,
                COALESCE(SUM(messages_outbound), 0) as total_messages_outbound
            FROM peer_connections
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to fetch connection stats")?;

        Ok(ConnectionStats {
            total_connections: row.get::<i64, _>("total_connections") as u64,
            active_connections: row.get::<i64, _>("active_connections") as u64,
            total_bytes_inbound: row.get::<i64, _>("total_bytes_inbound") as u64,
            total_bytes_outbound: row.get::<i64, _>("total_bytes_outbound") as u64,
            total_messages_inbound: row.get::<i64, _>("total_messages_inbound") as u64,
            total_messages_outbound: row.get::<i64, _>("total_messages_outbound") as u64,
        })
    }
}

/// Aggregate connection statistics
#[derive(Debug, Clone)]
pub struct ConnectionStats {
    pub total_connections: u64,
    pub active_connections: u64,
    pub total_bytes_inbound: u64,
    pub total_bytes_outbound: u64,
    pub total_messages_inbound: u64,
    pub total_messages_outbound: u64,
}

