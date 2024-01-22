use crate::{raft::RaftActorHandle, RpcApiServer};
use jsonrpsee::core::{async_trait, RpcResult};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::kv_error;

pub struct RpcBackend {
    raft_actor_handle: RaftActorHandle,
    kv: Arc<Mutex<HashMap<String, String>>>,
}

impl RpcBackend {
    pub fn new(peer_id: u8, peers: HashMap<u8, String>) -> Self {
        let raft_actor_handle = RaftActorHandle::new(peer_id, peers);
        RpcBackend {
            raft_actor_handle,
            kv: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
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

    async fn append_entries(
        &self,
        term: u64,
        leader_id: String,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<String>,
        leader_commit: u64,
    ) -> RpcResult<bool> {
        let result = self
            .raft_actor_handle
            .append_entries(
                term,
                leader_id,
                prev_log_index,
                prev_log_term,
                entries,
                leader_commit,
            )
            .await;
        Ok(result)
    }

    async fn request_vote(
        &self,
        term: u64,
        candidate_id: String,
        last_log_index: u64,
        last_log_term: u64,
    ) -> RpcResult<bool> {
        let result = self
            .raft_actor_handle
            .request_vote(term, candidate_id, last_log_index, last_log_term)
            .await;

        Ok(result)
    }
}
