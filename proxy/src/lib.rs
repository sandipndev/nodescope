mod config;
pub use config::ProxyConfig;

use app::NodeScopeApp;

pub async fn run(config: ProxyConfig, _app: NodeScopeApp) -> anyhow::Result<()> {
    println!("Proxy running on port {}", config.port);
    Ok(())
}
