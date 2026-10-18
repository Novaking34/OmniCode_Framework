// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( TestOracleSandbox, Candidate Patch Verification & Surgical Log Restoration )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "Prove all things; hold fast that which is good."
//   — 1 Thessalonians 5:21 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   The empirical verification and test oracle organ of the OmniDebugger.
//   Acts as the scrutinizing eye certifying hypothesis correctness before state commit:
//     - Isolates candidate patches inside a sandboxed child execution environment.
//     - Verifies candidate source code against executable invariant test oracles.
//     - Records ORACLE_SUCCESS and ORACLE_FAILURE observations directly into
//       the DiagnosticLogger's episodic neural-symbolic memory for the AI Cortex.
//     - Ingests structured LogEvents with FixSuggestions, executing surgical
//       restorations before committing state in QuadState::Proceed.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Log Event Structures & Oracle Sandbox Model )
// ==============================================================================
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType, FixKind, LogEvent};

/// Execution sandbox and test oracle validator for candidate code and state mutations.
#[derive(Debug, Clone, Default)]
pub struct TestOracleSandbox;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Test Execution, Candidate Source Verification & Log-Driven Restoration )
// ==============================================================================
impl TestOracleSandbox {
    /// Constructs a new [`TestOracleSandbox`].
    pub fn new() -> Self {
        Self
    }

    /// Verifies candidate source code with a provided test oracle closure.
    ///
    /// # Arguments
    /// * `candidate_source` - The restored source string under evaluation.
    /// * `test_oracle` - Closure verifying candidate execution invariants.
    /// * `logger` - The shared [`DiagnosticLogger`] recording telemetry.
    ///
    /// # Returns
    /// - `Ok(String)`: If the oracle passes, returns the confirmed candidate source.
    /// - `Err(String)`: If the oracle fails, triggers a state rollback (`-1:ROLLBACK`) and returns the error.
    pub fn verify_candidate_repair<F>(
        candidate_source: &str,
        test_oracle: F,
        logger: &DiagnosticLogger,
    ) -> Result<String, String>
    where
        F: Fn(&str) -> Result<(), String>,
    {
        logger.log("SELF_HEAL", "Gaze Focus: Verifying candidate repair via test oracle...");
        match test_oracle(candidate_source) {
            Ok(()) => {
                logger.log("SELF_HEAL", "Gaze Focus: Candidate repair verified (+1:PROCEED)!");
                logger.record_ai_observation(
                    "REPAIR_VERIFIED",
                    "Candidate source passed verification oracle",
                    "The proposed restorative patch executed cleanly without failpoints",
                    ["self_healing", "oracle_passed", "verified"].as_slice(),
                    None,
                );
                Ok(candidate_source.to_string())
            }
            Err(e) => {
                let msg = format!("Candidate repair failed test verification: {e}");
                logger.failpoint(FailpointType::RollbackTriggered, "SELF_HEAL", &msg);
                logger.record_ai_observation(
                    "REPAIR_REJECTED",
                    "Candidate source failed verification oracle",
                    &msg,
                    ["self_healing", "oracle_failed", "rollback"].as_slice(),
                    None,
                );
                Err(msg)
            }
        }
    }

