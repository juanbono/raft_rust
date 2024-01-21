pub enum RaftStateType {
    Follower,
    Candidate,
    Leader,
}

pub struct RaftState {
    pub state_type: RaftStateType,
    pub current_term: u64,
    pub voted_for: Option<String>,
    pub log: Vec<String>,
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
            log: Vec::new(),
            commit_index: 0,
            last_applied: 0,
            next_index: Vec::new(),
            match_index: Vec::new(),
        }
    }
}
