// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniLinter Coordinator, Subsystem Exports & Invariant Architecture )
// ==============================================================================
//! OmniCode Diagnostics: Polymorphic Structural, Grammar & Spatial Linter
//!
//! Enforces the structural invariants, lexical syntax, grammar ASTs, and physical
//! spatial geometry across all OmniCode compilation units and execution ribbons:
//! 1. `.omni` (Source Reel): Canonical 4-Block Architecture (Metadata, Opening, Body, Closing).
//! 2. `.tape` (Process Ribbon): Sequential Tape Head, Cardinal Coordinates, Rolls Stream & Commit Status.
//! 3. Spatial Geometry: Strict 80-column ruler for dual side-panel IDE viewports.
//! 4. Grammar & Syntax: SVO verbs, state-prime synchronization, Living Asides, and boundary physics.
//! 5. Assembly Pre-flight: Partition bounds, silicon MMIO targets, and register verification.

use std::path::Path;

use crate::core::filetype::OmniFileType;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};

pub mod audit;
pub mod body;
pub mod closing;
pub mod metadata;
pub mod opening;
pub mod preflight;
pub mod scaffold;
pub mod structure;
pub mod syntax;

pub use audit::{FileAuditScore, QuadStateGate};
pub use body::{BodyLinter, BodyScore, ReelBodyLinter, TapeRollsLinter};
pub use closing::{ClosingLinter, ClosingScore, ReelClosingLinter, TapeCommitLinter};
pub use metadata::{MetadataLinter, MetadataScore, ReelMetadataLinter, TapeHeadLinter};
pub use opening::{OpeningLinter, OpeningScore, ReelOpeningLinter, TapeSetupLinter};
pub use preflight::AssemblyLinter;
pub use scaffold::LinterScaffolder;
pub use structure::{BlockDetector, BlockKind, BlockSlice, DetectedBlock, FileBlockSlices, OmniReelLinter, TapeLinter};
pub use syntax::{GrammarLinter, SpatialLintConfig, SpatialLinter};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( LintReport & OmniLinter Initialization )
// ==============================================================================

#[derive(Debug, Clone)]
pub struct LintReport {
    pub file_path: String,
    pub file_type: Option<OmniFileType>,
    pub blocks_found: Vec<DetectedBlock>,
    pub metadata_score: Option<MetadataScore>,
    pub audit_score: Option<FileAuditScore>,
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub struct OmniLinter {
    pub logger: DiagnosticLogger,
    pub debugger: OmniDebugger,
    pub config: SpatialLintConfig,
}

impl OmniLinter {
    /// Creates a new linter bound to a diagnostic logger with default spatial settings.
    pub fn new(logger: DiagnosticLogger) -> Self {
        let debugger = OmniDebugger::new(logger.clone(), 100);
        Self {
            logger,
            debugger,
            config: SpatialLintConfig::default(),
        }
    }

    /// Creates a new linter with custom debugger and spatial configuration.
    pub fn with_debugger(logger: DiagnosticLogger, debugger: OmniDebugger) -> Self {
        Self {
            logger,
            debugger,
            config: SpatialLintConfig::default(),
        }
    }

