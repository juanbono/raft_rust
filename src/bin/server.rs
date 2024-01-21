use anyhow::Error;
use jsonrpsee::{
    server::{ServerBuilder, ServerHandle},
    RpcModule,
};
use raft_kv::rpc::{
    consensus::{ConsensusRpcServer, RaftRpcBackend},
    kv::{KvRpcBackend, KvRpcServer},
};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let port = 1234;
    let handle = start_server(port).await;

    match handle {
        Ok(handle) => {
            info!("Server started and listening on port: {}", port);
            handle.stopped().await;
        }
        Err(e) => warn!("An error has occurred while starting the server: {}", e),
    }
}

async fn start_server(port: u16) -> Result<ServerHandle, Error> {
    let server = ServerBuilder::default()
        .build(format!("127.0.0.1:{}", port))
        .await?;

    let mut rpc_module = RpcModule::new(());

    rpc_module
        .merge(KvRpcBackend::new().into_rpc())
        .expect("Cant' initialize KV RPC module");

    rpc_module
        .merge(RaftRpcBackend::new().into_rpc())
        .expect("Cant' initialize Consensus RPC module");

    let server_handle = server.start(rpc_module);

    Ok(server_handle)
}
