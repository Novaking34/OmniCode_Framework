// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Physical Machine ISA, Silicon Registers, Word Synthesis & Mappings )
// ==============================================================================
//! OmniCode Runtime: Physical Machine ISA & Silicon Registers
//!
//! # Architectural Scope & Purpose:
//! Defines the core Instruction Set Architecture (ISA) of the OmniCode
//! Balanced-Ternary Virtual Machine:
//! 1. `MachineOp`: Hardware machine opcodes mapping abstract grammatical statements
//!    to physical silicon execution.
//! 2. `MachineRegister`: The 13 Named Physical Silicon Registers mapping natural
//!    tape cursors and grammatical perspectives to physical CPU cells.
//! 3. Universal linguistic-to-silicon mappings (`pronoun_to_register`, `pronoun_to_op`,
//!    `polarity_to_op`).
//! 4. 64-bit balanced-ternary machine word synthesis (`synthesize_machine_word`,
//!    `compute_word_checksum`).
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable archetype of physical inscription upon silicon and stone).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic ISA definitions before building lowering engines).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of physical memory boundaries, CPU registers, and opcodes).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Silicon Invariants**: Explicit opcode numbers and register indices guarantee
//!    zero drift between high-level language tapes and bare-metal execution targets.
// ==============================================================================

use crate::core::boundaries::BoundaryType;
use crate::core::linguistics::PronounKind;
use crate::core::operators::Polarity;
use crate::core::ternary::QuadState;
use crate::core::word::{Word32Trit, Word64};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( MachineOp & MachineRegister Enums )
// ==============================================================================

/// Hardware Machine Opcodes for the OmniCode Balanced-Ternary Virtual Machine.
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
        match self {
            Self::TransduceVector | Self::FeedbackVector | Self::PartitionSlice | Self::SynthesizeLayer => true,
            _ => false,
        }
    }

    /// Returns `true` if this opcode reserves or affects physical silicon memory cells.
    pub fn is_memory_allocation(&self) -> bool {
        match self {
            Self::AllocConcrete | Self::PartitionSlice => true,
            _ => false,
        }
    }
}

/// Physical Machine CPU Registers for the Balanced-Ternary Virtual Machine.
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

    /// Resolves any register string back to its enum variant.
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
            "from" | "input" | "in" | "fr" | "source" => Some(Self::FROM),
            "into" | "output" | "out" | "to" | "target" => Some(Self::INTO),
            "discard" | "void" | "_" | "dc" => Some(Self::DISCARD),
            _ => None,
        }
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Universal Linguistic Bridges & Machine Word Synthesis )
// ==============================================================================

/// Universal mapping from grammatical pronouns to physical CPU registers.
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

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Verification Invariants )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_op_mnemonics() {
        assert_eq!(MachineOp::Nop.mnemonic(), "NOP");
        assert_eq!(MachineOp::from_mnemonic("ALLOC_CONCRETE"), Some(MachineOp::AllocConcrete));
        assert!(MachineOp::TransduceVector.is_causal());
        assert!(MachineOp::AllocConcrete.is_memory_allocation());
    }

    #[test]
    fn test_machine_register_resolution() {
        assert_eq!(MachineRegister::HEAD.index(), 0);
        assert_eq!(MachineRegister::from_name("%head"), Some(MachineRegister::HEAD));
        assert_eq!(MachineRegister::from_name("ACC"), Some(MachineRegister::IT));
        assert_eq!(MachineRegister::from_name("void"), Some(MachineRegister::DISCARD));
    }

    #[test]
    fn test_machine_word_synthesis() {
        let w = synthesize_machine_word(
            0,
            MachineOp::AllocConcrete,
            Some(0x10),
            QuadState::Proceed,
            Polarity::Concrete,
            BoundaryType::Denotation,
        );
        assert_eq!(w.state(), QuadState::Proceed);
        assert_eq!(w.polarity(), Polarity::Concrete);
    }
}
