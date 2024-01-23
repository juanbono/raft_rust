use std::{collections::HashMap, time::Duration};

use crate::raft::{message::RaftMessage, state::RaftState};
use tokio::sync::mpsc;
use super::state::RaftStateType;

pub struct RaftActor {
    peer_id: u8,
    peers: HashMap<u8, String>,
    self_: mpsc::Sender<RaftMessage>,
    receiver: mpsc::Receiver<RaftMessage>,
    state: RaftState,
}

impl RaftActor {
    pub fn new(
        peer_id: u8,
        peers: HashMap<u8, String>,
        self_: mpsc::Sender<RaftMessage>,
        receiver: mpsc::Receiver<RaftMessage>,
    ) -> Self {
        RaftActor {
            peer_id,
            peers,
            self_,
            receiver,
            state: RaftState::new(),
        }
    }

    pub async fn run(mut actor: RaftActor) {
        loop {
            match tokio::time::timeout(Duration::from_secs(10), actor.receiver.recv()).await {
                Ok(Some(message)) => {
                    tracing::info!("Received message: {:?}", message);
                    actor.handle_message(message);
                }
                Ok(None) | Err(_) => {
                    tracing::info!("Timeout, starting new election");
                    actor.start_election();
                }
            }
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
                // reply false if term < currentTerm (§5.1)
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
            RaftMessage::GetRaftStateType { respond_to } => {
                respond_to.send(self.state.state_type).unwrap();
            }

            RaftMessage::Timeout => {
                // TODO: start election after timeout
                tracing::info!("Received: Timeout");
            }
        }
    }

    fn start_election(&mut self) {
        // change state to candidate
        self.state.state_type = RaftStateType::Candidate;
        // increment current term by 1
        self.state.current_term += 1;
        // vote for self
        self.state.voted_for = Some(self.peer_id.to_string());

        // TODO: send RequestVote RPCs to all other servers
    }
}
