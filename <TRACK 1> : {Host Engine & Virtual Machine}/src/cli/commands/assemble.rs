// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Machine Manifest, Scheduling Priority, Dependency Contract & Tripwire Registry )
// ==============================================================================
//! MODULE:              omnicode::cli::commands::assemble
//! IDENTITY:            Physical Assembly & Process Tape Lowering Gear
//! NATURE:              Lowering Engine from High-Level SVO Tapes to Low-Level Memory Layouts & Silicon Word Streams
//! VERSION:             0.3.0
//! CREATED:             2026-10-11
//! LAST_SEALED:         2026-10-11
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
//!   WHO:               OmniCode Host Assembler Gear & Transpilation Lowering Driver
//!   WHAT:              Transduce high-level procedural `.tape` process ribbons into low-level declarative
//!                      `.omni` compiled memory reels, physical symbol tables, and 64-bit balanced-ternary
//!                      machine words ([`crate::core::word::Word64`]).
//!   WHERE:             Across Track 2 (`lexicon/`, `lib/kernel/`) or caller paths -> silicon memory partitions -> stdout / disk.
//!   WHEN:              Invoked via CLI command `omnicode assemble <file.tape> [--output <PATH>] [--format <listing|words|json>] [--verbose] [--dry-run]`
//!   WHY:               Eliminate ungrounded "python-ese" scripting drift by enforcing physical memory partition
//!                      allocation (`@ offset`), hardware register binding (`%self`, `%caller`, `%acc`, `%swarm`),
//!                      pronoun capability verification (`my`, `your`, `their`), and formal logic proofs (`! [ Invariant ]`).
//!   HOW:               Balanced Ternary (+1:PROCEED sealed assembly, -1:ROLLBACK trapped failpoint, 0:AWAIT dry-run).
//!
//! THE TRANSPILER TRIAD PHILOSOPHY:
//!   OmniCode operates across three isomorphic manifestations of code:
//!   1. `.omni` (Low-Level Compiled Form / Spatial Architecture):
//!      Static, declarative, frozen memory layouts, concrete denotations `[...]`, and invariant boundaries `<...>`.
//!   2. `.tape` (Interpreted Form / Temporal Execution Ribbon):
//!      Dynamic, sequential, time-bound rolls stepping across the tape head in real time.
//!   3. `Rust` (Silicon Host Substrate):
//!      Bedrock systems language defining opcodes, bitfields, and zero-cost native execution.
//!   This gear serves as the mechanical spindle turning Time (`.tape`) into Space (`.omni`) and Silicon (`Word64`).
//!
//! GAME DEV IDEOLOGY (THE STAGE INCEPTION & SIMULATION TICK):
//!   - Trigger Volumes & Collision Bounds: Every file resolution, syntax parse, partition slice, and seal
//!     is guarded by [`crate::cogs::tripwire::Tripwire`] collision volumes.
//!   - Flight Recorder: Step counts, instruction rolls, and physical offsets are logged to [`crate::diagnostics::logger::DiagnosticLogger`].
//!   - Respawn / Healing: Any syntax anomaly or invariant violation produces `#remediation_ready` telemetry
//!     for autonomous recovery by the [`crate::diagnostics::restorator::Restorator`].
//!
//! TRIPWIRE REGISTRY:
//!   - `TRIPWIRE_INPUT_RESOLVE`      (Resolution of input tape file across tracks via `resolve_track_path`)
//!   - `TRIPWIRE_FILETYPE_VALIDATION`(Enforcement of `.tape` or `.omni` extensions)
//!   - `TRIPWIRE_SYNTAX_TOKENIZATION`(Lexical analysis and token classification verification)
//!   - `TRIPWIRE_MEMORY_BOUNDS`      (Partition capacity and offset bounds verification)
//!   - `TRIPWIRE_COMMIT_INVARIANT`   (Assertion of final affirmative `+1:PROCEED` seal)
//!   - `TRIPWIRE_OUTPUT_SERIALIZE`   (File system permissions and disk serialization integrity)
//!
//! NAVIGATION GUIDE:
//!   - Block 1 (Lines 1-80): Manifest, Transpiler Triad Philosophy, Game Dev Ideology, Tripwire Registry.
//!   - Block 2 (Lines 81-220): Opening Block, Configurations (`AssembleConfig`), Context (`AssembleContext`), Enums.
//!   - Block 3 (Lines 221-420): Body Block & Execution Pipeline (`run_assemble_with_config`, `execute_assemble_pipeline`).
//!   - Block 4 (Lines 421+): Unit tests validating flag parsing, path resolution, and physical machine word output.

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Scoped Imports, Environment Setup, Data Structures & Interface Definitions )
// ==============================================================================

