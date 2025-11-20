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
    pub messages_inbound: Option<u64>,
    pub messages_outbound: Option<u64>,
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
            messages_inbound: conn.messages_inbound,
            messages_outbound: conn.messages_outbound,
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
