// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Linguistic Engine Identity, 8 Parts of Speech, & Grammatical Classification )
// ==============================================================================
//! OmniCode Core: Universal Linguistics & Grammatical Classification
//!
//! Maps every token, delimiter, vector, and quad-state directly into the
//! 8 canonical parts of speech in human natural language:
//!     1. Noun          (Entities, types, structures, literals, values)
//!     2. Pronoun       (Pointers, self-references, context handles)
//!     3. Verb          (Vectors, state transitions, operations, relations of being)
//!     4. Adjective     (Invariants, type qualifiers, contracts, quad-states)
//!     5. Adverb        (Execution manner, scheduling priority, negation)
//!     6. Preposition   (Indexing, spatial offsets, projections, scoping)
//!     7. Conjunction   (Connectives, boolean logic, pipeline caveats)
//!     8. Interjection   (Interrupts, traps, halts, parenthetical Asides)

use crate::core::boundaries::BoundaryType;
use crate::core::operators::{BeingRelation, Connective, DirectionalVector, Polarity};
use crate::core::ternary::QuadState;
use crate::parser::lexer::TokenType;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Grammatical Role Definitions, Classifications & Mapping Structures )
// ==============================================================================

/// The canonical 8 parts of speech in natural human grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartOfSpeech {
    Noun,
    Pronoun,
    Verb,
    Adjective,
    Adverb,
    Preposition,
    Conjunction,
    Interjection,
}

impl PartOfSpeech {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Noun => "Noun",
            Self::Pronoun => "Pronoun",
            Self::Verb => "Verb",
            Self::Adjective => "Adjective",
            Self::Adverb => "Adverb",
            Self::Preposition => "Preposition",
            Self::Conjunction => "Conjunction",
            Self::Interjection => "Interjection",
        }
    }
}

/// A comprehensive grammatical classification supporting primary, secondary,
/// and tertiary functional roles across natural language and computation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammaticalClassification {
    /// The primary syntactic and semantic role in the sentence.
    pub primary: PartOfSpeech,
    /// The secondary role when functioning within different syntactic contexts.
    pub secondary: Option<PartOfSpeech>,
    /// The tertiary role for edge cases or multi-tier semantics.
    pub tertiary: Option<PartOfSpeech>,
    /// Natural language explanation of its linguistic function.
    pub linguistic_role: &'static str,
    /// Corresponding computer science / systems counterpart.
    pub computing_counterpart: &'static str,
}

impl GrammaticalClassification {
    pub fn new(
        primary: PartOfSpeech,
        secondary: Option<PartOfSpeech>,
        tertiary: Option<PartOfSpeech>,
        linguistic_role: &'static str,
        computing_counterpart: &'static str,
    ) -> Self {
        Self {
            primary,
            secondary,
            tertiary,
            linguistic_role,
            computing_counterpart,
        }
    }

    pub fn is(&self, pos: PartOfSpeech) -> bool {
        self.primary == pos || self.secondary == Some(pos) || self.tertiary == Some(pos)
    }
}

/// Grammatical case of a pronoun in sentence structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PronounCase {
    /// Subjective / Nominative: Initiates action / holds execution frame (e.g., `I`, `we`, `you`, `he`, `she`, `it`, `they`).
    Subjective,
    /// Objective / Accusative: Recipient / target of action / inbound buffer (e.g., `me`, `us`, `him`, `her`, `them`).
    Objective,
    /// Reflexive / Introspective: Self-referential inspection / recursion (e.g., `myself`, `itself`, `themselves`).
    Reflexive,
    /// Possessive Determiner / Capability Token: Ownership / access rights (e.g., `my`, `your`, `his`, `her`, `its`, `our`, `their`).
    Possessive,
}

/// Person perspective of a pronoun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PronounPerson {
    /// First person: Active self, kernel engine, or cooperative covenant (`I`, `me`, `we`, `us`, `myself`).
    First,
    /// Second person: Interlocutor, caller, user, or client socket (`you`, `yourself`, `your`).
    Second,
    /// Third person: External entity, data vessel, antecedent, or peer thread (`it`, `he`, `she`, `they`).
    Third,
}

/// Grammatical number of a pronoun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PronounNumber {
    Singular,
    Plural,
}

/// The canonical spectrum of grammatical pronouns in OmniCode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PronounKind {
    // --- First Person Singular (Kernel / Self) ---
    I,
    Me,
    Myself,
    My,
    Mine,

    // --- First Person Plural (Covenant / Composite) ---
    We,
    Us,
    Ourselves,
    Our,
    Ours,

    // --- Second Person (Caller / User / Interlocutor) ---
    You,
    Yourself,
    Yourselves,
    Your,
    Yours,

    // --- Third Person Singular Animate (Autonomous Worker / Peer) ---
    He,
    Him,
    Himself,
    His,
    She,
    Her,
    Herself,
    Hers,

    // --- Third Person Singular Inanimate (Data Vessel / Antecedent / Accumulator) ---
    It,
    Itself,
    Its,

    // --- Third Person Plural (Worker Pool / Swarm / Quorum) ---
    They,
    Them,
    Themselves,
    Their,
    Theirs,

    // --- System Context Handles & Wildcards ---
    SelfHandle,
    CallerHandle,
    OwnerHandle,
    Wildcard,
}

