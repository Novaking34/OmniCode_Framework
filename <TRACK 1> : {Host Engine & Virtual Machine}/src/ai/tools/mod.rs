// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AI Tool Belt: Five-Fold Sensory & Diagnostic Subsystem for CPI-SI )
// ==============================================================================
// SCRIPTURAL WITNESSES:
//   - 2 Timothy 3:16-17: "All scripture is given by inspiration of God, and is
//     profitable for doctrine, for reproof, for correction, for instruction in
//     righteousness: That the man of God may be perfect, throughly furnished
//     unto all good works."
//     (The complete equipment of the agent for doctrine, reproof, correction, and healing)
//   - Proverbs 24:3-4: "Through wisdom is an house builded; and by understanding
//     it is established: And by knowledge shall the chambers be filled with all
//     precious and pleasant riches."
//     (The multi-faceted tools of wisdom, understanding, and knowledge)
//   - 1 Corinthians 14:40: "Let all things be done decently and in order."
//     (The immutable invariant of structural order and discipline)
//
// ARCHITECTURAL TOPOLOGY:
//   The CPI-SI Autonomous Diagnostic Tool Belt transforms the five Track 1 diagnostics
//   into first-class sensory and autonomic organs for the AI Cortex:
//     1. LoggerTool: Flight telemetry, circular roll frames, and incident timelines.
//     2. DebuggerTool: Step-by-step spatial execution, AST validation, and verification oracles.
//     3. LinterTool: 4-Block architectural auditing, 80-col spatial geometry, and rule enforcement.
//     4. RestoratorTool: 5-pass autonomous code repair and Two-Way Checksum (Logger <-> Debugger).
//     5. HoverTool: 3-tier ontological symbol lookup, coordinate resolution, and doc generation.
//
//   Every tool functions independently in isolation, yet synergizes within AIToolBelt
//   to enable closed-loop self-inspection, diagnosis, repair, and verification.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Module Declarations, Public Exports & Tool Belt Definition )
// ==============================================================================
pub mod debugger_tool;
pub mod hover_tool;
pub mod linter_tool;
pub mod logger_tool;
pub mod restorator_tool;

pub use debugger_tool::{DebuggerTool, DebuggerToolReport};
pub use hover_tool::{HoverTool, HoverToolReport};
pub use linter_tool::{LinterTool, LinterToolReport};
pub use logger_tool::{LoggerTool, LoggerToolReport};
pub use restorator_tool::{RestoratorTool, RestoratorToolReport};

use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::hover::{HoverCard, HoverEngine};
use crate::diagnostics::linter::OmniLinter;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::diagnostics::restorator::Restorator;

/// The unified Five-Fold Diagnostic Tool Belt for CPI-SI autonomous operations.
#[derive(Debug, Clone)]
pub struct AIToolBelt {
    pub logger: LoggerTool,
    pub debugger: DebuggerTool,
    pub linter: LinterTool,
    pub restorator: RestoratorTool,
    pub hover: HoverTool,
}

impl AIToolBelt {
    /// Creates a complete tool belt initialized with standard diagnostic backends.
    pub fn new() -> Self {
        let logger_backend = DiagnosticLogger::new(false);
        let debugger_backend = OmniDebugger::new(logger_backend.clone(), 100);
        let linter_backend = OmniLinter::new(logger_backend.clone());
        let restorator_backend = Restorator::new(logger_backend.clone());
        let hover_backend = HoverEngine::new(logger_backend.clone());

        Self {
            logger: LoggerTool::new(logger_backend),
            debugger: DebuggerTool::new(debugger_backend),
            linter: LinterTool::new(linter_backend),
            restorator: RestoratorTool::new(restorator_backend),
            hover: HoverTool::new(hover_backend),
        }
    }

    /// Creates a tool belt bound to a specific existing logger.
    pub fn with_logger(logger_backend: DiagnosticLogger) -> Self {
        let debugger_backend = OmniDebugger::new(logger_backend.clone(), 100);
        let linter_backend = OmniLinter::new(logger_backend.clone());
        let restorator_backend = Restorator::new(logger_backend.clone());
        let hover_backend = HoverEngine::new(logger_backend.clone());

        Self {
            logger: LoggerTool::new(logger_backend),
            debugger: DebuggerTool::new(debugger_backend),
            linter: LinterTool::new(linter_backend),
            restorator: RestoratorTool::new(restorator_backend),
            hover: HoverTool::new(hover_backend),
        }
    }
}

impl Default for AIToolBelt {
    fn default() -> Self {
        Self::new()
    }
}

