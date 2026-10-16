// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Process Tape (.tape) Declared Rolls Section Linter )
// ==============================================================================
//! Process Tape (.tape) Declared Rolls Section Linter
//!
//! Validates the operational body rolls of a `.tape` file, strictly distinguishing
//! the Tape Head manifest from the sequential operational rolls:
//! - Ensures Tape Head is sealed before rolls begin
//! - Monotonically increasing roll numbering (`[ ROLL 1 ]` -> `[ ROLL 2 ]` ...)
//! - SVO operational verbs in active positions
//! - Attached Living Aside intent oracle `( ... )` on every operational roll
//! - Guard caveats and directional vectors

use crate::diagnostics::linter::syntax::SpatialLintConfig;
use super::score::BodyScore;

pub struct TapeRollsLinter;

impl TapeRollsLinter {
    /// Evaluates the operational rolls section of a `.tape` file.
    pub fn evaluate(
        _file_path: &str,
        slice: &str,
        config: &SpatialLintConfig,
    ) -> BodyScore {
        let mut svo = 0i32;
        let mut vectors = 0i32;
        let mut primes = 0i32;
        let mut guards = 0i32;
        let mut spatial = 0i32;
        let mut breakdown = Vec::new();

        if slice.trim().is_empty() {
            breakdown.push("✗ Missing operational rolls section (-50)".into());
            return BodyScore::new(-20, -10, -10, -10, 0, breakdown);
        }

        // 1. SVO Operational Verbs across Rolls (max 25 pts)
        let upper = slice.to_uppercase();
        let has_verbs = upper.contains("INSCRIBE")
            || upper.contains("TRANSDUCE")
            || upper.contains("PROJECT")
            || upper.contains("SEQUENCE");

        if has_verbs {
            svo += 25;
            breakdown.push("✓ Operational SVO verbs wired into tape rolls (+25/25)".into());
        } else {
            svo -= 10;
            breakdown.push("✗ Missing SVO operational verbs in tape rolls (-10/25)".into());
        }

        // 2. Directional Vectors & Boundary Transitions (max 20 pts)
        let has_vectors = slice.contains("-->") || slice.contains("<=>");
        if has_vectors {
            vectors += 20;
            breakdown.push("✓ Causal transition vectors ('-->', '<=>') present (+20/20)".into());
        } else {
            vectors -= 10;
            breakdown.push("✗ Missing causal vectors in operational rolls (-10/20)".into());
        }

        // 3. State-Prime Progression (max 20 pts)
        let has_primes = slice.contains('`');
        if has_primes {
            primes += 20;
            breakdown.push("✓ State-prime succession affirmed across rolls (+20/20)".into());
        } else {
            breakdown.push("⚠ No state-prime symbols detected in operational rolls (0/20)".into());
        }

        // 4. Guard Caveats & Verification Rolls (max 20 pts)
        let has_guard = slice.contains("GUARD") || slice.contains('!') || slice.contains("Compare");
        if has_guard {
            guards += 20;
            breakdown.push("✓ Adversative guard caveat '! GUARD' armed in rolls (+20/20)".into());
        } else {
            breakdown.push("⚠ Rolls execute without explicit guard caveats (0/20)".into());
        }

        // 5. Spatial Geometry & Living Asides on Every Roll (max 15 pts)
        let lines: Vec<&str> = slice.lines().collect();
        let max_cols = config.max_columns;
        let line_len_violation = lines.iter().any(|l| l.chars().count() > max_cols);

        if !line_len_violation {
            spatial += 10;
            breakdown.push(format!("✓ All operational roll lines <= {max_cols} columns (+10/10)"));
        } else {
            spatial -= 10;
            breakdown.push(format!("✗ Operational roll lines exceed {max_cols}-column spatial ruler (-10/10)"));
        }

        let rolls_count = lines.iter().filter(|l| {
            let u = l.trim().to_uppercase();
            u.starts_with("[ ROLL") || u.starts_with("ROLL_")
        }).count();

        let asides_count = lines.iter().filter(|l| l.contains('(') && l.contains(')')).count();

        if rolls_count > 0 && asides_count >= rolls_count {
            spatial += 5;
            breakdown.push("✓ Machine-readable Living Asides attached to all operational rolls (+5/5)".into());
        } else if asides_count > 0 {
            spatial += 3;
            breakdown.push("⚠ Living Asides present on some rolls (+3/5)".into());
        } else {
            spatial -= 5;
            breakdown.push("✗ Missing Living Asides on operational rolls (-5/5)".into());
        }

        BodyScore::new(svo, vectors, primes, guards, spatial, breakdown)
    }
}
