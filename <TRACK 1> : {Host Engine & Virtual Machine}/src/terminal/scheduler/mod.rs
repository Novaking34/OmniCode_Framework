// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Multi-Track Scheduler Subsystem Identity & Public Exports )
// ==============================================================================
//! OmniCode Terminal: Multi-Track Scheduler Subsystem Module
//!
//! Provides the primordial process and task scheduling engine,
//! 3-track execution channels, priority queues, and cooperative tick loops.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**: Tables of testimony, written with the finger of God.
//! - **Proverbs 24:27 (KJV)**: Prepare thy work without, and make it fit for thyself in the field.
//! - **Proverbs 22:28 (KJV)**: Remove not the ancient landmark, which thy fathers have set.
// ==============================================================================

pub mod tasks;
pub mod scheduler;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Scheduler & Tasks )
// ==============================================================================
pub use tasks::{
    Track,
    TaskPriority,
    OmniTask,
};

pub use scheduler::{
    OmniScheduler,
    SharedScheduler,
    create_shared_scheduler,
};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Invariants & Cooperative Scheduling Guarantees )
// ==============================================================================
// Task execution is isolated across designated channels (Kernel, Terminal, App).

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// Multi-Track Scheduler subsystem verified and closed.
