use tokio::sync::oneshot;

#[derive(Debug)]
pub enum RaftMessage {
    AppendEntries {
        respond_to: oneshot::Sender<bool>,
        term: u64,
        leader_id: String,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<String>,
        leader_commit: u64,
    },
    RequestVote {
        respond_to: oneshot::Sender<bool>,
        term: u64,
        candidate_id: String,
        last_log_index: u64,
        last_log_term: u64,
    },
}

impl RaftMessage {
    pub fn new_heartbeat(
        respond_to: oneshot::Sender<bool>,
        term: u64,
        leader_id: String,
        prev_log_index: u64,
        prev_log_term: u64,
        leader_commit: u64,
    ) -> Self {
        RaftMessage::AppendEntries {
            respond_to,
            term,
            leader_id,
            prev_log_index,
            prev_log_term,
            entries: vec![],
            leader_commit,
        }
    }
}
