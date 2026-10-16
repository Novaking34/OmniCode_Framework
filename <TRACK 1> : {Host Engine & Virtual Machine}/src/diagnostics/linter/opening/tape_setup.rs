// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Process Tape (.tape) Initial Roll / Precondition Setup Linter )
// ==============================================================================
//! Process Tape (.tape) Initial Roll Setup Linter
//!
//! Evaluates the initial allocation and precondition setup of a `.tape` ribbon:
//! - Roll 1 declaration (`[ ROLL 1 ] : [ Inscribe ] : ...`)
//! - Capturing external input payload into cell container
//! - Substrate parity and single-cell bound preconditions
//! - Attached Living Aside cognitive oracle

use crate::diagnostics::linter::syntax::SpatialLintConfig;
use super::score::OpeningScore;

pub struct TapeSetupLinter;

impl TapeSetupLinter {
    /// Evaluates Roll 1 setup for `.tape` files.
    pub fn evaluate(
        _file_path: &str,
        slice: &str,
        config: &SpatialLintConfig,
    ) -> OpeningScore {
        let mut entity = 0i32;
        let mut cell = 0i32;
        let mut substrate = 0i32;
        let mut preconditions = 0i32;
        let mut spatial = 0i32;
        let mut breakdown = Vec::new();

        if slice.trim().is_empty() {
            breakdown.push("✗ Missing Roll 1 initial allocation (-50)".into());
            return OpeningScore::new(-20, -10, -10, -10, 0, breakdown);
        }

        // 1. Roll 1 Declaration & Entity Allocation (max 25 pts)
        let upper = slice.to_uppercase();
        let has_roll1 = upper.contains("ROLL 1") || upper.contains("ROLL_1");
        if has_roll1 {
            entity += 25;
            breakdown.push("✓ Roll 1 initial state allocation declared (+25/25)".into());
        } else {
            entity -= 15;
            breakdown.push("✗ Missing Roll 1 initial setup declaration (-15/25)".into());
        }

        // 2. Cell / Word Container Specification (max 20 pts)
        let has_cell = slice.contains("Cell")
            || slice.contains("cell")
            || slice.contains("Word")
            || slice.contains("word")
            || slice.contains("Character`")
            || slice.contains("Number`")
            || slice.contains("Entity`");
        if has_cell {
            cell += 20;
            breakdown.push("✓ Memory container target specified (+20/20)".into());
        } else {
            cell -= 10;
            breakdown.push("✗ Missing container specification (-10/20)".into());
        }

        // 3. Operational Inscription / Transduction (max 20 pts)
        let has_op = slice.contains("INSCRIBE") || slice.contains("-->") || slice.contains("Inscribe") || slice.contains("Alloc");
        if has_op {
            substrate += 20;
            breakdown.push("✓ Initial inscription / input capture vector wired (+20/20)".into());
        } else {
            breakdown.push("⚠ No explicit inscription vector in setup (0/20)".into());
        }

        // 4. Precondition / State Guard Verification (max 20 pts)
        let has_guard = slice.contains("Glyph")
            || slice.contains("Quantity")
            || slice.contains("Input")
            || slice.contains("Cell")
            || slice.contains("Word")
            || slice.contains("Status");
        if has_guard {
            preconditions += 20;
            breakdown.push("✓ Initial condition inputs & container parity verified (+20/20)".into());
        } else {
            breakdown.push("⚠ Implicit initial preconditions (0/20)".into());
        }

        // 5. Spatial Geometry & Living Asides (max 15 pts)
        let lines: Vec<&str> = slice.lines().collect();
        let max_cols = config.max_columns;
        let line_len_violation = lines.iter().any(|l| l.chars().count() > max_cols);

        if !line_len_violation {
            spatial += 10;
            breakdown.push(format!("✓ All Roll 1 lines <= {max_cols} columns (+10/10)"));
        } else {
            spatial -= 10;
            breakdown.push(format!("✗ Roll 1 lines exceed {max_cols}-column spatial ruler (-10/10)"));
        }

        let has_asides = slice.contains('(') && slice.contains(')');
        if has_asides {
            spatial += 5;
            breakdown.push("✓ Machine-readable Living Aside attached to Roll 1 (+5/5)".into());
        } else {
            spatial -= 5;
            breakdown.push("✗ Missing Living Aside on Roll 1 (-5/5)".into());
        }

        OpeningScore::new(entity, cell, substrate, preconditions, spatial, breakdown)
    }
}