    /// Configures strict 80-column ruler enforcement.
    pub fn with_strict_geometry(mut self, strict: bool) -> Self {
        self.config.strict_column_limit = strict;
        self
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Linting Pipelines: Polymorphic Dispatch & Assembly Pre-flight )
// ==============================================================================

impl OmniLinter {
    /// Lints a source string across structural, grammar, and spatial geometry invariants.
    pub fn lint_source(&self, file_path: &str, content: &str) -> LintReport {
        let file_type = OmniFileType::from_path(file_path);

        let mut report = LintReport {
            file_path: file_path.to_string(),
            file_type,
            blocks_found: Vec::new(),
            metadata_score: None,
            audit_score: None,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        // Pass 1: Spatial Geometry & Layout Ruler (<= 80 columns)
        SpatialLinter::lint(content, &mut report, &self.config);

        // Pass 2: Delimiter & Assembly Pre-flight Checks
        AssemblyLinter::preflight(content, &mut report);

        // Pass 3: Lexical Syntax & Grammar AST Verification
        GrammarLinter::lint(
            file_path,
            content,
            self.logger.clone(),
            self.debugger.clone(),
            &mut report,
        );

        // Pass 4: Polymorphic Structural Architecture Dispatch
        let is_tape = match file_type {
            Some(OmniFileType::Tape) => true,
            _ => {
                let trimmed_start = content.trim_start();
                trimmed_start.starts_with("= [Tape")
                    || trimmed_start.starts_with("[Tape.")
                    || (trimmed_start.contains("[ ROLL") && !trimmed_start.contains("BLOCK 1"))
            }
        };

        if is_tape {
            TapeLinter::lint(file_path, content, &mut report);
        } else {
            OmniReelLinter::lint(file_path, content, &mut report);
        }

        // Pass 5: Block 1 Invariant & Metadata Scoring
        let meta_score = MetadataLinter::evaluate(file_path, content, is_tape, &self.config, &mut report);
        report.metadata_score = Some(meta_score.clone());

        // Pass 6: Full 4-Block Invariant Audit & QuadState Gate Synthesis
        let open_score = OpeningLinter::evaluate(file_path, content, is_tape, &self.config);
        let body_score = BodyLinter::evaluate(file_path, content, is_tape, &self.config);
        let close_score = ClosingLinter::evaluate(file_path, content, is_tape, &self.config);

        report.is_valid = report.errors.is_empty();

        let audit = FileAuditScore::calculate(
            meta_score,
            open_score,
            body_score,
            close_score,
            !report.is_valid,
        );
        report.audit_score = Some(audit);

        report.is_valid = report.errors.is_empty();

        if !report.is_valid {
            for err in &report.errors {
                self.logger.failpoint(
                    FailpointType::SyntaxAnomaly,
                    "LINTER",
                    format!("[{file_path}] {err}"),
                );
            }
        } else {
            self.logger.log(
                "LINTER",
                format!("File [{file_path}] satisfies structural, grammar, and spatial invariants."),
            );
        }

        report
    }

    /// Lints a file directly on disk.
    pub fn lint_file(&self, path: impl AsRef<Path>) -> Result<LintReport, String> {
        let p = path.as_ref();
        let content = std::fs::read_to_string(p)
            .map_err(|e| format!("Failed to read file {}: {e}", p.display()))?;
        Ok(self.lint_source(&p.display().to_string(), &content))
    }

    /// Pre-flight gate for the Assembly pipeline: returns Ok only if zero fatal errors exist.
    pub fn lint_for_assembly(&self, file_path: &str, content: &str) -> Result<LintReport, String> {
        let report = self.lint_source(file_path, content);
        if !report.is_valid {
            Err(format!(
                "Pre-assembly linting failed for '{}' with {} errors: {}",
                file_path,
                report.errors.len(),
                report.errors.join("; ")
            ))
        } else {
            Ok(report)
        }
    }

    /// Auto-scaffolds a compliant 4-Block skeleton for an episodic cognitive scroll (.scroll).
    pub fn auto_scaffold_scroll(session_id: &str, prompt: &str) -> String {
        LinterScaffolder::auto_scaffold_scroll(session_id, prompt)
    }

    /// Auto-scaffolds a compliant 4-Block skeleton for a given module name (.omni).
    pub fn auto_scaffold_omni(module_name: &str, intent_aside: &str) -> String {
        LinterScaffolder::auto_scaffold_omni(module_name, intent_aside)
    }

    /// Auto-scaffolds a compliant `.tape` process ribbon skeleton.
    pub fn auto_scaffold_tape(tape_name: &str, intent_aside: &str) -> String {
        LinterScaffolder::auto_scaffold_tape(tape_name, intent_aside)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Comprehensive Unit Tests & Subsystem Sealing )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linter_character_omni_source_reel_passes() {
        let logger = DiagnosticLogger::new(false);
        let linter = OmniLinter::new(logger);
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.omni");
        if path.exists() {
            let report = linter.lint_file(&path).expect("Linting character.omni");
            assert!(report.is_valid, "character.omni must pass linter with 0 errors: {:?}", report.errors);
            assert_eq!(report.blocks_found.len(), 4, "Must find all 4 blocks in character.omni");
            let audit = report.audit_score.as_ref().expect("Audit score must be computed");
            assert!(audit.eligible_to_compile, "character.omni must be eligible to compile: {:?}", audit.gate);
            assert_eq!(audit.gate, QuadStateGate::Proceed);
            assert_eq!(audit.composite_score, 100, "character.omni must achieve composite score 100");
        }
    }

    #[test]
    fn test_linter_character_tape_process_ribbon_passes() {
        let logger = DiagnosticLogger::new(false);
        let linter = OmniLinter::new(logger);
        let path = crate::core::filetype::resolve_track_path("lexicon/01_nouns/ideas/types/character.tape");
        if path.exists() {
            let report = linter.lint_file(&path).expect("Linting character.tape");
            assert!(report.is_valid, "character.tape must pass linter with 0 errors: {:?}", report.errors);
            assert!(!report.blocks_found.is_empty(), "Must detect blocks/head in character.tape");
            let audit = report.audit_score.as_ref().expect("Audit score must be computed");
            assert!(audit.eligible_to_compile, "character.tape must be eligible to compile: {:?}", audit.gate);
            assert_eq!(audit.gate, QuadStateGate::Proceed);
            assert_eq!(audit.composite_score, 100, "character.tape must achieve composite score 100");
        }
    }

    #[test]
    fn test_linter_number_omni_source_reel_passes() {
        let logger = DiagnosticLogger::new(false);
        let linter = OmniLinter::new(logger);
        let path = crate::core::filetype::resolve_track_path(
            "lexicon/01_nouns/ideas/types/number.omni",
        );
        if path.exists() {
            let report = linter.lint_file(&path).expect("Linting number.omni");
            assert!(
                report.is_valid,
                "number.omni must pass linter: {:?}",
                report.errors
            );
            assert_eq!(
                report.blocks_found.len(),
                4,
                "Must find all 4 blocks in number.omni"
            );
            let audit = report
                .audit_score
                .as_ref()
                .expect("Audit score must be computed");
            assert!(
                audit.eligible_to_compile,
                "number.omni must be eligible: {:?}",
                audit.gate
            );
            assert_eq!(audit.gate, QuadStateGate::Proceed);
            assert_eq!(
                audit.composite_score, 100,
                "number.omni must achieve composite score 100"
            );
        }
    }

    #[test]
    fn test_linter_number_tape_process_ribbon_passes() {
        let logger = DiagnosticLogger::new(false);
        let linter = OmniLinter::new(logger);
        let path = crate::core::filetype::resolve_track_path(
            "lexicon/01_nouns/ideas/types/number.tape",
        );
        if path.exists() {
            let report = linter.lint_file(&path).expect("Linting number.tape");
            assert!(
                report.is_valid,
                "number.tape must pass linter: {:?}",
                report.errors
            );
            assert!(
                !report.blocks_found.is_empty(),
                "Must detect head in number.tape"
            );
            let audit = report
                .audit_score
                .as_ref()
                .expect("Audit score must be computed");
            assert!(
                audit.eligible_to_compile,
                "number.tape must be eligible: {:?}",
                audit.gate
            );
            assert_eq!(audit.gate, QuadStateGate::Proceed);
            assert_eq!(
                audit.composite_score, 100,
                "number.tape must achieve composite score 100"
            );
        }
    }

    #[test]
    fn test_linter_conjunctions_suite_passes() {
        let logger = DiagnosticLogger::new(false);
        let linter = OmniLinter::new(logger);
        let files = [
            "lexicon/07_conjunctions/caveat_branches.omni",
            "lexicon/07_conjunctions/caveat_branches.tape",
            "lexicon/07_conjunctions/inseparable_pairs.omni",
            "lexicon/07_conjunctions/inseparable_pairs.tape",
            "lexicon/07_conjunctions/pipeline_junctions.omni",
            "lexicon/07_conjunctions/pipeline_junctions.tape",
            "lexicon/07_conjunctions/alternative_branches.omni",
            "lexicon/07_conjunctions/alternative_branches.tape",
            "lexicon/07_conjunctions/relational_attributions.omni",
            "lexicon/07_conjunctions/relational_attributions.tape",
        ];

        for f in &files {
            let path = crate::core::filetype::resolve_track_path(f);
            if path.exists() {
                let report = linter.lint_file(&path).unwrap();
                assert!(
                    report.is_valid,
                    "{} must pass linter: {:?}",
                    f, report.errors
                );
                assert!(
                    report.errors.is_empty(),
                    "{} must have 0 syntax errors: {:?}",
                    f, report.errors
                );
            }
        }
    }

    #[test]
    fn test_linter_spatial_80_column_strict_rejection() {
        let logger = DiagnosticLogger::new(false);
        let linter = OmniLinter::new(logger).with_strict_geometry(true);
        let wide_code = format!("# {}\n", "A".repeat(85));
        let report = linter.lint_source("wide.omni", &wide_code);
        assert!(!report.is_valid);
        assert!(report.errors.iter().any(|e| e.contains("Line exceeds 80 columns")));
    }

    #[test]
    fn test_linter_assembly_preflight_partition_bounds() {
        let logger = DiagnosticLogger::new(false);
        let linter = OmniLinter::new(logger);
        let bad_partition_tape = r#"
            = [Tape]: < Lexicon > & [ Test ] {
                WHERE {partition} = [ 17 ] ::
                WHO {System} :: WHAT {Test} :: WHEN {Event} :: WHY (Test) :: HOW [+1:PROCEED]
            }
            [ ROLL 1 ] : [ Action ] : [ Output ]
            [ ROLL 2 ] : [ Commit Status ] : [ +1:PROCEED ]
        "#;
        let report = linter.lint_source("bad_partition.tape", bad_partition_tape);
        assert!(report.warnings.iter().any(|w| w.contains("below minimum 64 cells") || w.contains("not a power of 2")));
    }
}
