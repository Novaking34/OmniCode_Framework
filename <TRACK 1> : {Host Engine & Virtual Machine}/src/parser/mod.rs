// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Parser Subsystem Identity, Architecture Manifest & Re-exports )
// ==============================================================================
//! # OmniCode Parser Subsystem & Ingestion Library (`parser`)
//!
//! The parser subsystem is a pure, stateless software library responsible for
//! transcribing raw source text reels into validated Abstract Syntax Trees (AST).
//!
//! ## Scriptural Grounding:
//! - **Exodus 31:18**: *"And he gave unto Moses, when he had made an end of communing
//!   with him upon mount Sinai, two tables of testimony, tables of stone, written with
//!   the finger of God."*
//! - **Proverbs 24:27**: *"Prepare thy work without, and make it fit for thyself in
//!   the field; and afterwards build thine house."*
//! - **Proverbs 22:28**: *"Remove not the ancient landmark, which thy fathers have set."*
//!
//! ## Subsystem Topology:
//! - [`universal`]: Language-agnostic Unicode character and token stream cursors.
//! - [`lexical`]: Ontological token taxonomy, morpheme compounding, and lexer.
//! - [`syntactic`]: Morphological boundary parser and dual-vector precedence cascade.

pub mod universal;
pub mod lexical;
pub mod syntactic;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Public Library Re-exports & Backward Compatibility Layer )
// ==============================================================================
pub use universal::{CharCheckpoint, CharCursor, TokenCursor};
pub use lexical::{Lexer, SourceLocation, Span, Token, TokenType};
pub use syntactic::{GrammarEngine, Parser};

// Aliases preserving 100% backward compatibility for internal imports
pub use lexical::tokens;
pub use lexical::lexer;
pub use universal::cursor;
pub use universal::char_stream;
pub use syntactic::grammar;
pub use syntactic::parser;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Parser Subsystem Core Coordination )
// ==============================================================================
// Parser library components are actively utilized across compilation and execution passes.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification & Module Status )
// ==============================================================================
// Parser subsystem verified, modularized, and closed.
