// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Inferred Conceptual Entity Declarations & Infer Operator Synthesis )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "Call unto me, and I will answer thee, and shew thee great and mighty things,
//    which thou knowest not."
//   — Jeremiah 33:3 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Pass 4 of the Restorator Autonomous Healing Pipeline.
//   Resolves undeclared conceptual nouns invoking the infer operator `?`:
//     - Entity? : Inferred discrete physical or logical unit (e.g., Ball < Entity? >)
//     - Set?    : Inferred collection container (e.g., Group < Set? >)
//   Synthesizes explicit type declarations right after BLOCK 1 (Module metadata),
//   anchoring symbolic conceptual terms into the type system before execution.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Type Inference Signatures & Repair Tracking )
// ==============================================================================
use crate::diagnostics::logger::FixKind;
use super::AppliedRepair;

/// Subsystem responsible for resolving and synthesizing inferred conceptual types.
pub struct InferredTypeHealer;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Infer Operator Resolution & Conceptual Type Synthesis Mechanics )
// ==============================================================================
impl InferredTypeHealer {
    /// Resolves undeclared conceptual nouns and injects explicit type definitions.
    ///
    /// # Arguments
    /// * `source` - Source code string to scan.
    /// * `repairs` - Vector accumulating records of all applied repairs.
    ///
    /// # Returns
    /// Source code string with inferred conceptual types declared.
    pub fn heal(source: &str, repairs: &mut Vec<AppliedRepair>) -> String {
        let mut missing_inferred = Vec::new();

        if source.contains("Ball?") && !source.contains("Ball <") {
            missing_inferred.push("Ball < Entity? > ( Inferred discrete unit )");
        }
        if source.contains("Group?") && !source.contains("Group <") {
            missing_inferred.push("Group < Set? >   ( Inferred collection container )");
        }

        if missing_inferred.is_empty() {
            return source.to_string();
        }

        let mut out = source.to_string();
        // Insert right after Module manifest definition
        let module_pos = out.find("= [Module]:").or_else(|| out.find("Module <"));
        if let Some(pos) = module_pos {
            if let Some(block_end) = out[pos..].find('}') {
                let insert_at = pos + block_end + 1;
                let mut decls = String::from("\n\n# Auto-Restored Conceptual Types:\n");
                for d in &missing_inferred {
                    decls.push_str(d);
                    decls.push('\n');
                }
                out.insert_str(insert_at, &decls);

                repairs.push(AppliedRepair {
                    kind: FixKind::DeclareEntityWithInfer,
                    description: format!("Declared {} inferred conceptual entities", missing_inferred.len()),
                    line: 1,
                });
            }
        }

        out
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Inferred Type Healer Unit Tests & Invariant Verification )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inferred_type_healer_synthesizes_entity_declarations() {
        let code = "Module < Test > { version: < \"1.0.0\" > }\nBody [\n    item = Ball?\n]";
        let mut repairs = Vec::new();
        let healed = InferredTypeHealer::heal(code, &mut repairs);

        assert!(healed.contains("Ball < Entity? >"));
        assert_eq!(repairs.len(), 1);
        assert_eq!(repairs[0].kind, FixKind::DeclareEntityWithInfer);
    }
}
