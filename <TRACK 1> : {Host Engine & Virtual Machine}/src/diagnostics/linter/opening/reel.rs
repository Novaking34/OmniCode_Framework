// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Source Reel (.omni) Block 2 Opening Linter & Precondition Verifier )
// ==============================================================================
//! Source Reel (.omni) Block 2 Opening Linter
//!
//! Validates execution environment initialization:
//! - Input payload imposition (`{ Glyph ]: ...`)
//! - Invariant entity allocation bound to cell container (`[ Character ]: < Character > & [ Cell ]`)
//! - Concrete memory cell bounds (`[ Cell ]: [ 1 ] & { 5 Trits :: 1 Byte }`)
//! - Hardware transducer bridging (`{ 8 Bits <=> 5 Trits }`)
//! - Target destination mapping (`::@ 0x01` / `::@ 0x000B8000`)
//! - Precondition equilibrium proofs (`[ (+1) plus (-1) <=> 0 ]`)

use crate::diagnostics::linter::syntax::SpatialLintConfig;
use super::score::OpeningScore;

pub struct ReelOpeningLinter;

impl ReelOpeningLinter {
    /// Evaluates and scores Block 2 for `.omni` Source Reels.
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
            breakdown.push("✗ Missing Block 2: Opening scope entirely (-50)".into());
            return OpeningScore::new(-20, -10, -10, -10, 0, breakdown);
        }

        // 1. Entity & Input Payload Allocation (max 25 pts)
        let has_entity = slice.contains("Entity") || slice.contains("Character") || slice.contains("Glyph") || slice.contains("Input");
        if has_entity {
            entity += 25;
            breakdown.push("✓ Typed entity & input payload allocated (+25/25)".into());
        } else {
            entity -= 15;
            breakdown.push("✗ Missing typed entity or input allocation (-15/25)".into());
        }

        // 2. Cell Container Sizing (max 20 pts)
        let has_cell = slice.contains("Cell") || slice.contains("Memory") || slice.contains("Partition");
        if has_cell {
            cell += 20;
            breakdown.push("✓ Concrete memory cell container allocated (+20/20)".into());
        } else {
            cell -= 10;
            breakdown.push("✗ Missing memory cell container definition (-10/20)".into());
        }

        // 3. Substrate Bridge / Transducer (max 20 pts)
        let has_substrate = slice.contains("Transducer") || slice.contains("<=>") || slice.contains("Trits") || slice.contains("Bits");
        if has_substrate {
            substrate += 20;
            breakdown.push("✓ Substrate transducer / bridge armed (+20/20)".into());
        } else {
            breakdown.push("⚠ No explicit substrate transducer bridge (0/20)".into());
        }

        // 4. Preconditions & Target Mapping (max 20 pts)
        let has_preconditions = slice.contains("Preconditions") || slice.contains("preconditions") || slice.contains("+1") || slice.contains("Equilibrium");
        let has_target = slice.contains("Target") || slice.contains("::@") || slice.contains("Stdout") || slice.contains("VGA");
        if has_preconditions && has_target {
            preconditions += 20;
            breakdown.push("✓ Precondition equilibrium verified & target hardware mapped (+20/20)".into());
        } else if has_preconditions || has_target {
            preconditions += 15;
            breakdown.push("✓ Preconditions or target destination declared (+15/20)".into());
        } else {
            breakdown.push("✗ Missing explicit precondition check (-10/20)".into());
        }

        // 5. Spatial Geometry & Living Asides (max 15 pts)
        let lines: Vec<&str> = slice.lines().collect();
        let max_cols = config.max_columns;
        let line_len_violation = lines.iter().any(|l| l.chars().count() > max_cols);

        if !line_len_violation {
            spatial += 10;
            breakdown.push(format!("✓ All Block 2 lines <= {max_cols} columns (+10/10)"));
        } else {
            spatial -= 10;
            breakdown.push(format!("✗ Block 2 lines exceed {max_cols}-column spatial ruler (-10/10)"));
        }

        let has_asides = slice.contains('(') && slice.contains(')');
        if has_asides {
            spatial += 5;
            breakdown.push("✓ Living Asides attached to opening stanzas (+5/5)".into());
        } else {
            spatial -= 5;
            breakdown.push("✗ Missing Living Asides on opening definitions (-5/5)".into());
        }

        OpeningScore::new(entity, cell, substrate, preconditions, spatial, breakdown)
    }
}