// ------------------------------------------------------------------------------
// 2.1. Scoped Namespace Ingestion & Multi-Platform Imports
// ------------------------------------------------------------------------------
use std::fmt;
use std::fs;
use std::path::PathBuf;

use crate::cogs::tripwire::{Tripwire, TripwireError};
use crate::core::filetype::resolve_track_path;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};
use crate::runtime::assembler::{AssembledTape, OmniAssembler};

// ------------------------------------------------------------------------------
// 2.2. Output Serialization Format Enumeration
// ------------------------------------------------------------------------------
/// Output format for the assembled process tape or spatial reel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AssembleFormat {
    /// Human-readable annotated machine listing (Default).
    #[default]
    Listing,
    /// Raw physical 64-bit machine words (Hex dump).
    Words,
    /// Structured JSON representation for tooling, hovercards, and IDEs.
    Json,
    /// Declarative 4-Block spatial architecture reel (`.omni`).
    Omni,
    /// Bare-metal x86 bootstrap assembly (`.asm`) for MilleniumOS.
    Asm,
}

impl AssembleFormat {
    /// Resolves a CLI format string into an `AssembleFormat` variant.
    pub fn from_str_name(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "listing" | "txt" => Some(Self::Listing),
            "words" | "hex" | "bin" => Some(Self::Words),
            "json" => Some(Self::Json),
            "omni" | "reel" => Some(Self::Omni),
            "asm" | "assembly" | "nasm" => Some(Self::Asm),
            _ => None,
        }
    }

    /// Canonical lowercase format identifier string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Listing => "listing",
            Self::Words => "words",
            Self::Json => "json",
            Self::Omni => "omni",
            Self::Asm => "asm",
        }
    }
}

impl fmt::Display for AssembleFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ------------------------------------------------------------------------------
// 2.3. Assembly Configuration Specification
// ------------------------------------------------------------------------------
/// Immutable configuration parameters for the assembly run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssembleConfig {
    /// Relative or absolute path to the target process tape (`.tape`) or reel (`.omni`).
    pub input_path: PathBuf,
    /// Optional output file path for the assembled listing or binary artifact.
    pub output_path: Option<PathBuf>,
    /// Selected output serialization format.
    pub format: AssembleFormat,
    /// Enables verbose step-by-step diagnostic logging.
    pub verbose: bool,
    /// Strict mode: treat warnings or unspecified offsets as hard failpoints.
    pub strict: bool,
    /// Dry run: simulate lowering without writing output artifacts.
    pub dry_run: bool,
}

impl AssembleConfig {
    /// Constructs a default configuration for a given input path.
    pub fn new(input_path: impl Into<PathBuf>) -> Self {
        Self {
            input_path: input_path.into(),
            output_path: None,
            format: AssembleFormat::default(),
            verbose: false,
            strict: false,
            dry_run: false,
        }
    }

    /// Parses CLI arguments into an `AssembleConfig`.
    pub fn from_args(args: &[String]) -> Result<Self, String> {
        let mut input_path: Option<PathBuf> = None;
        let mut output_path: Option<PathBuf> = None;
        let mut format = AssembleFormat::Listing;
        let mut verbose = false;
        let mut strict = false;
        let mut dry_run = false;

        let mut idx = 0;
        while idx < args.len() {
            let arg = args[idx].as_str();
            match arg {
                "-o" | "--output" => {
                    idx += 1;
                    if idx >= args.len() {
                        return Err("FAILPOINT [MissingOutputArg]: Flag '--output' requires a file path parameter".into());
                    }
                    output_path = Some(PathBuf::from(&args[idx]));
                }
                "-f" | "--format" => {
                    idx += 1;
                    if idx >= args.len() {
                        return Err("FAILPOINT [MissingFormatArg]: Flag '--format' requires a format name (listing|words|json)".into());
                    }
                    let fmt_str = &args[idx];
                    format = AssembleFormat::from_str_name(fmt_str)
                        .ok_or_else(|| format!("FAILPOINT [InvalidFormat]: Unknown format '{fmt_str}' (expected: listing, words, json)"))?;
                }
                "-v" | "--verbose" => {
                    verbose = true;
                }
                "-s" | "--strict" => {
                    strict = true;
                }
                "-n" | "--dry-run" => {
                    dry_run = true;
                }
                other => {
                    if !other.starts_with('-') && input_path.is_none() {
                        input_path = Some(PathBuf::from(other));
                    }
                }
            }
            idx += 1;
        }

        let input_path = input_path.ok_or_else(|| {
            "FAILPOINT [MissingInputPath]: No target tape file specified. Usage: omnicode assemble <file.tape> [--output <PATH>] [--format <listing|words|json>]".to_string()
        })?;

        Ok(Self {
            input_path,
            output_path,
            format,
            verbose,
            strict,
            dry_run,
        })
    }

