use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::Row;
use std::path::Path;
use tracing::{debug, info, warn};

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
}

/// Message type enum corresponding to Bitcoin P2P message types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    Version,
    Verack,
    Addr,
    Inv,
    GetData,
    NotFound,
    GetBlocks,
    GetHeaders,
    Tx,
    Block,
    Headers,
    GetAddr,
    MemPool,
    Ping,
    Pong,
    SendHeaders,
    FeeFilter,
    SendCmpct,
    CmpctBlock,
    GetBlockTxn,
    BlockTxn,
    GetCFilters,
    CFilter,
    GetCFHeaders,
    CFHeaders,
    GetCFCheckpt,
    CFCheckpt,
    AddrV2,
    SendAddrV2,
    WtxidRelay,
    FilterLoad,
    FilterAdd,
    FilterClear,
    MerkleBlock,
    Reject,
    Alert,
    Unknown,
}

impl MessageType {
    /// Parse from command string
    pub fn from_command(command: &str) -> Self {
        match command.to_lowercase().as_str() {
            "version" => MessageType::Version,
            "verack" => MessageType::Verack,
            "addr" => MessageType::Addr,
            "inv" => MessageType::Inv,
            "getdata" => MessageType::GetData,
            "notfound" => MessageType::NotFound,
            "getblocks" => MessageType::GetBlocks,
            "getheaders" => MessageType::GetHeaders,
            "tx" => MessageType::Tx,
            "block" => MessageType::Block,
            "headers" => MessageType::Headers,
            "getaddr" => MessageType::GetAddr,
            "mempool" => MessageType::MemPool,
            "ping" => MessageType::Ping,
            "pong" => MessageType::Pong,
            "sendheaders" => MessageType::SendHeaders,
            "feefilter" => MessageType::FeeFilter,
            "sendcmpct" => MessageType::SendCmpct,
            "cmpctblock" => MessageType::CmpctBlock,
            "getblocktxn" => MessageType::GetBlockTxn,
            "blocktxn" => MessageType::BlockTxn,
            "getcfilters" => MessageType::GetCFilters,
            "cfilter" => MessageType::CFilter,
            "getcfheaders" => MessageType::GetCFHeaders,
            "cfheaders" => MessageType::CFHeaders,
            "getcfcheckpt" => MessageType::GetCFCheckpt,
            "cfcheckpt" => MessageType::CFCheckpt,
            "addrv2" => MessageType::AddrV2,
            "sendaddrv2" => MessageType::SendAddrV2,
            "wtxidrelay" => MessageType::WtxidRelay,
            "filterload" => MessageType::FilterLoad,
            "filteradd" => MessageType::FilterAdd,
            "filterclear" => MessageType::FilterClear,
            "merkleblock" => MessageType::MerkleBlock,
            "reject" => MessageType::Reject,
            "alert" => MessageType::Alert,
            _ => MessageType::Unknown,
        }
    }

    /// Convert to string for database storage
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageType::Version => "version",
            MessageType::Verack => "verack",
            MessageType::Addr => "addr",
            MessageType::Inv => "inv",
            MessageType::GetData => "getdata",
            MessageType::NotFound => "notfound",
            MessageType::GetBlocks => "getblocks",
            MessageType::GetHeaders => "getheaders",
            MessageType::Tx => "tx",
            MessageType::Block => "block",
            MessageType::Headers => "headers",
            MessageType::GetAddr => "getaddr",
            MessageType::MemPool => "mempool",
            MessageType::Ping => "ping",
            MessageType::Pong => "pong",
            MessageType::SendHeaders => "sendheaders",
            MessageType::FeeFilter => "feefilter",
            MessageType::SendCmpct => "sendcmpct",
            MessageType::CmpctBlock => "cmpctblock",
            MessageType::GetBlockTxn => "getblocktxn",
            MessageType::BlockTxn => "blocktxn",
            MessageType::GetCFilters => "getcfilters",
            MessageType::CFilter => "cfilter",
            MessageType::GetCFHeaders => "getcfheaders",
            MessageType::CFHeaders => "cfheaders",
            MessageType::GetCFCheckpt => "getcfcheckpt",
            MessageType::CFCheckpt => "cfcheckpt",
            MessageType::AddrV2 => "addrv2",
            MessageType::SendAddrV2 => "sendaddrv2",
            MessageType::WtxidRelay => "wtxidrelay",
            MessageType::FilterLoad => "filterload",
            MessageType::FilterAdd => "filteradd",
            MessageType::FilterClear => "filterclear",
            MessageType::MerkleBlock => "merkleblock",
            MessageType::Reject => "reject",
            MessageType::Alert => "alert",
            MessageType::Unknown => "unknown",
        }
    }
}

