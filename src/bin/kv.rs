use jsonrpsee::http_client::HttpClientBuilder;
use raft_kv::RpcApiClient;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    initialize_logs();
    // TODO: read from args or env
    let client = HttpClientBuilder::default().build("http://127.0.0.1:8080")?;

    let response: String = client.version().await?;
    info!("response: {:?}", response);

    let response: bool = client.request_vote(0, "".into(), 0, 0).await?;
    info!("response: {:?}", response);

    let response: bool = client.append_entries(0, "".into(), 0, 0, vec![], 0).await?;
    info!("response: {:?}", response);

    Ok(())
}

#[inline]
fn initialize_logs() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