    /// Performs pre-flight verification, resolves paths across tracks, and constructs an `AssembleContext`.
    pub fn open(&self, logger: &DiagnosticLogger) -> Result<AssembleContext, TripwireError> {
        // Tripwire 1: Input file resolution across tracks
        let resolved = resolve_track_path(&self.input_path);
        Tripwire::assert_path_exists(
            logger,
            "TRIPWIRE_INPUT_RESOLVE",
            &resolved,
            "Target process tape or reel across Track roots",
        )?;

        // Tripwire 2: File extension check
        let ext = resolved.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "tape" && ext != "omni" {
            let msg = format!("File '{}' has unsupported extension '.{}' (expected .tape or .omni)", resolved.display(), ext);
            return Err(Tripwire::trip(
                logger,
                "TRIPWIRE_FILETYPE_VALIDATION",
                FailpointType::SyntaxAnomaly,
                msg,
            ));
        }

        let debugger = OmniDebugger::new(logger.clone(), 100);
        let assembler = OmniAssembler::new(logger.clone(), debugger);

        Ok(AssembleContext {
            resolved_input: resolved,
            config: self.clone(),
            assembler,
            logger: logger.clone(),
        })
    }
}

// ------------------------------------------------------------------------------
// 2.4. Assembly Execution Context
// ------------------------------------------------------------------------------
/// Active runtime context holding resolved paths, tripwires, and the physical assembler.
#[derive(Debug)]
pub struct AssembleContext {
    /// Fully qualified path resolved across track roots.
    pub resolved_input: PathBuf,
    /// Immutable caller configuration.
    pub config: AssembleConfig,
    /// Physical assembler engine.
    pub assembler: OmniAssembler,
    /// Attached diagnostic logger for tripwires and telemetry.
    pub logger: DiagnosticLogger,
}

// ------------------------------------------------------------------------------
// 2.5. Initial Truth States & Equilibrium Defaults
// ------------------------------------------------------------------------------
/// Default output serialization format (human-readable annotated listing).
pub const DEFAULT_ASSEMBLE_FORMAT: AssembleFormat = AssembleFormat::Listing;
/// Baseline cell capacity reserved for unspecified memory partitions.
pub const DEFAULT_PARTITION_CELL_CAPACITY: usize = 64;
/// Maximum delimiter boundary nesting depth for structural invariant assertions.
pub const DEFAULT_ASSEMBLER_DEBUGGER_DEPTH: usize = 100;


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Assembly Execution Pipeline, Output Serialization & Tripwire Sealing )
// ==============================================================================

// ------------------------------------------------------------------------------
// 3.1. Public CLI Dispatchers
// ------------------------------------------------------------------------------
/// Legacy single-path entrypoint maintaining backwards compatibility with CLI dispatchers.
pub fn run_assemble(path: &str) {
    let config = AssembleConfig::new(path);
    if let Err(e) = run_assemble_with_config(config) {
        eprintln!("\x1b[1;31mAssembly Failed:\x1b[0m {e}");
        std::process::exit(1);
    }
}

/// Executes the modular assembly pipeline with full configuration, tripwires, and formatting.
pub fn run_assemble_with_config(config: AssembleConfig) -> Result<AssembledTape, TripwireError> {
    let logger = DiagnosticLogger::new(config.verbose);
    println!("\x1b[1;36mAssembling OmniCode Process Tape:\x1b[0m {}", config.input_path.display());

    let mut ctx = config.open(&logger)?;
    let assembled = execute_assemble_pipeline(&mut ctx)?;

    // Handle output formatting and serialization
    serialize_assembled_output(&assembled, &ctx.config, &logger)?;

    println!(
        "\x1b[1;32m✓ Assembly Successful: Machine Tape Sealed in {}\x1b[0m ({} rolls, {} symbols, {} words)",
        assembled.commit_status.macro_op(),
        assembled.instructions.len(),
        assembled.symbols.len(),
        assembled.instructions.len()
    );

    Ok(assembled)
}

