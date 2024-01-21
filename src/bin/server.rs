use anyhow::Error;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use raft_kv::{RpcApiServer, RpcBackend};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    initialize_logs();
    let port = 1234;
    let handle = start_server(port).await;

    match handle {
        Ok(handle) => {
            info!("Server started and listening on port: {}", port);
            handle.stopped().await;
        }
        Err(e) => error!("An error has occurred while starting the server: {}", e),
    }
}

async fn start_server(port: u16) -> Result<ServerHandle, Error> {
    let server = ServerBuilder::default()
        .build(format!("127.0.0.1:{}", port))
        .await?;
    let server_handle = server.start(RpcBackend::new().into_rpc());

    Ok(server_handle)
}

#[inline]
fn initialize_logs() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
