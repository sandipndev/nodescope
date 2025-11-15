mod config;
pub use config::ProxyConfig;

use app::NodeScopeApp;

pub async fn run(_config: ProxyConfig, _app: NodeScopeApp) -> anyhow::Result<()> {
    println!("Proxy is running...");
    Ok(())
}
