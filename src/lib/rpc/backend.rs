use crate::{
    raft::{LogEntry, RaftActorHandle, RaftStateType},
    RpcApiClient, RpcApiServer,
};
use jsonrpsee::core::{async_trait, RpcResult};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tracing::info;

use super::kv_error;

pub struct RpcBackend {
    raft_actor_handle: RaftActorHandle,
    // TODO: remove
    kv: Arc<Mutex<HashMap<String, String>>>,
}

impl RpcBackend {
    pub fn new(peer_id: u8, peers: HashMap<u8, String>) -> Self {
        let raft_actor_handle = RaftActorHandle::new(peer_id, peers.clone());
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

    async fn get(&self, key: String) -> RpcResult<Option<String>> {
        match self.raft_actor_handle.raft_state_type().await {
            RaftStateType::Follower => {
                // if the state is follower, then we need to redirect the request to the current leader
                let current_leader_id = self.raft_actor_handle.current_leader_id().await.unwrap();
                // create the client
                // send an appendEntries with the request (or make a new message type?)
                todo!();
            },
            RaftStateType::Leader => {
                // if the state is leader, then we can just get the value from the kv
                // also we need to append the KvCommand to the log
                todo!();
            },
            RaftStateType::Candidate => {
                // if the state is candidate, then we respond with an error
                RpcResult::Err(kv_error(format!("Cannot get value from candidate state")))
            },
        }
    }

    async fn set(&self, key: String, value: String) -> RpcResult<()> {
        match self.raft_actor_handle.raft_state_type().await {
            RaftStateType::Follower => {
                // if the state is follower, then we need to redirect the request to the current leader
                let current_leader_id = self.raft_actor_handle.current_leader_id().await.unwrap();
                // create the client
                // send an appendEntries with the request (or make a new message type?)
                todo!();
            },
            RaftStateType::Leader => {
                // if the state is leader, then we can just get the value from the kv
                // also we need to append the KvCommand to the log
                todo!();
            },
            RaftStateType::Candidate => {
                // if the state is candidate, then we respond with an error
                RpcResult::Err(kv_error(format!("Cannot get value from candidate state")))
            },
        }
    }

    async fn remove(&self, key: String) -> RpcResult<()> {
        match self.raft_actor_handle.raft_state_type().await {
            RaftStateType::Follower => {
                // if the state is follower, then we need to redirect the request to the current leader
                let current_leader_id = self.raft_actor_handle.current_leader_id().await.unwrap();
                // create the client
                // send an appendEntries with the request (or make a new message type?)
                todo!();
            },
            RaftStateType::Leader => {
                // if the state is leader, then we can just get the value from the kv
                // also we need to append the KvCommand to the log
                todo!();
            },
            RaftStateType::Candidate => {
                // if the state is candidate, then we respond with an error
                RpcResult::Err(kv_error(format!("Cannot get value from candidate state")))
            },
        }
    }

    async fn append_entries(
        &self,
        term: u64,
        leader_id: u8,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<LogEntry>,
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

    async fn raft_state(&self) -> RpcResult<String> {
        let state = self.raft_actor_handle.raft_state_type().await;
        Ok(format!("{:?}", state).to_ascii_lowercase())
    }
}
