// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Subsystem Identity, Module Manifest & Exports )
// ==============================================================================
//! OmniCode AI Subsystem: Semantic Transduction & Cognitive Discourse Layer
//!
//! Exposes:
//!   - `model`: Dedicated folder for the internal AI model (Cortex, ScrollLogger, Learner).
//!   - `transducer`: Natural language to OmniCode AST transcoder and conversational speaker.

pub mod lexicon;
pub mod model;
pub mod neural;
pub mod tools;
pub mod transducer;

// ============================================================================
// BLOCK 2: OPENING BLOCK
// ( Subsystem Re-exports )
// ============================================================================
pub use lexicon::{
    BiblicalLexicon, ConcordanceRef, LexiconDomain, LexiconEntry, MoralEvaluation,
    ScriptureTranslation,
};
pub use model::{
    AICortex, AILearner, AIMemory, CognitiveResult, ConversationTurn,
    DiscourseEngine, DiscourseSession, DiscourseTopic, EpisodicExperience,
    EpisodicSummary, ExecutionScroll, MemoryAssociation, ScrollLogger,
    ThoughtRecord,
};
pub use neural::{
    BrainGrowthHistory, BrainGrowthSnapshot, BrainVisualizer, Curriculum,
    CurriculumChapter, DegradationMode, DegradationStep, NeuralBrain,
    NeuralTrainer, NoiseEngine, StructuredNoiseConfig, SynapticWeight,
    TernaryLayer, TernaryNeuron, TrainingReport,
};
pub use tools::{
    AIToolBelt, DebuggerTool, DebuggerToolReport, HoverTool, HoverToolReport,
    LinterTool, LinterToolReport, LoggerTool, LoggerToolReport, RestoratorTool,
    RestoratorToolReport, ToolCall, ToolResult,
};
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
