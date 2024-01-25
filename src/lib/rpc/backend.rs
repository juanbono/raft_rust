use super::kv_error;
use crate::raft::{KvCommand, KvResponse, LogEntry, RaftActorHandle, RaftStateType};
use crate::{RpcApiClient, RpcApiServer};
use jsonrpsee::core::{async_trait, RpcResult};
use jsonrpsee::http_client::HttpClientBuilder;
use std::collections::HashMap;
use std::time::Duration;

pub struct RpcBackend {
    raft_actor_handle: RaftActorHandle,
}

impl RpcBackend {
    pub fn new(peer_id: u8, peers: HashMap<u8, String>) -> Self {
        let raft_actor_handle = RaftActorHandle::new(peer_id, peers.clone());
        RpcBackend { raft_actor_handle }
    }

    async fn send_command_to_leader(&self, command: KvCommand) -> RpcResult<KvResponse> {
        let (leader_id, leader_host) = self
            .raft_actor_handle
            .current_leader()
            .await
            .ok_or(kv_error("protocol error: follower without leader".into()))?;

        let (prev_log_index, prev_log_term) =
            self.raft_actor_handle.last_log_index_and_term().await;

        // get term,leader_commit
        let term = 0;
        let leader_commit = 0;
        let log_entry = LogEntry::new(term, command.clone());

        // send the command to the leader
        let client = HttpClientBuilder::default()
            .request_timeout(Duration::from_millis(3000))
            .build(format!("http://{}", leader_host))
            .map_err(|_| kv_error("cannot create client for the leader".to_string()))?;

        let leader_response = client
            .append_entries(
                term,
                leader_id,
                prev_log_index,
                prev_log_term,
                vec![log_entry],
                leader_commit,
            )
            .await
            .map_err(|_| kv_error("cannot send appendEntries to the leader".to_string()))?;

        // apply the command if the leader accepted the new entries
        if leader_response {
            RpcResult::Ok(self.raft_actor_handle.apply_command(command).await)
        } else {
            RpcResult::Err(kv_error("Failed to send command to leader".into()))
        }
    }
}

#[async_trait]
impl RpcApiServer for RpcBackend {
    async fn get(&self, key: String) -> RpcResult<Option<String>> {
        match self.raft_actor_handle.raft_state_type().await {
            RaftStateType::Follower => {
                let command = KvCommand::Get { key };
                match self.send_command_to_leader(command).await {
                    Ok(KvResponse::Get(Some(value))) => RpcResult::Ok(Some(value)),
                    _ => RpcResult::Err(kv_error("Failed to get value".into())),
                }
            }
            RaftStateType::Leader => {
                // if the state is leader, then we can just get the value from the kv
                // also we need to append the KvCommand to the log
                let response = self
                    .raft_actor_handle
                    .apply_and_broadcast(KvCommand::Get {
                        key: key.clone(),
                    })
                    .await;

                match response {
                    KvResponse::Get(Some(value)) => RpcResult::Ok(Some(value)),
                    _ => RpcResult::Err(kv_error("Failed to get value".into())),
                }
            }
            RaftStateType::Candidate => {
                // if the state is candidate, return an error
                RpcResult::Err(kv_error(
                    "Cannot get value from candidate state".to_string(),
                ))
            }
        }
    }

    async fn set(&self, key: String, value: String) -> RpcResult<()> {
        match self.raft_actor_handle.raft_state_type().await {
            RaftStateType::Follower => {
                let command = KvCommand::Set { key, value };
                match self.send_command_to_leader(command).await {
                    Ok(KvResponse::Set(true)) => RpcResult::Ok(()),
                    _ => RpcResult::Err(kv_error("Failed to set value".into())),
                }
            }
            RaftStateType::Leader => {
                // if the state is leader, then we can just set the value in the kv
                // also we need to append the KvCommand to the log and broadcast
                self.raft_actor_handle
                    .apply_and_broadcast(KvCommand::Set {
                        key: key.clone(),
                        value: value.clone(),
                    })
                    .await;
                Ok(())
            }
            RaftStateType::Candidate => {
                // if the state is candidate, return an error
                RpcResult::Err(kv_error(
                    "Cannot get value from candidate state".to_string(),
                ))
            }
        }
    }

    async fn remove(&self, key: String) -> RpcResult<()> {
        match self.raft_actor_handle.raft_state_type().await {
            RaftStateType::Follower => {
                let command = KvCommand::Remove { key };
                match self.send_command_to_leader(command).await {
                    Ok(KvResponse::Remove(true)) => RpcResult::Ok(()),
                    _ => RpcResult::Err(kv_error("Failed to remove".into())),
                }
            }
            RaftStateType::Leader => {
                // if the state is leader, then we can just get the value from the kv
                // also we need to append the KvCommand to the log
                todo!();
            }
            RaftStateType::Candidate => {
                // if the state is candidate, return an error
                RpcResult::Err(kv_error(
                    "Cannot get value from candidate state".to_string(),
                ))
            }
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

    fn version(&self) -> RpcResult<String> {
        Ok("0.1.0".into())
    }
}
