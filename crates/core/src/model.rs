//! Data types shared by every part of Pult.

use serde::{Deserialize, Serialize};

/// The state of a session as shown by its lamp in the list.
///
/// "Elsewhere" is not a status: a session that runs outside Pult's tmux keeps its normal
/// status and gets an extra tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Claude is busy: a prompt was submitted or a tool is running.
    Working,
    /// Claude waits for the user: a permission request or a question.
    NeedsYou,
    /// Claude finished a turn and the user has not opened the session since.
    Done,
    /// Claude finished and the result was seen, or the registry reports it idle.
    Idle,
    /// The last turn stopped with an error.
    Error,
    /// No live process; only the history file exists.
    NotRunning,
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [(Status, &str); 6] = [
        (Status::Working, "working"),
        (Status::NeedsYou, "needs_you"),
        (Status::Done, "done"),
        (Status::Idle, "idle"),
        (Status::Error, "error"),
        (Status::NotRunning, "not_running"),
    ];

    #[test]
    fn serializes_to_snake_case_strings() {
        for (status, name) in ALL {
            let json = serde_json::to_string(&status).unwrap();
            assert_eq!(json, format!("\"{name}\""));
        }
    }

    #[test]
    fn round_trips_through_json() {
        for (status, _) in ALL {
            let json = serde_json::to_string(&status).unwrap();
            let back: Status = serde_json::from_str(&json).unwrap();
            assert_eq!(back, status);
        }
    }

    #[test]
    fn rejects_unknown_values() {
        assert!(serde_json::from_str::<Status>("\"elsewhere\"").is_err());
        assert!(serde_json::from_str::<Status>("\"Working\"").is_err());
    }
}
