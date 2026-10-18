// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Model Architecture Identity, Subsystem Manifest & Exports )
// ==============================================================================
//! OmniCode Internal AI Model Subsystem
//!
//! Organizes the core cognitive neural-symbolic engine:
//!   - `cortex`: Central cognitive coordinator orchestrating the nervous system
//!     (Logger, Debugger, Restorator, Transducer, ScrollLogger).
//!   - `scroll`: Dual-format episodic logging (.scroll 4-Block Turing reel & .log telemetry).
//!   - `learner`: Experience ingestion engine transforming past scrolls into supervised training data.

pub mod attractor;
pub mod cortex;
pub mod discourse;
pub mod learner;
pub mod memory;
pub mod scroll;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( AI Model Re-exports )
// ==============================================================================
pub use attractor::{AttractorBasin, AttractorMatch, EpisodicAttractorNetwork};
pub use cortex::{AICortex, CognitiveResult};
pub use discourse::{ConversationTurn, DiscourseEngine, DiscourseSession, DiscourseTopic};
pub use learner::{AILearner, EpisodicExperience, EpisodicSummary};
pub use memory::{AIMemory, MemoryAssociation, ThoughtRecord};
pub use scroll::{ExecutionScroll, ScrollLogger};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Cognitive Subsystem Invariants )
// ==============================================================================
// Internal AI Model cognitive pathways verified and active.

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification and Subsystem Sealing )
// ==============================================================================
// AI Model module sealed.