impl PronounKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::I => "I",
            Self::Me => "me",
            Self::Myself => "myself",
            Self::My => "my",
            Self::Mine => "mine",
            Self::We => "we",
            Self::Us => "us",
            Self::Ourselves => "ourselves",
            Self::Our => "our",
            Self::Ours => "ours",
            Self::You => "you",
            Self::Yourself => "yourself",
            Self::Yourselves => "yourselves",
            Self::Your => "your",
            Self::Yours => "yours",
            Self::He => "he",
            Self::Him => "him",
            Self::Himself => "himself",
            Self::His => "his",
            Self::She => "she",
            Self::Her => "her",
            Self::Herself => "herself",
            Self::Hers => "hers",
            Self::It => "it",
            Self::Itself => "itself",
            Self::Its => "its",
            Self::They => "they",
            Self::Them => "them",
            Self::Themselves => "themselves",
            Self::Their => "their",
            Self::Theirs => "theirs",
            Self::SelfHandle => "self",
            Self::CallerHandle => "caller",
            Self::OwnerHandle => "owner",
            Self::Wildcard => "_",
        }
    }

    pub fn case(&self) -> PronounCase {
        match self {
            Self::I | Self::We | Self::You | Self::He | Self::She | Self::It | Self::They | Self::SelfHandle | Self::CallerHandle | Self::OwnerHandle | Self::Wildcard => PronounCase::Subjective,
            Self::Me | Self::Us | Self::Him | Self::Her | Self::Them => PronounCase::Objective,
            Self::Myself | Self::Ourselves | Self::Yourself | Self::Yourselves | Self::Himself | Self::Herself | Self::Itself | Self::Themselves => PronounCase::Reflexive,
            Self::My | Self::Mine | Self::Our | Self::Ours | Self::Your | Self::Yours | Self::His | Self::Hers | Self::Its | Self::Their | Self::Theirs => PronounCase::Possessive,
        }
    }

    pub fn person(&self) -> PronounPerson {
        match self {
            Self::I | Self::Me | Self::Myself | Self::My | Self::Mine |
            Self::We | Self::Us | Self::Ourselves | Self::Our | Self::Ours |
            Self::SelfHandle => PronounPerson::First,

            Self::You | Self::Yourself | Self::Yourselves | Self::Your | Self::Yours |
            Self::CallerHandle => PronounPerson::Second,

            Self::He | Self::Him | Self::Himself | Self::His |
            Self::She | Self::Her | Self::Herself | Self::Hers |
            Self::It | Self::Itself | Self::Its |
            Self::They | Self::Them | Self::Themselves | Self::Their | Self::Theirs |
            Self::OwnerHandle | Self::Wildcard => PronounPerson::Third,
        }
    }

    pub fn number(&self) -> PronounNumber {
        match self {
            Self::We | Self::Us | Self::Ourselves | Self::Our | Self::Ours |
            Self::Yourselves |
            Self::They | Self::Them | Self::Themselves | Self::Their | Self::Theirs => PronounNumber::Plural,
            _ => PronounNumber::Singular,
        }
    }

    pub fn from_keyword(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "i" => Some(Self::I),
            "me" => Some(Self::Me),
            "myself" => Some(Self::Myself),
            "my" => Some(Self::My),
            "mine" => Some(Self::Mine),
            "we" => Some(Self::We),
            "us" => Some(Self::Us),
            "ourselves" => Some(Self::Ourselves),
            "our" => Some(Self::Our),
            "ours" => Some(Self::Ours),
            "you" => Some(Self::You),
            "yourself" => Some(Self::Yourself),
            "yourselves" => Some(Self::Yourselves),
            "your" => Some(Self::Your),
            "yours" => Some(Self::Yours),
            "he" => Some(Self::He),
            "him" => Some(Self::Him),
            "himself" => Some(Self::Himself),
            "his" => Some(Self::His),
            "she" => Some(Self::She),
            "her" => Some(Self::Her),
            "herself" => Some(Self::Herself),
            "hers" => Some(Self::Hers),
            "it" => Some(Self::It),
            "itself" => Some(Self::Itself),
            "its" => Some(Self::Its),
            "they" => Some(Self::They),
            "them" => Some(Self::Them),
            "themselves" => Some(Self::Themselves),
            "their" => Some(Self::Their),
            "theirs" => Some(Self::Theirs),
            "self" => Some(Self::SelfHandle),
            "caller" => Some(Self::CallerHandle),
            "owner" => Some(Self::OwnerHandle),
            "_" => Some(Self::Wildcard),
            _ => None,
        }
    }

    pub fn is_subjective(&self) -> bool {
        self.case() == PronounCase::Subjective
    }

    pub fn is_objective(&self) -> bool {
        self.case() == PronounCase::Objective
    }

    pub fn is_reflexive(&self) -> bool {
        self.case() == PronounCase::Reflexive
    }

    pub fn is_possessive(&self) -> bool {
        self.case() == PronounCase::Possessive
    }

    pub fn linguistic_role(&self) -> &'static str {
        match self {
            Self::I => "First-person singular subjective; represents the active executing identity / kernel",
            Self::Me => "First-person singular objective; represents self as the receiver of inbound action",
            Self::Myself => "First-person reflexive; drives introspective self-audit and invariant verification",
            Self::My | Self::Mine => "First-person possessive; denotes exclusive ownership and write capability",
            Self::We => "First-person plural subjective; denotes cooperative covenant (Kernel + User or Multi-Agent)",
            Self::Us => "First-person plural objective; denotes composite entity as target",
            Self::Ourselves => "First-person plural reflexive; drives collective group audit",
            Self::Our | Self::Ours => "First-person plural possessive; denotes shared covenant resource access",
            Self::You => "Second-person pronoun; identifies external interlocutor, user, or caller socket",
            Self::Yourself | Self::Yourselves => "Second-person reflexive; triggers remote caller verification",
            Self::Your | Self::Yours => "Second-person possessive; denotes borrowed caller capability token",
            Self::He | Self::She => "Third-person singular animate; designates specific peer worker or thread",
            Self::Him | Self::Her => "Third-person singular objective; identifies designated worker as target",
            Self::Himself | Self::Herself => "Third-person singular reflexive; isolates worker-internal state",
            Self::His | Self::Hers => "Third-person singular possessive; denotes worker-delegated capability",
            Self::It => "Third-person singular inanimate; represents data vessel, antecedent, or accumulator",
            Self::Itself => "Third-person reflexive; drives homoiconic tape rewriting or fixed-point recursion",
            Self::Its => "Third-person possessive; denotes vessel-internal property encapsulation",
            Self::They => "Third-person plural subjective; identifies worker pool, quorum, or parallel swarm",
            Self::Them => "Third-person plural objective; identifies worker pool as destination targets",
            Self::Themselves => "Third-person plural reflexive; verifies Byzantine swarm agreement or consensus",
            Self::Their | Self::Theirs => "Third-person plural possessive; denotes process-group capability mask",
            Self::SelfHandle => "Architectural local frame handle; resolves active stack/partition base",
            Self::CallerHandle => "Architectural caller handle; resolves invoking principal and frame return",
            Self::OwnerHandle => "Architectural owner handle; resolves root capability authority",
            Self::Wildcard => "Anonymous discard handle; suppresses destination binding or ignores noise",
        }
    }

    pub fn computing_counterpart(&self) -> &'static str {
        match self {
            Self::I => "Active Task PID / Execution Context Register / `&mut self`",
            Self::Me => "Destination Buffer in Self / Callee Register Inflow",
            Self::Myself => "Self-Audit Telemetry / Local Frame Invariant Check / Introspection",
            Self::My | Self::Mine => "Exclusive Memory Partition Lease / Write Capability Token",
            Self::We => "Composite Covenant Task / Dual-Principal Session / Multi-Agent Swarm",
            Self::Us => "Composite Destination Queue / Shared Inbound Buffer",
            Self::Ourselves => "Group Invariant Consensus Check / Quorum Self-Verification",
            Self::Our | Self::Ours => "Shared Memory Partition Capability / Covenant Lock",
            Self::You => "Caller Context Handle / `STDIN` Channel / Remote IPC Socket",
            Self::Yourself | Self::Yourselves => "Caller Self-Test RPC / Remote Health Probe",
            Self::Your | Self::Yours => "Borrowed Caller Capability Token / Foreign Slice Reference",
            Self::He | Self::She => "Designated Worker Thread PID / Specific Actor Descriptor",
            Self::Him | Self::Her => "Worker Inflow Channel / Task Mailbox",
            Self::Himself | Self::Herself => "Worker Local State Isolation / Actor Invariant Seal",
            Self::His | Self::Hers => "Worker Capability Delegation / Sub-Partition Lease",
            Self::It => "Pipeline Antecedent / Accumulator Register (`ACC`) / Top of Stack",
            Self::Itself => "Homoiconic Tape Rewriter / Circular Reel Wrap / Fixed-Point Recursion",
            Self::Its => "Vessel Field Encapsulation / Internal Property Offset",
            Self::They => "Process Group Descriptor / Parallel Thread Pool / Consensus Quorum",
            Self::Them => "Broadcast Channel / Multi-Partition Destination Slices",
            Self::Themselves => "Swarm Peer-to-Peer Consensus Alignment / Byzantine Quorum Proof",
            Self::Their | Self::Theirs => "Process Group Access Control List / Collective Capability Mask",
            Self::SelfHandle => "Frame Pointer Register (`FP`) / Local Execution Fiber",
            Self::CallerHandle => "Return Address / Caller Task Descriptor (`CALLER`)",
            Self::OwnerHandle => "Capability Authority Root / Security Principal Token",
            Self::Wildcard => "Bit Bucket (`/dev/null`) / Black Hole Register (`_`)",
        }
    }
}

