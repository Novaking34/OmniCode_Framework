// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode File Type Subsystem Identity, Taxonomy & Invariants )
// ==============================================================================
//! OmniCode File Type Taxonomy & Formal Extension System
//!
//! Classifies and handles all recognized OmniCode file artifacts:
//!   - `.omni`: Canonical OmniCode Source Reel (Primary executable program logic,
//!     modules, libraries, and ontological domain models).
//!   - `.scroll`: Cognitive Episodic Scroll (Homoiconic execution trace reel,
//!     recording prompts, synthesized programs, Restorator repairs, failpoints,
//!     and natural responses for AI continual learning).

use std::fmt;
use std::path::Path;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( File Type Enumeration, Mime Types, and Constructor Declarations )
// ==============================================================================
/// Formal file classifications in the OmniCode ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OmniFileType {
    /// `.omni` - Canonical executable source reel.
    SourceReel,
    /// `.scroll` - Homoiconic episodic cognitive trace and AI learning reel.
    EpisodicScroll,
}

impl OmniFileType {
    pub const EXT_OMNI: &'static str = "omni";
    pub const EXT_SCROLL: &'static str = "scroll";

    pub const ALL: &'static [OmniFileType] = &[OmniFileType::SourceReel, OmniFileType::EpisodicScroll];

