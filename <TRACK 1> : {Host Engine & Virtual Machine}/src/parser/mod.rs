// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Parser Subsystem Identity, Architecture Manifest & Re-exports )
// ==============================================================================
//! OmniCode Parser Subsystem
//! Combines Lexical Tokenizer and Recursive-Descent Ast Generator.

pub mod lexer;
pub mod parser;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Parser Components )
// ==============================================================================
pub use lexer::{Lexer, Token, TokenType};
pub use parser::Parser;


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Parser Subsystem Core Coordination )
// ==============================================================================
// Parser components are actively utilized across compilation passes.


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification & Module Status )
// ==============================================================================
// Parser subsystem verified and closed.