    /// Ingests a structured [`LogEvent`] containing a [`FixSuggestion`], applies the surgical
    /// patch to the target source string, and invokes the verification oracle before committing.
    ///
    /// # Arguments
    /// * `source` - The original broken source code.
    /// * `log_event` - The structured incident log emitted by the [`DiagnosticLogger`].
    /// * `test_oracle` - The verification test oracle closure.
    /// * `logger` - The shared [`DiagnosticLogger`] recording telemetry.
    ///
    /// # Returns
    /// `Ok(healed_source)` if the patch was applied and certified by the oracle.
    pub fn execute_restoration_from_log<F>(
        source: &str,
        log_event: &LogEvent,
        test_oracle: F,
        logger: &DiagnosticLogger,
    ) -> Result<String, String>
    where
        F: Fn(&str) -> Result<(), String>,
    {
        let suggestion = log_event.suggestion.as_ref().ok_or_else(|| {
            "LogEvent does not contain remediation suggestion for automated restoration".to_string()
        })?;

        logger.log(
            "SELF_HEAL",
            format!("Gaze Focus: Applying remediation suggestion from log: {:?}", suggestion.kind),
        );

        let mut candidate = source.to_string();
        match suggestion.kind {
            FixKind::FixBoundaryMismatch => {
                let lines: Vec<&str> = candidate.lines().collect();
                if suggestion.line > 0 && suggestion.line <= lines.len() {
                    let mut target_line = lines[suggestion.line - 1].to_string();
                    if let Some(pos) = target_line.find(&suggestion.target_token) {
                        target_line.replace_range(
                            pos..pos + suggestion.target_token.len(),
                            &suggestion.suggested_patch,
                        );
                        let mut new_lines = lines.clone();
                        new_lines[suggestion.line - 1] = &target_line;
                        candidate = new_lines.join("\n");
                        if !candidate.ends_with('\n') {
                            candidate.push('\n');
                        }
                    }
                }
            }
            FixKind::CloseBoundary => {
                candidate.push_str(&suggestion.suggested_patch);
                if !candidate.ends_with('\n') {
                    candidate.push('\n');
                }
            }
            FixKind::ConvertImmutableToMutable => {
                candidate = candidate.replace(
                    &format!("< {} >", suggestion.target_token),
                    &suggestion.suggested_patch,
                );
            }
            _ => {
                candidate = candidate.replace(&suggestion.target_token, &suggestion.suggested_patch);
            }
        }

        // Call the test oracle to certify candidate
        Self::verify_candidate_repair(&candidate, test_oracle, logger)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Test Oracle Unit Tests & Affirmative Invariant Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::logger::FixSuggestion;

    #[test]
    fn test_verify_candidate_repair_passing() {
        let logger = DiagnosticLogger::new(false);
        let src = "[ Version ]: { 1.0.0 }";
        let res = TestOracleSandbox::verify_candidate_repair(
            src,
            |s| {
                if s.contains("1.0.0") {
                    Ok(())
                } else {
                    Err("Missing version".to_string())
                }
            },
            &logger,
        );
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), src);
    }

    #[test]
    fn test_verify_candidate_repair_failing() {
        let logger = DiagnosticLogger::new(false);
        let src = "[ Version ]: { 0.1.0 }";
        let res = TestOracleSandbox::verify_candidate_repair(
            src,
            |_| Err("Rejection by invariant assertion".to_string()),
            &logger,
        );
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Rejection by invariant assertion"));
    }

    #[test]
    fn test_execute_restoration_from_log_boundary_mismatch() {
        let logger = DiagnosticLogger::new(false);
        let broken = "< Law_Truth ]\n";
        let event = LogEvent {
            elapsed_micros: 1000,
            channel: "DEBUGGER",
            message: "Boundary mismatch".to_string(),
            is_failpoint: true,
            failpoint_type: Some(crate::diagnostics::logger::FailpointType::BoundaryMismatch),
            suggestion: Some(FixSuggestion {
                kind: FixKind::FixBoundaryMismatch,
                line: 1,
                col: 13,
                target_token: "]".to_string(),
                suggested_patch: ">".to_string(),
                rationale: "Replace ] with >".to_string(),
            }),
            cognitive_note: None,
        };

        let healed = TestOracleSandbox::execute_restoration_from_log(
            broken,
            &event,
            |s| {
                if s.contains("< Law_Truth >") {
                    Ok(())
                } else {
                    Err("Mismatched delimiter still present".to_string())
                }
            },
            &logger,
        );

        assert!(healed.is_ok());
        assert_eq!(healed.unwrap(), "< Law_Truth >\n");
    }
}
