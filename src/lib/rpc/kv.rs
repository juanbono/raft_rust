use jsonrpsee::{core::RpcResult, proc_macros::rpc};

#[rpc(server, client, namespace = "kv")]
pub trait KvRpc {
    #[method(name = "version")]
    fn version(&self) -> RpcResult<String>;
}

pub struct KvRpcBackend {}

impl KvRpcBackend {
    pub fn new() -> Self {
        KvRpcBackend {}
    }
}

impl KvRpcServer for KvRpcBackend {
    fn version(&self) -> RpcResult<String> {
        Ok("0.1.0".into())
    }
}
