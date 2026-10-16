// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Physical Assembler Core Coordinator, Lifecycle & Hardware Invariants )
// ==============================================================================
//! OmniCode Runtime: Physical Assembler Core Coordinator
//!
//! # Architectural Scope & Purpose:
//! Encapsulates the main [`OmniAssembler`] coordinator struct:
//! 1. Telemetry and failpoint recording via [`DiagnosticLogger`].
//! 2. Structural boundary invariant enforcement via [`OmniDebugger`].
//! 3. Probing candidate lexicon search paths (`lexicon/`, `lib/`).
//! 4. High-level file-based assembly entry points (`assemble_file`).
//! 5. Transpilation convenience wrappers (`transpile_omni_source_to_tape`, `transpile_omni_source_to_asm`).
//! 6. Comprehensive unit test oracles covering all 13 hardware registers, SVO lowering, and 28 lexicon tapes.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of physical machine code onto bounded hardware substrates).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic types and substrates before constructing systems).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The inviolable preservation of historical silicon addresses, MMIO ports, and opcodes).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Silicon Truth**: Coordinates all memory allocation, register binding, and machine word packaging.
//! 2. **Baton Pass**: Connects callers (CLI, REPL, JIT) to lowered silicon layouts and bare-metal emitters.
// ==============================================================================

use std::path::{Path, PathBuf};

use crate::core::tape::Tape;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::runtime::assembler::transpiler::baremetal::transpile_tape_to_asm;
use crate::runtime::assembler::transpiler::omni::transpile_omni_to_tape;
use crate::runtime::assembler::universal::types::AssembledTape;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( OmniAssembler Coordinator Struct & Constructor )
// ==============================================================================

/// Encapsulates the runtime diagnostic logger, boundary invariant debugger ([`OmniDebugger`]),
/// and candidate lexicon search paths required to resolve and lower ontological symbols into silicon memory.
#[derive(Debug, Clone)]
pub struct OmniAssembler {
    /// Attached telemetry and error interception logger.
    pub logger: DiagnosticLogger,
    /// Delimiter boundary stack tracking engine for structural invariants (`[...]`, `<...>`).
    pub debugger: OmniDebugger,
    /// Candidate filesystem search directories for modular lexicon resolution.
    pub lexicon_paths: Vec<PathBuf>,
}

impl OmniAssembler {
    /// Constructs a new physical silicon assembler bound to the given diagnostic logger and debugger.
    ///
    /// Automatically probes standard candidate lexicon and library paths (`lexicon/`, `lib/`)
    /// to support cross-track ontological symbol resolution.
    pub fn new(logger: DiagnosticLogger, debugger: OmniDebugger) -> Self {
        let mut lexicon_paths = Vec::new();
        let candidate_dirs = ["lexicon", "lib"];
        for dir in &candidate_dirs {
            let p = PathBuf::from(dir);
            if p.exists() {
                lexicon_paths.push(p);
            }
        }
        Self {
            logger,
            debugger,
            lexicon_paths,
        }
    }

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( High-Level Assembly & Transpilation Execution Pipelines )
// ==============================================================================

    /// Transpiles a raw declarative .omni string directly into an executable .tape string.
    pub fn transpile_omni_source_to_tape(&mut self, source: &str) -> Result<String, String> {
        let assembled = self.assemble_omni(source)?;
        Ok(transpile_omni_to_tape(&assembled))
    }

    /// Transpiles a raw declarative .omni string directly into bare-metal x86 bootstrap assembly (`.asm`).
    pub fn transpile_omni_source_to_asm(&mut self, source: &str) -> Result<String, String> {
        let assembled = self.assemble_omni(source)?;
        Ok(transpile_tape_to_asm(&assembled))
    }

