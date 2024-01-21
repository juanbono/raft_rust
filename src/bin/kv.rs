use jsonrpsee::http_client::HttpClientBuilder;
use raft_kv::rpc::{consensus::ConsensusRpcClient, kv::KvRpcClient};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    initialize_logs();
    let client = HttpClientBuilder::default().build("http://localhost:1234")?;

    let response: String = client.version().await?;
    info!("response: {:?}", response);

    let response: String = client.test().await?;
    info!("response: {:?}", response);

    Ok(())
}

#[inline]
fn initialize_logs() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