impl GrammaticalEntity for PronounKind {
    fn classify(&self) -> GrammaticalClassification {
        let sec = match self.case() {
            PronounCase::Possessive => Some(PartOfSpeech::Adjective),
            PronounCase::Reflexive => Some(PartOfSpeech::Verb),
            PronounCase::Objective => Some(PartOfSpeech::Noun),
            PronounCase::Subjective => Some(PartOfSpeech::Noun),
        };
        GrammaticalClassification::new(
            PartOfSpeech::Pronoun,
            sec,
            None,
            self.linguistic_role(),
            self.computing_counterpart(),
        )
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Trait Implementations & Grammatical Classification Mappings )
// ==============================================================================

pub trait GrammaticalEntity {
    fn classify(&self) -> GrammaticalClassification;
}

impl GrammaticalEntity for TokenType {
    fn classify(&self) -> GrammaticalClassification {
        match self {
            // Delimiters
            TokenType::LBracket | TokenType::RBracket => GrammaticalClassification::new(
                PartOfSpeech::Noun,
                Some(PartOfSpeech::Preposition),
                None,
                "Denotation boundary container; spatially indexes inner state",
                "Array / Struct container; memory offset indexing `buf[idx]`",
            ),
            TokenType::LBrace | TokenType::RBrace => GrammaticalClassification::new(
                PartOfSpeech::Adjective,
                Some(PartOfSpeech::Conjunction),
                None,
                "Invariant contract; specifies necessary ambient conditions",
                "Type invariant / Precondition constraint / Scope qualification",
            ),
            TokenType::LAngle | TokenType::RAngle => GrammaticalClassification::new(
                PartOfSpeech::Adjective,
                Some(PartOfSpeech::Noun),
                None,
                "Immutable axiom boundary; names high-order categorical domains",
                "Type parameter / Scale invariant / Module domain constant",
            ),
            TokenType::Aside(_) => GrammaticalClassification::new(
                PartOfSpeech::Interjection,
                Some(PartOfSpeech::Adverb),
                None,
                "First-class teleological Aside; states machine intent and rationale",
                "Non-executable intent annotation / Compiler reasoning trace",
            ),

            // Grammatical Pronouns (Perspective, Identity, Ownership & Capability)
            TokenType::Pronoun(p) => p.classify(),

            // Polarity & Boundary Cast Operators
            TokenType::AffixedIdent { polarity, .. } => match polarity {
                Polarity::Abstract => GrammaticalClassification::new(
                    PartOfSpeech::Adjective,
                    Some(PartOfSpeech::Noun),
                    None,
                    "Abstract archetype polarity modifier (+Word); denotes universal concept or intension",
                    "Uninstantiated archetype / Type specification / Zero-cost blueprint",
                ),
                Polarity::Concrete => GrammaticalClassification::new(
                    PartOfSpeech::Noun,
                    Some(PartOfSpeech::Adjective),
                    None,
                    "Concrete substance polarity modifier (-Word / Word-); denotes physical instance",
                    "Physical memory allocation / Concrete hardware register footprint",
                ),
            },
            TokenType::BoundaryCast { .. } => GrammaticalClassification::new(
                PartOfSpeech::Preposition,
                Some(PartOfSpeech::Verb),
                None,
                "Boundary casting operator; modulates entity essence across boundary rank",
                "Type cast / Transduction between abstract and concrete representations",
            ),

            // Directional Vectors (Verbs of Action & Causation)
            TokenType::ActUpon => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Forward causation; acts upon target entity",
                "Store / Mutation / Pipeline emission / MOV",
            ),
            TokenType::ActAgain => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Retroactive reflection; draws from or reflects back upon source",
                "Load / Feedback / State rollback inspection",
            ),
            TokenType::BuildsTo => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Ascending action; synthesizes lower primitive into higher concept",
                "Type promotion / Emergence / Layering up to abstraction",
            ),
            TokenType::BreaksDownFrom => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Descending action; decomposes abstract concept into primitives",
                "Destructuring / Register unpack / Compilation lowering",
            ),
            TokenType::Major => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Adjective),
                None,
                "Major scale vector; ascending forward diagonal leap",
                "Major scale promotion / Leap to higher abstraction tier (`/-^`)",
            ),
            TokenType::Minor => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Adjective),
                None,
                "Minor scale vector; descending forward diagonal step",
                "Minor scale reduction / Settling to lower execution tier (`\\-v`)",
            ),
            TokenType::Great => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Adjective),
                None,
                "Great scale vector; ascending backward diagonal reflection",
                "Great scale elevation / Architectural reflection (`\\-^`)",
            ),
            TokenType::Small => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Adjective),
                None,
                "Small scale vector; descending backward diagonal micro-adjustment",
                "Small scale reduction / Micro-destructuring (`/-v`)",
            ),
            TokenType::UpperRight => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Quadrant I compass vector; forward ascending northeast movement",
                "2D Quadrant I vector: +X, +Y (`/^`)",
            ),
            TokenType::LowerRight => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Quadrant IV compass vector; forward descending southeast movement",
                "2D Quadrant IV vector: +X, -Y (`\\v`)",
            ),
            TokenType::UpperLeft => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Quadrant II compass vector; backward ascending northwest movement",
                "2D Quadrant II vector: -X, +Y (`^\\`)",
            ),
            TokenType::LowerLeft => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Quadrant III compass vector; backward descending southwest movement",
                "2D Quadrant III vector: -X, -Y (`v/`)",
            ),

            // Relations of Being (Verbs of Identity & State)
            TokenType::Is => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Copula of being; asserts exact equivalence and identity",
                "Equality assertion (`==`) / Identity binding",
            ),
            TokenType::IsNot => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Negative copula; asserts distinctness and exclusion",
                "Inequality assertion (`!=`)",
            ),
            TokenType::ToBeNot => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "State copula; asserts negative state predicate (=!)",
                "State negation predicate (`=!`)",
            ),
            TokenType::ToBeLike => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Adjective),
                None,
                "Analogy copula; asserts structural likeness or homomorphism",
                "Structural compatibility / Duck typing / Approximate equivalence",
            ),
            TokenType::InvariantEquivalence => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Invariant copula; immutable balance, identity, and eternal equilibrium (<=>)",
                "Invariant to be / Axiomatic Equivalence (`<=>`)",
            ),
            TokenType::LessThan => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Relational ordering; strictly less than within context bounds (<)",
                "Contextual less than comparison (`[ { A } < { B } ]`)",
            ),
            TokenType::GreaterThan => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Relational ordering; strictly greater than within context bounds (>)",
                "Contextual greater than comparison (`[ { A } > { B } ]`)",
            ),
            TokenType::LessEqual => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Relational ordering; less than or equal within context bounds (<=)",
                "Contextual less than or equal comparison (`[ { A } <= { B } ]`)",
            ),
            TokenType::GreaterEqual => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Relational ordering; greater than or equal within context bounds (>=)",
                "Contextual greater than or equal comparison (`[ { A } >= { B } ]`)",
            ),

            // Connectives
            TokenType::But => GrammaticalClassification::new(
                PartOfSpeech::Conjunction,
                Some(PartOfSpeech::Interjection),
                None,
                "Adversative conjunction; introduces strict caveat condition",
                "Caveat guard; triggers `-1: ROLLBACK` if caveat fails",
            ),
            TokenType::And => GrammaticalClassification::new(
                PartOfSpeech::Conjunction,
                Some(PartOfSpeech::Preposition),
                None,
                "Cumulative conjunction; asserts simultaneous co-presence",
                "Conjunction (`&&`) / Simultaneous consensus",
            ),
            TokenType::HardwareLocator => GrammaticalClassification::new(
                PartOfSpeech::Preposition,
                Some(PartOfSpeech::Conjunction),
                None,
                "Dedicated hardware locator (::@); binds entity co-presence directly to MMIO address or port",
                "MMIO port locator / Hardware address binding (`Glyph & Cell ::@ Port`)",
            ),
            TokenType::InRelationTo => GrammaticalClassification::new(
                PartOfSpeech::Preposition,
                Some(PartOfSpeech::Verb),
                None,
                "Relational projection; positions entity in relation to a category",
                "Type annotation (`x : Type`) / Contextual role assignment",
            ),
            TokenType::Or => GrammaticalClassification::new(
                PartOfSpeech::Conjunction,
                Some(PartOfSpeech::Adverb),
                None,
                "Disjunctive conjunction; introduces alternatives or choices",
                "Disjunction (`||`) / Non-deterministic branch",
            ),
            TokenType::Not => GrammaticalClassification::new(
                PartOfSpeech::Adverb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Negating adverb; inverts the state or predicate across balance line",
                "Logical / Algebraic negation",
            ),
            TokenType::BoundPair => GrammaticalClassification::new(
                PartOfSpeech::Conjunction,
                Some(PartOfSpeech::Noun),
                None,
                "Hand-in-hand inseparable dual binding; binds entities so neither can be referenced without the other",
                "Inseparable atomic pair / Dual-rail entangled binding (`A&B`)",
            ),

            // Prepositions & Spatial Locators
            TokenType::At => GrammaticalClassification::new(
                PartOfSpeech::Preposition,
                Some(PartOfSpeech::Noun),
                None,
                "Spatial preposition specifying absolute or relative location, address, or memory site",
                "Memory address / MMIO locator / Absolute site (`Storehouse @ Offset`)",
            ),

            // Measures & Proportions
            TokenType::Percent => GrammaticalClassification::new(
                PartOfSpeech::Adjective,
                Some(PartOfSpeech::Noun),
                None,
                "Proportional measure specifying capacity fraction, ratio, or utilization percentage",
                "Capacity fullness ratio / Percentage scale (`Capacity % 80`)",
            ),

            // Ranges
            TokenType::DotDot => GrammaticalClassification::new(
                PartOfSpeech::Preposition,
                Some(PartOfSpeech::Noun),
                None,
                "Range span preposition; defines closed interval bounds between coordinates",
                "Coordinate interval / Memory span `[0 .. 256]`",
            ),

            // Cardinal Classification Clauses (The 5 W's and 1 H)
            TokenType::Who => GrammaticalClassification::new(
                PartOfSpeech::Pronoun,
                Some(PartOfSpeech::Noun),
                None,
                "Agent pronoun clause; identifies actor, authority, caller, or process identity",
                "Process owner / Security principal / Caller identity (`WHO`)",
            ),
            TokenType::What => GrammaticalClassification::new(
                PartOfSpeech::Noun,
                Some(PartOfSpeech::Pronoun),
                None,
                "Substance noun clause; identifies entity, payload, data vessel, or type",
                "Data vessel / Entity payload / Operand specification (`WHAT`)",
            ),
            TokenType::When => GrammaticalClassification::new(
                PartOfSpeech::Adverb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Temporal adverb clause; identifies execution cadence, lifecycle block, or trigger",
                "Execution phase / Scheduler quantum / Event trigger (`WHEN`)",
            ),
            TokenType::Where => GrammaticalClassification::new(
                PartOfSpeech::Preposition,
                Some(PartOfSpeech::Conjunction),
                None,
                "Spatial preposition clause; identifies memory location, address, scope, or invariant bounds",
                "Spatial address / MMIO site / Invariant scope (`WHERE`)",
            ),
            TokenType::Why => GrammaticalClassification::new(
                PartOfSpeech::Interjection,
                Some(PartOfSpeech::Adjective),
                None,
                "Teleological inquiry clause; specifies rationale, contractual reason, or axiom",
                "Teleological rationale / Contractual reason / Axiom link (`WHY`)",
            ),
            TokenType::How => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Adverb),
                None,
                "Causal mechanism clause; specifies vector method, transducer, or execution policy",
                "Causal mechanism / Vector method / Transducer (`HOW`)",
            ),

            // Natural Arithmetic Keywords
            TokenType::Plus => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Action of addition; unites quantities",
                "Arithmetic addition opcode",
            ),
            TokenType::Minus => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Action of subtraction; separates quantities",
                "Arithmetic subtraction opcode",
            ),
            TokenType::Times => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Action of multiplication; scales quantities",
                "Arithmetic multiplication opcode",
            ),
            TokenType::Div => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Action of division; partitions quantities",
                "Arithmetic division opcode",
            ),

            // Infer
            TokenType::Infer => GrammaticalClassification::new(
                PartOfSpeech::Interjection,
                Some(PartOfSpeech::Verb),
                None,
                "Interrogative inquiry; seeks resolution or truth state",
                "Pattern match query / Ontological inference operator",
            ),

            // Literals
            TokenType::Identifier(_) => GrammaticalClassification::new(
                PartOfSpeech::Noun,
                Some(PartOfSpeech::Pronoun),
                None,
                "Named subject, object, or entity in discourse",
                "Variable name / Symbol / Type identifier",
            ),
            TokenType::Number(_) => GrammaticalClassification::new(
                PartOfSpeech::Noun,
                Some(PartOfSpeech::Adjective),
                None,
                "Numeric scalar value; quantifies entities",
                "Immediate scalar numeric literal",
            ),
            TokenType::StringLit(_) => GrammaticalClassification::new(
                PartOfSpeech::Noun,
                None,
                None,
                "Textual noun; semantic content literal",
                "String literal / Byte payload",
            ),
            TokenType::CharLit(_) => GrammaticalClassification::new(
                PartOfSpeech::Noun,
                None,
                None,
                "Atomic character glyph rune literal ('A'); discrete single-cell textual scalar",
                "Character literal / 5-trit scalar byte (`'A' <=> 65`)",
            ),
            TokenType::Ternary(state) => state.classify(),

            TokenType::Eof => GrammaticalClassification::new(
                PartOfSpeech::Interjection,
                None,
                None,
                "Terminal punctuation sealing the discourse",
                "End of file / Stream termination",
            ),
        }
    }
}