/// Generic invocation request dispatched to the AI Tool Belt.
#[derive(Debug, Clone)]
pub enum ToolCall {
    /// Inspect circular flight recorder telemetry.
    InspectTelemetry { max_frames: usize },
    /// Execute source code via spatial debugger.
    DebugEvaluate { code: String, step_by_step: bool },
    /// Lint source code for 4-Block architectural compliance.
    LintSource { source: String },
    /// Autonomously restore and repair source code.
    RestoreSource { source: String, verify_with_debugger: bool },
    /// Look up documentation for a symbol.
    HoverQuery { symbol: String, file_hint: Option<String> },
    /// Query hover documentation at 2D coordinates.
    HoverCoordinate { file_path: String, line: usize, col: usize },
}

/// Generic result returned by tool execution.
#[derive(Debug, Clone)]
pub enum ToolResult {
    Telemetry(LoggerToolReport),
    Debug(DebuggerToolReport),
    Lint(LinterToolReport),
    Restoration(RestoratorToolReport),
    Hover(HoverToolReport),
    HoverCard(Option<HoverCard>),
    Error(String),
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Tool Dispatch Coordinator & Unified Invocation )
// ==============================================================================
impl AIToolBelt {
    /// Dispatches a structured [`ToolCall`] to the corresponding diagnostic organ.
    pub fn dispatch(&mut self, call: ToolCall) -> ToolResult {
        match call {
            ToolCall::InspectTelemetry { max_frames } => {
                let report = self.logger.inspect_recent_telemetry(max_frames);
                ToolResult::Telemetry(report)
            }
            ToolCall::DebugEvaluate { code, step_by_step } => {
                let report = self.debugger.evaluate_source(&code, step_by_step);
                ToolResult::Debug(report)
            }
            ToolCall::LintSource { source } => {
                let report = self.linter.lint_source(&source);
                ToolResult::Lint(report)
            }
            ToolCall::RestoreSource { source, verify_with_debugger } => {
                let mut report = self.restorator.heal_source(&source);
                if verify_with_debugger {
                    let agreement = self.restorator.check_two_way_agreement(&self.debugger.debugger, &report.restored_content);
                    if !agreement.is_certified() {
                        report.verified = false;
                        report.errors.push("Two-Way Checksum (Logger <-> Debugger) failed to establish parity".to_string());
                    }
                }
                ToolResult::Restoration(report)
            }
            ToolCall::HoverQuery { symbol, file_hint } => {
                let report = self.hover.document_symbol(&symbol, file_hint.as_deref());
                ToolResult::Hover(report)
            }
            ToolCall::HoverCoordinate { file_path, line, col } => {
                let card = self.hover.resolve_at_coordinate(&file_path, line, col);
                ToolResult::HoverCard(card)
            }
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification Invariants & Independent Tool Belt Unit Tests )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_tool_belt_initialization() {
        let belt = AIToolBelt::new();
        assert!(belt.logger.logger.events().len() >= 2);
    }

    #[test]
    fn test_ai_tool_belt_lint_dispatch() {
        let mut belt = AIToolBelt::new();
        let malformed = "let x = 1";
        let res = belt.dispatch(ToolCall::LintSource {
            source: malformed.to_string(),
        });

        match res {
            ToolResult::Lint(report) => {
                assert!(!report.is_valid, "Malformed code should not pass lint");
                assert!(report.error_count > 0);
            }
            _ => panic!("Expected Lint result"),
        }
    }

    #[test]
    fn test_ai_tool_belt_restore_dispatch() {
        let mut belt = AIToolBelt::new();
        let legacy = "val x = 10;";
        let res = belt.dispatch(ToolCall::RestoreSource {
            source: legacy.to_string(),
            verify_with_debugger: false,
        });

        match res {
            ToolResult::Restoration(report) => {
                assert!(report.was_modified);
                assert!(report.restored_content.contains("BLOCK 1: METADATA BLOCK"));
            }
            _ => panic!("Expected Restoration result"),
        }
    }

    #[test]
    fn test_ai_tool_belt_hover_dispatch() {
        let mut belt = AIToolBelt::new();
        let res = belt.dispatch(ToolCall::HoverQuery {
            symbol: "Law_Truth".to_string(),
            file_hint: None,
        });

        match res {
            ToolResult::Hover(report) => {
                assert_eq!(report.symbol, "Law_Truth");
            }
            _ => panic!("Expected Hover result"),
        }
    }

    #[test]
    fn test_ai_tool_belt_telemetry_dispatch() {
        let mut belt = AIToolBelt::new();
        let res = belt.dispatch(ToolCall::InspectTelemetry { max_frames: 5 });

        match res {
            ToolResult::Telemetry(report) => {
                assert!(report.total_logs >= 2);
            }
            _ => panic!("Expected Telemetry result"),
        }
    }
}
