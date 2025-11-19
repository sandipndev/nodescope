use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProxyConfig {
    /// Port to listen on for SOCKS5 proxy
    #[serde(default = "default_port")]
    pub port: u16,
    
    /// Bitcoin network (mainnet, testnet, signet, regtest)
    #[serde(default = "default_network")]
    pub network: NetworkConfig,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NetworkConfig {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

impl From<NetworkConfig> for bitcoin::Network {
    fn from(config: NetworkConfig) -> Self {
        match config {
            NetworkConfig::Mainnet => bitcoin::Network::Bitcoin,
            NetworkConfig::Testnet => bitcoin::Network::Testnet,
            NetworkConfig::Signet => bitcoin::Network::Signet,
            NetworkConfig::Regtest => bitcoin::Network::Regtest,
        }
    }
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            network: default_network(),
        }
    }
}

fn default_port() -> u16 {
    6788
}

fn default_network() -> NetworkConfig {
    NetworkConfig::Mainnet
}
