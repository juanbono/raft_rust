use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use jsonrpsee::{core::RpcResult, proc_macros::rpc, types::ErrorObjectOwned};

#[rpc(server, client, namespace = "kv")]
pub trait KvRpc {
    #[method(name = "version")]
    fn version(&self) -> RpcResult<String>;

    #[method(name = "get")]
    fn get(&self, key: String) -> RpcResult<Option<String>>;

    #[method(name = "set")]
    fn set(&self, key: String, value: String) -> RpcResult<()>;

    #[method(name = "remove")]
    fn remove(&self, key: String) -> RpcResult<()>;
}

pub struct KvRpcBackend {
    kv: Arc<Mutex<HashMap<String, String>>>,
}

impl KvRpcBackend {
    pub fn new() -> Self {
        KvRpcBackend {
            kv: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl KvRpcServer for KvRpcBackend {
    fn version(&self) -> RpcResult<String> {
        Ok("0.1.0".into())
    }

    fn get(&self, key: String) -> RpcResult<Option<String>> {
        let kv = self.kv.lock().unwrap();
        match kv.get(&key) {
            Some(value) => Ok(Some(value.clone())),
            None => RpcResult::Err(kv_error(format!("Key not found: {}", key))),
        }
    }

    fn set(&self, key: String, value: String) -> RpcResult<()> {
        let mut kv = self.kv.lock().unwrap();
        // insert into the kv if it exists or not
        kv.entry(key).or_insert(value);

        Ok(())
    }

    fn remove(&self, key: String) -> RpcResult<()> {
        let mut kv = self.kv.lock().unwrap();
        match kv.remove(&key) {
            Some(_) => Ok(()),
            None => RpcResult::Err(kv_error(format!("Key not found: {}", key))),
        }
    }
}

fn kv_error(message: String) -> ErrorObjectOwned {
    ErrorObjectOwned::owned(0, message, None as Option<bool>)
}
