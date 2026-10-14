// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal Stream Navigation Subsystem Identity & Exports )
// ==============================================================================
//! # Universal Stream & Navigation Machinery
//!
//! Provides pure, language-agnostic character-reel and token-stream cursors with
//! arbitrary lookahead, coordinate tracking, and speculative checkpointing.
//!
//! ## Scriptural Grounding:
//! - **Exodus 31:18**: Inscription of truth on physical reels.
//! - **Proverbs 24:27**: Preparing the raw stream in the field before assembly.
//! - **Proverbs 22:28**: Preserving stream landmarks and boundary coordinates.

pub mod char_stream;
pub mod cursor;

pub use char_stream::{CharCheckpoint, CharCursor};
pub use cursor::TokenCursor;
