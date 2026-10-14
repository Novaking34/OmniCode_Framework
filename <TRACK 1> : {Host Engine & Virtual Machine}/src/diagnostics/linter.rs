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
use crate::core::filetype::OmniFileType;
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
    pub file_type: Option<OmniFileType>,
    pub blocks_found: Vec<DetectedBlock>,
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
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
    /// Lints a source string to verify that all 4 blocks are present, in strict order,
    /// and comply with OmniCode semantic architecture and invariant canons.
    pub fn lint_source(&self, file_path: &str, content: &str) -> LintReport {
        let mut blocks = Vec::new();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let file_type = OmniFileType::from_path(file_path);

        for (idx, line) in content.lines().enumerate() {
            let line_num = idx + 1;
            // Ignore indented comments (e.g. nested sub-reels inside body blocks or scrolls)
            if line.starts_with(' ') || line.starts_with('\t') {
                continue;
            }
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

        // Check 3: Semantic Content Analysis Across Detected Block Regions
        if has_metadata && has_opening && has_body && has_closing && errors.is_empty() {
            let meta_start = blocks.iter().find(|b| b.kind == BlockKind::Metadata).unwrap().line_number;
            let open_start = blocks.iter().find(|b| b.kind == BlockKind::Opening).unwrap().line_number;
            let body_start = blocks.iter().find(|b| b.kind == BlockKind::Body).unwrap().line_number;
            let close_start = blocks.iter().find(|b| b.kind == BlockKind::Closing).unwrap().line_number;

            let lines: Vec<&str> = content.lines().collect();

            let get_slice = |start: usize, end: usize| -> String {
                if start <= end && end <= lines.len() {
                    lines[(start - 1)..(end - 1)].join("\n")
                } else if start <= lines.len() {
                    lines[(start - 1)..].join("\n")
                } else {
                    String::new()
                }
            };

            let meta_slice = get_slice(meta_start, open_start);
            let open_slice = get_slice(open_start, body_start);
            let body_slice = get_slice(body_start, close_start);
            let close_slice = get_slice(close_start, lines.len() + 1);

            // Block 1 Validation
            let has_module_decl = meta_slice.contains("= [Module]:")
                || meta_slice.contains("Module <")
                || meta_slice.contains("Module [")
                || meta_slice.contains("Scroll <");
            if !has_module_decl {
                warnings.push("BLOCK 1: Missing formal module identity declaration (e.g. '= [Module]: < ... > & [ ... ]').".into());
            }
            if !meta_slice.contains("Version") && !meta_slice.contains("version") {
                warnings.push("BLOCK 1: Missing version specification in metadata context.".into());
            }
            if file_path.contains("lexicon") || file_path.contains("manifest") {
                if !meta_slice.contains("Exodus") && !meta_slice.contains("Scripture") && !meta_slice.contains("Witness") {
                    warnings.push("BLOCK 1: Lexicon/Covenant specification should ground its invariant witness (e.g. '< Scripture { ... } >').".into());
                }
            }

            // Block 2 Validation
            if !open_slice.contains("Opening") {
                errors.push("BLOCK 2: Missing 'Opening' block scope declaration.".into());
            }
            if !open_slice.contains("preconditions") && !open_slice.contains("Preconditions") && !open_slice.contains("+1") {
                warnings.push("BLOCK 2: Opening block should explicitly verify preconditions / armed state.".into());
            }

            // Block 3 Validation
            if !body_slice.contains("Body") {
                errors.push("BLOCK 3: Missing 'Body' block scope declaration.".into());
            }

            // Block 4 Validation: The 5 Pillars & State Machine Completeness
            if !close_slice.contains("Closing") {
                errors.push("BLOCK 4: Missing 'Closing' block scope declaration.".into());
            }

            // Commit Status State Machine Completeness Check
            let commit_upper = close_slice.to_uppercase();
            if commit_upper.contains("COMMIT STATUS") || commit_upper.contains("COMMIT_STATUS") {
                // If the commit status specifies a branching state machine (|), all four states are required
                if close_slice.contains('|') {
                    let has_proceed = commit_upper.contains("PROCEED");
                    let has_await = commit_upper.contains("AWAIT");
                    let has_halt = commit_upper.contains("HALT");
                    let has_rollback = commit_upper.contains("ROLLBACK");

                    let mut missing_states = Vec::new();
                    if !has_proceed { missing_states.push("PROCEED (+1)"); }
                    if !has_await { missing_states.push("AWAIT (0)"); }
                    if !has_halt { missing_states.push("HALT (NULL)"); }
                    if !has_rollback { missing_states.push("ROLLBACK (-1)"); }

                    if !missing_states.is_empty() {
                        errors.push(format!(
                            "BLOCK 4: Incomplete Commit Status state machine. When using conditional branching, all four end states must be defined. Missing: {}.",
                            missing_states.join(", ")
                        ));
                    }
                }
            }
        }

        // Check 4: Universal Canons (First-class Asides and Delimiter Hygiene)
        let total_lines = content.lines().count();
        let has_any_asides = content.contains('(') && content.contains(')');
        if total_lines > 15 && !has_any_asides {
            warnings.push("Canon 1: Missing first-class intent Asides '(...)'. OmniCode mandates Asides for human-AI pronunciation and semantic verification.".into());
        }

        // Check 5: Snake_case hygiene inside brackets
        for (idx, line) in content.lines().enumerate() {
            let line_num = idx + 1;
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                continue;
            }
            if let Some(start_bracket) = line.find('[') {
                if let Some(end_bracket) = line[start_bracket..].find(']') {
                    let bracket_content = &line[start_bracket + 1..start_bracket + end_bracket];
                    // Skip directives, tape heads or raw registers (e.g. 0x000B8000, +1, etc.)
                    for word in bracket_content.split_whitespace() {
                        if word.contains('_') && !word.starts_with("0x") && !word.starts_with("roll_") {
                            warnings.push(format!(
                                "Syntax Hygiene (line {line_num}): Identifier '{word}' uses snake_case with underscores inside brackets. In OmniCode, use natural spaced phrases (e.g. '[ ... ]') without underscores.",
                            ));
                            break;
                        }
                    }
                }
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
            file_type,
            blocks_found: blocks,
            is_valid,
            errors,
            warnings,
        }
    }

    /// Lints a file on disk.
    pub fn lint_file(&self, path: impl AsRef<Path>) -> Result<LintReport, String> {
        let p = path.as_ref();
        let content = std::fs::read_to_string(p)
            .map_err(|e| format!("Failed to read file {}: {e}", p.display()))?;
        Ok(self.lint_source(&p.display().to_string(), &content))
    }

    /// Auto-scaffolds a compliant 4-Block skeleton for an episodic cognitive scroll (.scroll).
    pub fn auto_scaffold_scroll(session_id: &str, prompt: &str) -> String {
        OmniFileType::EpisodicScroll.scaffold_template(session_id, prompt)
    }

    /// Auto-scaffolds a compliant 4-Block skeleton for a given module name (.omni).
    pub fn auto_scaffold_omni(module_name: &str, intent_aside: &str) -> String {
        format!(
r#"# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ==============================================================================
= [Module]: < Lexicon {{ {module_name} }} > & [ Noun {{ {module_name} }} ] ( {intent_aside} ) {{
    [ Version ]:   {{ "1.0.0" }} ( Version 1.0.0 ),
    [ Author ]:    {{ "CreativeWorkzStudio LLC / ProfessorSeanEX" }} ( System Architect ),
    [ System ]:    {{ OmniCode {{ Core }} }} ( Core Subsystem ),
    [ Witness ]:   < Scripture {{ Exodus 31:18 KJV }} > {{ "Written with the finger of God." }} ( Invariant Scripture Anchor ),
    [ Nature ]:    < InvariantType > & {{ BaseBlock }} ( Nature is Invariant Type ),
    [ Semantics ]: {{ StandardLibrary :: Specification }} ( Semantics )
}}

< {module_name} > = < Idea {{ Type }} > & [ Noun {{ {module_name} }} ] ( Invariant Definition ) {{
    [ Relation ]: [ {module_name} ] : [ Unit ] :: [ Trit ] : [ Word ] ( Ontological Relation ),
    [ Substrate ]: [ 1 Byte {{ 8 Bits }} ] & [ 1 Cell {{ 5 Trits }} ] ( Dual Substrate ),
    [ Bound ]: ENCLOSE [ {module_name} ] WITHIN [ 1 Cell ] ( Invariant Single Cell Boundary )
}}

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ==============================================================================
Opening {{
    [ Entity ]: < {module_name} > & [ Cell ] ( Entity allocated in cell ),
    [ Cell ]: [ 1 ] & {{ 5 Trits :: 1 Byte }} ( One cell container ),
    [ Preconditions ]: [ (+1) plus (-1) <=> 0 ] & [ Status = +1:PROCEED ] ( Equilibrium verified )
}}

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ==============================================================================
Body [
    [ ROLL 1 ] : [ Inscribe ] : [ Input ] --> [ Cell ] = [ Entity` ] 
        ( Inscribe input into Cell yielding new state Entity-prime ),

    [ ROLL 2 ] : [ Compare ] : {{ Entity` = Input }} --> [ Status` = +1:PROCEED ] | {{ Entity` != Input }} --> [ Status` = -1:ROLLBACK ] 
        ( Evaluate identity and verify bounds )
]

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ==============================================================================
Closing ( Sealing Gate: Reconcile, tripwire, debug, log, and affirm permanent execution ) {{
    [ Verify ]: [
        [ ROLL 1 ] = PROCEED ::
        [ ROLL 2 ] = PROCEED
    ] ( Verify that all rolls achieved affirmative consensus in PROCEED ),

    [ Validate ]: {{ Entity` <= [ 1 Cell ] }} & {{ [ 8 Bits ] <=> [ 5 Trits ] }} 
        ( Validate single cell bounds and substrate parity ),

    [ Tripwire ]: {{ Status` = +1:PROCEED }} --> [ ARMED ] | {{ Status` != +1:PROCEED }} --> [ TRIPPED ] --> [ Commit Status = -1:ROLLBACK ] 
        ( Circuit breaker vetoes on anomaly ),

    [ Debug ]: {{ [ Entity` ] :: [ Cell` ] :: [ Status` ] }} 
        ( Expose diagnostic telemetry snapshot ),

    [ Log ]: [ Journal ]: INSCRIBE {{ [ {module_name} Verified ] :: [ Entity` ] }} 
        ( Inscribe event record into system journal ),

    [ Commit Status ]: [
        {{ [ Verify ] = PROCEED }} & {{ [ Tripwire ] = ARMED }} --> [ +1:PROCEED ] |
        {{ [ Status` ] = PENDING }} --> [ 0:AWAIT ] :: REPEAT |
        {{ [ Status` ] = NULL }} --> [ HALT:NULL ] |
        {{ [ Tripwire ] = TRIPPED }} | {{ [ Verify ] != PROCEED }} --> [ -1:ROLLBACK ]
    ] ( Fourfold state machine: Proceed, Await, Halt, Rollback )
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
