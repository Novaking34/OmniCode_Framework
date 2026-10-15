// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Terminal Universal Subsystem Identity & Public Exports )
// ==============================================================================
//! OmniCode Terminal: Universal Subsystem Module
//!
//! Provides the fundamental hardware MMIO ports (`0x03F8`, `0x000B8000`),
//! directional I/O stream buffers, and the native 5-trit Character Echo Engine.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**: Tables of testimony, written with the finger of God.
//! - **Proverbs 24:27 (KJV)**: Prepare thy work without, and make it fit for thyself in the field.
//! - **Proverbs 22:28 (KJV)**: Remove not the ancient landmark, which thy fathers have set.
// ==============================================================================

pub mod ports;
pub mod character_echo;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Universal Terminal Substrates )
// ==============================================================================
pub use ports::{
    UART_COM1_INPUT_PORT,
    VGA_TEXT_OUTPUT_PORT,
    DEFAULT_TERMINAL_BUFFER_CAPACITY,
    TerminalInputStream,
    TerminalOutputStream,
};

pub use character_echo::{
    CharacterEchoResult,
    CharacterEchoEngine,
};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Universal Invariants & Subsystem Proofs )
// ==============================================================================
// Hardware MMIO ports and the Character Invariant lifecycle are decoupled
// from interactive session shells and process dispatchers.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Module Closure )
// ==============================================================================
// Universal Terminal Subsystem verified and closed.