/// Represents an individual message exchanged between peers
#[derive(Debug, Clone)]
pub struct Message {
    pub id: i64,
    pub connection_id: u64,
    pub timestamp: DateTime<Utc>,
    pub direction: String,
    pub source_peer: String,
    pub destination_peer: String,
    pub message_type: MessageType,
    pub payload_size: u64,
    pub description: String,
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

    /// Run database migrations from the migrations directory
    async fn run_migrations(&self) -> Result<()> {
        debug!("Running database migrations");

        // Read and execute migration files in order
        let migrations_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
        
        if migrations_dir.exists() {
            let mut entries: Vec<_> = std::fs::read_dir(&migrations_dir)
                .context("Failed to read migrations directory")?
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path().extension().and_then(|s| s.to_str()) == Some("sql")
                })
                .collect();
            
            entries.sort_by_key(|e| e.path());

            for entry in entries {
                let path = entry.path();
                let migration_sql = std::fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read migration file: {:?}", path))?;
                
                debug!("Executing migration: {:?}", path.file_name());
                
                // Remove comments and split by semicolon
                let cleaned_sql: String = migration_sql
                    .lines()
                    .filter(|line| {
                        let trimmed = line.trim();
                        !trimmed.is_empty() && !trimmed.starts_with("--")
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                
                // Split by semicolon and execute each statement
                for statement in cleaned_sql.split(';') {
                    let statement = statement.trim();
                    if !statement.is_empty() {
                        debug!("Executing SQL: {}", statement);
                        sqlx::query(statement)
                            .execute(&self.pool)
                            .await
                            .with_context(|| {
                                format!("Failed to execute statement in migration {:?}: {}", 
                                    path.file_name(), statement)
                            })?;
                    }
                }
            }
        } else {
            warn!("Migrations directory not found at {:?}", migrations_dir);
        }

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
    ) -> Result<()> {
        let disconnected_at = Utc::now();

        sqlx::query(
            r#"
            UPDATE peer_connections 
            SET disconnected_at = ?,
                bytes_inbound = ?,
                bytes_outbound = ?
            WHERE connection_id = ?
            AND disconnected_at IS NULL
            "#,
        )
        .bind(disconnected_at.to_rfc3339())
        .bind(bytes_inbound as i64)
        .bind(bytes_outbound as i64)
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
                   bytes_inbound, bytes_outbound
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
                   bytes_inbound, bytes_outbound
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
                COALESCE(SUM(bytes_outbound), 0) as total_bytes_outbound
            FROM peer_connections
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to fetch connection stats")?;

        // Count messages from the messages table
        let message_row = sqlx::query(
            r#"
            SELECT 
                COUNT(CASE WHEN direction = 'inbound' THEN 1 END) as total_messages_inbound,
                COUNT(CASE WHEN direction = 'outbound' THEN 1 END) as total_messages_outbound
            FROM messages
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to fetch message stats")?;

        Ok(ConnectionStats {
            total_connections: row.get::<i64, _>("total_connections") as u64,
            active_connections: row.get::<i64, _>("active_connections") as u64,
            total_bytes_inbound: row.get::<i64, _>("total_bytes_inbound") as u64,
            total_bytes_outbound: row.get::<i64, _>("total_bytes_outbound") as u64,
            total_messages_inbound: message_row.get::<i64, _>("total_messages_inbound") as u64,
            total_messages_outbound: message_row.get::<i64, _>("total_messages_outbound") as u64,
        })
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
        let timestamp = Utc::now();
        
        let result = sqlx::query(
            r#"
            INSERT INTO messages 
            (connection_id, timestamp, direction, source_peer, destination_peer, message_type, payload_size, description)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(connection_id as i64)
        .bind(timestamp.to_rfc3339())
        .bind(direction)
        .bind(source_peer)
        .bind(destination_peer)
        .bind(message_type)
        .bind(payload_size as i64)
        .bind(description)
        .execute(&self.pool)
        .await
        .context("Failed to record message")?;

        Ok(result.last_insert_rowid())
    }

    /// Get all messages for a specific connection
    pub async fn get_messages_by_connection(&self, connection_id: u64) -> Result<Vec<Message>> {
        let rows = sqlx::query(
            r#"
            SELECT id, connection_id, timestamp, direction, source_peer, destination_peer, 
                   message_type, payload_size, description
            FROM messages
            WHERE connection_id = ?
            ORDER BY timestamp ASC
            "#,
        )
        .bind(connection_id as i64)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch messages by connection")?;

        let messages = rows
            .into_iter()
            .map(|row| {
                let timestamp_str: String = row.get("timestamp");
                let message_type_str: String = row.get("message_type");
                Message {
                    id: row.get("id"),
                    connection_id: row.get::<i64, _>("connection_id") as u64,
                    timestamp: DateTime::parse_from_rfc3339(&timestamp_str)
                        .unwrap()
                        .with_timezone(&Utc),
                    direction: row.get("direction"),
                    source_peer: row.get("source_peer"),
                    destination_peer: row.get("destination_peer"),
                    message_type: MessageType::from_command(&message_type_str),
                    payload_size: row.get::<i64, _>("payload_size") as u64,
                    description: row.get("description"),
                }
            })
            .collect();

        Ok(messages)
    }

    /// Get recent messages across all connections (limited to last N messages)
    pub async fn get_recent_messages(&self, limit: i64) -> Result<Vec<Message>> {
        let rows = sqlx::query(
            r#"
            SELECT id, connection_id, timestamp, direction, source_peer, destination_peer, 
                   message_type, payload_size, description
            FROM messages
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch recent messages")?;

        let messages = rows
            .into_iter()
            .map(|row| {
                let timestamp_str: String = row.get("timestamp");
                let message_type_str: String = row.get("message_type");
                Message {
                    id: row.get("id"),
                    connection_id: row.get::<i64, _>("connection_id") as u64,
                    timestamp: DateTime::parse_from_rfc3339(&timestamp_str)
                        .unwrap()
                        .with_timezone(&Utc),
                    direction: row.get("direction"),
                    source_peer: row.get("source_peer"),
                    destination_peer: row.get("destination_peer"),
                    message_type: MessageType::from_command(&message_type_str),
                    payload_size: row.get::<i64, _>("payload_size") as u64,
                    description: row.get("description"),
                }
            })
            .collect();

        Ok(messages)
    }

    /// Get messages by peer address (either source or destination)
    pub async fn get_messages_by_peer(&self, peer_addr: &str) -> Result<Vec<Message>> {
        let rows = sqlx::query(
            r#"
            SELECT id, connection_id, timestamp, direction, source_peer, destination_peer, 
                   message_type, payload_size, description
            FROM messages
            WHERE source_peer = ? OR destination_peer = ?
            ORDER BY timestamp DESC
            "#,
        )
        .bind(peer_addr)
        .bind(peer_addr)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch messages by peer")?;

        let messages = rows
            .into_iter()
            .map(|row| {
                let timestamp_str: String = row.get("timestamp");
                let message_type_str: String = row.get("message_type");
                Message {
                    id: row.get("id"),
                    connection_id: row.get::<i64, _>("connection_id") as u64,
                    timestamp: DateTime::parse_from_rfc3339(&timestamp_str)
                        .unwrap()
                        .with_timezone(&Utc),
                    direction: row.get("direction"),
                    source_peer: row.get("source_peer"),
                    destination_peer: row.get("destination_peer"),
                    message_type: MessageType::from_command(&message_type_str),
                    payload_size: row.get::<i64, _>("payload_size") as u64,
                    description: row.get("description"),
                }
            })
            .collect();

        Ok(messages)
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

