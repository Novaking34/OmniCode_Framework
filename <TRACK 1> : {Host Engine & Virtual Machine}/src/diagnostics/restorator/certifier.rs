// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Two-Way Checksum Certification Engine & Round-Trip Test Oracle Sandbox )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "In the mouth of two or three witnesses shall every word be established."
//   — 2 Corinthians 13:1 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Pass 5 of the Restorator Autonomous Healing Pipeline.
//   Acts as the Supreme Certifier and Two-Way Checksum Gatekeeper:
//     - Witness 1 (DiagnosticLogger): Verifies temporal/causal incident stream and flight telemetry.
//     - Witness 2 (OmniDebugger):     Verifies spatial boundary invariants, immutability, and oracles.
//   The Restorator is structurally forbidden from committing any patch if the two witnesses
//   disagree. If parity breaks, the Certifier returns `0:AWAIT` (homeostatic wait).
//   When both witnesses agree, it executes round-trip lint, lex, parse, and VM evaluation,
//   certifying candidate code before state commitment.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Verification Envelopes & Checksum Agreement Models )
// ==============================================================================
use crate::ast::nodes::ASTNode;
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::linter::OmniLinter;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::runtime::environment::{Environment, RuntimeValue};
use crate::runtime::evaluator::Evaluator;

/// Dual-witness parity agreement status between Logger and Debugger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TwoWayAgreement {
    /// Both witnesses agree: state is certified and aligned for forward commit (+1:PROCEED).
    Certified,
    /// Disagreement detected: Logger and Debugger witnesses do not converge (0:AWAIT).
    AwaitDisagreement {
        logger_senses_fault: bool,
        debugger_senses_fault: bool,
        reason: String,
    },
    /// Both witnesses confirm a structural breach requiring active restorative intervention (-1:ROLLBACK).
    BothConfirmedBreach,
}

impl TwoWayAgreement {
    /// Returns true if both logger and debugger witnesses certify the candidate state.
    pub fn is_certified(&self) -> bool {
        match self {
            TwoWayAgreement::Certified => true,
            _ => false,
        }
    }
}

/// Subsystem executing two-way checksum certification and round-trip verification.
pub struct RestoratorCertifier;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( 2-Way Checksum Verification, Round-Trip Pipeline & Test Oracle Certification )
// ==============================================================================
impl RestoratorCertifier {
    /// Evaluates the 2-Way Checksum between the DiagnosticLogger and OmniDebugger.
    ///
    /// # The Agreement Invariant
    /// - If Logger says an anomaly exists, but Debugger sees clean boundaries: WAITS (0:AWAIT).
    /// - If Debugger flags a breach, but Logger hasn't inscribed the event: WAITS (0:AWAIT).
    /// - If both agree the code is clean: returns `TwoWayAgreement::Certified` (+1:PROCEED).
    /// - If both confirm a fault: returns `TwoWayAgreement::BothConfirmedBreach` (-1:ROLLBACK).
    pub fn evaluate_two_way_checksum(
        logger: &DiagnosticLogger,
        debugger: &OmniDebugger,
        candidate_source: &str,
    ) -> TwoWayAgreement {
        let logger_fault = logger.has_failpoints();
        let debugger_fault = !debugger.boundary_stack.is_empty() || !debugger.remediation_queue.is_empty();

        match (logger_fault, debugger_fault) {
            (false, false) => {
                // Both agree: environment is clean and certified
                TwoWayAgreement::Certified
            }
            (true, true) => {
                // Both witnesses agree on the existence of an active fault
                TwoWayAgreement::BothConfirmedBreach
            }
            (true, false) => {
                // Logger trapped an event, but Debugger sees no active boundary flaw
                logger.log(
                    "TWO_WAY_CHECKSUM",
                    format!(
                        "Witness Disagreement: Logger recorded failpoints but Debugger boundary stack is clean. Holding in 0:AWAIT for candidate: {} bytes",
                        candidate_source.len()
                    ),
                );
                TwoWayAgreement::AwaitDisagreement {
                    logger_senses_fault: true,
                    debugger_senses_fault: false,
                    reason: "Logger recorded incident but Debugger sees sound boundaries (Potential false alarm)".into(),
                }
            }
            (false, true) => {
                // Debugger has boundary frames/remediations, but Logger has no inscribed failpoint
                logger.log(
                    "TWO_WAY_CHECKSUM",
                    format!(
                        "Witness Disagreement: Debugger holds boundary frames but Logger has no failpoint. Inscribing incident before repair: {} bytes",
                        candidate_source.len()
                    ),
                );
                TwoWayAgreement::AwaitDisagreement {
                    logger_senses_fault: false,
                    debugger_senses_fault: true,
                    reason: "Debugger detected boundary fault without corresponding Logger flight inscription (Loss of custody)".into(),
                }
            }
        }
    }