    /// Assembles a .tape or .omni file directly from disk.
    pub fn assemble_file(&mut self, path: impl AsRef<Path>) -> Result<AssembledTape, String> {
        let p = path.as_ref();
        let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
        match ext {
            "tape" => {
                let tape = Tape::from_file(p, self.logger.clone())?;
                self.assemble_tape(&tape)
            }
            "omni" => {
                let source = std::fs::read_to_string(p)
                    .map_err(|e| format!("Failed to read .omni file '{}': {e}", p.display()))?;
                self.assemble_omni(&source)
            }
            other => Err(format!(
                "Unsupported file extension '.{}' for '{}' (expected .tape or .omni)",
                other, p.display()
            )),
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Baton Pass Contract, Invariant Verification Proofs & Unit Oracles )
// ==============================================================================

// ------------------------------------------------------------------------------
// 4.1. The Physical Baton Pass Contract
// ------------------------------------------------------------------------------
// BATON PASS CONTRACT:
// 1. INPUT INGESTION: `OmniAssembler::assemble_file` or `assemble_tape` ingests high-level
//    procedural tapes (`.tape`) or spatial reels (`.omni`) from callers (CLI, REPL, JIT).
// 2. ATOMIC LOWERING: `assemble_roll` tokenizes each statement, asserts boundary integrity via
//    `OmniDebugger`, resolves polarities (`+` abstract vs `-` concrete), assigns physical memory
//    partition offsets (`@ 0x0000`), binds agentic pronouns to hardware CPU registers (`%self`,
//    `%caller`, `%acc`, `%swarm`), and packs 64-bit balanced-ternary machine words (`Word64`).
// 3. INVARIANT & SEAL CERTIFICATION: The tape head covenant and commit status are evaluated,
//    guaranteeing that execution only seals in affirmative `+1:PROCEED`.
// 4. OUTPUT ARTIFACT SYNTHESIS: `AssembledTape` provides universal emission methods (`format_listing`,
//    `format_words`, `format_json`, `format_omni`, `to_machine_words`).
// 5. BATON HANDOFF: The resulting `AssembledTape` or `Vec<Word64>` is handed off to downstream
//    virtual machine runners, the QEMU bare-metal hypervisor, or CLI output serializers.

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::core::tape::Roll;
    use crate::core::ternary::QuadState;
    use crate::runtime::assembler::universal::isa::{MachineOp, MachineRegister};

    /// Test 1: Verifies that the physical assembler lowers living world ontological tapes,
    /// synthesizing concrete memory cell offsets and affirmative PROCEED commit status.
    #[test]
    fn test_assembler_lowers_living_world_tape() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let mut asm = OmniAssembler::new(logger, debugger);

        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/living_world.tape");
        if path.exists() {
            let assembled = asm.assemble_file(&path).expect("Assembling living_world.tape");
            assert_eq!(assembled.head.name, "LivingWorldInstantiation");
            assert!(assembled.symbols.contains_key("Dog"));
            assert_eq!(assembled.commit_status, QuadState::Proceed);
            let listing = assembled.format_listing();
            assert!(listing.contains("ASSEMBLED MACHINE TAPE"));
            assert!(listing.contains("Dog"));
        }
    }

    /// Test 2: Verifies polarity transformation Lowering (`+Archetype --> Concrete-`).
    /// Ensures that abstract entities register in the symbol table with Abstract polarity,
    /// and concrete entities receive designated silicon memory cells.
    #[test]
    fn test_assembler_lowers_polarity_tape() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let mut asm = OmniAssembler::new(logger, debugger);

