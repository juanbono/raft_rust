use jsonrpsee::{core::RpcResult, proc_macros::rpc};

#[rpc(server, client, namespace = "consensus")]
pub trait ConsensusRpc {
    #[method(name = "test")]
    fn test(&self) -> RpcResult<String>;
}

pub struct RaftRpcBackend {}

impl RaftRpcBackend {
    pub fn new() -> Self {
        RaftRpcBackend {}
    }
}

impl ConsensusRpcServer for RaftRpcBackend {
    fn test(&self) -> RpcResult<String> {
        Ok("test passed".into())
    }
}
