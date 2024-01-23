use jsonrpsee::{core::RpcResult, proc_macros::rpc};

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
        leader_id: String,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<String>,
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
    #[method(name = "echo")]
    async fn echo(&self, peer_id: u8, message: String) -> RpcResult<String>;

    #[method(name = "raft_state")]
    async fn raft_state(&self) -> RpcResult<String>;
}