impl GrammaticalEntity for QuadState {
    fn classify(&self) -> GrammaticalClassification {
        match self {
            QuadState::Proceed => GrammaticalClassification::new(
                PartOfSpeech::Adjective,
                Some(PartOfSpeech::Noun),
                Some(PartOfSpeech::Interjection),
                "Affirmative state of truth, covenant alignment, and forward momentum",
                "Logic +1 / Monotonic commit / Success",
            ),
            QuadState::Await => GrammaticalClassification::new(
                PartOfSpeech::Adjective,
                Some(PartOfSpeech::Verb),
                None,
                "Superposition state; yielding in wait for consensus",
                "Logic 0 / Cooperative schedule yield / Non-blocking pending",
            ),
            QuadState::Rollback => GrammaticalClassification::new(
                PartOfSpeech::Interjection,
                Some(PartOfSpeech::Verb),
                Some(PartOfSpeech::Adjective),
                "Exclamation of veto; immediate transactional rewind",
                "Logic -1 / Transaction abort and rollback",
            ),
            QuadState::Halt => GrammaticalClassification::new(
                PartOfSpeech::Interjection,
                Some(PartOfSpeech::Noun),
                None,
                "Terminal trap of void and non-existence; complete cessation",
                "Logic NULL / Fatal CPU trap / Irreducible collapse",
            ),
        }
    }
}