        let path = crate::core::filetype::resolve_track_path("lexicon/operators/polarity.tape");
        if path.exists() {
            let assembled = asm.assemble_file(&path).expect("Assembling polarity.tape");
            assert_eq!(assembled.head.name, "PolarityTransformation");
            assert!(assembled.symbols.contains_key("Archetype"));
            assert!(assembled.symbols.contains_key("Concrete"));
            assert_eq!(assembled.commit_status, QuadState::Proceed);
        }
    }

    /// Test 3: Exhaustive regression oracle across the entire modular Track 2 Lexicon.
    /// Assembles all 30 foundational grammar and kernel tapes, asserting that every single
    /// tape seals with affirmative `+1:PROCEED` without unhandled delimiter or syntax errors.
    #[test]
    fn test_assembler_lowers_all_modular_lexicon_tapes() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let mut asm = OmniAssembler::new(logger, debugger);

        let tape_paths = [
            "lexicon/operators/polarity.tape",
            "lexicon/operators/casting.tape",
            "lexicon/01_nouns/atomic_units.tape",
            "lexicon/01_nouns/vessels.tape",
            "lexicon/01_nouns/agents.tape",
            "lexicon/01_nouns/living_world.tape",
            "lexicon/02_pronouns/self_handles.tape",
            "lexicon/02_pronouns/wildcards.tape",
            "lexicon/03_verbs/axial_vectors.tape",
            "lexicon/03_verbs/scale_vectors.tape",
            "lexicon/03_verbs/diagonal_vectors.tape",
            "lexicon/03_verbs/relational_being.tape",
            "lexicon/03_verbs/arithmetic_verbs.tape",
            "lexicon/04_adjectives/quad_states.tape",
            "lexicon/04_adjectives/invariant_axioms.tape",
            "lexicon/04_adjectives/proportions.tape",
            "lexicon/05_adverbs/negation.tape",
            "lexicon/05_adverbs/cadence_policies.tape",
            "lexicon/06_prepositions/spatial_sites.tape",
            "lexicon/06_prepositions/ranges.tape",
            "lexicon/07_conjunctions/inseparable_pairs.tape",
            "lexicon/07_conjunctions/pipeline_junctions.tape",
            "lexicon/07_conjunctions/caveat_branches.tape",
            "lexicon/07_conjunctions/alternative_branches.tape",
            "lexicon/07_conjunctions/relational_attributions.tape",
            "lexicon/08_interjections/kernel_traps.tape",
            "lexicon/08_interjections/teleology_asides.tape",
            "lexicon/03_verbs/cinematic_track_loop.tape",
            "lexicon/operators/boot_kernel.tape",
            "lib/kernel/boot.tape",
        ];

        for path_str in &tape_paths {
            let path = crate::core::filetype::resolve_track_path(path_str);
            assert!(path.exists(), "Tape file does not exist: {}", path_str);
            let assembled = asm.assemble_file(&path).unwrap_or_else(|e| panic!("Failed to assemble {}: {}", path_str, e));
            assert_eq!(assembled.commit_status, QuadState::Proceed, "Tape {} did not commit in PROCEED", path_str);
            assert!(!assembled.instructions.is_empty(), "Tape {} had no instructions", path_str);
        }
    }

    /// Test 4: Verifies grammatical pronoun lowering into physical hardware CPU registers.
    /// Asserts that self-handles (`self`) map to `%self` (Register 4), wildcards and
    /// antecedents (`it`) map to `%acc` (Register 3), and callers map to `%caller`.
    #[test]
    fn test_assembler_lowers_pronouns_and_registers() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let mut asm = OmniAssembler::new(logger, debugger);

        let path = crate::core::filetype::resolve_track_path("lexicon/02_pronouns/self_handles.tape");
        if path.exists() {
            let assembled = asm.assemble_file(&path).expect("Assembling self_handles.tape");
            let self_roll = assembled.instructions.iter().find(|i| i.original_text.contains("self"));
            assert!(self_roll.is_some(), "Expected self roll in instructions");
            assert_eq!(self_roll.unwrap().register, Some(MachineRegister::SELF));
        }

        let wild_path = crate::core::filetype::resolve_track_path("lexicon/02_pronouns/wildcards.tape");
        if wild_path.exists() {
            let assembled = asm.assemble_file(&wild_path).expect("Assembling wildcards.tape");
            let it_roll = assembled.instructions.iter().find(|i| i.original_text.contains("it"));
            assert!(it_roll.is_some(), "Expected it roll in instructions");
            assert_eq!(it_roll.unwrap().register, Some(MachineRegister::ACC));
        }
    }

    /// Test 5: Verifies universal serialization methods on `AssembledTape`.
    /// Validates `format_words()` (physical silicon hex dump), `format_json()` (IDE/tooling AST),
    /// and `format_omni()` (decompilation/transpilation into declarative 4-Block spatial reels).
    #[test]
    fn test_assembler_universal_formatting_methods() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let mut asm = OmniAssembler::new(logger, debugger);

        let path = crate::core::filetype::resolve_track_path("lib/kernel/boot.tape");
        if path.exists() {
            let assembled = asm.assemble_file(&path).expect("Assembling boot.tape");

            // 1. Format Words
            let words_dump = assembled.format_words();
            assert!(words_dump.contains("PHYSICAL SILICON WORD STREAM"));
            assert!(words_dump.contains("TOTAL WORDS:"));

            // 2. Format JSON
            let json_dump = assembled.format_json();
            assert!(json_dump.contains("\"tape\":"));
            assert!(json_dump.contains("\"symbols\":"));
            assert!(json_dump.contains("\"instructions\":"));

            // 3. Format Omni (Transpilation)
            let omni_dump = assembled.format_omni();
            assert!(omni_dump.contains("BLOCK 1: METADATA BLOCK"));
            assert!(omni_dump.contains("BLOCK 2: OPENING BLOCK"));
            assert!(omni_dump.contains("BLOCK 3: BODY BLOCK"));
            assert!(omni_dump.contains("BLOCK 4: CLOSING BLOCK"));
            assert!(omni_dump.contains("commit_status [ PROCEED ]"));
        }
    }

    /// Test 6: Verifies declarative `.omni` spatial reel lowering into physical machine layout.
    /// Confirms that `assemble_omni` correctly parses multi-block `.omni` code, constructs
    /// the symbol table and instruction stream, and verifies the Closing block seal.
    #[test]
    fn test_assembler_lowers_declarative_omni_reel() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let mut asm = OmniAssembler::new(logger, debugger);

        let path = crate::core::filetype::resolve_track_path("lexicon/03_verbs/axial_vectors.omni");
        if path.exists() {
            let assembled = asm.assemble_file(&path).expect("Assembling axial_vectors.omni");
            assert_eq!(assembled.commit_status, QuadState::Proceed);
            assert!(!assembled.instructions.is_empty(), "Must produce instruction stream");
            assert_eq!(assembled.head.who, "OmniAssembler");
        }
    }

    /// Test 7: Verifies standalone atomic single-roll lowering via `assemble_roll`.
    /// Confirms that an individual roll can be lowered outside of a Tape container,
    /// correctly allocating memory partition offsets and updating the physical symbol table.
    #[test]
    fn test_assembler_assemble_roll_standalone() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let mut asm = OmniAssembler::new(logger, debugger);

        let roll = Roll {
            index: 1,
            line_number: 10,
            text: "roll_1 [ +Memory --> Memory- @ 0 ]".to_string(),
            status: QuadState::Proceed,
            is_head: false,
        };

        let mut symbols = HashMap::new();
        let mut next_offset = 0;
        let mut commit_status = QuadState::Await;

        let inst = asm.assemble_roll(
            &roll,
            0,
            "TestPartition",
            &mut symbols,
            &mut next_offset,
            &mut commit_status,
        ).expect("Assembling standalone roll");

        assert_eq!(inst.op, MachineOp::AllocConcrete);
        assert!(symbols.contains_key("Memory"));
        assert_eq!(symbols["Memory"].offset, Some(0));
        assert_ne!(inst.word.raw, 0xFFFFFFFF_FFFFFFFF);
    }

    /// Test 8: Verifies full coverage of all 13 Named Silicon Registers,
    /// backward-compatible aliases, ALL CAPS and lowercase duality, affix handling,
    /// and dynamic roll register binding.
    #[test]
    fn test_named_silicon_registers_coverage() {
        let registers = [
            (MachineRegister::HEAD, 0, "HEAD", "%head"),
            (MachineRegister::HOME, 1, "HOME", "%home"),
            (MachineRegister::SPOOL, 2, "SPOOL", "%spool"),
            (MachineRegister::IT, 3, "IT", "%it"),
            (MachineRegister::SELF, 4, "SELF", "%self"),
            (MachineRegister::CALLER, 5, "CALLER", "%caller"),
            (MachineRegister::OTHER, 6, "OTHER", "%other"),
            (MachineRegister::STATE, 7, "STATE", "%state"),
            (MachineRegister::TURN, 8, "TURN", "%turn"),
            (MachineRegister::BOUND, 9, "BOUND", "%bound"),
            (MachineRegister::FROM, 10, "FROM", "%from"),
            (MachineRegister::INTO, 11, "INTO", "%into"),
            (MachineRegister::DISCARD, 12, "DISCARD", "%discard"),
        ];

        for (reg, expected_idx, expected_hw, expected_name) in registers {
            assert_eq!(reg.index(), expected_idx);
            assert_eq!(reg.hardware_name(), expected_hw);
            assert_eq!(reg.name(), expected_name);
            assert_eq!(MachineRegister::from_name(expected_hw), Some(reg));
            assert_eq!(MachineRegister::from_name(&expected_hw.to_lowercase()), Some(reg));
            assert_eq!(MachineRegister::from_name(&format!("%{}", expected_hw.to_lowercase())), Some(reg));
            assert_eq!(MachineRegister::from_name(&format!("+{}", expected_hw)), Some(reg));
            assert_eq!(MachineRegister::from_name(&format!("-{}", expected_hw)), Some(reg));
        }

        // Backward compatibility constants
        assert_eq!(MachineRegister::IP, MachineRegister::HEAD);
        assert_eq!(MachineRegister::FP, MachineRegister::HOME);
        assert_eq!(MachineRegister::SP, MachineRegister::SPOOL);
        assert_eq!(MachineRegister::ACC, MachineRegister::IT);
        assert_eq!(MachineRegister::FLAGS, MachineRegister::STATE);
        assert_eq!(MachineRegister::HERE, MachineRegister::HEAD);
        assert_eq!(MachineRegister::ORIGIN, MachineRegister::HOME);
        assert_eq!(MachineRegister::OWNER, MachineRegister::SPOOL);
        assert_eq!(MachineRegister::SWARM, MachineRegister::OTHER);
        assert_eq!(MachineRegister::STEP, MachineRegister::TURN);
        assert_eq!(MachineRegister::INPUT, MachineRegister::FROM);
        assert_eq!(MachineRegister::OUTPUT, MachineRegister::INTO);

        // Natural, positional, and shorthand aliases
        assert_eq!(MachineRegister::from_name("ip"), Some(MachineRegister::HEAD));
        assert_eq!(MachineRegister::from_name("here"), Some(MachineRegister::HEAD));
        assert_eq!(MachineRegister::from_name("origin"), Some(MachineRegister::HOME));
        assert_eq!(MachineRegister::from_name("spool"), Some(MachineRegister::SPOOL));
        assert_eq!(MachineRegister::from_name("owner"), Some(MachineRegister::SPOOL));
        assert_eq!(MachineRegister::from_name("turn"), Some(MachineRegister::TURN));
        assert_eq!(MachineRegister::from_name("step"), Some(MachineRegister::TURN));
        assert_eq!(MachineRegister::from_name("bound"), Some(MachineRegister::BOUND));
        assert_eq!(MachineRegister::from_name("from"), Some(MachineRegister::FROM));
        assert_eq!(MachineRegister::from_name("input"), Some(MachineRegister::FROM));
        assert_eq!(MachineRegister::from_name("into"), Some(MachineRegister::INTO));
        assert_eq!(MachineRegister::from_name("output"), Some(MachineRegister::INTO));
        assert_eq!(MachineRegister::from_name("_"), Some(MachineRegister::DISCARD));
        assert_eq!(MachineRegister::from_name("void"), Some(MachineRegister::DISCARD));

        // Verify that assemble_roll binds Named Silicon Registers directly
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let mut asm = OmniAssembler::new(logger, debugger);

        let roll_turn = Roll {
            index: 1,
            line_number: 1,
            text: "roll_turn [ TURN plus 1 --> TURN ]".to_string(),
            status: QuadState::Proceed,
            is_head: false,
        };
        let mut syms = HashMap::new();
        let mut off = 0;
        let mut commit = QuadState::Await;
        let inst_turn = asm.assemble_roll(&roll_turn, 0, "Part", &mut syms, &mut off, &mut commit).expect("Assemble TURN");
        assert_eq!(inst_turn.register, Some(MachineRegister::TURN));

        let roll_bound = Roll {
            index: 2,
            line_number: 2,
            text: "roll_bound [ BOUND ]".to_string(),
            status: QuadState::Proceed,
            is_head: false,
        };
        let inst_bound = asm.assemble_roll(&roll_bound, 1, "Part", &mut syms, &mut off, &mut commit).expect("Assemble BOUND");
        assert_eq!(inst_bound.register, Some(MachineRegister::BOUND));
    }

    /// Test 9: Verifies SVO action verbs (INSCRIBE, TRANSDUCE, PROJECT, SEQUENCE),
    /// bijective Radix-3 equivalence (<=>), and MMIO port lowering for Character tapes.
    #[test]
    fn test_character_tape_svo_lowering_and_equivalence() {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 100);
        let mut asm = OmniAssembler::new(logger, debugger);
        let mut syms = HashMap::new();
        let mut off = 0;
        let mut commit = QuadState::Await;

        // 1. ROLL 1: INSCRIBE verb -> AllocConcrete / INSCRIBE_CELL
        let roll1 = Roll {
            index: 1,
            line_number: 1,
            text: "[ ROLL 1 ] : [ Inscribe ] : { Glyph ] --> INSCRIBE [ Cell ] = [ Character` ]".to_string(),
            status: QuadState::Proceed,
            is_head: false,
        };
        let inst1 = asm.assemble_roll(&roll1, 0, "CharacterTape", &mut syms, &mut off, &mut commit).expect("Assemble ROLL 1");
        assert_eq!(inst1.op_code, "INSCRIBE_CELL");
        assert_eq!(inst1.op, MachineOp::AllocConcrete);

        // 2. ROLL 2: Bijective equivalence <=> and TRANSDUCE verb -> TransduceVector / TRANSDUCE_SUBSTRATE
        let roll2 = Roll {
            index: 2,
            line_number: 2,
            text: "[ ROLL 2 ] : [ Transduce ] : [ Transducer ]: [ Character` { 8 Bits } ] <=> TRANSDUCE [ Cell` { 5 Trits } ]".to_string(),
            status: QuadState::Proceed,
            is_head: false,
        };
        let inst2 = asm.assemble_roll(&roll2, 1, "CharacterTape", &mut syms, &mut off, &mut commit).expect("Assemble ROLL 2");
        assert_eq!(inst2.op_code, "TRANSDUCE_SUBSTRATE");
        assert_eq!(inst2.op, MachineOp::TransduceVector);

        // 3. ROLL 4: PROJECT verb and Target -> TransduceVector / PROJECT_TARGET with register INTO
        let roll4 = Roll {
            index: 4,
            line_number: 4,
            text: "[ ROLL 4 ] : [ Project ] : [ Character` } --> PROJECT [ Target ] = [ Target` }".to_string(),
            status: QuadState::Proceed,
            is_head: false,
        };
        let inst4 = asm.assemble_roll(&roll4, 3, "CharacterTape", &mut syms, &mut off, &mut commit).expect("Assemble ROLL 4");
        assert_eq!(inst4.op_code, "PROJECT_TARGET");
        assert_eq!(inst4.op, MachineOp::TransduceVector);
        assert_eq!(inst4.register, Some(MachineRegister::INTO));

        // 4. ROLL 5: SEQUENCE verb -> BindChannel / SEQUENCE_TRANSITION
        let roll5 = Roll {
            index: 5,
            line_number: 5,
            text: "[ ROLL 5 ] : [ Sequence ] : [ Character` } --> SEQUENCE [ Text ] = [ Text` ]".to_string(),
            status: QuadState::Proceed,
            is_head: false,
        };
        let inst5 = asm.assemble_roll(&roll5, 4, "CharacterTape", &mut syms, &mut off, &mut commit).expect("Assemble ROLL 5");
        assert_eq!(inst5.op_code, "SEQUENCE_TRANSITION");
        assert_eq!(inst5.op, MachineOp::BindChannel);

        // 5. MMIO addresses: FROM ::@ 0x03F8 and INTO ::@ 0x000B8000
        let roll_uart = Roll {
            index: 6,
            line_number: 6,
            text: "[ COM1 ] : FROM ::@ 0x03F8".to_string(),
            status: QuadState::Proceed,
            is_head: false,
        };
        let inst_uart = asm.assemble_roll(&roll_uart, 5, "CharacterTape", &mut syms, &mut off, &mut commit).expect("Assemble UART");
        assert_eq!(inst_uart.address, Some(0x03F8));
        assert_eq!(inst_uart.register, Some(MachineRegister::FROM));

        let roll_vga = Roll {
            index: 7,
            line_number: 7,
            text: "[ VGA ] : INTO ::@ 0x000B8000".to_string(),
            status: QuadState::Proceed,
            is_head: false,
        };
        let inst_vga = asm.assemble_roll(&roll_vga, 6, "CharacterTape", &mut syms, &mut off, &mut commit).expect("Assemble VGA");
        assert_eq!(inst_vga.address, Some(0x000B8000));
        assert_eq!(inst_vga.register, Some(MachineRegister::INTO));
    }
}
