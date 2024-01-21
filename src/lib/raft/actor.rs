use crate::raft::{message::RaftMessage, state::RaftState};
use tokio::sync::mpsc;

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

    pub async fn run(mut actor: RaftActor) {
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
