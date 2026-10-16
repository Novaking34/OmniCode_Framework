// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Process Tape (.tape) Terminal Commit Status Roll Linter )
// ==============================================================================
//! Process Tape (.tape) Terminal Commit Status Roll Linter
//!
//! Validates the terminal sealing roll of a `.tape` process ribbon:
//! - Terminal roll identity (`[ ROLL N ] : [ Commit Status ] : [ ... ]`)
//! - Verification of preceding rolls achieving `+1:PROCEED`
//! - Tripwire circuit breaker arming
//! - Fourfold state machine completeness:
//!   `+1:PROCEED`, `0:AWAIT`, `NULL:HALT`, `-1:ROLLBACK`
//! - Attached Living Aside intent oracle

use crate::diagnostics::linter::syntax::SpatialLintConfig;
use super::score::ClosingScore;

pub struct TapeCommitLinter;

impl TapeCommitLinter {
    /// Evaluates the terminal commit status roll of a `.tape` file.
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
            breakdown.push("✗ Missing terminal commit status roll (-50)".into());
            return ClosingScore::new(-20, -10, -10, -10, 0, breakdown);
        }

        // 1. Terminal Roll Identity & Verification (max 20 pts)
        let upper = slice.to_uppercase();
        let is_commit = upper.contains("COMMIT STATUS") || upper.contains("COMMIT_STATUS");
        if is_commit {
            verify += 20;
            breakdown.push("✓ Terminal Commit Status roll verified (+20/20)".into());
        } else {
            verify -= 10;
            breakdown.push("✗ Terminal roll is not a designated Commit Status roll (-10/20)".into());
        }

        // 2. Preceding Roll Affirmation (max 20 pts)
        let has_affirm = slice.contains("Verify") || slice.contains("PROCEED") || slice.contains("+1");
        if has_affirm {
            validate += 20;
            breakdown.push("✓ Affirmative execution condition verified (+20/20)".into());
        } else {
            breakdown.push("⚠ Implicit execution conditions (0/20)".into());
        }

        // 3. Tripwire Circuit Breaker Guard (max 20 pts)
        let has_tripwire = slice.contains("Tripwire") || slice.contains("ARMED") || slice.contains("TRIPPED");
        if has_tripwire {
            tripwire += 20;
            breakdown.push("✓ Zero-tolerance tripwire circuit breaker armed (+20/20)".into());
        } else {
            breakdown.push("⚠ Terminal commit operates without explicit tripwire check (0/20)".into());
        }

        // 4. Fourfold State Machine Completeness (max 25 pts)
        let states = ["PROCEED", "AWAIT", "HALT", "ROLLBACK"];
        let present_states = states.iter().filter(|&&s| upper.contains(s)).count();

        if present_states == 4 {
            fourfold += 25;
            breakdown.push("✓ Complete Balanced Ternary Fourfold State Machine (T4) affirmed (+25/25)".into());
        } else if present_states >= 2 {
            let pts = (present_states as i32) * 5;
            fourfold += pts;
            breakdown.push(format!("⚠ Partial commit state machine ({present_states}/4 states) (+{pts}/25)"));
        } else if is_commit {
            fourfold += 10;
            breakdown.push("✓ Basic commit status declared (+10/25)".into());
        } else {
            fourfold -= 15;
            breakdown.push("✗ Missing fourfold commit state machine (-15/25)".into());
        }

        // 5. Spatial Geometry & Living Asides (max 15 pts)
        let lines: Vec<&str> = slice.lines().collect();
        let max_cols = config.max_columns;
        let line_len_violation = lines.iter().any(|l| l.chars().count() > max_cols);

        if !line_len_violation {
            telemetry += 8;
            breakdown.push(format!("✓ All commit status lines <= {max_cols} columns (+8/8)"));
        } else {
            telemetry -= 8;
            breakdown.push(format!("✗ Commit status lines exceed {max_cols}-column spatial ruler (-8/8)"));
        }

        let has_aside = slice.contains('(') && slice.contains(')');
        if has_aside {
            telemetry += 7;
            breakdown.push("✓ Machine-readable Living Aside attached to terminal commit (+7/7)".into());
        } else {
            telemetry -= 5;
            breakdown.push("✗ Missing Living Aside on terminal commit (-5/7)".into());
        }

        ClosingScore::new(verify, validate, tripwire, fourfold, telemetry, breakdown)
    }
}
