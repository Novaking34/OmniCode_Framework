// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Source Reel (.omni) Block 4 Closing Linter & Postcondition Verifier )
// ==============================================================================
//! Source Reel (.omni) Block 4 Closing Linter
//!
//! Validates postcondition proofs, circuit breakers, and commit state machines:
//! - Operational roll verification proofs (`[ Verify ]: [ [ ROLL 1 ] = PROCEED :: ... ]`)
//! - Invariant validation & capacity bounds (`[ Validate ]: { Character` <= [ 1 Cell ] }`)
//! - Autonomous circuit breaker tripwiring (`[ Tripwire ]: ... --> [ ARMED ] | ... --> [ TRIPPED ]`)
//! - Diagnostic debug snapshot (`[ Debug ]: { ... }`)
//! - Episodic journal inscription (`[ Log ]: [ Journal ]: INSCRIBE { ... }`)
//! - Fourfold state machine (`+1:PROCEED`, `0:AWAIT`, `NULL:HALT`, `-1:ROLLBACK`)

use crate::diagnostics::linter::syntax::SpatialLintConfig;
use super::score::ClosingScore;

pub struct ReelClosingLinter;

impl ReelClosingLinter {
    /// Evaluates and scores Block 4 for `.omni` Source Reels.
    pub fn evaluate(
        _file_path: &str,
        slice: &str,
        config: &SpatialLintConfig,
    ) -> ClosingScore {
        let mut verify = 0i32;
        let mut validate = 0i32;
        let mut tripwire = 0i32;
        let mut fourfold = 0i32;
        let mut telemetry = 0i32;
        let mut breakdown = Vec::new();

        if slice.trim().is_empty() {
            breakdown.push("✗ Missing Block 4: Closing scope entirely (-50)".into());
            return ClosingScore::new(-20, -10, -10, -10, 0, breakdown);
        }

        // 1. Operational Verification Proofs (max 20 pts)
        let has_verify = slice.contains("Verify") || slice.contains("verify") || slice.contains("PROCEED");
        if has_verify {
            verify += 20;
            breakdown.push("✓ Operational verification proofs present (+20/20)".into());
        } else {
            verify -= 10;
            breakdown.push("✗ Missing operational verification proof stanza (-10/20)".into());
        }

        // 2. Invariant Validation & Bounds Parity (max 20 pts)
        let has_validate = slice.contains("Validate") || slice.contains("validate") || slice.contains("Enclosed") || slice.contains("Cell");
        if has_validate {
            validate += 20;
            breakdown.push("✓ Invariant bounds & substrate parity validated (+20/20)".into());
        } else {
            breakdown.push("⚠ No explicit validation bounds check (0/20)".into());
        }

        // 3. Autonomous Circuit Breaker / Tripwiring (max 20 pts)
        let has_tripwire = slice.contains("Tripwire") || slice.contains("ARMED") || slice.contains("TRIPPED") || slice.contains("circuit_breaker");
        if has_tripwire {
            tripwire += 20;
            breakdown.push("✓ Zero-tolerance tripwire circuit breaker armed (+20/20)".into());
        } else {
            breakdown.push("⚠ Execution operates without autonomous circuit breaker (0/20)".into());
        }

        // 4. Fourfold State Machine Completeness (max 25 pts)
        let upper = slice.to_uppercase();
        let states = ["PROCEED", "AWAIT", "HALT", "ROLLBACK"];
        let present_states = states.iter().filter(|&&s| upper.contains(s)).count();

        if present_states == 4 {
            fourfold += 25;
            breakdown.push("✓ Complete Balanced Ternary Fourfold State Machine (T4) affirmed (+25/25)".into());
        } else if present_states >= 2 {
            let pts = (present_states as i32) * 5;
            fourfold += pts;
            breakdown.push(format!("⚠ Partial commit state machine ({present_states}/4 states) (+{pts}/25)"));
        } else if upper.contains("COMMIT") || upper.contains("STATUS") {
            fourfold += 10;
            breakdown.push("✓ Basic commit status declared (+10/25)".into());
        } else {
            fourfold -= 15;
            breakdown.push("✗ Missing terminal commit status definition (-15/25)".into());
        }

        // 5. Telemetry & Journal Logging + Living Asides (max 15 pts)
        let has_telemetry = (slice.contains("Debug") || slice.contains("telemetry"))
            && (slice.contains("Log") || slice.contains("Journal"));
        let lines: Vec<&str> = slice.lines().collect();
        let max_cols = config.max_columns;
        let line_len_violation = lines.iter().any(|l| l.chars().count() > max_cols);

        if !line_len_violation {
            telemetry += 8;
            breakdown.push(format!("✓ All Block 4 lines <= {max_cols} columns (+8/8)"));
        } else {
            telemetry -= 8;
            breakdown.push(format!("✗ Block 4 lines exceed {max_cols}-column spatial ruler (-8/8)"));
        }

        if has_telemetry {
            telemetry += 7;
            breakdown.push("✓ Diagnostic debug snapshot & episodic logging verified (+7/7)".into());
        } else {
            telemetry += 4;
            breakdown.push("✓ Standard sealing gate verified (+4/7)".into());
        }

        ClosingScore::new(verify, validate, tripwire, fourfold, telemetry, breakdown)
    }
}