impl GrammaticalEntity for BoundaryType {
    fn classify(&self) -> GrammaticalClassification {
        match self {
            BoundaryType::Denotation => GrammaticalClassification::new(
                PartOfSpeech::Noun,
                Some(PartOfSpeech::Preposition),
                None,
                "Concrete essence and mutable data literal",
                "Data structure container / Mutable memory slice",
            ),
            BoundaryType::Invariant => GrammaticalClassification::new(
                PartOfSpeech::Adjective,
                Some(PartOfSpeech::Noun),
                None,
                "Immutable axiom and universal physical constant",
                "Constant literal / Read-only memory / Axiomatic constraint",
            ),
            BoundaryType::Connotation => GrammaticalClassification::new(
                PartOfSpeech::Adjective,
                Some(PartOfSpeech::Conjunction),
                None,
                "Ambient context, required capabilities, and environmental predicates",
                "Precondition contract / Ambient capabilities / Context scope",
            ),
            BoundaryType::TopDown => GrammaticalClassification::new(
                PartOfSpeech::Preposition,
                Some(PartOfSpeech::Adjective),
                None,
                "Outside-in conditioning: the environment dictates entity essence",
                "Hardware MMIO window / Environmental dependency injection",
            ),
            BoundaryType::BottomUp => GrammaticalClassification::new(
                PartOfSpeech::Preposition,
                Some(PartOfSpeech::Verb),
                None,
                "Inside-out emergence: entity essence projects into environment",
                "Polymorphic emission / Register export to ABI",
            ),
            BoundaryType::Aside => GrammaticalClassification::new(
                PartOfSpeech::Interjection,
                Some(PartOfSpeech::Adverb),
                None,
                "Aside to; teleological intent container preserving reasoning trace",
                "First-class intent container / AI reasoning trace",
            ),
        }
    }
}