    /// Executes complete round-trip verification: Lint -> Lex -> Parse -> VM Evaluate.
    ///
    /// # Arguments
    /// * `source` - Restored candidate source code string.
    ///
    /// # Returns
    /// Tuple of `(verified: bool, final_state: Option<QuadState>, errors: Vec<String>)`.
    pub fn verify_restoration(source: &str) -> (bool, Option<QuadState>, Vec<String>) {
        let mut errors = Vec::new();
        let logger = DiagnosticLogger::new(false);

        // 1. Lint check
        let linter = OmniLinter::new(logger.clone());
        let lint_rep = linter.lint_source("verify", source);
        if !lint_rep.is_valid {
            for err in lint_rep.errors {
                errors.push(format!("Lint error: {err}"));
            }
        }

        // 2. Tokenize & Parse
        let mut lexer = Lexer::new(source, logger.clone());
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                errors.push(format!("Lex error: {e}"));
                return (false, None, errors);
            }
        };

        let debugger = OmniDebugger::new(logger.clone(), 10);
        let mut parser = Parser::new(tokens, debugger);
        let prog = match parser.parse_program() {
            Ok(p) => p,
            Err(e) => {
                errors.push(format!("Parse error: {e}"));
                return (false, None, errors);
            }
        };

        // 3. VM Execution check
        let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
        match evaluator.eval(&ASTNode::Program(prog)) {
            Ok(val) => {
                let state = match val {
                    RuntimeValue::State(s) => s,
                    _ => QuadState::Proceed,
                };
                let verified = errors.is_empty() && (state == QuadState::Proceed || state == QuadState::Await);
                (verified, Some(state), errors)
            }
            Err(e) => {
                errors.push(format!("Eval error: {e}"));
                (false, None, errors)
            }
        }
    }

    /// Evaluates a candidate repair against an external test oracle closure.
    ///
    /// # Arguments
    /// * `candidate` - Restored code string to test.
    /// * `test_oracle` - Closure asserting domain-specific invariants.
    /// * `logger` - Logger recording certified traces or rollback failpoints.
    pub fn verify_with_test_oracle<F>(
        candidate: &str,
        test_oracle: F,
        logger: &DiagnosticLogger,
    ) -> Result<(), String>
    where
        F: Fn(&str) -> Result<(), String>,
    {
        match test_oracle(candidate) {
            Ok(()) => {
                logger.log("RESTORATOR", "Restoration validated by test oracle (+1:PROCEED)");
                logger.record_ai_observation(
                    "ORACLE_CERTIFIED",
                    "Restored code certified by test oracle",
                    "The candidate passed the verification test suite cleanly",
                    ["self_healing", "test_oracle", "certified"].as_slice(),
                    None,
                );
                Ok(())
            }
            Err(err) => {
                logger.failpoint(
                    FailpointType::RollbackTriggered,
                    "RESTORATOR",
                    format!("Restoration failed test oracle verification: {err}"),
                );
                logger.record_ai_observation(
                    "ORACLE_REJECTED",
                    "Restored code failed test oracle verification",
                    &err,
                    ["self_healing", "test_oracle", "rollback"].as_slice(),
                    None,
                );
                Err(err)
            }
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Certifier Unit Verification Tests & Dual-Witness Invariant Proofs )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_way_checksum_both_clean_certified() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 5);
        let agreement = RestoratorCertifier::evaluate_two_way_checksum(&logger, &debugger, "clean_code");
        assert_eq!(agreement, TwoWayAgreement::Certified);
    }

    #[test]
    fn test_two_way_checksum_disagreement_waits() {
        let logger = DiagnosticLogger::new(false);
        // Force failpoint into logger without debugger boundary flaw
        logger.failpoint(FailpointType::SyntaxAnomaly, "TEST", "Transient noise");
        let debugger = OmniDebugger::new(logger.clone(), 5);

        let agreement = RestoratorCertifier::evaluate_two_way_checksum(&logger, &debugger, "code");
        match agreement {
            TwoWayAgreement::AwaitDisagreement { logger_senses_fault, debugger_senses_fault, .. } => {
                assert!(logger_senses_fault);
                assert!(!debugger_senses_fault);
            }
            _ => panic!("Must wait on witness disagreement"),
        }
    }
}
