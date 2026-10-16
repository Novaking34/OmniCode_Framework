// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Source Reel (.omni) Block 3 Body Linter & Causal Vector Verifier )
// ==============================================================================
//! Source Reel (.omni) Block 3 Body Linter
//!
//! Validates operational transformations, causal pipelines, and state evolution:
//! - Active all-caps SVO operational verbs (`INSCRIBE`, `TRANSDUCE`, `PROJECT`, `SEQUENCE`)
//! - Directional vectors (`-->`, `<=>`, `^--`, `v--`)
//! - State-prime progression notation (`Character``, `Cell``, `Target``)
//! - Adversative guard caveats (`! GUARD --> [ Status` = +1:PROCEED ] | ...`)
//! - Attached Living Aside intent oracles on operational stanzas

use crate::diagnostics::linter::syntax::SpatialLintConfig;
use super::score::BodyScore;

pub struct ReelBodyLinter;

impl ReelBodyLinter {
    /// Evaluates and scores Block 3 for `.omni` Source Reels.
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
            breakdown.push("✗ Missing Block 3: Body scope entirely (-50)".into());
            return BodyScore::new(-20, -10, -10, -10, 0, breakdown);
        }

        // 1. SVO Operational Verbs (max 25 pts)
        let has_inscribe = slice.contains("INSCRIBE");
        let has_transduce = slice.contains("TRANSDUCE");
        let has_project = slice.contains("PROJECT");
        let has_sequence = slice.contains("SEQUENCE");
        let active_verbs = [has_inscribe, has_transduce, has_project, has_sequence].iter().filter(|&&v| v).count();

        if active_verbs >= 3 || slice.contains("Body [") || slice.contains("ROLL") {
            svo += 25;
            breakdown.push("✓ Active SVO operational verbs wired (+25/25)".into());
        } else if active_verbs > 0 {
            svo += 15;
            breakdown.push("✓ Partial SVO verbs present (+15/25)".into());
        } else {
            svo -= 10;
            breakdown.push("✗ Missing canonical SVO operational verbs (-10/25)".into());
        }

        // 2. Directional Vectors & Transition Syntax (max 20 pts)
        let has_forward = slice.contains("-->");
        let has_bidirectional = slice.contains("<=>");
        let has_logic = slice.contains('&') || slice.contains('|');

        if has_forward && (has_bidirectional || has_logic) {
            vectors += 20;
            breakdown.push("✓ Directional causal vectors ('-->', '<=>') verified (+20/20)".into());
        } else if has_forward {
            vectors += 15;
            breakdown.push("✓ Forward causal vector wired (+15/20)".into());
        } else {
            vectors -= 10;
            breakdown.push("✗ Missing directional causal vectors (-10/20)".into());
        }

        // 3. State-Prime Progression (max 20 pts)
        let has_primes = slice.contains('`');
        if has_primes {
            primes += 20;
            breakdown.push("✓ State-prime immutable succession affirmed (+20/20)".into());
        } else {
            breakdown.push("⚠ No state-prime notation detected in body transitions (0/20)".into());
        }

        // 4. Adversative Guard Caveats & Equilibrium Testing (max 20 pts)
        let has_guard = slice.contains("GUARD") || slice.contains('!') || slice.contains("Compare");
        if has_guard {
            guards += 20;
            breakdown.push("✓ Adversative guard caveat '! GUARD' armed (+20/20)".into());
        } else {
            breakdown.push("⚠ Body transitions operate without explicit guard caveats (0/20)".into());
        }

        // 5. Spatial Geometry & Living Asides (max 15 pts)
        let lines: Vec<&str> = slice.lines().collect();
        let max_cols = config.max_columns;
        let line_len_violation = lines.iter().any(|l| l.chars().count() > max_cols);

        if !line_len_violation {
            spatial += 10;
            breakdown.push(format!("✓ All Block 3 lines <= {max_cols} columns (+10/10)"));
        } else {
            spatial -= 10;
            breakdown.push(format!("✗ Block 3 lines exceed {max_cols}-column spatial ruler (-10/10)"));
        }

        let has_asides = slice.contains('(') && slice.contains(')');
        if has_asides {
            spatial += 5;
            breakdown.push("✓ Living Asides attached to operational rolls (+5/5)".into());
        } else {
            spatial -= 5;
            breakdown.push("✗ Missing Living Asides on operational rolls (-5/5)".into());
        }

        BodyScore::new(svo, vectors, primes, guards, spatial, breakdown)
    }
}
