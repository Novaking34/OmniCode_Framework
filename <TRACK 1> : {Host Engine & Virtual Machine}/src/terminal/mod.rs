// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Terminal Subsystem Identity, Public Facade & Architectural Invariants )
// ==============================================================================
//! OmniCode Terminal Subsystem Facade
//!
//! # Architectural Scope & Purpose:
//! Provides the primary interactive userspace shell, cooperative process scheduler,
//! hardware MMIO directional stream buffers, native Character Echo Engine, and
//! cinematic HUD projector for the OmniCode platform and upcoming Operating System.
//!
//! Subsystems:
//! - `universal`: Hardware MMIO ports (`0x03F8`, `0x000B8000`), directional streams, and 5-trit Character Echo Engine.
//! - `scheduler`: Multi-Track cooperative task manager (`Track::Kernel`, `Track::Terminal`, `Track::App`).
//! - `session`: Interactive REPL shell, persistent variable state, and direct in-memory roll evaluator.
//! - `studio`: Cinematic HUD projector rendering 4-directional Memory Reel execution frames.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of physical data into bounded terminal tablets).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic terminal I/O engines before constructing user interfaces).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of historical hardware ports and execution track landmarks).
// ==============================================================================

pub mod universal;
pub mod scheduler;
pub mod session;
pub mod studio;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Universal Access & 100% Backward Compatibility )
// ==============================================================================

// Universal Terminal Substrate
pub use universal::{
    UART_COM1_INPUT_PORT,
    VGA_TEXT_OUTPUT_PORT,
    DEFAULT_TERMINAL_BUFFER_CAPACITY,
    TerminalInputStream,
    TerminalOutputStream,
    CharacterEchoResult,
    CharacterEchoEngine,
};

// Multi-Track Scheduler
pub use scheduler::{
    OmniScheduler,
    OmniTask,
    SharedScheduler,
    TaskPriority,
    Track,
    create_shared_scheduler,
};

// Interactive Session
pub use session::{
    ExecutionResult,
    TerminalSession,
};

// Cinema Studio
pub use studio::OmniStudio;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Proofs & Invariants )
// ==============================================================================
// Terminal processes and character echo loops are coordinated cooperatively
// across the 3 execution channels with persistent state in the Environment.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// Terminal Subsystem facade verified and sealed in affirmative PROCEED.
