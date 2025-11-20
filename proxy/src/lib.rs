mod bitcoin_protocol;
mod config;
mod connection;
mod socks5;

pub use config::ProxyConfig;

use app::NodeScopeApp;
use connection::ConnectionHandler;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info, warn};

/// Bitcoin P2P Proxy Server
pub struct ProxyServer {
    config: ProxyConfig,
    app: NodeScopeApp,
    connection_counter: Arc<AtomicU64>,
}

impl ProxyServer {
    pub fn new(config: ProxyConfig, app: NodeScopeApp) -> Self {
        Self {
            config,
            app,
            connection_counter: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Start the proxy server
    pub async fn start(&self) -> anyhow::Result<()> {
        let bind_addr = format!("0.0.0.0:{}", self.config.port);
        let listener = TcpListener::bind(&bind_addr).await?;

        info!(
            "Bitcoin SOCKS5 Proxy listening on {} (network: {:?})",
            bind_addr, self.config.network
        );

        let network: bitcoin_protocol::Network = self.config.network.into();
        let app = self.app.clone();

        loop {
            match listener.accept().await {
                Ok((client_stream, client_addr)) => {
                    let connection_id = self.connection_counter.fetch_add(1, Ordering::SeqCst);

                    info!(
                        "[conn:{}] New connection from {}",
                        connection_id, client_addr
                    );

                    // Spawn a task to handle this connection
                    let app_clone = app.clone();
                    tokio::spawn(async move {
                        if let Err(e) =
                            handle_connection(connection_id, client_stream, network, app_clone).await
                        {
                            error!("[conn:{}] Connection error: {}", connection_id, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

/// Handle a single SOCKS5 proxied connection
async fn handle_connection(
    connection_id: u64,
    mut client_stream: TcpStream,
    network: bitcoin_protocol::Network,
    app: NodeScopeApp,
) -> anyhow::Result<()> {
    let client_addr = client_stream.peer_addr()?.to_string();

    // Handle SOCKS5 handshake
    let socks5_req = socks5::handle_socks5_handshake(&mut client_stream, connection_id).await?;
    let target = socks5_req.to_string();

    // Connect to the requested target
    let target_stream = match TcpStream::connect(&target).await {
        Ok(stream) => stream,
        Err(e) => {
            warn!(
                "[conn:{}] Failed to connect to target {}: {}",
                connection_id, target, e
            );
            return Err(e.into());
        }
    };

    // Create and run the connection handler
    let handler = ConnectionHandler::new(connection_id, client_addr, target, network, app);
    handler.handle(client_stream, target_stream).await
}

/// Run the proxy server (public API)
pub async fn run(config: ProxyConfig, app: NodeScopeApp) -> anyhow::Result<()> {
    let server = ProxyServer::new(config, app);
    server.start().await
}
