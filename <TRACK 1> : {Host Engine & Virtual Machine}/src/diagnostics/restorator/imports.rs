// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Lexicon Entity Binding, Substrate Linking & Legacy Syntax Sanitization )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "Supply that which is lacking in your faith."
//   — 1 Thessalonians 3:10 (KJV)
//
//   "Let all things be done decently and in order."
//   — 1 Corinthians 14:40 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Pass 3 of the Restorator Autonomous Healing Pipeline.
//   Strictly adheres to the canonical syntax established in character.omni and number.omni.
//   - Zero Tolerance for Obsolete `Core.` Namespaces:
//     Arithmetic (`plus`, `minus`, `times`, `div`), ternary logic (`PROCEED`, `ROLLBACK`,
//     `AWAIT`, `HALT`), and directional vectors (`-->`, `<--`, `<=>`) are native language
//     primitives, NOT module imports. Any legacy `Core.Arithmetic`, `Core.Logic`, `Core.Vectors`,
//     or `OmniCode.Core` references are sanitized and removed.
//   - Canonical Lexicon Entity Allocation:
//     When source operations reference fundamental Lexicon types (`Character`, `Cell`,
//     `Number`, `Memory Word`, `ALU`, `Transducer`) that lack environmental bindings,
//     ImportHealer injects their canonical definitions into BLOCK 2: OPENING BLOCK.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Dependency Rule Registries & Applied Repair Types )
// ==============================================================================
use crate::diagnostics::logger::FixKind;
use super::AppliedRepair;

/// Subsystem responsible for analyzing Lexicon substrate dependencies and purging obsolete `Core.` syntax.
pub struct ImportHealer;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Lexicon Entity Binding Injection & Core. Sanitization Mechanics )
// ==============================================================================
impl ImportHealer {
    /// Ingests source code, purges obsolete `Core.` syntax, and binds missing Lexicon substrate entities.
    ///
    /// # Arguments
    /// * `source` - Source code string to analyze.
    /// * `repairs` - Vector accumulating records of all applied repairs.
    ///
    /// # Returns
    /// Source code string cleansed of `Core.` and equipped with canonical Lexicon bindings.
    pub fn heal(source: &str, repairs: &mut Vec<AppliedRepair>) -> String {
        let mut text = source.to_string();

        // 1. Sanitize legacy Core.* namespaces and obsolete import: [ ... ] statements
        if text.contains("OmniCode.Core") {
            text = text.replace("OmniCode.Core", "OmniCode { Lexicon }");
            repairs.push(AppliedRepair {
                kind: FixKind::AddImport,
                description: "Sanitized legacy 'OmniCode.Core' into canonical 'OmniCode { Lexicon }'".into(),
                line: 1,
            });
        }

        if text.contains("Core.Arithmetic") || text.contains("Core.Logic") || text.contains("Core.Vectors") {
            // Strip out obsolete import: [ Core.* ] statements
            let mut cleaned_lines = Vec::new();
            for line in text.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("import:") && trimmed.contains("Core.") {
                    repairs.push(AppliedRepair {
                        kind: FixKind::AddImport,
                        description: "Removed obsolete 'Core.' import statement (primitives are intrinsic in OmniCode)".into(),
                        line: 1,
                    });
                    continue; // drop line
                }
                // Strip inline Core. prefixes
                let cleaned_line = line
                    .replace("Core.Arithmetic", "Lexicon { Arithmetic }")
                    .replace("Core.Logic", "Lexicon { Logic }")
                    .replace("Core.Vectors", "Lexicon { Vectors }");
                cleaned_lines.push(cleaned_line);
            }
            text = cleaned_lines.join("\n");
            if !text.ends_with('\n') {
                text.push('\n');
            }
        }

        // 2. Identify missing canonical Lexicon entity bindings needed in Opening { ... }
        let mut bindings_to_inject = Vec::new();

        // Character binding (character.omni: [ Character ]: < Character > & [ Cell ])
        if (text.contains("Character") || text.contains("Glyph")) && !text.contains("[ Character ]:") {
            bindings_to_inject.push("    [ Character ]: < Character > & [ Cell ]\n        ( Denoted Character is typed as invariant allocated in Cell ),");
        }

