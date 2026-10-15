// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Multi-Track Execution Channels, Task Priorities & Process Descriptor )
// ==============================================================================
//! OmniCode Terminal: Multi-Track Execution Channels & Task Models
//!
//! # Architectural Scope & Purpose:
//! Encapsulates process and task data models for the OmniCode Terminal Scheduler:
//! - 3 Physical execution channels: [`Track::Kernel`] (0), [`Track::Terminal`] (1), [`Track::App`] (2).
//! - 4 Priority tiers: [`TaskPriority::High`], [`TaskPriority::Normal`], [`TaskPriority::Low`], [`TaskPriority::Background`].
//! - [`OmniTask`]: Persistent descriptor managing process state, bound tape, memory partition, and logs.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of process descriptors into bounded memory tablets).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving process descriptor models before dispatching scheduler loops).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of historical execution channel boundaries across kernel, terminal, and apps).
// ==============================================================================

use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::reel::MemoryReel;
use crate::core::tape::Tape;
use crate::core::ternary::QuadState;
use crate::core::vessel::Vessel;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Track & TaskPriority Enums and OmniTask Definition )
// ==============================================================================

/// The 3 physical execution channels of the Multi-Track Scheduler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Track {
    /// Track 0: Hardware / Kernel Genesis / Core Daemons.
    Kernel = 0,
    /// Track 1: Terminal Shell & Console HUD.
    Terminal = 1,
    /// Track 2: Userspace & Worker Applications.
    App = 2,
}

impl Track {
    /// Returns the canonical display string for this execution channel.
    pub fn as_str(&self) -> &'static str {
        match self {
            Track::Kernel => "TRACK 0 [KERNEL]",
            Track::Terminal => "TRACK 1 [TERMINAL]",
            Track::App => "TRACK 2 [APP]",
        }
    }
}

/// Scheduling priority tier for active processes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TaskPriority {
    High = 3,
    Normal = 2,
    Low = 1,
    Background = 0,
}

impl TaskPriority {
    /// Returns the uppercase text representation of this priority.
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskPriority::High => "HIGH",
            TaskPriority::Normal => "NORMAL",
            TaskPriority::Low => "LOW",
            TaskPriority::Background => "BACKGROUND",
        }
    }

    /// Resolves a string into a TaskPriority variant.
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_uppercase().as_str() {
            "HIGH" | "3" => TaskPriority::High,
            "LOW" | "1" => TaskPriority::Low,
            "BACKGROUND" | "IDLE" | "0" => TaskPriority::Background,
            _ => TaskPriority::Normal,
        }
    }
}

/// Descriptor representing an active, waiting, or halted process in the terminal.
#[derive(Debug, Clone)]
pub struct OmniTask {
    /// Unique process identifier (PID).
    pub id: u64,
    /// Human-readable process name.
    pub name: String,
    /// Shell command or entry point string.
    pub command: String,
    /// Scheduling priority tier.
    pub priority: TaskPriority,
    /// Designated execution channel.
    pub track: Track,
    /// Balanced ternary process state (+1:Proceed, 0:Await, -1:Rollback, NULL:Halt).
    pub state: QuadState,
    /// UNIX timestamp of process creation.
    pub created_at: u64,
    /// UNIX timestamp of last update or tick.
    pub updated_at: u64,
    /// Total cooperative scheduler ticks elapsed.
    pub ticks: u64,
    /// Log output ring buffer.
    pub log_output: Vec<String>,
    /// Mounted process tape ribbon, if executing tape-based logic.
    pub tape: Option<Tape>,
    /// Bounded RAM partition carved from Root_Storehouse.
    pub partition: Option<Vessel>,
    /// 4-directional memory reel for cinematic HUD stepping.
    pub reel: Option<MemoryReel>,
    /// Current loop counter.
    pub loop_count: usize,
    /// Maximum allowed loops before watchdog intervention.
    pub max_loops: usize,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( OmniTask Constructors & Serializers )
// ==============================================================================

impl OmniTask {
    /// Constructs a new task on Track 2 (App) by default.
    pub fn new(id: u64, name: String, command: String, priority: TaskPriority) -> Self {
        Self::with_track(id, name, command, priority, Track::App)
    }

    /// Constructs a new task on the specified execution channel.
    pub fn with_track(id: u64, name: String, command: String, priority: TaskPriority, track: Track) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id,
            name: name.clone(),
            command,
            priority,
            track,
            state: QuadState::Proceed,
            created_at: now,
            updated_at: now,
            ticks: 0,
            log_output: Vec::new(),
            tape: None,
            partition: None,
            reel: Some(MemoryReel::new(name, 64)),
            loop_count: 0,
            max_loops: 100,
        }
    }

    /// Serializes the task descriptor into a structured JSON string.
    pub fn to_json(&self) -> String {
        let state_str = match self.state {
            QuadState::Proceed => "+1:PROCEED",
            QuadState::Await => "0:AWAIT",
            QuadState::Rollback => "-1:ROLLBACK",
            QuadState::Halt => "NULL:HALT",
        };

        let logs = self.log_output.join("\\n").replace('"', "\\\"");

        format!(
            r#"{{"id":{},"name":"{}","command":"{}","priority":"{}","track":"{}","state":"{}","ticks":{},"created_at":{},"logs":"{}"}}"#,
            self.id,
            self.name,
            self.command.replace('"', "\\\""),
            self.priority.as_str(),
            self.track.as_str(),
            state_str,
            self.ticks,
            self.created_at,
            logs
        )
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Verification Proofs & Invariants )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_hierarchy_and_names() {
        assert_eq!(Track::Kernel.as_str(), "TRACK 0 [KERNEL]");
        assert_eq!(Track::Terminal.as_str(), "TRACK 1 [TERMINAL]");
        assert_eq!(Track::App.as_str(), "TRACK 2 [APP]");
    }

    #[test]
    fn test_task_priority_resolution() {
        assert_eq!(TaskPriority::from_str("high"), TaskPriority::High);
        assert_eq!(TaskPriority::from_str("1"), TaskPriority::Low);
        assert_eq!(TaskPriority::from_str("idle"), TaskPriority::Background);
        assert_eq!(TaskPriority::from_str("unknown"), TaskPriority::Normal);
    }

    #[test]
    fn test_omni_task_creation_and_json() {
        let task = OmniTask::with_track(
            42,
            "InitDaemon".into(),
            "boot --init".into(),
            TaskPriority::High,
            Track::Kernel,
        );
        assert_eq!(task.id, 42);
        assert_eq!(task.track, Track::Kernel);
        assert_eq!(task.state, QuadState::Proceed);

        let json = task.to_json();
        assert!(json.contains("\"id\":42"));
        assert!(json.contains("\"name\":\"InitDaemon\""));
        assert!(json.contains("\"priority\":\"HIGH\""));
        assert!(json.contains("\"track\":\"TRACK 0 [KERNEL]\""));
    }
}
