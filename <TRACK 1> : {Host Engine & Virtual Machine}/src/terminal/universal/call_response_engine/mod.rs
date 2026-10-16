// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Call-Response Engine Subsystem Module )
// ==============================================================================
//! OmniCode Terminal: Call & Response Engine Module
//!
//! Subsumes character echo and number execution into the unified Call-and-Response
//! protocol, defining Echo as the Call-Response mechanism that repeats what was called for.
// ==============================================================================

pub mod call_response;
pub mod echo;

pub use call_response::{
    Call,
    Response,
    CharacterResponse,
    NumberResponse,
    StreamResponse,
    CallResponseEngine,
};

pub use echo::{
    EchoResult,
    EchoEngine,
};
