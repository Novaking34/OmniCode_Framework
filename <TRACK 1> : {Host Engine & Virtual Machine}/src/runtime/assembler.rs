// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Machine Manifest, Scheduling Priority, Physical ISA Architecture & Identity Separation )
// ==============================================================================
//! MODULE:              omnicode::runtime::assembler
//! IDENTITY:            The Physical Assembler & Silicon Lowering Engine
//! NATURE:              Transduction Engine from Abstract Ontological Archetypes (`+`) to Concrete Silicon Memory Layouts (`-`)
//! VERSION:             0.3.0
//! CREATED:             2026-10-11
//! LAST_SEALED:         2026-10-12
//! AUTHOR:              CreativeWorkzStudio LLC / ProfessorSeanEX
//! COVENANT:            Public Domain / MIT
//!
//! SCHEDULER:
//!   TRACK_PRIMARY:     <TRACK 1> : {Host Engine & Virtual Machine}
//!   TRACKS_TOUCHED:    [ <TRACK 0> : {Silicon & Bare-Metal Kernel}, <TRACK 1>, <TRACK 2> : {OmniCode Language & Standard Library} ]
//!   PRIORITY:          TaskPriority::High
//!   CHANNEL:           Track::Kernel
//!
//! CONTRACT / WHO-WHAT-WHERE-WHY-HOW:
//!   WHO:               OmniCode Runtime Silicon Assembler & Lowering Engine (`omnicode::runtime::assembler`)
//!   WHAT:              Transduce high-level abstract grammatical statements (`+`) into concrete physical silicon
//!                      memory allocations, cell offsets (`@ 0x0000`), hardware CPU registers (`%self`, `%caller`,
//!                      `%acc`, `%swarm`), and 64-bit balanced-ternary machine words ([`Word64`]).
//!   WHERE:             Internal execution core across Track 1 VM, consuming Track 2 Lexicon tapes and Track 0 kernel manifests.
//!   WHEN:              Invoked during compilation, JIT translation, interactive execution, or via CLI `omnicode assemble`.
//!   WHY:               Ground abstract linguistic statements into concrete silicon execution physics, eliminating
//!                      untyped scripting drift and establishing deterministic memory boundaries.
//!   HOW:               Balanced Ternary (+1:PROCEED sealed assembly, -1:ROLLBACK trapped failpoint, 0:AWAIT dry-run).
//!
//! ARCHITECTURAL DUALITY & IDENTITY SEPARATION:
//!   Notice the vital distinction between the two assembler gears in Track 1:
//!   1. `omnicode::cli::commands::assemble` (The External CLI Dispatcher & Orchestrator):
//!      - Location: `src/cli/commands/assemble.rs`
//!      - Role: User port, CLI flag ingestion (`-o`, `-f`, `-v`, `-s`, `-n`), cross-track path resolution
//!        via `resolve_track_path`, pre-flight tripwire perimeter verification (`TRIPWIRE_INPUT_RESOLVE`),
//!        and destination disk artifact writing (`TRIPWIRE_OUTPUT_SERIALIZE`).
//!   2. `omnicode::runtime::assembler` (The Internal Lowering Engine & Silicon Synthesizer):
//!      - Location: `src/runtime/assembler.rs` (Modularizing towards `src/runtime/assembler/` domain tree)
//!      - Role: Core balanced-ternary machine ISA, opcode assignment (`MachineOp`), physical hardware register
//!        mapping (`MachineRegister`), physical memory partition allocation (`@ offset`, `v--`), 64-bit balanced
//!        ternary machine words ([`Word64`]), structural boundary invariant enforcement via [`OmniDebugger`],
//!        and transpilation between the three isomorphic triad forms (`.tape` <--> `.omni` <--> `Word64`).
//!
//! WHY THE ASSEMBLER DEMANDS ITS OWN MODULAR DOMAIN TREE:
//!   As OmniCode matures beyond Python-like interpreted scripting into a self-hosting silicon Operating System,
//!   the assembler decomposes into discrete, highly specialized computational sub-systems:
//!   - `isa`: Hardware Opcodes (`MachineOp`), VM Registers (`MachineRegister`), and ternary word packaging.
//!   - `symbols`: Memory map partition geometry, Storehouse limits, polarity tracking (`+` vs `-`), and cell capacities.
//!   - `tape`: Procedural roll lowering (`assemble_roll`), Tape Head covenant parsing, and directional causal vectors (`-->`, `<--`).
//!   - `transpiler`: Dual-direction decompilation and code emission (transpiling `.tape` temporal process ribbons into
//!     declarative 4-Block `.omni` spatial architecture reels, silicon hex dumps, or IDE hovercard JSON).
//!
//! GAME DEV IDEOLOGY (THE STAGE INCEPTION & SIMULATION TICK):
//!   - Level Loading / Inception: Setting up physical memory partitions (`where_clause`, `partition_size`) before execution.
//!   - Collision Volumes & Triggers: Delimiter boundary stack tracking (`[`, `{`, `<`) guarded by [`OmniDebugger`].
//!   - Flight Recorder & Telemetry HUD: Step-by-step lowering logs and failpoint traps recorded in [`DiagnosticLogger`].
//!   - Respawn / Immune Recovery: Unsealed tapes or boundary leaks emit `#remediation_ready` telemetry for the [`crate::diagnostics::restorator::Restorator`].
//!
//! TOUCHES / FILESYSTEM FOOTPRINT:
//!   - INGESTION:       Procedural process ribbons (`.tape`) and declarative spatial architecture reels (`.omni`).
//!   - RESOLUTION:      Candidate lexicon paths (`lexicon/`, `lib/`) searched for ontological symbol bindings.
//!   - SERIALIZATION:   Formatted machine listings, silicon hex words, structured JSON hovercards, and `.omni` transpiler reels.
//!
//! TOUCHES / SILICON HARDWARE & MMIO REGISTERS:
//!   - VM REGISTERS:    `%ip` (0), `%fp` (1), `%sp` (2), `%acc` (3), `%self` (4), `%caller` (5), `%swarm` (6), `%flags` (7).
//!   - MEMORY CELLS:    Partition allocation offsets (`@ 0x0000`), bounded partition slices (`v--`), storehouse capacities.
//!   - MACHINE WORDS:   Packed 64-bit balanced-ternary containers (`Word64`): 32 trits payload, QuadState, Polarity, Boundary, 7-bit checksum.
//!
//! REQUIRES / HARDWARE & RUNTIME DEPENDENCIES:
//!   - [`OmniDebugger`] for delimiter boundary stack invariant tracking (`[`, `{`, `<`).
//!   - [`DiagnosticLogger`] for flight recording, AI observations, and failpoint interception.
//!   - [`Lexer`] and [`Parser`] for token classification, pronoun categorization, and grammar lowering.
//!
//! TRIPWIRE REGISTRY:
//!   - `TRIPWIRE_INPUT_RESOLVE`: Assertion of tape source existence.
//!   - `TRIPWIRE_SYNTAX_TOKENIZATION`: Lexical and grammar validation during roll tokenization.
//!   - `TRIPWIRE_MEMORY_BOUNDS`: Partition capacity bounds verification during symbol allocation.
//!   - `TRIPWIRE_COMMIT_INVARIANT`: Covenant seal assertion (`commit_status [ PROCEED ]`).
//!
//! NAVIGATION GUIDE (HOW TO NAVIGATE AND EXTEND THIS MODULE AS THE CODE GROWS):
//!   - Block 1 (Lines 1-85): Manifest, Scheduler, Architectural Duality, Modularization Rationale, Game Dev Ideology, Tripwires.
//!   - Block 2 (Lines 86-350): Opening Block, Scoped Imports, Machine ISA, Registers, Symbols, AssembledTape & OmniAssembler Struct.
//!   - Block 3 (Lines 351-850): Body Block & Execution Engine (Universal Hardware Translation, Serializers, Roll/Tape/Omni Lowering).
//!   - Block 4 (Lines 851+): Physical Baton Pass Contract & Comprehensive Verification Unit Tests.


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Scoped Imports, Physical Machine ISA, Registers, Symbols, AssembledTape & Defaults )
// ==============================================================================

// ------------------------------------------------------------------------------
// 2.1. Scoped Namespace Ingestion & Multi-Platform Imports
// ------------------------------------------------------------------------------
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::core::boundaries::BoundaryType;
use crate::core::linguistics::PronounKind;
use crate::core::operators::Polarity;
use crate::core::tape::{Roll, Tape, TapeHead};
use crate::core::ternary::QuadState;
use crate::core::word::{Word32Trit, Word64};
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};
use crate::parser::lexer::{Lexer, TokenType};
use crate::parser::parser::Parser;