// ------------------------------------------------------------------------------
// 3.2. Core Lowering Pipeline
// ------------------------------------------------------------------------------
/// Invokes the lowering engine within the tripwire collision hull.
pub fn execute_assemble_pipeline(ctx: &mut AssembleContext) -> Result<AssembledTape, TripwireError> {
    let tape_path = ctx.resolved_input.clone();
    
    let assembled = match ctx.assembler.assemble_file(&tape_path) {
        Ok(t) => t,
        Err(e) => {
            return Err(Tripwire::trip(
                &ctx.logger,
                "TRIPWIRE_SYNTAX_TOKENIZATION",
                FailpointType::SyntaxAnomaly,
                format!("Tape assembly lowered into failpoint: {e}"),
            ));
        }
    };

    // Assert final commit status invariant
    Tripwire::assert_quad_state_proceed(
        &ctx.logger,
        "TRIPWIRE_COMMIT_INVARIANT",
        assembled.commit_status,
        "Process tape must conclude with 'commit_status [ PROCEED ]'",
    )?;

    Ok(assembled)
}

// ------------------------------------------------------------------------------
// 3.3. Output Serialization & Formatting
// ------------------------------------------------------------------------------
/// Renders or writes the assembled tape to disk or stdout according to the chosen format.
///
/// Consumes the universal serialization methods ([`AssembledTape::format_listing`],
/// [`AssembledTape::format_words`], [`AssembledTape::format_json`], [`AssembledTape::format_omni`])
/// extracted from the runtime lowering engine.
pub fn serialize_assembled_output(
    assembled: &AssembledTape,
    config: &AssembleConfig,
    logger: &DiagnosticLogger,
) -> Result<(), TripwireError> {
    let rendered = match config.format {
        AssembleFormat::Listing => assembled.format_listing(),
        AssembleFormat::Words => assembled.format_words(),
        AssembleFormat::Json => assembled.format_json(),
        AssembleFormat::Omni => assembled.format_omni(),
        AssembleFormat::Asm => assembled.format_asm(),
    };

    if let Some(ref out_path) = config.output_path {
        if !config.dry_run {
            if let Err(e) = fs::write(out_path, &rendered) {
                return Err(Tripwire::trip(
                    logger,
                    "TRIPWIRE_OUTPUT_SERIALIZE",
                    FailpointType::SyntaxAnomaly,
                    format!("Failed to write assembled output to '{}': {e}", out_path.display()),
                ));
            }
            println!("\x1b[1;32m✓ Output Artifact Written:\x1b[0m {}", out_path.display());
        } else {
            println!("\x1b[1;33m[Dry Run] Output serialization skipped for:\x1b[0m {}", out_path.display());
        }
    } else {
        println!("{rendered}");
    }

    Ok(())
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Baton Pass Contract, Unit Tests & Verification Proof )
// ==============================================================================