impl GrammaticalEntity for DirectionalVector {
    fn classify(&self) -> GrammaticalClassification {
        match self {
            DirectionalVector::ActUpon => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Forward causation; acts upon target",
                "Forward pipeline dispatch / Store operation",
            ),
            DirectionalVector::ActAgain => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Retroactive reflection; draws from or re-evaluates source",
                "Feedback loop / Rollback inspection / Load operation",
            ),
            DirectionalVector::BuildsTo => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Ascending synthesis; primitives build up to abstraction",
                "Type promotion / Abstraction synthesis",
            ),
            DirectionalVector::BreaksDownFrom => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Descending decomposition; concept lowers into hardware primitives",
                "Destructuring / Register lowering",
            ),
            DirectionalVector::Major => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Adjective),
                None,
                "Major scale vector; ascending forward diagonal leap",
                "Major scale promotion (`/-^`)",
            ),
            DirectionalVector::Minor => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Adjective),
                None,
                "Minor scale vector; descending forward diagonal step",
                "Minor scale reduction (`\\-v`)",
            ),
            DirectionalVector::Great => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Adjective),
                None,
                "Great scale vector; ascending backward diagonal reflection",
                "Great scale elevation (`\\-^`)",
            ),
            DirectionalVector::Small => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Adjective),
                None,
                "Small scale vector; descending backward diagonal micro-adjustment",
                "Small scale reduction (`/-v`)",
            ),
            DirectionalVector::UpperRight => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Quadrant I compass vector; forward ascending movement",
                "2D Quadrant I vector: +X, +Y (`/^`)",
            ),
            DirectionalVector::LowerRight => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Quadrant IV compass vector; forward descending movement",
                "2D Quadrant IV vector: +X, -Y (`\\v`)",
            ),
            DirectionalVector::UpperLeft => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Quadrant II compass vector; backward ascending movement",
                "2D Quadrant II vector: -X, +Y (`^\\`)",
            ),
            DirectionalVector::LowerLeft => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Preposition),
                None,
                "Quadrant III compass vector; backward descending movement",
                "2D Quadrant III vector: -X, -Y (`v/`)",
            ),
        }
    }
}

