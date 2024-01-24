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
    raft_state: RaftState,
    state: HashMap<String, String>,
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
            raft_state: RaftState::new(),
            state: HashMap::new(),
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
                // TODO: consider changing to a different timeout
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
                    // TODO: use a proper timeout, it should be a random time between 150ms and 300ms
                    let election_timeout = tokio::time::Duration::from_secs(1);
                    let _ = tokio::time::timeout(election_timeout, actor.start_election()).await;
                }
            }
        }
    }

    fn handle_message(&mut self, message: RaftMessage) {
        match message {
            RaftMessage::AppendEntries {
                respond_to,
                term,
                prev_log_index,
                prev_log_term,
                entries,
                leader_id,
                leader_commit,
            } => {
                // if term < currentTerm, reject (§5.1)
                if term < self.raft_state.current_term {
                    let _ = respond_to.send(false);
                    return;
                }

                // if log doesn't contain an entry at prevLogIndex whose term matches prevLogTerm, reject (§5.3)
                if self.raft_state.log.last_log_index() != prev_log_index
                    || self.raft_state.log.last_entry_term() != prev_log_term
                {
                    let _ = respond_to.send(false);
                    return;
                }

                self.raft_state.current_term = term;
                self.raft_state.state_type = RaftStateType::Follower;

                // TODO: implement
                // if an existing entry conflicts with a new one (same index but different terms), delete the existing entry and all that follow it (§5.3)

                // append any new entries not already in the log
                for entry in entries {
                    self.raft_state.log.add_entry(entry);
                }

                // TODO: check correctness
                // if leaderCommit > commitIndex, set commitIndex = min(leaderCommit, index of last new entry)
                if leader_commit > self.raft_state.commit_index {
                    self.raft_state.commit_index =
                        leader_commit.min(self.raft_state.log.last_log_index());
                }
            }
            RaftMessage::RequestVote {
                respond_to,
                candidate_id,
                term,
                last_log_index,
                last_log_term,
            } => {
                // reply false if term < currentTerm (§5.1)
                if term < self.raft_state.current_term {
                    let _ = respond_to.send(false);
                    return;
                }
                // TODO: check how to handle log index correctly
                let log_is_consistent = self.raft_state.log.last_log_index() == last_log_index
                    && self.raft_state.log.last_entry_term() == last_log_term;
                if self.raft_state.voted_for.is_none() || log_is_consistent {
                    info!("Voting for candidate: {}", candidate_id);
                    self.raft_state.voted_for = Some(candidate_id);
                    self.raft_state.current_term = term;
                    let _ = respond_to.send(true);
                    return;
                }
            }
            RaftMessage::GetRaftStateType { respond_to } => {
                respond_to.send(self.raft_state.state_type).unwrap();
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
        let mut positive_votes = 0;

        // change state to candidate
        self.raft_state.state_type = RaftStateType::Candidate;
        // increment current term by 1
        self.raft_state.current_term += 1;
        // vote for self
        self.raft_state.voted_for = Some(self.peer_id.to_string());
        // here we accummulate the tasks to later wait for each one
        let mut joinset = tokio::task::JoinSet::new();

        for (_peer_id, peer_host) in self.peers.iter() {
            let host = peer_host.clone();
            let term = self.raft_state.current_term;
            let candidate_id = self.peer_id.to_string();
            let last_log_index = self.raft_state.log.last_log_index();
            let last_log_term = self.raft_state.log.last_entry_term();
            joinset.spawn(async move {
                if let Ok(client) = HttpClientBuilder::default().build(format!("http://{}", host)) {
                    // send requestVote to the peer. If the peer is down, it will return false
                    client
                        .request_vote(term, candidate_id, last_log_index, last_log_term)
                        .await
                        .unwrap_or(false)
                } else {
                    false
                }
            });
        }
        while let Some(vote_result) = joinset.join_next().await {
            if let Ok(true) = vote_result {
                positive_votes += 1;
            }
        }

        info!("Positive votes: {}", positive_votes);
        if positive_votes >= MAJORITY_QUORUM {
            info!("Elected leader");
            self.raft_state.state_type = RaftStateType::Leader;
            // send a heartbeat to all peers to establish authority and prevent new elections
            self.send_heartbeat_to_peers().await;
        }
    }

    async fn send_heartbeat_to_peers(&self) {
        for (_peer_id, peer_host) in self.peers.iter() {
            let host = peer_host.clone();
            let term = self.raft_state.current_term;
            let leader_id = self.peer_id.to_string();
            let prev_log_index = self.raft_state.log.last_log_index();
            let prev_log_term = self.raft_state.log.last_entry_term();
            let leader_commit = self.raft_state.commit_index;
            tokio::task::spawn(async move {
                if let Ok(client) = HttpClientBuilder::default().build(format!("http://{}", host)) {
                    let _ = client
                        .append_entries(
                            term,
                            leader_id,
                            prev_log_index,
                            prev_log_term,
                            // sending an empty entry array will be interpreted as a heartbeat
                            vec![],
                            leader_commit,
                        )
                        .await;
                }
            });
        }
    }
}
