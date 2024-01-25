use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum KvCommand {
    Get { key: String },
    Set { key: String, value: String },
    Remove { key: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub term: u64,
    pub command: KvCommand,
}

#[derive(Debug)]
pub struct Log {
    entries: Vec<LogEntry>,
}

impl Log {
    /// Creates a new `Log`.
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn add_entry(&mut self, entry: LogEntry) {
        // consistency check
        // TODO: revisit the paper to check if this is the one we need
        if let Some(last_entry) = self.entries.last() {
            assert!(last_entry.term < entry.term)
        }

        self.entries.push(entry);
    }

    /// Returns the index of the last entry in the log.
    /// It's 0 if the log is empty.
    pub fn last_log_index(&self) -> u64 {
        if self.entries.is_empty() {
            0
        } else {
            let last_log_index = self.entries.len() + 1;
            last_log_index as u64
        }
    }

    /// Returns the term of the last entry in the log.
    /// It's 0 if the log is empty.
    pub fn last_entry_term(&self) -> u64 {
        self.entries.last().map(|e| e.term).unwrap_or_default()
    }

    // TODO: possibly add methods for removing from an index to the end and replace entries.
}