impl GrammaticalEntity for BeingRelation {
    fn classify(&self) -> GrammaticalClassification {
        match self {
            BeingRelation::Is => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Identity copula: exact equivalence of essence",
                "Equality assertion / Equational binding",
            ),
            BeingRelation::IsNot => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Negative copula: distinctness and mutual exclusion",
                "Inequality assertion",
            ),
            BeingRelation::ToBeNot => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "State copula: asserts negative state predicate (=!)",
                "State negation predicate",
            ),
            BeingRelation::ToBeLike => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Adjective),
                None,
                "Analogy copula: structural or behavioral compatibility",
                "Homomorphism / Approximate equality",
            ),
            BeingRelation::Equivalence => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Invariant copula: immutable balance, identity, and eternal equilibrium (<=>)",
                "Invariant to be / Axiomatic Equivalence",
            ),
            BeingRelation::LessThan => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Relational ordering: strictly less than within context bounds (<)",
                "Less than relational query",
            ),
            BeingRelation::GreaterThan => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Relational ordering: strictly greater than within context bounds (>)",
                "Greater than relational query",
            ),
            BeingRelation::LessEqual => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Relational ordering: less than or equal within context bounds (<=)",
                "Less than or equal relational query",
            ),
            BeingRelation::GreaterEqual => GrammaticalClassification::new(
                PartOfSpeech::Verb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Relational ordering: greater than or equal within context bounds (>=)",
                "Greater than or equal relational query",
            ),
        }
    }
}

impl GrammaticalEntity for Connective {
    fn classify(&self) -> GrammaticalClassification {
        match self {
            Connective::But => GrammaticalClassification::new(
                PartOfSpeech::Conjunction,
                Some(PartOfSpeech::Interjection),
                None,
                "Adversative conjunction: caveat requirement that vetoes on failure",
                "Guard condition with automatic rollback",
            ),
            Connective::And => GrammaticalClassification::new(
                PartOfSpeech::Conjunction,
                Some(PartOfSpeech::Preposition),
                None,
                "Cumulative conjunction: co-present simultaneous execution",
                "Conjunction (`&&`) / Joint consensus",
            ),
            Connective::InRelationTo => GrammaticalClassification::new(
                PartOfSpeech::Preposition,
                Some(PartOfSpeech::Verb),
                None,
                "Relational projection: positions an entity within a role or type",
                "Type annotation (`:`) / Role assignment",
            ),
            Connective::Or => GrammaticalClassification::new(
                PartOfSpeech::Conjunction,
                Some(PartOfSpeech::Adverb),
                None,
                "Disjunctive conjunction: alternative path or branching",
                "Disjunction (`||`)",
            ),
            Connective::Not => GrammaticalClassification::new(
                PartOfSpeech::Adverb,
                Some(PartOfSpeech::Conjunction),
                None,
                "Negating adverb: inverts the evaluation balance",
                "Algebraic negation (`--`)",
            ),
        }
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Linguistic Unit Tests & Invariant Verification )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eight_parts_of_speech_coverage() {
        let act_upon = TokenType::ActUpon.classify();
        assert_eq!(act_upon.primary, PartOfSpeech::Verb);
        assert_eq!(act_upon.secondary, Some(PartOfSpeech::Preposition));

        let but = TokenType::But.classify();
        assert_eq!(but.primary, PartOfSpeech::Conjunction);
        assert_eq!(but.secondary, Some(PartOfSpeech::Interjection));

        let bracket = TokenType::LBracket.classify();
        assert_eq!(bracket.primary, PartOfSpeech::Noun);
        assert_eq!(bracket.secondary, Some(PartOfSpeech::Preposition));

        let brace = TokenType::LBrace.classify();
        assert_eq!(brace.primary, PartOfSpeech::Adjective);
        assert_eq!(brace.secondary, Some(PartOfSpeech::Conjunction));

        let aside = TokenType::Aside("intent".to_string()).classify();
        assert_eq!(aside.primary, PartOfSpeech::Interjection);
        assert_eq!(aside.secondary, Some(PartOfSpeech::Adverb));

        let not = TokenType::Not.classify();
        assert_eq!(not.primary, PartOfSpeech::Adverb);
        assert_eq!(not.secondary, Some(PartOfSpeech::Conjunction));

        let relation = TokenType::InRelationTo.classify();
        assert_eq!(relation.primary, PartOfSpeech::Preposition);
        assert_eq!(relation.secondary, Some(PartOfSpeech::Verb));

        let bound = TokenType::BoundPair.classify();
        assert_eq!(bound.primary, PartOfSpeech::Conjunction);
        assert_eq!(bound.secondary, Some(PartOfSpeech::Noun));

        let at = TokenType::At.classify();
        assert_eq!(at.primary, PartOfSpeech::Preposition);
        assert_eq!(at.secondary, Some(PartOfSpeech::Noun));

        let pct = TokenType::Percent.classify();
        assert_eq!(pct.primary, PartOfSpeech::Adjective);
        assert_eq!(pct.secondary, Some(PartOfSpeech::Noun));
    }

