//! This is the RPC module. It contains the RPC server and the RPC client.

use jsonrpsee::RpcModule;

use self::{
    consensus::{ConsensusRpcServer, RaftRpcBackend},
    kv::{KvRpcBackend, KvRpcServer},
};

pub mod consensus;
pub mod kv;

/// Returns a `RpcModule` that contains the methods for both the consensus layer and the KV store.
pub fn build_rpc() -> Result<RpcModule<()>, anyhow::Error> {
    let mut rpc_module = RpcModule::new(());
    rpc_module.merge(KvRpcBackend::new().into_rpc())?;
    rpc_module.merge(RaftRpcBackend::new().into_rpc())?;

    Ok(rpc_module)
}
