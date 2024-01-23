use std::collections::HashMap;

use super::{actor::RaftActor, message::RaftMessage, state::RaftStateType};
use tokio::sync::{mpsc, oneshot};

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
        let actor = RaftActor::new(peer_id, peers, sender.clone(), receiver);
        tokio::task::spawn(RaftActor::run(actor));

        Self { sender }
    }

    /// Get the current state of the Raft actor
    pub async fn raft_state_type(&self) -> RaftStateType {
        let (sender, receiver) = oneshot::channel();
        let message = RaftMessage::GetRaftStateType { respond_to: sender };

        self.sender.send(message).await.unwrap();
        receiver.await.expect("Actor task has been killed")
    }

    pub async fn append_entries(
        &self,
        term: u64,
        leader_id: String,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<String>,
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

        self.sender.send(message).await.unwrap();
        receiver.await.expect("Actor task has been killed")
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

        self.sender.send(message).await.unwrap();
        receiver.await.expect("Actor task has been killed")
    }
}
