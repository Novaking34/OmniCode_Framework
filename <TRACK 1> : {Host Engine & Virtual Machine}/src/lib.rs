// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Library Root Identity, Universal Subsystem Modules & Core Re-exports )
// ==============================================================================
//! OmniCode Language Engine
//! High-performance language engine based on formal ontology,
//! set theory, and balanced ternary quad-state execution,
//! engineered to be symmetrically human and machine readable.

pub mod core;
pub mod diagnostics;
pub mod ast;
pub mod parser;
pub mod runtime;
pub mod cli;
pub mod ai;
pub mod terminal;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Public Re-export Declarations & High-Level API Surface )
// ==============================================================================
pub use core::ternary::{QuadState, TransportAction, ROLLBACK, AWAIT, PROCEED, HALT};
pub use core::boundaries::BoundaryType;
pub use core::operators::{BeingRelation, DirectionalVector, Connective, Polarity, AffixPosition};
pub use core::word::{Word32Trit, Word64};
pub use core::scene::Scene;
pub use core::reel::MemoryReel;
pub use core::filetype::OmniFileType;
pub use core::tape::{Tape, Roll};
pub use core::mmio::{PhysicalMemoryMap, VgaBuffer, VgaColor, VGA_BUFFER_BASE, COM1_SERIAL_PORT};
pub use terminal::{OmniScheduler, OmniTask, Track, TaskPriority};

pub use diagnostics::logger::{DiagnosticLogger, FailpointType, FixKind, FixSuggestion};
pub use diagnostics::debugger::OmniDebugger;
pub use diagnostics::linter::{OmniLinter, LintReport, BlockKind};
pub use diagnostics::restorator::{Restorator, RestorationReport, AppliedRepair};
pub use diagnostics::hover::{HoverEngine, HoverCard, OmniDefinition, TapeRollContext};
pub use ai::{
    AITransducer, TransductionSession, AICortex, ScrollLogger, ExecutionScroll, AILearner,
    EpisodicSummary, EpisodicExperience, CognitiveResult, BiblicalLexicon, LexiconDomain,
    LexiconEntry, MoralEvaluation,
};
pub use parser::lexer::Lexer;
pub use parser::parser::Parser;
pub use runtime::environment::Environment;
pub use runtime::evaluator::Evaluator;
pub use terminal::OmniStudio;
pub use cli::{run_repl, run_gui_server};



// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Convenience Execution Pipeline )
// ==============================================================================
/// Convenience execution helper that tokenizes, parses, and evaluates an OmniCode string.
pub fn execute(source: &str, print_logs: bool) -> Result<runtime::RuntimeValue, String> {
    let logger = DiagnosticLogger::new(print_logs);
    let mut lexer = Lexer::new(source, logger.clone());
    let tokens = lexer.tokenize()?;

    let debugger = OmniDebugger::new(logger, 10);
    let mut parser = Parser::new(tokens, debugger);
    let program = parser.parse_program()?;

    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    evaluator.eval(&ast::nodes::ASTNode::Program(program))
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Engine Status & Library Invariants )
// ==============================================================================
// OmniCode library surface verified and sealed.
