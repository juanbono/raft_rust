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
            RaftMessage::RequestVote {
                respond_to,
                candidate_id,
                term,
                last_log_index,
                last_log_term,
            } => {
                // Reply false if term < currentTerm (§5.1)
                if term < self.state.current_term {
                    respond_to.send(false).unwrap();
                    return;
                }
                // TODO: check logs
                let missing_check = true;
                if self.state.voted_for.is_none() || missing_check {
                    self.state.voted_for = Some(candidate_id);
                    self.state.current_term = term;
                    respond_to.send(true).unwrap();
                    return;
                }
            }
        }
    }
}
