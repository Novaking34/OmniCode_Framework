// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniLinter Identity, BlockKind Enumeration & Structural Invariants )
// ==============================================================================
//! OmniCode Diagnostics: Structural 4-Block Linter & Auto-Organizer
//!
//! Enforces the Universal 4-Block Architecture across all files:
//!     - BLOCK 1: METADATA BLOCK  (Identity, Version, Invariant Axioms, Asides)
//!     - BLOCK 2: OPENING BLOCK   (Context, Preconditions, State Initialization)
//!     - BLOCK 3: BODY BLOCK      (Core Transformations, Causality Vectors, Logic)
//!     - BLOCK 4: CLOSING BLOCK   (Verification, Postconditions, Teardown, Commit)

use std::path::Path;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlockKind {
    Metadata = 1,
    Opening  = 2,
    Body     = 3,
    Closing  = 4,
}

impl BlockKind {
    pub fn name(&self) -> &'static str {
        match self {
            BlockKind::Metadata => "BLOCK 1: METADATA",
            BlockKind::Opening => "BLOCK 2: OPENING",
            BlockKind::Body => "BLOCK 3: BODY",
            BlockKind::Closing => "BLOCK 4: CLOSING",
        }
    }
}


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Report Structures and OmniLinter Initialization )
// ==============================================================================
#[derive(Debug, Clone)]
pub struct DetectedBlock {
    pub kind: BlockKind,
    pub line_number: usize,
    pub header_raw: String,
}

#[derive(Debug, Clone)]
pub struct LintReport {
    pub file_path: String,
    pub blocks_found: Vec<DetectedBlock>,
    pub is_valid: bool,
    pub errors: Vec<String>,
}

pub struct OmniLinter {
    pub logger: DiagnosticLogger,
}

impl OmniLinter {
    pub fn new(logger: DiagnosticLogger) -> Self {
        Self { logger }
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Linting Logic: 4-Block Detection, Ordering Checks, Auto-Scaffolding )
// ==============================================================================
impl OmniLinter {
    /// Lints a source string to verify that all 4 blocks are present and in strict order.
    pub fn lint_source(&self, file_path: &str, content: &str) -> LintReport {
        let mut blocks = Vec::new();
        let mut errors = Vec::new();

        for (idx, line) in content.lines().enumerate() {
            let line_num = idx + 1;
            let upper = line.to_uppercase();

            // Detect Block 1: Metadata
            if upper.contains("BLOCK 1") || upper.contains("METADATA BLOCK") {
                blocks.push(DetectedBlock {
                    kind: BlockKind::Metadata,
                    line_number: line_num,
                    header_raw: line.trim().to_string(),
                });
            }
            // Detect Block 2: Opening
            else if upper.contains("BLOCK 2") || upper.contains("OPENING BLOCK") {
                blocks.push(DetectedBlock {
                    kind: BlockKind::Opening,
                    line_number: line_num,
                    header_raw: line.trim().to_string(),
                });
            }
            // Detect Block 3: Body
            else if upper.contains("BLOCK 3") || upper.contains("BODY BLOCK") {
                blocks.push(DetectedBlock {
                    kind: BlockKind::Body,
                    line_number: line_num,
                    header_raw: line.trim().to_string(),
                });
            }
            // Detect Block 4: Closing
            else if upper.contains("BLOCK 4") || upper.contains("CLOSING BLOCK") {
                blocks.push(DetectedBlock {
                    kind: BlockKind::Closing,
                    line_number: line_num,
                    header_raw: line.trim().to_string(),
                });
            }
        }

        // Check 1: Must contain all 4 blocks
        let has_metadata = blocks.iter().any(|b| b.kind == BlockKind::Metadata);
        let has_opening = blocks.iter().any(|b| b.kind == BlockKind::Opening);
        let has_body = blocks.iter().any(|b| b.kind == BlockKind::Body);
        let has_closing = blocks.iter().any(|b| b.kind == BlockKind::Closing);

        if !has_metadata {
            errors.push("Missing BLOCK 1: METADATA BLOCK".into());
        }
        if !has_opening {
            errors.push("Missing BLOCK 2: OPENING BLOCK".into());
        }
        if !has_body {
            errors.push("Missing BLOCK 3: BODY BLOCK".into());
        }
        if !has_closing {
            errors.push("Missing BLOCK 4: CLOSING BLOCK".into());
        }

        // Check 2: Verify sequential ordering
        for i in 1..blocks.len() {
            if blocks[i].kind <= blocks[i - 1].kind {
                let err = format!(
                    "Block order violation at line {}: {} appeared after {}",
                    blocks[i].line_number,
                    blocks[i].kind.name(),
                    blocks[i - 1].kind.name()
                );
                errors.push(err);
            }
        }

        let is_valid = errors.is_empty();

        if !is_valid {
            for err in &errors {
                self.logger.failpoint(
                    FailpointType::SyntaxAnomaly,
                    "LINTER",
                    format!("[{file_path}] {err}"),
                );
            }
        } else {
            self.logger.log(
                "LINTER",
                format!("File [{file_path}] satisfies universal 4-Block Architecture."),
            );
        }

        LintReport {
            file_path: file_path.to_string(),
            blocks_found: blocks,
            is_valid,
            errors,
        }
    }

    /// Lints a file on disk.
    pub fn lint_file(&self, path: impl AsRef<Path>) -> Result<LintReport, String> {
        let p = path.as_ref();
        let content = std::fs::read_to_string(p)
            .map_err(|e| format!("Failed to read file {}: {e}", p.display()))?;
        Ok(self.lint_source(&p.display().to_string(), &content))
    }

    /// Auto-scaffolds a compliant 4-Block skeleton for a given module name.
    pub fn auto_scaffold_omni(module_name: &str, intent_aside: &str) -> String {
        format!(
r#"# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ( File Identity, Version, Author, Intent Aside, and Invariants )
# ==============================================================================
Module < {module_name} > ( {intent_aside} ) {{
    version: < "0.1.0" >,
    author: < "ProfessorSeanEX" >,
    system: < OmniCode.Core >,
    safety: < MemorySafe :: Deterministic >
}}

# Invariant Axioms & Types
Type_{module_name} < +1 > ( Invariant definition )


# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ( Environment Setup, Preconditions, Scoped Context, and Declarations )
# ==============================================================================
Opening {{
    requires: [ MemoryAligned.64 ],
    preconditions: [ +1 ]
}}

# Initial state instances
state [ +1 ] ( Initialized state instance )


# ==============================================================================
# BLOCK 3: BODY BLOCK
# ( Core Transformations, Causality Vectors, Logic, and Caveat Pipelines )
# ==============================================================================
Body [
    # Forward action: state acts upon transformation
    state --> next_state [ +1 ]
    next_state ! [ state = +1 ] ( Proceed only if precondition satisfied )
]


# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ( Postconditions, Invariant Proofs, Teardown, and Commit Status )
# ==============================================================================
Closing {{
    verify: [ next_state = +1 ],
    commit_status: [ +1 ] ( Commit verified state )
}}
"#
        )
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Linter Invariant Verification & Teardown )
// ==============================================================================
// OmniLinter structural validation invariants closed and verified.
