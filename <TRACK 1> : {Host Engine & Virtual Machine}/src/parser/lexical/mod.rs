// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Lexical Inscription & Token Vocabulary Subsystem )
// ==============================================================================
//! # Lexical Inscription Subsystem
//!
//! Transforms raw source reels into structured [`Token`] streams, enforcing
//! the physical and ontological boundary invariants of OmniCode.
//!
//! ## Scriptural Grounding:
//! - **Exodus 31:18**: Divine inscription of characters on stone.
//! - **Proverbs 24:27**: Preparing tokens in the field before syntactic assembly.
//! - **Proverbs 22:28**: Preserving ancient morphemic boundaries and operators.

pub mod tokens;
pub mod lexer;

pub use tokens::{SourceLocation, Span, Token, TokenType};
pub use lexer::Lexer;
