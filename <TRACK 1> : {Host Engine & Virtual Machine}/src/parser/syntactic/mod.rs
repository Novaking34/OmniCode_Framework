// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Syntactic Grammar & Parser Subsystem )
// ==============================================================================
//! # Syntactic Grammar & Parser Subsystem
//!
//! Builds the Abstract Syntax Tree (AST) by enforcing the 6 ranked morphological
//! boundaries and the 6-level dual-vector precedence cascade.
//!
//! ## Scriptural Grounding:
//! - **Exodus 31:18**: Building upon the inscribed tables of stone.
//! - **Proverbs 24:27**: Building the architectural house upon prepared tokens.
//! - **Proverbs 22:28**: Enforcing unalterable morphological boundaries.

pub mod grammar;
pub mod parser;

pub use grammar::GrammarEngine;
pub use parser::Parser;
