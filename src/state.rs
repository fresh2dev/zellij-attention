use serde::{Deserialize, Serialize};

/// Types of notifications a pane can have.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum NotificationType {
    /// Command needs user input
    Waiting,
    /// Command is actively running
    Working,
    /// Command has completed
    Completed,
}
