use super::{
    actor::RaftActor, log::LogEntry, message::RaftMessage, state::RaftStateType, KvCommand,
    KvResponse,
};
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

/// A handle to a Raft actor.
/// This can be used to interact with the actor instead of sending messages directly.
#[derive(Clone)]
pub struct RaftActorHandle {
    sender: mpsc::Sender<RaftMessage>,
}

impl RaftActorHandle {
    /// Create a new Raft actor handle.
    /// This will spawn a new task to run the actor.
    ///
    /// The handle can be cloned freely and can be used to send messages to the actor.
    pub fn new(peer_id: u8, peers: HashMap<u8, String>) -> Self {
        let (sender, receiver) = mpsc::channel(32);
        let actor = RaftActor::new(peer_id, peers, receiver);
        tokio::task::spawn(RaftActor::run(actor));

        Self { sender }
    }

    /// Get the current state of the Raft actor
    pub async fn raft_state_type(&self) -> RaftStateType {
        let (sender, receiver) = oneshot::channel();
        let message = RaftMessage::GetRaftStateType { respond_to: sender };

        let _ = self.sender.send(message).await;
        receiver.await.expect("Actor task has been killed")
    }

    pub async fn append_entries(
        &self,
        term: u64,
        leader_id: u8,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<LogEntry>,
        leader_commit: u64,
    ) -> bool {
        let (sender, receiver) = oneshot::channel();
        let message = RaftMessage::AppendEntries {
            respond_to: sender,
            term,
            leader_id,
            prev_log_index,
            prev_log_term,
            entries,
            leader_commit,
        };

        let _ = self.sender.send(message).await;
        // assume that something went wrong, just return false
        receiver.await.unwrap_or(false)
    }

    pub async fn request_vote(
        &self,
        term: u64,
        candidate_id: String,
        last_log_index: u64,
        last_log_term: u64,
    ) -> bool {
        let (sender, receiver) = oneshot::channel();
        let message = RaftMessage::RequestVote {
            respond_to: sender,
            term,
            candidate_id,
            last_log_index,
            last_log_term,
        };

        // if the send fails, just return false
        if self.sender.send(message).await.is_err() {
            return false;
        }

        // assume that something went wrong, just return false
        receiver.await.unwrap_or(false)
    }

    /// Returns the current leader id and host, if there is one
    pub async fn current_leader(&self) -> Option<(u8, String)> {
        let (sender, receiver) = oneshot::channel();
        let message = RaftMessage::GetCurrentLeader { respond_to: sender };

        let _ = self.sender.send(message).await;
        receiver.await.unwrap_or(None)
    }

    pub async fn last_log_index_and_term(&self) -> (u64, u64) {
        let (sender, receiver) = oneshot::channel();
        let message = RaftMessage::GetLastLogIndexAndTerm { respond_to: sender };

        let _ = self.sender.send(message).await;
        receiver.await.expect("Actor task has been killed")
    }

    pub async fn apply_command(&self, command: KvCommand) -> KvResponse {
        let (sender, receiver) = oneshot::channel();
        let message = RaftMessage::ApplyCommand {
            respond_to: sender,
            command,
        };

        let _ = self.sender.send(message).await;
        receiver.await.expect("Actor task has been killed")
    }

    pub async fn apply_and_broadcast(&self, command: KvCommand) -> KvResponse {
        let (sender, receiver) = oneshot::channel();
        let message = RaftMessage::ApplyAndBroadcast {
            respond_to: sender,
            command,
        };

        let _ = self.sender.send(message).await;
        receiver.await.expect("Actor task has been killed")
    }
}
