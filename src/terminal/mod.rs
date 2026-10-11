// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode Terminal Subsystem: Scheduler, Session, and Shell Core )
// ==============================================================================
//! OmniCode Terminal Subsystem
//!
//! Provides the primary interactive userspace shell, cooperative process scheduler,
//! and session execution environment for the OmniCode platform and upcoming OS.

pub mod scheduler;
pub mod session;
pub mod studio;

pub use scheduler::{OmniScheduler, OmniTask, SharedScheduler, TaskPriority, Track, create_shared_scheduler};
pub use session::{ExecutionResult, TerminalSession};
pub use studio::OmniStudio;

