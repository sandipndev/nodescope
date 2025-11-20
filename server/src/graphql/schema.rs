use async_graphql::*;
use app::NodeScopeApp;

pub struct Query;

#[Object]
impl Query {
    async fn hi(&self, _ctx: &Context<'_>) -> &str {
        "Hello, World!"
    }

    /// Get all peer connections
    async fn peer_connections(&self, ctx: &Context<'_>) -> Result<Vec<PeerConnection>> {
        let app = ctx.data::<NodeScopeApp>()?;
        let connections = app.get_all_connections().await?;
        Ok(connections.into_iter().map(Into::into).collect())
    }

    /// Get active peer connections
    async fn active_connections(&self, ctx: &Context<'_>) -> Result<Vec<PeerConnection>> {
        let app = ctx.data::<NodeScopeApp>()?;
        let connections = app.get_active_connections().await?;
        Ok(connections.into_iter().map(Into::into).collect())
    }

    /// Get connection statistics
    async fn connection_stats(&self, ctx: &Context<'_>) -> Result<ConnectionStats> {
        let app = ctx.data::<NodeScopeApp>()?;
        let stats = app.get_connection_stats().await?;
        Ok(stats.into())
    }

    /// Get messages by connection ID
    async fn messages_by_connection(
        &self,
        ctx: &Context<'_>,
        connection_id: u64,
    ) -> Result<Vec<Message>> {
        let app = ctx.data::<NodeScopeApp>()?;
        let messages = app.get_messages_by_connection(connection_id).await?;
        Ok(messages.into_iter().map(Into::into).collect())
    }

    /// Get recent messages (limited)
    async fn recent_messages(&self, ctx: &Context<'_>, #[graphql(default = 100)] limit: i32) -> Result<Vec<Message>> {
        let app = ctx.data::<NodeScopeApp>()?;
        let messages = app.get_recent_messages(limit as i64).await?;
        Ok(messages.into_iter().map(Into::into).collect())
    }

    /// Get messages by peer address
    async fn messages_by_peer(&self, ctx: &Context<'_>, peer_addr: String) -> Result<Vec<Message>> {
        let app = ctx.data::<NodeScopeApp>()?;
        let messages = app.get_messages_by_peer(&peer_addr).await?;
        Ok(messages.into_iter().map(Into::into).collect())
    }
}

/// GraphQL representation of a peer connection
#[derive(SimpleObject)]
pub struct PeerConnection {
    pub id: i64,
    pub connection_id: u64,
    pub client_addr: String,
    pub target_addr: String,
    pub connected_at: String,
    pub disconnected_at: Option<String>,
    pub bytes_inbound: Option<u64>,
    pub bytes_outbound: Option<u64>,
}

impl From<app::PeerConnection> for PeerConnection {
    fn from(conn: app::PeerConnection) -> Self {
        Self {
            id: conn.id,
            connection_id: conn.connection_id,
            client_addr: conn.client_addr,
            target_addr: conn.target_addr,
            connected_at: conn.connected_at.to_rfc3339(),
            disconnected_at: conn.disconnected_at.map(|dt| dt.to_rfc3339()),
            bytes_inbound: conn.bytes_inbound,
            bytes_outbound: conn.bytes_outbound,
        }
    }
}

/// GraphQL representation of a message
#[derive(SimpleObject)]
pub struct Message {
    pub id: i64,
    pub connection_id: u64,
    pub timestamp: String,
    pub direction: String,
    pub source_peer: String,
    pub destination_peer: String,
    pub message_type: String,
    pub payload_size: u64,
    pub description: String,
}

impl From<app::Message> for Message {
    fn from(msg: app::Message) -> Self {
        Self {
            id: msg.id,
            connection_id: msg.connection_id,
            timestamp: msg.timestamp.to_rfc3339(),
            direction: msg.direction,
            source_peer: msg.source_peer,
            destination_peer: msg.destination_peer,
            message_type: msg.message_type.as_str().to_string(),
            payload_size: msg.payload_size,
            description: msg.description,
        }
    }
}

/// GraphQL representation of connection statistics
#[derive(SimpleObject)]
pub struct ConnectionStats {
    pub total_connections: u64,
    pub active_connections: u64,
    pub total_bytes_inbound: u64,
    pub total_bytes_outbound: u64,
    pub total_messages_inbound: u64,
    pub total_messages_outbound: u64,
}

impl From<app::ConnectionStats> for ConnectionStats {
    fn from(stats: app::ConnectionStats) -> Self {
        Self {
            total_connections: stats.total_connections,
            active_connections: stats.active_connections,
            total_bytes_inbound: stats.total_bytes_inbound,
            total_bytes_outbound: stats.total_bytes_outbound,
            total_messages_inbound: stats.total_messages_inbound,
            total_messages_outbound: stats.total_messages_outbound,
        }
    }
}
