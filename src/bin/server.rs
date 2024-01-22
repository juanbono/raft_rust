use std::{collections::HashMap, env};

use anyhow::Error;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use raft_kv::{RpcApiServer, RpcBackend, ServerConfig};
use tokio::net::ToSocketAddrs;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    initialize_logs();
    let config = read_config();
    let handle = start_server(config.id, config.peers, (config.host, config.port)).await;

    match handle {
        Ok(handle) => {
            info!("Server started and listening on port: {}", config.port);
            handle.stopped().await;
        }
        Err(e) => error!("An error has occurred while starting the server: {}", e),
    }
}

async fn start_server(
    peer_id: u8,
    peers: HashMap<u8, String>,
    addr: impl ToSocketAddrs,
) -> Result<ServerHandle, Error> {
    let server = ServerBuilder::default().build(addr).await?;
    let server_handle = server.start(RpcBackend::new(peer_id, peers).into_rpc());

    Ok(server_handle)
}

#[inline]
fn read_config() -> ServerConfig {
    let id =
        env::var("PEER_ID").expect("unable to read server id (PEER_ID variable) from environment");
    let config_file = env::var("SERVER_CONFIG")
        .expect("unable to read server configuration (SERVER_CONFIG variable) from environment");

    let config_file_contents = std::fs::read_to_string(config_file)
        .expect("Unable to read server configuration file contents.");
    ServerConfig::from_str(
        id.parse::<u8>().expect("Unable to parse peer_id value"),
        &config_file_contents,
    )
    .expect("unable to parse server configuration file contents")
}

#[inline]
fn initialize_logs() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
