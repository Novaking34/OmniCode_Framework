// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( ImmutableRegistry, Rank 1 Axiom Protection & Mutation Veto Engine )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "For I am the LORD, I change not; therefore ye sons of Jacob are not consumed."
//   — Malachi 3:6 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   The immutability enforcement organ of the OmniDebugger.
//   Acts as the unblinking eye guarding Rank 1 Invariant Axioms:
//     - Invariants sealed inside `< ... >` (Laws, Moral Axioms, Truth Types)
//       are registered into the immutable registry.
//     - Any subsequent write or assignment operation targeting a registered
//       invariant triggers an immediate mutation veto (`-1:ROLLBACK`).
//     - Synthesizes a FixSuggestion proposing conversion to a mutable
//       Rank 2 Denotation `[ ... ]` if mutation was genuinely intended.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Standard Collections & Invariant Registry Model )
// ==============================================================================
use std::collections::HashSet;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType, FixKind, FixSuggestion};

/// Registry of identifiers declared as immutable Rank 1 Invariants `< ... >`.
#[derive(Debug, Clone, Default)]
pub struct ImmutableRegistry {
    /// Set of entity names sealed against state mutation.
    pub entities: HashSet<String>,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Invariant Registration, Mutation Authorization & Remediation Generation )
// ==============================================================================
impl ImmutableRegistry {
    /// Constructs a new empty [`ImmutableRegistry`].
    pub fn new() -> Self {
        Self { entities: HashSet::new() }
    }

    /// Registers an entity identifier as globally immutable Rank 1 axiom.
    pub fn register(&mut self, name: &str, logger: &DiagnosticLogger) {
        self.entities.insert(name.to_string());
        logger.log(
            "IMMUTABILITY",
            format!("Gaze Focus: Registered immutable invariant <{name}>"),
        );
    }

    /// Checks whether an entity identifier is currently protected as immutable.
    pub fn contains(&self, name: &str) -> bool {
        self.entities.contains(name)
    }

    /// Returns the total count of registered immutable entities.
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Inspects whether any immutable entities are registered.
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Verifies that mutation is permitted for the given entity identifier.
    ///
    /// # Failpoints Intercepted
    /// If the entity was registered as immutable, traps [`FailpointType::ImmutableMutation`],
    /// and suggests converting the declaration to a mutable Rank 2 denotation `[ ... ]`.
    pub fn assert_permitted(
        &self,
        name: &str,
        logger: &DiagnosticLogger,
    ) -> Result<(), (String, FixSuggestion)> {
        if self.entities.contains(name) {
            let msg = format!(
                "Illegal mutation attempted on immutable invariant <{name}>! Entity cannot be changed."
            );
            let suggestion = FixSuggestion {
                kind: FixKind::ConvertImmutableToMutable,
                line: 0,
                col: 0,
                target_token: name.to_string(),
                suggested_patch: format!("[ {name} ]"),
                rationale: format!(
                    "Entity <{name}> was sealed as a Rank 1 immutable invariant; declare it with '[ {name} ]' if updates are required"
                ),
            };

            logger.failpoint_with_remediation(
                FailpointType::ImmutableMutation,
                "DEBUGGER",
                &msg,
                suggestion.clone(),
            );

            return Err((format!("FAILPOINT: {msg}"), suggestion));
        }
        Ok(())
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Immutability Unit Tests & Affirmative Invariant Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_registry_lifecycle() {
        let logger = DiagnosticLogger::new(false);
        let mut registry = ImmutableRegistry::new();

        assert!(!registry.contains("Law_Truth"));
        assert_eq!(registry.len(), 0);

        registry.register("Law_Truth", &logger);
        assert!(registry.contains("Law_Truth"));
        assert_eq!(registry.len(), 1);

        // Mutable check for unsealed symbol passes
        assert!(registry.assert_permitted("Accumulator", &logger).is_ok());

        // Mutation attempt on Law_Truth is strictly vetoed
        let res = registry.assert_permitted("Law_Truth", &logger);
        assert!(res.is_err());

        let (msg, suggestion) = res.err().unwrap();
        assert!(msg.contains("Illegal mutation attempted on immutable invariant <Law_Truth>"));
        assert_eq!(suggestion.kind, FixKind::ConvertImmutableToMutable);
        assert_eq!(suggestion.suggested_patch, "[ Law_Truth ]");
    }
}