    /// Returns all supported OmniCode file types.
    pub fn all() -> &'static [OmniFileType] {
        Self::ALL
    }

    /// Detects file type from extension string (with or without leading dot).
    pub fn from_extension(ext: &str) -> Option<Self> {
        let clean = ext.trim().trim_start_matches('.').to_lowercase();
        match clean.as_str() {
            "omni" => Some(OmniFileType::SourceReel),
            "scroll" => Some(OmniFileType::EpisodicScroll),
            _ => None,
        }
    }

    /// Detects file type from a file path.
    pub fn from_path(path: impl AsRef<Path>) -> Option<Self> {
        let p = path.as_ref();
        p.extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }

    /// Checks if a file path is a known OmniCode artifact.
    pub fn is_omnicode_file(path: impl AsRef<Path>) -> bool {
        Self::from_path(path).is_some()
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Metadata Queries, Scaffolding Templates & Formatting Properties )
// ==============================================================================
impl OmniFileType {
    /// Canonical file extension without leading dot.
    pub fn extension(&self) -> &'static str {
        match self {
            OmniFileType::SourceReel => Self::EXT_OMNI,
            OmniFileType::EpisodicScroll => Self::EXT_SCROLL,
        }
    }

    /// File extension with leading dot.
    pub fn dotted_extension(&self) -> &'static str {
        match self {
            OmniFileType::SourceReel => ".omni",
            OmniFileType::EpisodicScroll => ".scroll",
        }
    }

    /// Human-readable display title.
    pub fn display_name(&self) -> &'static str {
        match self {
            OmniFileType::SourceReel => "OmniCode Source Reel",
            OmniFileType::EpisodicScroll => "OmniCode Episodic Scroll",
        }
    }

    /// VS Code / IDE language identifier.
    pub fn language_id(&self) -> &'static str {
        "omnicode"
    }

    /// Formal MIME type.
    pub fn mime_type(&self) -> &'static str {
        match self {
            OmniFileType::SourceReel => "text/x-omnicode",
            OmniFileType::EpisodicScroll => "text/x-omnicode-scroll",
        }
    }

    /// Short technical summary.
    pub fn description(&self) -> &'static str {
        match self {
            OmniFileType::SourceReel => {
                "Canonical 4-Block executable program module conforming to balanced ternary physics."
            }
            OmniFileType::EpisodicScroll => {
                "Homoiconic episodic execution reel recording cognitive discourse, repairs, and telemetry."
            }
        }
    }

    /// Generates canonical 4-Block scaffolding template for this file type.
    pub fn scaffold_template(&self, identifier: &str, intent_or_prompt: &str) -> String {
        match self {
            OmniFileType::SourceReel => {
                format!(
r#"# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ( File Identity, Version, Author, Intent Aside, and Invariants )
# ==============================================================================
Module < {name} > ( {intent} ) {{
    version: < "0.1.0" >,
    system: < OmniCode.Core >,
    safety: < MemorySafe :: Deterministic >
}}

# Invariant Axioms & Types
Type_{name} < +1 > ( Invariant definition )

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ( Environment Setup, Preconditions, Scoped Context, and Declarations )
# ==============================================================================
Opening {{
    import: [ Core.Arithmetic :: Core.Logic :: Core.Vectors ],
    preconditions: [ +1 ]
}}

# Initial state instance
state [ +1 ] ( Initialized state instance )

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ( Core Transformations, Causality Vectors, Logic, and Caveat Pipelines )
# ==============================================================================
Body [
    # Forward action: state acts upon transformation
    state --> next_state [ +1 ]
    next_state ! [ state = +1 ] ( Caveat verification )
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ( Postconditions, Invariant Proofs, Teardown, and Commit Status )
# ==============================================================================
Closing {{
    verify: [ next_state = +1 ],
    commit_status [ +1 ] ( Commit verified state )
}}
"#,
                    name = identifier,
                    intent = intent_or_prompt
                )
            }
            OmniFileType::EpisodicScroll => {
                format!(
r#"# ==============================================================================
# BLOCK 1: METADATA BLOCK (THE COGNITIVE SCROLL)
# ( Execution Trace ID, Prompt Intent, Inferred Types, and Neural Snapshot )
# ==============================================================================
Scroll < EpistemicTrace.{session} > ( {prompt} ) {{
    version: < "1.0.0" >,
    system: < OmniCode.AI.Model >,
    timestamp: < "{timestamp}" >,
    session_id: < "{session}" >,
    cognitive_mode: < NeuralSymbolicTransduction >
}}

# ==============================================================================
# BLOCK 2: OPENING BLOCK (EXPERIENCE INITIALIZATION)
# ( Hardware Invariants, Preconditions, and Initial Context )
# ==============================================================================
Opening {{
    import: [ Core.Arithmetic :: Core.Logic :: Core.Vectors ],
    preconditions: [ +1 ]
}}

# Original Cognitive Discourse Prompt:
user_prompt < "{prompt}" >

# ==============================================================================
# BLOCK 3: BODY BLOCK (CHRONICLE OF EXECUTION & RESTORATION)
# ( Replay Tape: Synthesized Program, Sensory Telemetry & Immune Interventions )
# ==============================================================================
Body [
    # Immune System Interventions (Restorator):
    repair_entry [ L1: CleanExecution --> "Zero repairs required." ]

    # Executed Turing Reel:
    synthesized_reel [
        state [ +1 ]
    ]
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK (EPISODIC COMMIT & LEARNING PROOF)
# ( Verification Proof, Final Quad-State, and Conversational Synthesis )
# ==============================================================================
Closing {{
    commit_status [ <PROCEED> ],
    machine_utterance ( "Execution verified to +1:PROCEED." )
}}
"#,
                    session = identifier,
                    prompt = intent_or_prompt,
                    timestamp = "0"
                )
            }
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Display Trait Implementation, Sealing & Unit Tests )
// ==============================================================================
impl fmt::Display for OmniFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(.{})", self.display_name(), self.extension())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_detection() {
        assert_eq!(OmniFileType::from_extension("omni"), Some(OmniFileType::SourceReel));
        assert_eq!(OmniFileType::from_extension(".omni"), Some(OmniFileType::SourceReel));
        assert_eq!(OmniFileType::from_extension("scroll"), Some(OmniFileType::EpisodicScroll));
        assert_eq!(OmniFileType::from_extension(".scroll"), Some(OmniFileType::EpisodicScroll));
        assert_eq!(OmniFileType::from_extension("txt"), None);

        assert_eq!(OmniFileType::from_path("src/core.omni"), Some(OmniFileType::SourceReel));
        assert_eq!(OmniFileType::from_path("scrolls/test.scroll"), Some(OmniFileType::EpisodicScroll));
        assert!(OmniFileType::is_omnicode_file("lib/vector.omni"));
        assert!(OmniFileType::is_omnicode_file("history/session.scroll"));
        assert!(!OmniFileType::is_omnicode_file("Cargo.toml"));
    }

    #[test]
    fn test_scaffold_generation() {
        let omni_code = OmniFileType::SourceReel.scaffold_template("MathModule", "Basic math");
        assert!(omni_code.contains("Module < MathModule >"));
        assert!(omni_code.contains("BLOCK 1: METADATA BLOCK"));
        assert!(omni_code.contains("BLOCK 4: CLOSING BLOCK"));

        let scroll_code = OmniFileType::EpisodicScroll.scaffold_template("sess_1234", "Add 2 and 3");
        assert!(scroll_code.contains("Scroll < EpistemicTrace.sess_1234 >"));
        assert!(scroll_code.contains("commit_status [ <PROCEED> ]"));
    }
}
