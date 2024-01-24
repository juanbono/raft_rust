use super::state::RaftStateType;
use crate::{
    raft::{message::RaftMessage, state::RaftState},
    RpcApiClient,
};
use jsonrpsee::http_client::HttpClientBuilder;
use rand::{self, Rng};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::info;

const ELECTION_TIMEOUT_MIN: u64 = 15;
const ELECTION_TIMEOUT_MAX: u64 = 30;
const MAJORITY_QUORUM: u8 = 2;

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

    /// The main loop of the Raft actor.
    /// It continually polls the receiver channel for new messages.
    /// If no message is received for `rand(ELECTION_TIMEOUT_MIN, ELECTION_TIMEOUT_MAX)` seconds, it will start a new election.
    pub async fn run(mut actor: RaftActor) {
        let mut timeout_time;
        loop {
            // generate a new waiting_time
            timeout_time = {
                let mut rng = rand::thread_rng();
                let timeout = rng.gen_range(ELECTION_TIMEOUT_MIN..ELECTION_TIMEOUT_MAX);
                tokio::time::Duration::from_millis(timeout)
            };
            tracing::info!("timeout for {:?}", timeout_time);
            match tokio::time::timeout(timeout_time, actor.receiver.recv()).await {
                Ok(Some(message)) => {
                    tracing::info!("Received message: {:?}", message);
                    actor.handle_message(message);
                }
                Ok(None) | Err(_) => {
                    tracing::info!("Timeout, starting new election");
                    // wait for the election to finish at least a second
                    // TODO: use a proper timeout
                    let election_timeout = tokio::time::Duration::from_secs(1);
                    let _ = tokio::time::timeout(election_timeout, actor.start_election()).await;
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
                    let _ = respond_to.send(false);
                    return;
                }
                let log_is_consistent = self.state.log.len() as u64 == last_log_index;
                // TODO: see what we store in the logs and use it to check the term
                // && self.state.log[last_log_index as usize] == last_log_term;
                if self.state.voted_for.is_none() || log_is_consistent {
                    info!("Voting for candidate: {}", candidate_id);
                    self.state.voted_for = Some(candidate_id);
                    self.state.current_term = term;
                    let _ = respond_to.send(true);
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

    /// Start a new election
    /// - change state to candidate
    /// - increment current term by 1
    /// - vote for self
    /// - send RequestVote RPCs to all other servers
    ///
    async fn start_election(&mut self) {
        // change state to candidate
        self.state.state_type = RaftStateType::Candidate;
        // increment current term by 1
        self.state.current_term += 1;
        // vote for self
        self.state.voted_for = Some(self.peer_id.to_string());

        let mut joinset = tokio::task::JoinSet::new();

        let mut positive_votes = 0;

        for (_peer_id, peer_host) in self.peers.iter() {
            let host = peer_host.clone();
            let term = self.state.current_term;
            let candidate_id = self.peer_id.to_string();
            let last_log_index = self.state.log.len() as u64;
            // TODO: this should be the last log term
            let last_log_term = self.state.current_term;
            joinset.spawn(async move {
                let client = HttpClientBuilder::default()
                    .build(format!("http://{}", host))
                    .unwrap();

                // send requestVote to the peer. If the peer is down, it will return false
                client
                    .request_vote(term, candidate_id, last_log_index, last_log_term)
                    .await
                    .unwrap_or(false)
            });
        }
        while let Some(vote_result) = joinset.join_next().await {
            match vote_result {
                Ok(true) => {
                    positive_votes += 1;
                }
                _ => {}
            }
        }

        info!("Positive votes: {}", positive_votes);
        if positive_votes >= MAJORITY_QUORUM {
            info!("Elected leader");
        }
    }
}