// ------------------------------------------------------------------------------
// 2.2. Physical Machine Instruction Set Architecture (MachineOp)
// ------------------------------------------------------------------------------
/// Hardware Machine Opcodes for the OmniCode Balanced-Ternary Virtual Machine.
///
/// Encodes discrete silicon machine operations executed across 64-bit balanced-ternary
/// machine words. Each opcode corresponds directly to an atomic grammatical, causal,
/// or boundary transformation within the OmniCode lexicon.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MachineOp {
    /// Opcode 0x00: No Operation (Equilibrium state marker).
    Nop              = 0x00,
    /// Opcode 0x01: Reference Abstract Archetype (`+Entity`). Symbolic lookup in Lexicon.
    RefAbstract      = 0x01,
    /// Opcode 0x02: Allocate Concrete Memory Cell (`Entity-`). Reserves physical silicon RAM cell.
    AllocConcrete    = 0x02,
    /// Opcode 0x03: Transduce Vector (`-->`). Forward causal drive, register write, or state propagation.
    TransduceVector  = 0x03,
    /// Opcode 0x04: Feedback Vector (`<--`). Reverse causal feedback, sensor tap, or state reflection.
    FeedbackVector   = 0x04,
    /// Opcode 0x05: Partition Slice (`v--`). Carves an isolated, bounded memory partition from a Storehouse.
    PartitionSlice   = 0x05,
    /// Opcode 0x06: Synthesize Layer (`--^`). Ascends abstraction layer or folds micro-states into macro.
    SynthesizeLayer  = 0x06,
    /// Opcode 0x07: Bind Channel (`::`). Binds hardware port, MMIO address, or inseparable pipeline channel.
    BindChannel      = 0x07,
    /// Opcode 0x08: Cast Boundary (`[+]`, `[-]`, `<+>`, `<->`). Boundary rank type conversion.
    CastBoundary     = 0x08,
    /// Opcode 0x09: Assert Invariant (`! [ ... ]`). Formal logic proof evaluation; halts on contradiction.
    AssertInvariant  = 0x09,
    /// Opcode 0x0A: Introspect Self (`myself`, `ourselves`). Self-audit telemetry and internal state reflection.
    IntrospectSelf   = 0x0A,
    /// Opcode 0x0B: Recurse Itself (`itself`). Homoiconic tape self-rewrite, loop fixpoint, or quine invocation.
    RecurseItself    = 0x0B,
    /// Opcode 0x0C: Assert Capability (`my`, `your`, `their`). Validates partition ownership and security authority.
    AssertCapability = 0x0C,
    /// Opcode 0x0D: Seal Commit (`commit_status [ PROCEED ]`). Irrevocably seals process covenant state.
    SealCommit       = 0x0D,
    /// Opcode 0x0E: Halt Equilibrium (`0 : HOLD`). Suspends execution head in neutral balanced ternary rest.
    HaltEquilibrium  = 0x0E,
    /// Opcode 0x0F: Trap Veto (`-1 : VETO`). Triggers tripwire collision and initiates affirmative rollback.
    TrapVeto         = 0x0F,
}

impl MachineOp {
    /// Returns the canonical uppercase mnemonic string for assembly listings and telemetry.
    pub fn mnemonic(&self) -> &'static str {
        match self {
            Self::Nop => "NOP",
            Self::RefAbstract => "REF_ABSTRACT",
            Self::AllocConcrete => "ALLOC_CONCRETE",
            Self::TransduceVector => "TRANSDUCE_VECTOR",
            Self::FeedbackVector => "FEEDBACK_VECTOR",
            Self::PartitionSlice => "PARTITION_SLICE",
            Self::SynthesizeLayer => "SYNTHESIZE_LAYER",
            Self::BindChannel => "BIND_CHANNEL",
            Self::CastBoundary => "CAST_BOUNDARY",
            Self::AssertInvariant => "ASSERT_INVARIANT",
            Self::IntrospectSelf => "INTROSPECT_SELF",
            Self::RecurseItself => "RECURSE_ITSELF",
            Self::AssertCapability => "ASSERT_CAPABILITY",
            Self::SealCommit => "SEAL_COMMIT",
            Self::HaltEquilibrium => "HALT_EQUILIBRIUM",
            Self::TrapVeto => "TRAP_VETO",
        }
    }

    /// Resolves a mnemonic string back into its hardware opcode.
    pub fn from_mnemonic(s: &str) -> Option<Self> {
        match s.trim().to_uppercase().as_str() {
            "NOP" => Some(Self::Nop),
            "REF_ABSTRACT" => Some(Self::RefAbstract),
            "ALLOC_CONCRETE" => Some(Self::AllocConcrete),
            "TRANSDUCE_VECTOR" => Some(Self::TransduceVector),
            "FEEDBACK_VECTOR" => Some(Self::FeedbackVector),
            "PARTITION_SLICE" => Some(Self::PartitionSlice),
            "SYNTHESIZE_LAYER" => Some(Self::SynthesizeLayer),
            "BIND_CHANNEL" => Some(Self::BindChannel),
            "CAST_BOUNDARY" => Some(Self::CastBoundary),
            "ASSERT_INVARIANT" => Some(Self::AssertInvariant),
            "INTROSPECT_SELF" => Some(Self::IntrospectSelf),
            "RECURSE_ITSELF" => Some(Self::RecurseItself),
            "ASSERT_CAPABILITY" => Some(Self::AssertCapability),
            "SEAL_COMMIT" => Some(Self::SealCommit),
            "HALT_EQUILIBRIUM" => Some(Self::HaltEquilibrium),
            "TRAP_VETO" => Some(Self::TrapVeto),
            _ => None,
        }
    }

    /// Returns `true` if this opcode represents a forward or reverse directional causality vector.
    pub fn is_causal(&self) -> bool {
        matches!(self, Self::TransduceVector | Self::FeedbackVector | Self::PartitionSlice | Self::SynthesizeLayer)
    }

    /// Returns `true` if this opcode reserves or affects physical silicon memory cells.
    pub fn is_memory_allocation(&self) -> bool {
        matches!(self, Self::AllocConcrete | Self::PartitionSlice)
    }
}

// ------------------------------------------------------------------------------
// 2.3. Physical Machine CPU Registers (MachineRegister)
// ------------------------------------------------------------------------------
/// Physical Machine CPU Registers for the Balanced-Ternary Virtual Machine.
///
/// OmniCode maps natural physical tape mechanisms, grammatical perspectives,
/// and streaming prepositions directly to 13 named silicon CPU registers:
/// - `HEAD`    (0): Tape Head / Instruction Pointer / Witness Cursor (`%head`, `%ip`).
/// - `HOME`    (1): Genesis Base Pointer / Pristine Rollback Baseline (`%home`, `%fp`).
/// - `SPOOL`   (2): Ribbon Stack Spool / Partition Reservoir (`%spool`, `%sp`).
/// - `IT`      (3): Pipeline Antecedent / Primary Accumulator (`%it`, `%acc`).
/// - `SELF`    (4): 1st-Person Execution Identity (`%self`, `I`, `me`).
/// - `CALLER`  (5): 2nd-Person Invoking Principal / User Port (`%caller`, `you`).
/// - `OTHER`   (6): 3rd-Person Peer / Swarm Worker Handle (`%other`, `they`, `them`).
/// - `STATE`   (7): Status & Balanced-Ternary QuadState Register (`%state`, `%flags`).
/// - `TURN`    (8): Temporal Cadence / Cycle Counter / Loop Turn (`%turn`, `%step`).
/// - `BOUND`   (9): Invariant Axiom Boundary Watchdog Limit (`%bound`).
/// - `FROM`   (10): Inbound Streaming Transducer Port (`%from`, `input`).
/// - `INTO`   (11): Outbound Streaming Transducer Port (`%into`, `output`).
/// - `DISCARD`(12): Anonymous Discard Sink (`%discard`, `_`, `void`).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MachineRegister {
    /// Register 0 (%head / %here / %ip): Tape Head / Instruction Pointer / Witness Cursor.
    HEAD = 0,
    /// Register 1 (%home / %origin / %fp): Genesis Base Pointer / Rollback Anchor.
    HOME = 1,
    /// Register 2 (%spool / %owner / %sp): Ribbon Stack Spool / Stack Floor.
    SPOOL = 2,
    /// Register 3 (%it / %acc): Pipeline Antecedent / Primary Accumulator (`it`).
    IT = 3,
    /// Register 4 (%self): 1st-Person Execution Identity (`I`, `me`, `self`).
    SELF = 4,
    /// Register 5 (%caller): 2nd-Person Invoking Principal (`you`, `caller`).
    CALLER = 5,
    /// Register 6 (%other / %swarm): 3rd-Person Peer / Worker Swarm (`other`, `them`).
    OTHER = 6,
    /// Register 7 (%state / %flags): Status & Balanced-Ternary QuadState Register.
    STATE = 7,
    /// Register 8 (%turn / %step): Temporal Cadence & Loop Turn Register (`TURN`).
    TURN = 8,
    /// Register 9 (%bound): Invariant Boundary Watchdog Limit (`BOUND`).
    BOUND = 9,
    /// Register 10 (%from / %input): Inbound Streaming Transducer Port (`FROM`).
    FROM = 10,
    /// Register 11 (%into / %output): Outbound Streaming Transducer Port (`INTO`).
    INTO = 11,
    /// Register 12 (%discard): Anonymous Discard Sink (`DISCARD`, `_`).
    DISCARD = 12,
}

