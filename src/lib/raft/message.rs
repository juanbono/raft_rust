use super::{log::LogEntry, state::RaftStateType};
use tokio::sync::oneshot;

pub enum RaftMessage {
    //===== Raft RPC messages =====
    /// Append entries request from another Raft node
    /// It serves as both a heartbeat and a log replication mechanism
    AppendEntries {
        respond_to: oneshot::Sender<bool>,
        term: u64,
        leader_id: u8,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<LogEntry>,
        leader_commit: u64,
    },
    /// Vote request from another Raft node
    RequestVote {
        respond_to: oneshot::Sender<bool>,
        term: u64,
        candidate_id: String,
        last_log_index: u64,
        last_log_term: u64,
    },

    //===== Utility messages =====
    /// Get the current Raft state type
    GetRaftStateType {
        respond_to: oneshot::Sender<RaftStateType>,
    },

    /// Get the current leader id and host, if any
    GetCurrentLeader {
        respond_to: oneshot::Sender<Option<(u8, String)>>,
    },

    /// Get the last log entry index and term
    GetLastLogIndexAndTerm {
        respond_to: oneshot::Sender<(u64, u64)>,
    },
}

impl std::fmt::Debug for RaftMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AppendEntries {
                term,
                leader_id,
                prev_log_index,
                prev_log_term,
                entries,
                leader_commit,
                ..
            } => f
                .debug_struct("AppendEntries")
                .field("term", term)
                .field("leader_id", leader_id)
                .field("prev_log_index", prev_log_index)
                .field("prev_log_term", prev_log_term)
                .field("entries", entries)
                .field("leader_commit", leader_commit)
                .finish(),
            Self::RequestVote {
                term,
                candidate_id,
                last_log_index,
                last_log_term,
                ..
            } => f
                .debug_struct("RequestVote")
                .field("term", term)
                .field("candidate_id", candidate_id)
                .field("last_log_index", last_log_index)
                .field("last_log_term", last_log_term)
                .finish(),
            Self::GetRaftStateType { .. } => f.debug_struct("GetRaftStateType").finish(),
            Self::GetCurrentLeader { .. } => f.debug_struct("GetCurrentLeader").finish(),
            Self::GetLastLogIndexAndTerm { .. } => {
                f.debug_struct("GetLastLogIndexAndTerm").finish()
            }
        }
    }
}
