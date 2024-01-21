use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .expect("setting default subscriber failed");
    let client = HttpClientBuilder::default()
        .build("http://localhost:1234")
        .unwrap();

    let response: String = client.request("kv_version", rpc_params![]).await.unwrap();
    info!("response: {:?}", response);

    let response: String = client
        .request("consensus_test", rpc_params![])
        .await
        .unwrap();
    info!("response: {:?}", response);
}
