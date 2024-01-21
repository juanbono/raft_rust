use crate::raft::{message::RaftMessage, state::RaftState};
use tokio::sync::{mpsc, oneshot};

pub struct RaftActor {
    receiver: mpsc::Receiver<RaftMessage>,
    state: RaftState,
}

impl RaftActor {
    pub fn new(receiver: mpsc::Receiver<RaftMessage>) -> Self {
        RaftActor {
            receiver,
            state: RaftState::new(),
        }
    }

    async fn run(mut actor: RaftActor) {
        while let Some(message) = actor.receiver.recv().await {
            actor.handle_message(message);
        }
    }

    fn handle_message(&mut self, message: RaftMessage) {
        match message {
            RaftMessage::AppendEntries { respond_to, .. } => {
                // TODO: Add the real implementation

                respond_to.send(false).unwrap();
            }
            RaftMessage::RequestVote { respond_to, .. } => {
                // TODO: Add the real implementation

                respond_to.send(true).unwrap();
            }
        }
    }
}

#[derive(Clone)]
pub struct RaftActorHandle {
    sender: mpsc::Sender<RaftMessage>,
}

impl RaftActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(32);
        let actor = RaftActor::new(receiver);
        tokio::spawn(RaftActor::run(actor));

        Self { sender }
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