    #[test]
    fn test_quad_state_linguistic_classifications() {
        let proceed = QuadState::Proceed.classify();
        assert_eq!(proceed.primary, PartOfSpeech::Adjective);
        assert!(proceed.is(PartOfSpeech::Interjection));

        let rollback = QuadState::Rollback.classify();
        assert_eq!(rollback.primary, PartOfSpeech::Interjection);
        assert!(rollback.is(PartOfSpeech::Verb));
    }

    #[test]
    fn test_pronoun_matrix_and_capability_classifications() {
        // 1. Subjective (Agency / Execution context)
        let p_i = PronounKind::I;
        assert_eq!(p_i.case(), PronounCase::Subjective);
        assert_eq!(p_i.person(), PronounPerson::First);
        assert_eq!(p_i.number(), PronounNumber::Singular);
        assert!(p_i.is_subjective());
        let class_i = p_i.classify();
        assert_eq!(class_i.primary, PartOfSpeech::Pronoun);
        assert_eq!(class_i.secondary, Some(PartOfSpeech::Noun));
        assert!(class_i.computing_counterpart.contains("&mut self"));

        // 2. Objective (Inbound target)
        let p_me = PronounKind::Me;
        assert_eq!(p_me.case(), PronounCase::Objective);
        assert_eq!(p_me.person(), PronounPerson::First);
        assert!(p_me.is_objective());

        // 3. Reflexive (Introspection & self-audit)
        let p_myself = PronounKind::Myself;
        assert_eq!(p_myself.case(), PronounCase::Reflexive);
        assert!(p_myself.is_reflexive());
        let class_myself = p_myself.classify();
        assert_eq!(class_myself.secondary, Some(PartOfSpeech::Verb));
        assert!(class_myself.computing_counterpart.contains("Self-Audit"));

        let p_itself = PronounKind::Itself;
        assert_eq!(p_itself.case(), PronounCase::Reflexive);
        assert_eq!(p_itself.person(), PronounPerson::Third);
        let class_itself = p_itself.classify();
        assert!(class_itself.computing_counterpart.contains("Homoiconic"));

        let p_themselves = PronounKind::Themselves;
        assert_eq!(p_themselves.case(), PronounCase::Reflexive);
        assert_eq!(p_themselves.number(), PronounNumber::Plural);
        assert!(p_themselves.classify().computing_counterpart.contains("Consensus"));

        // 4. Possessive (Ownership & Capability tokens)
        let p_my = PronounKind::My;
        assert_eq!(p_my.case(), PronounCase::Possessive);
        assert!(p_my.is_possessive());
        let class_my = p_my.classify();
        assert_eq!(class_my.secondary, Some(PartOfSpeech::Adjective));
        assert!(class_my.computing_counterpart.contains("Capability Token"));

        let p_their = PronounKind::Their;
        assert_eq!(p_their.case(), PronounCase::Possessive);
        assert_eq!(p_their.number(), PronounNumber::Plural);

        // 5. Keyword parsing round-trip
        assert_eq!(PronounKind::from_keyword("we"), Some(PronounKind::We));
        assert_eq!(PronounKind::from_keyword("you"), Some(PronounKind::You));
        assert_eq!(PronounKind::from_keyword("it"), Some(PronounKind::It));
        assert_eq!(PronounKind::from_keyword("she"), Some(PronounKind::She));
        assert_eq!(PronounKind::from_keyword("he"), Some(PronounKind::He));
        assert_eq!(PronounKind::from_keyword("herself"), Some(PronounKind::Herself));
        assert_eq!(PronounKind::from_keyword("himself"), Some(PronounKind::Himself));
        assert_eq!(PronounKind::from_keyword("theirs"), Some(PronounKind::Theirs));
        assert_eq!(PronounKind::from_keyword("self"), Some(PronounKind::SelfHandle));
        assert_eq!(PronounKind::from_keyword("caller"), Some(PronounKind::CallerHandle));
        assert_eq!(PronounKind::from_keyword("owner"), Some(PronounKind::OwnerHandle));
        assert_eq!(PronounKind::from_keyword("_"), Some(PronounKind::Wildcard));

        // 6. TokenType delegation
        let tok = TokenType::Pronoun(PronounKind::Myself);
        assert_eq!(tok.classify().primary, PartOfSpeech::Pronoun);
    }
}
