use jsonrpsee::{core::RpcResult, proc_macros::rpc};

use crate::raft::LogEntry;

#[rpc(server, client, namespace = "kv")]
pub trait RpcApi {
    // KV RPCs
    #[method(name = "version")]
    fn version(&self) -> RpcResult<String>;

    #[method(name = "get")]
    fn get(&self, key: String) -> RpcResult<Option<String>>;

    #[method(name = "set")]
    fn set(&self, key: String, value: String) -> RpcResult<()>;

    #[method(name = "remove")]
    fn remove(&self, key: String) -> RpcResult<()>;

    // Raft RPCs
    #[method(name = "appendEntries")]
    async fn append_entries(
        &self,
        term: u64,
        leader_id: u8,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<LogEntry>,
        leader_commit: u64,
    ) -> RpcResult<bool>;

    #[method(name = "requestVote")]
    async fn request_vote(
        &self,
        term: u64,
        candidate_id: String,
        last_log_index: u64,
        last_log_term: u64,
    ) -> RpcResult<bool>;

    // Test RPCs
    #[method(name = "raft_state")]
    async fn raft_state(&self) -> RpcResult<String>;
}