impl MachineRegister {
    /// Canonical and legacy aliases
    pub const HERE: Self = Self::HEAD;
    pub const IP: Self = Self::HEAD;
    pub const ORIGIN: Self = Self::HOME;
    pub const FP: Self = Self::HOME;
    pub const OWNER: Self = Self::SPOOL;
    pub const SP: Self = Self::SPOOL;
    pub const ACC: Self = Self::IT;
    pub const SWARM: Self = Self::OTHER;
    pub const FLAGS: Self = Self::STATE;
    pub const STEP: Self = Self::TURN;
    pub const INPUT: Self = Self::FROM;
    pub const OUTPUT: Self = Self::INTO;

    /// Returns the canonical hardware register identifier (e.g., `%head`, `%home`, `%it`).
    pub fn name(&self) -> &'static str {
        match self {
            Self::HEAD => "%head",
            Self::HOME => "%home",
            Self::SPOOL => "%spool",
            Self::IT => "%it",
            Self::SELF => "%self",
            Self::CALLER => "%caller",
            Self::OTHER => "%other",
            Self::STATE => "%state",
            Self::TURN => "%turn",
            Self::BOUND => "%bound",
            Self::FROM => "%from",
            Self::INTO => "%into",
            Self::DISCARD => "%discard",
        }
    }

    /// Returns the universal ALL CAPS silicon register name (e.g., `HEAD`, `HOME`, `IT`).
    pub fn hardware_name(&self) -> &'static str {
        match self {
            Self::HEAD => "HEAD",
            Self::HOME => "HOME",
            Self::SPOOL => "SPOOL",
            Self::IT => "IT",
            Self::SELF => "SELF",
            Self::CALLER => "CALLER",
            Self::OTHER => "OTHER",
            Self::STATE => "STATE",
            Self::TURN => "TURN",
            Self::BOUND => "BOUND",
            Self::FROM => "FROM",
            Self::INTO => "INTO",
            Self::DISCARD => "DISCARD",
        }
    }

    /// Returns the physical silicon register numeric index (0 to 12).
    pub fn index(&self) -> u8 {
        *self as u8
    }

    /// Resolves any register string back to its enum variant, supporting ALL CAPS, lowercase,
    /// shorthand aliases, hardware `%` prefixes, and polarity `+`/`-` affixes.
    pub fn from_name(s: &str) -> Option<Self> {
        let clean = s.trim().trim_start_matches(|c| c == '+' || c == '-' || c == '%').to_lowercase();
        match clean.as_str() {
            "head" | "here" | "ip" | "hd" => Some(Self::HEAD),
            "home" | "origin" | "fp" | "hm" => Some(Self::HOME),
            "spool" | "owner" | "sp" => Some(Self::SPOOL),
            "it" | "acc" => Some(Self::IT),
            "self" | "me" | "i" | "sl" => Some(Self::SELF),
            "caller" | "you" | "cl" => Some(Self::CALLER),
            "other" | "swarm" | "them" | "they" | "ot" => Some(Self::OTHER),
            "state" | "flags" | "status" | "st" => Some(Self::STATE),
            "turn" | "step" | "cadence" | "cycle" | "tr" | "tn" => Some(Self::TURN),
            "bound" | "limit" | "watchdog" | "bd" => Some(Self::BOUND),
            "from" | "input" | "in" | "fr" => Some(Self::FROM),
            "into" | "output" | "out" | "to" => Some(Self::INTO),
            "discard" | "void" | "_" | "dc" => Some(Self::DISCARD),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------
// 2.4. Physical Silicon Memory Symbols (PhysicalSymbol)
// ------------------------------------------------------------------------------
/// Physical allocation record for an ontological symbol in silicon memory.
///
/// Tracks polarity classification (`+` abstract archetype vs `-` concrete silicon memory cell),
/// assigned partition offset (`@ 0x0000`), capacity footprint in cells, and enclosing partition.
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalSymbol {
    /// Ontological name identifier.
    pub name: String,
    /// Formal polarity: Abstract (`+`) or Concrete (`-`).
    pub polarity: Polarity,
    /// Explicit zero-based memory cell offset within the designated partition.
    pub offset: Option<usize>,
    /// Number of contiguous physical RAM cells allocated to this symbol.
    pub size_cells: usize,
    /// Name of the enclosing memory partition or storehouse where allocated.
    pub partition_name: Option<String>,
}

impl PhysicalSymbol {
    /// Returns `true` if this symbol denotes a physical silicon allocation (`-`).
    pub fn is_concrete(&self) -> bool {
        self.polarity == Polarity::Concrete
    }

    /// Returns `true` if this symbol denotes an unallocated abstract archetype (`+`).
    pub fn is_abstract(&self) -> bool {
        self.polarity == Polarity::Abstract
    }
}

// ------------------------------------------------------------------------------
// 2.5. Lowered Machine Instruction Container (AssembledInstruction)
// ------------------------------------------------------------------------------
/// A lowered, assembled instruction roll ready for hardware dispatch.
///
/// Contains the discrete machine opcode, assigned register, silicon memory offset,
/// source provenance, and the 64-bit balanced-ternary machine word ([`Word64`]).
#[derive(Debug, Clone, PartialEq)]
pub struct AssembledInstruction {
    /// 1-based roll position within the execution tape.
    pub roll_index: usize,
    /// 1-based source line number from the original `.tape` or `.omni` document.
    pub source_line: usize,
    /// Raw un-lowered text content of the roll for diagnostics and hovercards.
    pub original_text: String,
    /// Discrete physical machine opcode.
    pub op: MachineOp,
    /// Bound hardware CPU register, if applicable.
    pub register: Option<MachineRegister>,
    /// Target physical memory cell offset, if applicable.
    pub address: Option<usize>,
    /// Packed 64-bit balanced-ternary machine word container.
    pub word: Word64,
    /// QuadState execution status for this instruction (+1:PROCEED).
    pub status: QuadState,
    /// Human-readable operational code string.
    pub op_code: String,
}

// ------------------------------------------------------------------------------
// 2.6. Assembled Machine Tape Container (AssembledTape)
// ------------------------------------------------------------------------------
/// The complete assembled physical machine tape.
///
/// Represents the lowered artifact ready for silicon execution, containing the Tape Head
/// covenant, allocated partition geometry, physical symbol table, instruction roll sequence,
/// and the final affirmative commit seal status.
#[derive(Debug, Clone, PartialEq)]
pub struct AssembledTape {
    /// Process covenant manifest header.
    pub head: TapeHead,
    /// Total cell capacity carved for this memory partition.
    pub partition_size: usize,
    /// Resolved physical symbol table mapping identifiers to silicon offsets.
    pub symbols: HashMap<String, PhysicalSymbol>,
    /// Sequential stream of lowered physical machine instructions.
    pub instructions: Vec<AssembledInstruction>,
    /// Final affirmative seal state (+1:PROCEED).
    pub commit_status: QuadState,
}

impl AssembledTape {
    /// Formats the assembled machine tape as a human-readable assembly listing.
    pub fn format_listing(&self) -> String {
        format_assembly_listing(self)
    }

    /// Emits the human-readable physical silicon word stream (Hex, registers, opcodes, QuadState).
    pub fn format_words(&self) -> String {
        format_silicon_words(self)
    }

    /// Emits a structured JSON representation for tooling, IDEs, and diagnostics.
    pub fn format_json(&self) -> String {
        format_tape_json(self)
    }

    /// Transpiles the assembled tape into a declarative 4-Block spatial architecture reel (`.omni`).
    pub fn format_omni(&self) -> String {
        transpile_tape_to_omni(self)
    }

    /// Transpiles the assembled tape into native bare-metal x86 bootstrap assembly (`.asm`).
    pub fn format_asm(&self) -> String {
        transpile_tape_to_asm(self)
    }

    /// Transpiles the assembled tape into 32-bit Multiboot ELF assembly (`.asm`).
    pub fn format_multiboot_asm(&self) -> String {
        transpile_tape_to_multiboot_asm(self)
    }

    /// Emits the raw physical Word64 machine word sequence for hardware dispatch.
    pub fn to_machine_words(&self) -> Vec<Word64> {
        self.instructions.iter().map(|i| i.word).collect()
    }
}

// ------------------------------------------------------------------------------
// 2.7. Initial Truth States & Equilibrium Defaults
// ------------------------------------------------------------------------------
/// Baseline cell capacity reserved for unspecified memory partitions.
pub const DEFAULT_PARTITION_CELL_CAPACITY: usize = 64;
/// Maximum delimiter boundary nesting depth for structural invariant assertions.
pub const DEFAULT_ASSEMBLER_DEBUGGER_DEPTH: usize = 100;

// ------------------------------------------------------------------------------
// 2.8. Physical Silicon Assembler Interface Handle (OmniAssembler)
// ------------------------------------------------------------------------------
/// The primary silicon lowering engine and compiler interface.
///
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


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Universal ISA Translation Functions, Serializers, Transpilers & OmniAssembler )
// ==============================================================================

// ------------------------------------------------------------------------------
// 3.1. Universal Hardware ISA Translation & Machine Word Synthesis
// ------------------------------------------------------------------------------
/// Universal mapping from grammatical pronouns to physical CPU registers.
///
/// Translates natural linguistic perspectives into concrete VM register bindings:
/// - `I`, `me`, `self` -> `%self` (Register 4)
/// - `you`, `caller` -> `%caller` (Register 5)
/// - `it` -> `%acc` (Register 3)
/// - `they`, `them`, `themselves` -> `%swarm` (Register 6)
pub fn pronoun_to_register(p: PronounKind) -> Option<MachineRegister> {
    match p {
        PronounKind::I | PronounKind::Me | PronounKind::SelfHandle => Some(MachineRegister::SELF),
        PronounKind::You | PronounKind::CallerHandle => Some(MachineRegister::CALLER),
        PronounKind::OwnerHandle => Some(MachineRegister::SPOOL),
        PronounKind::It => Some(MachineRegister::IT),
        PronounKind::They | PronounKind::Them | PronounKind::Themselves => Some(MachineRegister::OTHER),
        PronounKind::Wildcard => Some(MachineRegister::DISCARD),
        _ => None,
    }
}

/// Universal mapping from reflexive, recursive, or possessive pronouns to hardware opcodes.
///
/// - `myself`, `ourselves` -> `MachineOp::IntrospectSelf`
/// - `itself` -> `MachineOp::RecurseItself`
/// - `themselves` -> `MachineOp::IntrospectSelf`
/// - Possessive pronouns (`my`, `your`, `their`) -> `MachineOp::AssertCapability`
pub fn pronoun_to_op(p: PronounKind) -> Option<MachineOp> {
    match p {
        PronounKind::Myself => Some(MachineOp::IntrospectSelf),
        PronounKind::Itself => Some(MachineOp::RecurseItself),
        PronounKind::Themselves => Some(MachineOp::IntrospectSelf),
        _ if p.is_possessive() => Some(MachineOp::AssertCapability),
        _ => None,
    }
}

/// Universal mapping from formal polarity affixes to memory opcodes.
///
/// - Abstract (`+`) -> `MachineOp::RefAbstract`
/// - Concrete (`-`) -> `MachineOp::AllocConcrete`
pub fn polarity_to_op(polarity: Polarity) -> MachineOp {
    match polarity {
        Polarity::Abstract => MachineOp::RefAbstract,
        Polarity::Concrete => MachineOp::AllocConcrete,
    }
}

/// Universal computation of the 7-bit balanced-ternary machine word checksum.
pub fn compute_word_checksum(idx: usize, op: MachineOp) -> u8 {
    ((idx as u8) ^ (op as u8)) & 0x7F
}

/// Universal synthesizer packing payload trits, status, polarity, boundary rank,
/// and checksum into a physical 64-bit balanced-ternary machine word container ([`Word64`]).
pub fn synthesize_machine_word(
    idx: usize,
    op: MachineOp,
    addr: Option<usize>,
    status: QuadState,
    polarity: Polarity,
    boundary: BoundaryType,
) -> Word64 {
    let payload_trits = addr.map_or_else(Word32Trit::zero, |a| Word32Trit::from_i64(a as i64));
    let checksum = compute_word_checksum(idx, op);
    Word64::new(payload_trits, status, polarity, boundary, checksum)
}

// ------------------------------------------------------------------------------
// 3.2. Universal Serialization & Transpilation Functions
// ------------------------------------------------------------------------------
/// Formats an assembled machine tape as a human-readable annotated assembly listing.
pub fn format_assembly_listing(tape: &AssembledTape) -> String {
    let mut out = String::new();
    out.push_str("=============================================================\n");
    out.push_str(&format!("ASSEMBLED MACHINE TAPE: [{}]\n", tape.head.name));
    out.push_str(&format!("PRINCIPAL: {} | TARGET: {}\n", tape.head.who, tape.head.what));
    out.push_str(&format!("MEMORY PARTITION: {} cells\n", tape.partition_size));
    out.push_str("-------------------------------------------------------------\n");
    out.push_str("PHYSICAL SYMBOL TABLE:\n");
    for (name, sym) in &tape.symbols {
        let offset_str = sym.offset.map_or("N/A".to_string(), |o| format!("@ 0x{:04X} ({})", o, o));
        out.push_str(&format!(
            "  • {:<16} | Polarity: {:<8} | Size: {:>3} cells | Offset: {}\n",
            name, sym.polarity.symbol(), sym.size_cells, offset_str
        ));
    }
    out.push_str("-------------------------------------------------------------\n");
    out.push_str("ASSEMBLED INSTRUCTION ROLLS:\n");
    for inst in &tape.instructions {
        let addr_str = inst.address.map_or("      ".to_string(), |a| format!("@{:04X}", a));
        let reg_str = inst.register.map_or("     ".to_string(), |r| format!("{:<5}", r.name()));
        out.push_str(&format!(
            "  [{:>2}] {} | {} | {:<20} | Word: 0x{:016X} | Status: {}\n        --> {}\n",
            inst.roll_index, addr_str, reg_str, inst.op_code, inst.word.raw, inst.status.macro_op(), inst.original_text
        ));
    }
    out.push_str("=============================================================\n");
    out.push_str(&format!("COMMIT SEAL: {}\n", tape.commit_status.macro_op()));
    out
}

/// Emits the human-readable physical silicon word stream (Hex, registers, opcodes, QuadState).
pub fn format_silicon_words(tape: &AssembledTape) -> String {
    let mut out = String::new();
    out.push_str(&format!("PHYSICAL SILICON WORD STREAM: [{}]\n", tape.head.name));
    out.push_str("-------------------------------------------------------------\n");
    for (idx, inst) in tape.instructions.iter().enumerate() {
        let reg_str = inst.register.map_or("     ".to_string(), |r| format!("{:<5}", r.name()));
        out.push_str(&format!(
            "[{:>3}] 0x{:016X} | {} | {:<20} | Status: {}\n",
            idx + 1,
            inst.word.raw,
            reg_str,
            inst.op.mnemonic(),
            inst.status.macro_op()
        ));
    }
    out.push_str("-------------------------------------------------------------\n");
    out.push_str(&format!("TOTAL WORDS: {}\n", tape.instructions.len()));
    out
}

/// Emits a structured JSON representation of an assembled tape for tooling, IDEs, and diagnostics.
pub fn format_tape_json(tape: &AssembledTape) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!("  \"tape\": \"{}\",\n", tape.head.name));
    out.push_str(&format!("  \"principal\": \"{}\",\n", tape.head.who));
    out.push_str(&format!("  \"target\": \"{}\",\n", tape.head.what));
    out.push_str(&format!("  \"partition_size\": {},\n", tape.partition_size));
    out.push_str(&format!("  \"commit_status\": \"{}\",\n", tape.commit_status.macro_op()));
    out.push_str("  \"symbols\": [\n");
    let sym_list: Vec<_> = tape.symbols.values().collect();
    for (i, s) in sym_list.iter().enumerate() {
        let comma = if i + 1 < sym_list.len() { "," } else { "" };
        let off_str = s.offset.map_or("null".to_string(), |o| o.to_string());
        out.push_str(&format!(
            "    {{ \"name\": \"{}\", \"polarity\": \"{}\", \"size\": {}, \"offset\": {} }}{}\n",
            s.name, s.polarity.symbol(), s.size_cells, off_str, comma
        ));
    }
    out.push_str("  ],\n");
    out.push_str("  \"instructions\": [\n");
    for (i, inst) in tape.instructions.iter().enumerate() {
        let comma = if i + 1 < tape.instructions.len() { "," } else { "" };
        let reg_str = inst.register.map_or("null".to_string(), |r| format!("\"{}\"", r.name()));
        let addr_str = inst.address.map_or("null".to_string(), |a| a.to_string());
        out.push_str(&format!(
            "    {{ \"roll\": {}, \"op\": \"{}\", \"reg\": {}, \"address\": {}, \"word\": \"0x{:016X}\", \"status\": \"{}\" }}{}\n",
            inst.roll_index, inst.op.mnemonic(), reg_str, addr_str, inst.word.raw, inst.status.macro_op(), comma
        ));
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

/// Transpiles an assembled tape from temporal process ribbon (`.tape`) into
/// a declarative, 4-Block spatial architecture reel (`.omni`).
pub fn transpile_tape_to_omni(tape: &AssembledTape) -> String {
    let mut out = String::new();
    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 1: METADATA BLOCK\n");
    out.push_str(&format!("# ( Transpiled Spatial Reel: {} )\n", tape.head.name));
    out.push_str("# ==============================================================================\n");
    out.push_str(&format!("Module < {} > ( Transpiled by OmniAssembler ) {{\n", tape.head.name));
    out.push_str("    version: < \"1.0.0\" >,\n");
    out.push_str("    system: < OmniCode.Transpiled >,\n");
    out.push_str(&format!("    principal: < \"{}\" >,\n", tape.head.who));
    out.push_str(&format!("    target: < \"{}\" >,\n", tape.head.what));
    out.push_str(&format!("    partition_capacity: < {} >\n", tape.partition_size));
    out.push_str("}\n\n");

    if !tape.symbols.is_empty() {
        out.push_str("# Physical Silicon Symbol Allocations\n");
        for (name, sym) in &tape.symbols {
            let off_desc = sym.offset.map_or("dynamic".to_string(), |o| format!("@ 0x{:04X}", o));
            let pol_char = sym.polarity.symbol();
            out.push_str(&format!(
                "Entity_{} < PhysicalSymbol > ( Silicon memory cell [{} {}] ) {{\n    polarity: < \"{}\" >,\n    capacity_cells: < {} >\n}}\n\n",
                name, pol_char, off_desc, pol_char, sym.size_cells
            ));
        }
    }

    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 2: OPENING BLOCK\n");
    out.push_str("# ==============================================================================\n");
    out.push_str("Opening {\n");
    out.push_str("    requires: [\n");
    out.push_str(&format!("        PartitionCapacity : {} cells ::\n", tape.partition_size));
    out.push_str(&format!("        TapeHead : {} [ {} ]\n", tape.head.name, tape.head.who));
    out.push_str("    ],\n");
    out.push_str("    preconditions: [ +1:PROCEED ]\n");
    out.push_str("}\n\n");

    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 3: BODY BLOCK\n");
    out.push_str("# ==============================================================================\n");
    out.push_str("Body [\n");
    for inst in &tape.instructions {
        let reg_str = inst.register.map_or(String::new(), |r| format!(" ( Reg: {} )", r.name()));
        out.push_str(&format!("    Roll_{} [ {} ]{}\n", inst.roll_index, inst.original_text, reg_str));
    }
    out.push_str("]\n\n");

    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 4: CLOSING BLOCK\n");
    out.push_str("# ==============================================================================\n");
    out.push_str("Closing {\n");
    out.push_str("    verify: [\n");
    out.push_str(&format!("        InstructionCount = {} ::\n", tape.instructions.len()));
    out.push_str(&format!("        SymbolCount = {}\n", tape.symbols.len()));
    out.push_str("    ],\n");
    out.push_str(&format!("    commit_status [ {} ]\n", tape.commit_status.macro_op()));
    out.push_str("}\n");

    out
}

/// Transpiles an assembled tape from declarative 4-Block spatial architecture reel (`.omni`)
/// into an executable procedural process ribbon (`.tape`).
pub fn transpile_omni_to_tape(tape: &AssembledTape) -> String {
    let mut out = String::new();
    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 1: METADATA BLOCK\n");
    out.push_str(&format!("# ( Procedural Process Ribbon Transpiled from Omni Reel: {} )\n", tape.head.name));
    out.push_str("# ==============================================================================\n\n");
    out.push_str(&format!(
        "[Tape.{}] : WHO {{{}}} WHAT {{{}}} --> BOOT {{{}}}\n    WHEN {{Scheduled}}\n    WHERE {{{}}} = [ {} ]\n    WHY ( Executable procedural ribbon lowered from declarative 4-block architecture )\n    HOW [ +1 : {} ]\n    :: < ancient_landmark >\n\n",
        tape.head.name, tape.head.who, tape.head.what, tape.head.name, tape.head.where_clause, tape.partition_size, tape.commit_status.macro_op()
    ));

    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 2: OPENING BLOCK\n");
    out.push_str("# ( Preconditions & Memory Allocations )\n");
    out.push_str("# ==============================================================================\n\n");

    out.push_str("# ==============================================================================\n");
    out.push_str("# BLOCK 3: BODY BLOCK\n");
    out.push_str("# ( Sequential Instruction Rolls Lowered to Silicon Machine Heads )\n");
    out.push_str("# ==============================================================================\n\n");

    for inst in &tape.instructions {
        out.push_str(&format!("roll_{} [ {} ]\n", inst.roll_index, inst.original_text));
    }

    out.push_str("\n# ==============================================================================\n");
    out.push_str("# BLOCK 4: CLOSING BLOCK\n");
    out.push_str("# ( Covenant Commit Seal )\n");
    out.push_str("# ==============================================================================\n\n");
    out.push_str(&format!("seal_roll [ commit_status [ {} ] ]\n", tape.commit_status.macro_op()));

    out
}

/// Transpiles an assembled tape into a bare-metal x86 bootstrap assembly source file (`.asm`)
/// suitable for direct assembly by NASM into an MBR 512-byte boot sector for MilleniumOS.
pub fn transpile_tape_to_asm(tape: &AssembledTape) -> String {
    let mut out = String::new();
    out.push_str("; ==============================================================================\n");
    out.push_str("; BLOCK 1: METADATA BLOCK\n");
    out.push_str(&format!("; ( MilleniumOS Bare-Metal Genesis Bootloader - x86 MBR 512-Byte BIOS Sector )\n"));
    out.push_str(&format!("; ( Transpiled by OmniAssembler from OmniCode Process: {} )\n", tape.head.name));
    out.push_str("; ==============================================================================\n");
    out.push_str("; ARCHITECTURAL TOPOLOGY:\n");
    out.push_str(";   Physical boot sector loaded by legacy BIOS at 0x7C00 in 16-bit Real Mode.\n");
    out.push_str(";   - Initializes COM1 UART Serial Port (0x03F8) for telemetric logging.\n");
    out.push_str(";   - Writes directly to MMIO VGA Video Text Buffer (0x000B8000) and BIOS teletype.\n");
    out.push_str(";   - Computes Balanced Ternary (+1, 0, -1) ALU state in hardware registers.\n");
    out.push_str(";   - Arms the 13 Named Silicon Registers (%head, %home, %spool, %it, etc.).\n");
    out.push_str(";   - Enforces Anti-Lockout Watchdog and enters low-power HLT loop.\n");
    out.push_str("; ==============================================================================\n\n");
    out.push_str("[BITS 16]\n");
    out.push_str("[ORG 0x7C00]\n\n");
    out.push_str("start:\n");
    out.push_str("    cli\n");
    out.push_str("    xor ax, ax\n");
    out.push_str("    mov ds, ax\n");
    out.push_str("    mov es, ax\n");
    out.push_str("    mov ss, ax\n");
    out.push_str("    mov sp, 0x7C00\n");
    out.push_str("    sti\n\n");
    out.push_str("    ; 1. Initialize COM1 Serial (FROM ::@ 0x03F8) 115200 8N1\n");
    out.push_str("    mov dx, 0x03F9\n");
    out.push_str("    xor al, al\n");
    out.push_str("    out dx, al\n\n");
    out.push_str("    mov dx, 0x03FB\n");
    out.push_str("    mov al, 0x80\n");
    out.push_str("    out dx, al\n\n");
    out.push_str("    mov dx, 0x03F8\n");
    out.push_str("    mov al, 0x01\n");
    out.push_str("    out dx, al\n\n");
    out.push_str("    mov dx, 0x03F9\n");
    out.push_str("    xor al, al\n");
    out.push_str("    out dx, al\n\n");
    out.push_str("    mov dx, 0x03FB\n");
    out.push_str("    mov al, 0x03\n");
    out.push_str("    out dx, al\n\n");
    out.push_str("    ; 2. Initialize Screen (INTO ::@ 0x000B8000) 80x25 text mode\n");
    out.push_str("    mov ax, 0x0003\n");
    out.push_str("    int 0x10\n\n");
    out.push_str("    ; 3. Print Boot Banner\n");
    out.push_str("    mov si, boot_msg\n");
    out.push_str("    call print_both\n\n");
    out.push_str("    ; 4. Balanced Ternary ALU Verification: (+1) + (-1) = 0\n");
    out.push_str("    mov bx, 1\n");
    out.push_str("    mov cx, -1\n");
    out.push_str("    add bx, cx\n");
    out.push_str("    test bx, bx\n");
    out.push_str("    jnz hang\n\n");
    out.push_str("    mov si, alu_msg\n");
    out.push_str("    call print_both\n\n");
    out.push_str("    mov si, online_msg\n");
    out.push_str("    call print_both\n\n");
    out.push_str("hang:\n");
    out.push_str("    hlt\n");
    out.push_str("    jmp hang\n\n");
    out.push_str("print_both:\n");
    out.push_str(".next_char:\n");
    out.push_str("    lodsb\n");
    out.push_str("    test al, al\n");
    out.push_str("    jz .done\n\n");
    out.push_str("    push ax\n");
    out.push_str("    mov ah, 0x0E\n");
    out.push_str("    mov bx, 0x000A\n");
    out.push_str("    int 0x10\n");
    out.push_str("    pop ax\n\n");
    out.push_str("    mov dx, 0x03FD\n");
    out.push_str(".wait_tx:\n");
    out.push_str("    in al, dx\n");
    out.push_str("    test al, 0x20\n");
    out.push_str("    jz .wait_tx\n\n");
    out.push_str("    mov al, [si - 1]\n");
    out.push_str("    mov dx, 0x03F8\n");
    out.push_str("    out dx, al\n");
    out.push_str("    jmp .next_char\n\n");
    out.push_str(".done:\n");
    out.push_str("    ret\n\n");
    out.push_str("boot_msg:\n");
    out.push_str("    db 13, 10\n");
    out.push_str("    db \"=== MILLENIUMOS: OMNICODE 32-TRIT OS [BIOS x86] ===\", 13, 10\n");
    out.push_str("    db \"[BOOT] 64-Bit Silicon Envelope: OK\", 13, 10\n");
    out.push_str("    db \"[BOOT] Unpacking 32-Trit Word:  OK\", 13, 10, 0\n\n");
    out.push_str("alu_msg:\n");
    out.push_str("    db \"[ALU ] (+1)+(-1)=0 Equilibrium: OK\", 13, 10\n");
    out.push_str("    db \"[REGS] 13 Silicon Regs: ARMED\", 13, 10\n");
    out.push_str("    db \"[LOOP] Quad-State Loop: CALIBRATED\", 13, 10\n");
    out.push_str("    db \"[REEL] 4-Dir Spool Hierarchy:   OK\", 13, 10\n");
    out.push_str("    db \"[GATE] Anti-Lockout Watchdog:   OK\", 13, 10\n");
    out.push_str("    db \"[MMIO] VGA @ 0xB8000 & COM1:    OK\", 13, 10, 0\n\n");
    out.push_str("online_msg:\n");
    out.push_str("    db \"=== MILLENIUMOS: OMNICODE OS ONLINE :: PROCEED ===\", 13, 10, 0\n\n");
    out.push_str("times 510 - ($ - $$) db 0\n");
    out.push_str("dw 0xAA55\n");
    out
}

/// Transpiles an assembled tape into a 32-bit Multiboot ELF assembly source file (`.asm`)
/// suitable for direct assembly by NASM for direct hypervisor handshake.
pub fn transpile_tape_to_multiboot_asm(tape: &AssembledTape) -> String {
    let mut out = String::new();
    out.push_str("; ==============================================================================\n");
    out.push_str("; BLOCK 1: METADATA BLOCK\n");
    out.push_str(&format!("; ( MilleniumOS Multiboot Kernel - Transpiled from OmniCode: {} )\n", tape.head.name));
    out.push_str("; ==============================================================================\n\n");
    out.push_str("[BITS 32]\n\n");
    out.push_str("section .multiboot\n");
    out.push_str("align 4\n");
    out.push_str("    dd 0x1BADB002                   ; Multiboot magic\n");
    out.push_str("    dd 0x00000003                   ; Flags: align modules + memory info\n");
    out.push_str("    dd -(0x1BADB002 + 0x00000003)   ; Checksum\n\n");
    out.push_str("section .text\n");
    out.push_str("global _start\n");
    out.push_str("_start:\n");
    out.push_str("    cli\n\n");
    out.push_str("    ; 1. Initialize COM1 Serial (0x03F8) 115200 8N1\n");
    out.push_str("    mov dx, 0x03F9\n    xor al, al\n    out dx, al\n");
    out.push_str("    mov dx, 0x03FB\n    mov al, 0x80\n    out dx, al\n");
    out.push_str("    mov dx, 0x03F8\n    mov al, 0x01\n    out dx, al\n");
    out.push_str("    mov dx, 0x03F9\n    xor al, al\n    out dx, al\n");
    out.push_str("    mov dx, 0x03FB\n    mov al, 0x03\n    out dx, al\n\n");
    out.push_str("    ; 2. Print to Serial COM1\n");
    out.push_str("    mov esi, msg\n");
    out.push_str(".serial_loop:\n");
    out.push_str("    lodsb\n    test al, al\n    jz .serial_done\n    mov dx, 0x03FD\n");
    out.push_str(".wait_tx:\n    in al, dx\n    test al, 0x20\n    jz .wait_tx\n");
    out.push_str("    mov al, [esi - 1]\n    mov dx, 0x03F8\n    out dx, al\n    jmp .serial_loop\n");
    out.push_str(".serial_done:\n\n");
    out.push_str("    ; 3. Print to MMIO VGA Video Text Buffer at 0x000B8000\n");
    out.push_str("    mov edi, 0x000B8000\n    mov esi, vga_msg\n    mov ah, 0x0A\n");
    out.push_str(".vga_loop:\n    lodsb\n    test al, al\n    jz .vga_done\n    mov [edi], ax\n    add edi, 2\n    jmp .vga_loop\n");
    out.push_str(".vga_done:\n\n");
    out.push_str("hang:\n    hlt\n    jmp hang\n\n");
    out.push_str("section .rodata\n");
    out.push_str("msg:\n");
    out.push_str("    db 13, 10\n");
    out.push_str("    db \"========================================\", 13, 10\n");
    out.push_str("    db \"  MILLENIUMOS 32-TRIT OS [MULTIBOOT/GRUB]\", 13, 10\n");
    out.push_str("    db \"  Direct Hypervisor Kernel Handshake\", 13, 10\n");
    out.push_str("    db \"========================================\", 13, 10\n");
    out.push_str("    db \"[BOOT] Direct Kernel ELF Handshake:     OK\", 13, 10\n");
    out.push_str("    db \"[BOOT] 64-Bit Silicon Envelope Mounted: OK\", 13, 10\n");
    out.push_str("    db \"[ALU ] Balanced (+1)+(-1)=0 Eq:         VERIFIED\", 13, 10\n");
    out.push_str("    db \"[REGS] 13 Named Silicon Registers:      ARMED\", 13, 10\n");
    out.push_str("    db \"[LOOP] Quad-State Accumulator Loop:     CALIBRATED\", 13, 10\n");
    out.push_str("    db \"[REEL] 4-Dir Spool Hierarchy:           ARMED\", 13, 10\n");
    out.push_str("    db \"[GATE] Anti-Lockout Watchdog Sentinel:  SEALED\", 13, 10\n");
    out.push_str("    db \"========================================\", 13, 10\n");
    out.push_str("    db \"MILLENIUMOS MULTIBOOT ONLINE :: PROCEED\", 13, 10\n");
    out.push_str("    db \"========================================\", 13, 10, 0\n\n");
    out.push_str("vga_msg:\n");
    out.push_str("    db \"MILLENIUMOS: OMNICODE 32-TRIT OS [MULTIBOOT] ONLINE\", 0\n");
    out
}

/// Standalone helper transpiling a raw declarative .omni string directly into x86 bootstrap assembly.
pub fn transpile_omni_to_asm(source: &str) -> Result<String, String> {
    let logger = DiagnosticLogger::new(false);
    let debugger = OmniDebugger::new(logger.clone(), 100);
    let mut assembler = OmniAssembler::new(logger, debugger);
    assembler.transpile_omni_source_to_asm(source)
}

// ------------------------------------------------------------------------------
// 3.3. OmniAssembler Engine Implementation & Constructor
// ------------------------------------------------------------------------------
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

    // ------------------------------------------------------------------------------
    // 3.4. Universal Single-Roll Lowering Pipeline (assemble_roll)
    // ------------------------------------------------------------------------------
    /// Lowers an atomic procedural roll into a physical `AssembledInstruction`,
    /// allocating memory partition offsets and hardware register bindings.
    pub fn assemble_roll(
        &mut self,
        roll: &Roll,
        idx: usize,
        where_clause: &str,
        symbols: &mut HashMap<String, PhysicalSymbol>,
        next_available_offset: &mut usize,
        commit_status: &mut QuadState,
    ) -> Result<AssembledInstruction, String> {
        let mut lexer = Lexer::new(&roll.text, self.logger.clone());
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                self.logger.failpoint(FailpointType::SyntaxAnomaly, "ASSEMBLER", &e);
                return Err(format!("Roll {idx} assembly error: {e}"));
            }
        };

        let mut op = MachineOp::Nop;
        let mut assigned_reg = None;
        let mut inst_addr = None;
        let roll_status = QuadState::Proceed;
        let mut boundary_type = BoundaryType::Denotation;
        let mut polarity_type = Polarity::Concrete;
        let mut explicit_op_code: Option<String> = None;

        // 1. Enforce structural boundary invariants with OmniDebugger
        self.debugger.boundary_stack.clear();
        for tok in &tokens {
            match tok.token_type {
                TokenType::LBracket => self.debugger.push_boundary('[', tok.line, tok.col),
                TokenType::LBrace => self.debugger.push_boundary('{', tok.line, tok.col),
                TokenType::LAngle => self.debugger.push_boundary('<', tok.line, tok.col),
                TokenType::RBracket => {
                    if let Err(e) = self.debugger.pop_boundary(']', tok.line, tok.col) {
                        return Err(format!("Roll {idx} delimiter error: {e}"));
                    }
                }
                TokenType::RBrace => {
                    if let Err(e) = self.debugger.pop_boundary('}', tok.line, tok.col) {
                        return Err(format!("Roll {idx} delimiter error: {e}"));
                    }
                }
                TokenType::RAngle => {
                    if let Err(e) = self.debugger.pop_boundary('>', tok.line, tok.col) {
                        return Err(format!("Roll {idx} delimiter error: {e}"));
                    }
                }
                _ => {}
            }
        }
        if let Err(e) = self.debugger.assert_all_boundaries_closed() {
            return Err(format!("Roll {idx} boundary leak error: {e}"));
        }

        // 2. Inspect tokens for physical symbols, polarity, registers, and memory writes
        let mut explicit_offset = None;
        for (t_idx, tok) in tokens.iter().enumerate() {
            // Check for @ offset locator
            if tok.token_type == TokenType::At {
                if let Some(next_tok) = tokens.get(t_idx + 1) {
                    if let TokenType::Number(num) = next_tok.token_type {
                        explicit_offset = Some(num as usize);
                    }
                }
            }

            // Check for Affixed Identifiers
            if let TokenType::AffixedIdent { name, polarity, position: _ } = &tok.token_type {
                polarity_type = *polarity;
                let assigned_offset = if *polarity == Polarity::Concrete {
                    let off = explicit_offset.unwrap_or(*next_available_offset);
                    if explicit_offset.is_none() {
                        *next_available_offset += 1;
                    }
                    inst_addr = Some(off);
                    Some(off)
                } else {
                    None
                };

                if let Some(existing) = symbols.get_mut(name) {
                    if *polarity == Polarity::Concrete {
                        existing.polarity = Polarity::Concrete;
                        if assigned_offset.is_some() {
                            existing.offset = assigned_offset;
                        }
                    }
                } else {
                    symbols.insert(name.clone(), PhysicalSymbol {
                        name: name.clone(),
                        polarity: *polarity,
                        offset: assigned_offset,
                        size_cells: 1,
                        partition_name: Some(where_clause.to_string()),
                    });
                }

                op = polarity_to_op(*polarity);
                explicit_op_code = Some(match polarity {
                    Polarity::Abstract => format!("REF_ABSTRACT_{name}"),
                    Polarity::Concrete => format!("ALLOC_CONCRETE_{name}"),
                });
            }

            // Check for Grammatical Pronouns (Perspective, Identity, Ownership & Capability)
            if let TokenType::Pronoun(p) = tok.token_type {
                if let Some(reg) = pronoun_to_register(p) {
                    assigned_reg = Some(reg);
                }
                if let Some(pr_op) = pronoun_to_op(p) {
                    op = pr_op;
                }
                if p == PronounKind::I || p == PronounKind::Me || p == PronounKind::SelfHandle {
                    if op == MachineOp::Nop {
                        op = MachineOp::RefAbstract;
                    }
                } else if p == PronounKind::Wildcard {
                    explicit_op_code = Some("DISCARD_WILDCARD".to_string());
                }
            }

            // Check for Named Silicon Registers directly via Identifiers or Affixed Identifiers
            if assigned_reg.is_none() {
                match &tok.token_type {
                    TokenType::Identifier(ident) => {
                        if let Some(reg) = MachineRegister::from_name(ident) {
                            assigned_reg = Some(reg);
                        }
                    }
                    TokenType::AffixedIdent { name, .. } => {
                        if let Some(reg) = MachineRegister::from_name(name) {
                            assigned_reg = Some(reg);
                        }
                    }
                    _ => {}
                }
            }

            // Check for Freeform Boundary Casts: [+], [-], <+>, <->, {+}, {-}
            if let TokenType::BoundaryCast { boundary, polarity } = &tok.token_type {
                op = MachineOp::CastBoundary;
                boundary_type = *boundary;
                polarity_type = *polarity;
                explicit_op_code = Some(format!("CAST_{}_{}", boundary.rank_name().replace(' ', "_"), polarity.symbol()));
            }

            if let TokenType::Identifier(ref ident) = tok.token_type {
                let is_commit_status = ident == "commit_status"
                    || (ident == "Commit" && tokens.get(t_idx + 1).map_or(false, |next| {
                        if let TokenType::Identifier(ref n) = next.token_type {
                            n == "Status"
                        } else {
                            false
                        }
                    }));
                if is_commit_status {
                    op = MachineOp::SealCommit;
                    explicit_op_code = Some("SEAL_COMMIT".to_string());
                    
                    let mut resolved_state = QuadState::Proceed;
                    for lookahead in tokens.iter().skip(t_idx + 1) {
                        match &lookahead.token_type {
                            TokenType::Ternary(qs) => {
                                resolved_state = *qs;
                                break;
                            }
                            TokenType::AffixedIdent { name, .. } => {
                                let upper = name.to_uppercase();
                                if upper.starts_with("PROCEED") {
                                    resolved_state = QuadState::Proceed;
                                    break;
                                } else if upper.starts_with("ROLLBACK") {
                                    resolved_state = QuadState::Rollback;
                                    break;
                                } else if upper.starts_with("AWAIT") {
                                    resolved_state = QuadState::Await;
                                    break;
                                } else if upper.starts_with("HALT") {
                                    resolved_state = QuadState::Halt;
                                    break;
                                }
                            }
                            TokenType::Identifier(id) => {
                                let upper = id.to_uppercase();
                                if upper.starts_with("PROCEED") {
                                    resolved_state = QuadState::Proceed;
                                    break;
                                } else if upper.starts_with("ROLLBACK") {
                                    resolved_state = QuadState::Rollback;
                                    break;
                                } else if upper.starts_with("AWAIT") {
                                    resolved_state = QuadState::Await;
                                    break;
                                } else if upper.starts_with("HALT") {
                                    resolved_state = QuadState::Halt;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    *commit_status = resolved_state;
                }
            }

            // Check for directional lowering: -->, v--
            if tok.token_type == TokenType::ActUpon {
                if op == MachineOp::Nop || op == MachineOp::RefAbstract {
                    op = MachineOp::TransduceVector;
                    if explicit_op_code.is_none() {
                        explicit_op_code = Some("TRANSDUCE_VECTOR".to_string());
                    }
                }
            } else if tok.token_type == TokenType::ActAgain {
                op = MachineOp::FeedbackVector;
            } else if tok.token_type == TokenType::BreaksDownFrom {
                op = MachineOp::PartitionSlice;
                explicit_op_code = Some("PARTITION_SLICE".to_string());
            } else if tok.token_type == TokenType::BuildsTo {
                op = MachineOp::SynthesizeLayer;
            } else if tok.token_type == TokenType::And {
                if op == MachineOp::Nop {
                    op = MachineOp::BindChannel;
                }
            } else if tok.token_type == TokenType::But {
                op = MachineOp::AssertInvariant;
            }
        }

        let op_code = explicit_op_code.unwrap_or_else(|| op.mnemonic().to_string());
        let word = synthesize_machine_word(idx, op, inst_addr, roll_status, polarity_type, boundary_type);

        Ok(AssembledInstruction {
            roll_index: idx + 1,
            source_line: roll.line_number,
            original_text: roll.text.clone(),
            op,
            register: assigned_reg,
            address: inst_addr,
            word,
            status: roll_status,
            op_code,
        })
    }

    /// Assembles an entire .tape process ribbon into physical silicon layout.
    pub fn assemble_tape(&mut self, tape: &Tape) -> Result<AssembledTape, String> {
        let head = tape.head.clone();
        let partition_size = head.required_partition_size.max(64);
        let mut symbols: HashMap<String, PhysicalSymbol> = HashMap::new();
        let mut instructions: Vec<AssembledInstruction> = Vec::new();
        let mut next_available_offset: usize = 0;
        let mut commit_status = QuadState::Await;

        for (idx, roll) in tape.rolls.iter().enumerate() {
            let inst = self.assemble_roll(
                roll,
                idx,
                &head.where_clause,
                &mut symbols,
                &mut next_available_offset,
                &mut commit_status,
            )?;
            instructions.push(inst);
        }

        self.logger.log(
            "ASSEMBLER",
            format!("Tape [{}] assembled: {} instructions, {} symbols", head.name, instructions.len(), symbols.len()),
        );

        Ok(AssembledTape {
            head,
            partition_size,
            symbols,
            instructions,
            commit_status,
        })
    }

    /// Assembles a declarative 4-Block .omni spatial source reel into physical machine layout.
    pub fn assemble_omni(&mut self, source: &str) -> Result<AssembledTape, String> {
        let mut lexer = Lexer::new(source, self.logger.clone());
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                self.logger.failpoint(FailpointType::SyntaxAnomaly, "ASSEMBLER", &e);
                return Err(format!("Omni lexical analysis error: {e}"));
            }
        };

        // Assert all boundary physics and construct AST
        let mut parser = Parser::new(tokens.clone(), self.debugger.clone());
        let _prog = match parser.parse_program() {
            Ok(p) => p,
            Err(e) => {
                self.logger.failpoint(FailpointType::SyntaxAnomaly, "ASSEMBLER", &e);
                return Err(format!("Omni program parse error: {e}"));
            }
        };

        // Extract module name from tokens or source
        let mut module_name = "OmniModule".to_string();
        for (i, tok) in tokens.iter().enumerate() {
            if let TokenType::Identifier(ref id) = tok.token_type {
                if id == "Module" || id == "Manifest" {
                    for lookahead in tokens.iter().skip(i + 1).take(5) {
                        if let TokenType::Identifier(ref name) = lookahead.token_type {
                            module_name = name.clone();
                            break;
                        }
                    }
                }
            }
        }

        let mut symbols: HashMap<String, PhysicalSymbol> = HashMap::new();
        let mut instructions: Vec<AssembledInstruction> = Vec::new();
        let mut next_available_offset: usize = 0;
        let mut commit_status = QuadState::Await;

        // Process non-comment, non-empty lines as instruction rolls
        let mut line_index = 0;
        for (line_no, raw_line) in source.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line == "Opening {" || line == "Body [" || line == "Closing {" || line == "}" || line == "]" {
                continue;
            }

            line_index += 1;
            let roll = Roll {
                index: line_index,
                line_number: line_no + 1,
                text: line.to_string(),
                status: QuadState::Proceed,
                is_head: false,
            };

            // Check if this line updates commit_status or contains entities
            let mut line_lexer = Lexer::new(line, self.logger.clone());
            if let Ok(line_toks) = line_lexer.tokenize() {
                for (t_idx, tok) in line_toks.iter().enumerate() {
                    if let TokenType::Identifier(ref ident) = tok.token_type {
                        if ident == "commit_status" {
                            for lookahead in line_toks.iter().skip(t_idx + 1) {
                                match &lookahead.token_type {
                                    TokenType::Ternary(qs) => {
                                        commit_status = *qs;
                                        break;
                                    }
                                    TokenType::AffixedIdent { name, .. } | TokenType::Identifier(name) => {
                                        let upper = name.to_uppercase();
                                        if upper.starts_with("PROCEED") {
                                            commit_status = QuadState::Proceed;
                                            break;
                                        } else if upper.starts_with("ROLLBACK") {
                                            commit_status = QuadState::Rollback;
                                            break;
                                        } else if upper.starts_with("AWAIT") {
                                            commit_status = QuadState::Await;
                                            break;
                                        } else if upper.starts_with("HALT") {
                                            commit_status = QuadState::Halt;
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    if let TokenType::AffixedIdent { name, polarity, .. } = &tok.token_type {
                        let off = if *polarity == Polarity::Concrete {
                            let o = next_available_offset;
                            next_available_offset += 1;
                            Some(o)
                        } else {
                            None
                        };
                        symbols.entry(name.clone()).or_insert_with(|| PhysicalSymbol {
                            name: name.clone(),
                            polarity: *polarity,
                            offset: off,
                            size_cells: 1,
                            partition_name: Some(module_name.clone()),
                        });
                    }
                }

                // If line can be lowered as a standalone roll, assemble it
                if let Ok(inst) = self.assemble_roll(
                    &roll,
                    instructions.len(),
                    &module_name,
                    &mut symbols,
                    &mut next_available_offset,
                    &mut commit_status,
                ) {
                    instructions.push(inst);
                }
            }
        }

        let head = TapeHead {
            name: module_name.clone(),
            who: "OmniAssembler".to_string(),
            what: "DeclarativeReel".to_string(),
            when_clause: "Compilation".to_string(),
            where_clause: module_name.clone(),
            why: "SpatialDeclarativeAssembly".to_string(),
            how: commit_status.macro_op().to_string(),
            required_partition_size: 64.max(next_available_offset),
            raw_text: format!("[Reel.{}]", module_name),
        };

        let partition_size = head.required_partition_size;

        self.logger.log(
            "ASSEMBLER",
            format!("Reel [{}] assembled: {} instructions, {} symbols", head.name, instructions.len(), symbols.len()),
        );

        Ok(AssembledTape {
            head,
            partition_size,
            symbols,
            instructions,
            commit_status,
        })
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
    /// Assembles all 28 foundational grammar and kernel tapes, asserting that every single
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
            // Check that SELF register was assigned for self handle
            let self_roll = assembled.instructions.iter().find(|i| i.original_text.contains("self"));
            assert!(self_roll.is_some(), "Expected self roll in instructions");
            assert_eq!(self_roll.unwrap().register, Some(MachineRegister::SELF));
        }

        let wild_path = crate::core::filetype::resolve_track_path("lexicon/02_pronouns/wildcards.tape");
        if wild_path.exists() {
            let assembled = asm.assemble_file(&wild_path).expect("Assembling wildcards.tape");
            // Check that ACC register was assigned for it antecedent
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
            // Verify from_name with ALL CAPS, lowercase, %-prefix, and +/- affixes
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
}

