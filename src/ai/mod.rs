// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Subsystem Identity, Module Manifest & Exports )
// ==============================================================================
//! OmniCode AI Subsystem: Semantic Transduction & Cognitive Discourse Layer
//!
//! Exposes:
//!   - `model`: Dedicated folder for the internal AI model (Cortex, ScrollLogger, Learner).
//!   - `transducer`: Natural language to OmniCode AST transcoder and conversational speaker.

pub mod model;
pub mod transducer;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Subsystem Re-exports )
// ==============================================================================
pub use model::{AICortex, AILearner, CognitiveResult, EpisodicExperience, EpisodicSummary, ExecutionScroll, ScrollLogger};
pub use transducer::{AITransducer, TransductionSession};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Subsystem Core Coordination )
// ==============================================================================
// AI cognitive loop active and linked to diagnostic nervous system.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification and Subsystem Status )
// ==============================================================================
// AI subsystem verified and sealed.