        // Cell container binding (character.omni: [ Cell ]: [ 1 ] & { 5 Trits :: 1 Byte })
        if text.contains("Cell") && !text.contains("[ Cell ]:") {
            bindings_to_inject.push("    [ Cell ]: [ 1 ] & { 5 Trits :: 1 Byte }\n        ( Concrete 1-cell container in context of 5 Trits and 1 Byte ),");
        }

        // Transducer binding (character.omni: [ Transducer ]: { [ 8 Bits ] <=> [ 5 Trits ] })
        if text.contains("Transducer") && !text.contains("[ Transducer ]:") {
            bindings_to_inject.push("    [ Transducer ]: { [ 8 Bits ] <=> [ 5 Trits ] }\n        ( Transducer bridges 8 bits of silicon to 5 trits of memory ),");
        }

        // Number binding (number.omni: [ Number ]: < Number > & [ Memory Word ])
        if (text.contains("Number") || text.contains("Scalar")) && !text.contains("[ Number ]:") {
            bindings_to_inject.push("    [ Number ]: < Number > & [ Memory Word ]\n        ( Denoted Number is typed as invariant Number allocated in Word ),");
        }

        // Memory Word binding (number.omni: [ Memory Word ]: [ 1 Memory Word ] & { 32 Trits <=> 64 Bits })
        if text.contains("Memory Word") && !text.contains("[ Memory Word ]:") {
            bindings_to_inject.push("    [ Memory Word ]: [ 1 Memory Word ] & { 32 Trits <=> 64 Bits }\n        ( Concrete 1-word container in context of 32 Trits and 64 Bits ),");
        }

        // ALU binding (number.omni: [ ALU ]: { (+1) plus (-1) <=> [ 0 ] } & { 64 Bits <=> 32 Trits })
        if text.contains("ALU") && !text.contains("[ ALU ]:") {
            bindings_to_inject.push("    [ ALU ]: { (+1) plus (-1) <=> [ 0 ] } & { 64 Bits <=> 32 Trits }\n        ( ALU enforces ternary zero-sum balance and bridges 64 bits to trits ),");
        }

        if bindings_to_inject.is_empty() {
            return text;
        }

        // 3. Inject bindings into Opening { ... } block
        if let Some(op_idx) = text.find("Opening {") {
            let insert_pos = op_idx + "Opening {".len();
            let mut injection = String::new();
            injection.push('\n');
            for b in bindings_to_inject {
                injection.push_str(b);
                injection.push('\n');
                repairs.push(AppliedRepair {
                    kind: FixKind::AddImport,
                    description: "Injected canonical Lexicon entity binding conforming to character.omni/number.omni".into(),
                    line: 1,
                });
            }

            text.insert_str(insert_pos, &injection);
        }

        text
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Import Healer Unit Verification Tests & Invariant Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_healer_sanitizes_legacy_core_syntax() {
        let legacy_code = "Module < Test > {\n    system: < OmniCode.Core >\n}\nOpening {\n    import: [ Core.Arithmetic :: Core.Logic ],\n    preconditions: [ +1 ]\n}\nBody [\n    val = 10 plus 20\n]";
        let mut repairs = Vec::new();
        let healed = ImportHealer::heal(legacy_code, &mut repairs);

        assert!(!healed.contains("OmniCode.Core"));
        assert!(!healed.contains("Core.Arithmetic"));
        assert!(!healed.contains("Core.Logic"));
        assert!(healed.contains("OmniCode { Lexicon }"));
        assert!(!repairs.is_empty());
    }

    #[test]
    fn test_import_healer_injects_canonical_lexicon_entity_bindings() {
        let code = "Opening {\n    [ Preconditions ]: [ +1:PROCEED ]\n}\nBody [\n    [ ROLL 1 ] : [ Inscribe ] : { Glyph ] --> INSCRIBE [ Cell ] = [ Character` ]\n]";
        let mut repairs = Vec::new();
        let healed = ImportHealer::heal(code, &mut repairs);

        assert!(healed.contains("[ Character ]: < Character > & [ Cell ]"));
        assert!(healed.contains("[ Cell ]: [ 1 ] & { 5 Trits :: 1 Byte }"));
        assert_eq!(repairs.len(), 2);
    }
}