// ------------------------------------------------------------------------------
// 4.1. The Baton Pass Contract
// ------------------------------------------------------------------------------
// BATON PASS CONTRACT:
// 1. `AssembleConfig` parses and validates user CLI flags without side effects.
// 2. `open()` verifies that the process tape exists across Track roots and establishes tripwire bounds.
// 3. `execute_assemble_pipeline()` drives lowering into `OmniAssembler`, verifying `MachineOp`, registers, and `Word64`.
// 4. `serialize_assembled_output()` formats or writes the output, yielding a sealed `AssembledTape`.
// 5. Downstream dispatchers in `main.rs` receive verified artifacts ready for scheduling or execution.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ternary::QuadState;

    #[test]
    fn test_assemble_config_default_and_new() {
        let config = AssembleConfig::new("lib/kernel/boot.tape");
        assert_eq!(config.input_path, PathBuf::from("lib/kernel/boot.tape"));
        assert_eq!(config.format, AssembleFormat::Listing);
        assert!(!config.verbose);
        assert!(!config.strict);
        assert!(!config.dry_run);
        assert_eq!(config.output_path, None);
    }

    #[test]
    fn test_assemble_config_argument_parsing() {
        let args = vec![
            "lexicon/operators/polarity.tape".to_string(),
            "--output".to_string(),
            "/tmp/polarity.asm".to_string(),
            "--format".to_string(),
            "words".to_string(),
            "--verbose".to_string(),
            "--dry-run".to_string(),
        ];
        let config = AssembleConfig::from_args(&args).expect("Parsing CLI args");
        assert_eq!(config.input_path, PathBuf::from("lexicon/operators/polarity.tape"));
        assert_eq!(config.output_path, Some(PathBuf::from("/tmp/polarity.asm")));
        assert_eq!(config.format, AssembleFormat::Words);
        assert!(config.verbose);
        assert!(config.dry_run);
    }

    #[test]
    fn test_assemble_config_missing_input_fails_gracefully() {
        let args: Vec<String> = vec!["--verbose".to_string()];
        let err = AssembleConfig::from_args(&args).unwrap_err();
        assert!(err.contains("MissingInputPath"));
    }

    #[test]
    fn test_assemble_format_conversions() {
        assert_eq!(AssembleFormat::from_str_name("listing"), Some(AssembleFormat::Listing));
        assert_eq!(AssembleFormat::from_str_name("words"), Some(AssembleFormat::Words));
        assert_eq!(AssembleFormat::from_str_name("hex"), Some(AssembleFormat::Words));
        assert_eq!(AssembleFormat::from_str_name("json"), Some(AssembleFormat::Json));
        assert_eq!(AssembleFormat::from_str_name("omni"), Some(AssembleFormat::Omni));
        assert_eq!(AssembleFormat::from_str_name("reel"), Some(AssembleFormat::Omni));
        assert_eq!(AssembleFormat::from_str_name("unknown"), None);
    }

    #[test]
    fn test_assemble_pipeline_lowers_boot_tape_and_transpiles_to_omni() {
        let tape_path = resolve_track_path("lib/kernel/boot.tape");
        if tape_path.exists() {
            let config = AssembleConfig {
                input_path: tape_path,
                output_path: None,
                format: AssembleFormat::Omni,
                verbose: false,
                strict: false,
                dry_run: true,
            };
            let logger = DiagnosticLogger::new(false);
            let mut ctx = config.open(&logger).expect("Opening assemble context");
            let assembled = execute_assemble_pipeline(&mut ctx).expect("Executing assemble pipeline");

            let omni_output = assembled.format_omni();
            assert!(omni_output.contains("Transpiled Spatial Reel"));
            assert!(omni_output.contains("BLOCK 1: METADATA BLOCK"));
            assert!(omni_output.contains("BLOCK 2: OPENING BLOCK"));
            assert!(omni_output.contains("BLOCK 3: BODY BLOCK"));
            assert!(omni_output.contains("BLOCK 4: CLOSING BLOCK"));
        }
    }

    #[test]
    fn test_assemble_pipeline_lowers_boot_tape_and_emits_words() {
        let tape_path = resolve_track_path("lib/kernel/boot.tape");
        if tape_path.exists() {
            let config = AssembleConfig {
                input_path: tape_path,
                output_path: None,
                format: AssembleFormat::Words,
                verbose: false,
                strict: false,
                dry_run: true,
            };
            let logger = DiagnosticLogger::new(false);
            let mut ctx = config.open(&logger).expect("Opening assemble context");
            let assembled = execute_assemble_pipeline(&mut ctx).expect("Executing assemble pipeline");

            assert_eq!(assembled.commit_status, QuadState::Proceed);
            assert!(!assembled.instructions.is_empty());

            // Assert that instructions carry real physical Word64 machine words
            for inst in &assembled.instructions {
                assert_ne!(inst.word.raw, 0xFFFFFFFF_FFFFFFFF);
            }
        }
    }

    #[test]
    fn test_assemble_preflight_missing_file_trips() {
        let config = AssembleConfig::new("nonexistent_directory/missing.tape");
        let logger = DiagnosticLogger::new(false);
        let err = config.open(&logger).unwrap_err();
        assert!(err.to_string().contains("TRIPWIRE_INPUT_RESOLVE"));
    }

    #[test]
    fn test_assemble_preflight_invalid_extension_trips() {
        let config = AssembleConfig::new("Cargo.toml");
        let logger = DiagnosticLogger::new(false);
        let err = config.open(&logger).unwrap_err();
        assert!(err.to_string().contains("TRIPWIRE_FILETYPE_VALIDATION"));
    }
}
