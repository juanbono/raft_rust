use super::log::Log;

/// Raft state machine states
/// =========================
/// Follower:
/// - responds to RPCs from candidates and leaders
/// - if election timeout elapses without receiving AppendEntries RPC from
///   current leader or granting vote to candidate: convert to candidate and
///   starts a new election
///
/// Candidate:
/// - sends RequestVote RPCs to all other servers
/// - receives votes from other servers
/// - if votes received from majority of servers: become leader
/// - if AppendEntries RPC received from new leader: convert to follower
/// - if election timeout elapses: start new election
///
/// Leader:
/// - Upon election: send initial empty AppendEntries RPCs (heartbeat) to each
///  server; repeat during idle periods to prevent election timeouts (§5.2)
/// - If command received from client: append entry to local log, respond after
///  entry applied to state machine (§5.3)
/// - If last log index ≥ nextIndex for a follower: send AppendEntries RPC with
/// log entries starting at nextIndex
/// - If successful: update nextIndex and matchIndex for follower (§5.3)
/// - If AppendEntries fails because of log inconsistency: decrement nextIndex
/// and retry (§5.3)
#[derive(Debug, Clone, Copy)]
pub enum RaftStateType {
    /// - Responds to RPCs from candidates and leaders
    /// - If election timeout elapses without receiving AppendEntries RPC from
    ///   current leader or granting vote to candidate: convert to candidate and
    ///   starts a new election
    Follower,
    /// - Sends RequestVote RPCs to all other servers
    /// - Receives votes from other servers
    /// - If votes received from majority of servers: become leader
    /// - If AppendEntries RPC received from new leader: convert to follower
    /// - If election timeout elapses: start new election
    Candidate,
    /// - Upon election: send initial empty AppendEntries RPCs (heartbeat) to each
    ///  server; repeat during idle periods to prevent election timeouts (§5.2)
    /// - If command received from client: append entry to local log, respond after
    ///  entry applied to state machine (§5.3)
    /// - If last log index ≥ nextIndex for a follower: send AppendEntries RPC with
    /// log entries starting at nextIndex
    /// - If successful: update nextIndex and matchIndex for follower (§5.3)
    /// - If AppendEntries fails because of log inconsistency: decrement nextIndex
    /// and retry (§5.3)
    ///
    Leader,
}

pub struct RaftState {
    pub state_type: RaftStateType,
    pub current_term: u64,
    pub voted_for: Option<String>,
    pub log: Log,
    pub commit_index: u64,
    pub last_applied: u64,
    // TODO: consider moving the following to Leader variant in `RaftState`
    pub next_index: Vec<u64>,
    pub match_index: Vec<u64>,
}

impl RaftState {
    pub fn new() -> Self {
        RaftState {
            state_type: RaftStateType::Follower,
            current_term: 0,
            voted_for: None,
            log: Log::new(),
            commit_index: 0,
            last_applied: 0,
            next_index: Vec::new(),
            match_index: Vec::new(),
        }
    }
}
