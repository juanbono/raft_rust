mod raft;
mod rpc_api;
mod config;
mod utils;

pub use rpc_api::{RpcApiClient, RpcApiServer};
use jsonrpsee::core::RpcResult;
use utils::kv_error;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct RpcBackend {
    consensus: Arc<Mutex<raft::Raft>>,
    kv: Arc<Mutex<HashMap<String, String>>>,
}

impl RpcBackend {
    pub fn new() -> Self {
        RpcBackend {
            kv: Arc::new(Mutex::new(HashMap::new())),
            consensus: Arc::new(Mutex::new(raft::Raft::new())),
        }
    }
}

impl RpcApiServer for RpcBackend {
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

    fn append_entries(
        &self,
        term: u64,
        leader_id: String,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<String>,
        leader_commit: u64,
    ) -> RpcResult<bool> {
        let raft = self.consensus.lock().unwrap();
        if term < raft.current_term {
            return Ok(false);
        }

        // TODO: Rest of the impl

        Ok(true)
    }

    fn request_vote(
        &self,
        term: u64,
        candidate_id: String,
        last_log_index: u64,
        last_log_term: u64,
    ) -> RpcResult<bool> {
        let raft = self.consensus.lock().unwrap();
        if term < raft.current_term {
            return Ok(false);
        }

        // TODO: rest of the impl

        Ok(true)
    }
}

