use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct PeerConfig {
    pub peers: HashMap<u8, String>,
}

impl PeerConfig {
    pub fn remove_peer(&mut self, id: u8) {
        self.peers.remove(&id);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub id: u8,
    pub host: String,
    pub port: u16,
    pub peers: HashMap<u8, String>,
}

impl ServerConfig {
    pub fn new(id: u8, host: String, port: u16, peers: HashMap<u8, String>) -> Self {
        Self {
            id,
            host,
            port,
            peers,
        }
    }

    pub fn from_str(id: u8, peer_config_str: &str) -> std::io::Result<Self> {
        let mut peers: PeerConfig = serde_yaml::from_str(peer_config_str)
            .map_err(|e| Error::new(ErrorKind::NotFound, e))?;

        let host_port = peers
            .peers
            .get(&id)
            .expect("Malformed peer configuration file.")
            .split(':')
            .collect::<Vec<&str>>();
        let host = host_port[0].to_string();
        let port = host_port[1]
            .parse::<u16>()
            .expect("Malformed peer configuration file.");

        peers.remove_peer(id);

        Ok(Self {
            id,
            host,
            port,
            peers: peers.peers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // test that the configuration can be parsed from a YAML string.
    #[test]
    fn test_config_from_yaml() {
        let yaml = r#"
            peers:
                1: 127.0.0.1:8080
                2: 127.0.0.1:8081
                3: 127.0.0.1:8082
                4: 127.0.0.1:8083
                5: 127.0.0.1:8084
        "#;
        let config: ServerConfig = ServerConfig::from_str(1, yaml).unwrap();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert_eq!(config.peers.len(), 4);
    }
}
