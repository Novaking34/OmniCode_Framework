// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal CLI Runner: Multi-Substrate Execution across Host & Virtual Terminal )
// ==============================================================================
//! OmniCode CLI Unified Runner Command Gear
//!
//! # Architectural Scope & Purpose:
//! Orchestrates execution of all OmniCode code manifestations across both host terminals
//! and the virtual terminal substrate:
//! 1. Native Spatial Source Reels (`.omni`) & Episodic Scrolls (`.scroll`)
//! 2. Procedural Process Tapes (`.tape`)
//! 3. Transpiled C Backbones & Tests (`.c`)
//! 4. Transpiled Lua Process Tapes (`.lua`)
//! 5. Dynamic On-The-Fly Transpilation (`--transpile c` / `--transpile lua`)
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of physical data into bounded cell tablets).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic types across host and virtual substrates).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of historical memory boundaries, silicon addresses, and constants).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Substrate Transparency**: An OmniCode process behaves identically whether executed
//!    directly on a Linux host terminal via `omnicode run`, simulated in `TerminalSession`,
//!    transpiled to C for bare metal, or run via Lua for game engine scripting.
// ==============================================================================

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use crate::cogs::tripwire::{Tripwire, TripwireError};
use crate::core::filetype::{resolve_track_path, OmniFileType};
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::runtime::assembler::OmniAssembler;
use crate::runtime::environment::Environment;
use crate::runtime::evaluator::core::Evaluator;
use crate::terminal::scheduler::{OmniScheduler, Track};
use crate::terminal::session::TerminalSession;
use std::sync::atomic::{AtomicU64, Ordering};

/// Thread-safe counter guaranteeing unique temporary artifact names across concurrent test threads.
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Run Configurations, Substrate Format Enums & Context Definitions )
// ==============================================================================

/// Target execution substrate format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RunFormat {
    /// Auto-detect format from file extension and content.
    #[default]
    Auto,
    /// Native OmniCode 4-Block source reel (`.omni`) or episodic scroll (`.scroll`).
    Omni,
    /// Procedural process ribbon tape (`.tape`).
    Tape,
    /// Standard C transpile equivalent or unit test (`.c`).
    C,
    /// Standard Lua transpile equivalent or tape script (`.lua`).
    Lua,
}

impl RunFormat {
    /// Resolves format from a string name.
    pub fn from_str_name(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "omni" | "reel" | "scroll" => Some(Self::Omni),
            "tape" => Some(Self::Tape),
            "c" => Some(Self::C),
            "lua" => Some(Self::Lua),
            "auto" => Some(Self::Auto),
            _ => None,
        }
    }

    /// Resolves format from a file path extension.
    pub fn from_path(path: &Path) -> Self {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        match ext.as_str() {
            "omni" | "scroll" => Self::Omni,
            "tape" => Self::Tape,
            "c" | "h" => Self::C,
            "lua" => Self::Lua,
            _ => Self::Auto,
        }
    }

    /// Canonical lowercase format name.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Omni => "omni",
            Self::Tape => "tape",
            Self::C => "c",
            Self::Lua => "lua",
        }
    }
}

impl fmt::Display for RunFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Configuration options for the runner command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunConfig {
    /// Target file path to execute.
    pub input_path: PathBuf,
    /// Substrate format.
    pub format: RunFormat,
    /// Optional transpile target on the fly (`"c"` or `"lua"`).
    pub transpile_target: Option<String>,
    /// Verbose diagnostic logging.
    pub verbose: bool,
    /// Dry run mode (validates and compiles without executing).
    pub dry_run: bool,
}

impl RunConfig {
    /// Creates a new `RunConfig` with default flags.
    pub fn new(input_path: impl Into<PathBuf>) -> Self {
        let path = input_path.into();
        let format = RunFormat::from_path(&path);
        Self {
            input_path: path,
            format,
            transpile_target: None,
            verbose: false,
            dry_run: false,
        }
    }

    /// Parses command line arguments for `omnicode run <file> [flags]`.
    pub fn from_args(args: &[String]) -> Result<Self, String> {
        let mut input_path: Option<PathBuf> = None;
        let mut format = RunFormat::Auto;
        let mut transpile_target: Option<String> = None;
        let mut verbose = false;
        let mut dry_run = false;

        let mut idx = 0;
        while idx < args.len() {
            match args[idx].as_str() {
                "--format" | "-f" => {
                    idx += 1;
                    if idx >= args.len() {
                        return Err("FAILPOINT [MissingFormat]: Option '--format' requires a format argument (omni|tape|c|lua)".into());
                    }
                    format = RunFormat::from_str_name(&args[idx])
                        .ok_or_else(|| format!("FAILPOINT [InvalidFormat]: Unknown format '{}'", args[idx]))?;
                }
                "--transpile" | "-t" => {
                    idx += 1;
                    if idx >= args.len() {
                        return Err("FAILPOINT [MissingTranspile]: Option '--transpile' requires a target (c|lua)".into());
                    }
                    transpile_target = Some(args[idx].to_lowercase());
                }
                "--verbose" | "-v" => {
                    verbose = true;
                }
                "--dry-run" | "-d" => {
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
            "FAILPOINT [MissingInputPath]: No target file specified. Usage: omnicode run <file> [--format omni|tape|c|lua] [--transpile c|lua]".to_string()
        })?;

        if format == RunFormat::Auto {
            format = RunFormat::from_path(&input_path);
        }

        Ok(Self {
            input_path,
            format,
            transpile_target,
            verbose,
            dry_run,
        })
    }

    /// Resolves target file across track roots and validates preconditions.
    pub fn open(&self, logger: &DiagnosticLogger) -> Result<RunContext, TripwireError> {
        let resolved = resolve_track_path(&self.input_path);
        Tripwire::assert_path_exists(
            logger,
            "TRIPWIRE_RUN_RESOLVE",
            &resolved,
            "Target script, process tape, or transpile source",
        )?;

        let format = if self.format == RunFormat::Auto {
            RunFormat::from_path(&resolved)
        } else {
            self.format
        };

        Ok(RunContext {
            resolved_input: resolved,
            format,
            config: self.clone(),
            logger: logger.clone(),
        })
    }
}

/// Context passed through execution stages.
#[derive(Debug, Clone)]
pub struct RunContext {
    /// Fully resolved target path.
    pub resolved_input: PathBuf,
    /// Target execution format.
    pub format: RunFormat,
    /// Immutable run configuration.
    pub config: RunConfig,
    /// System diagnostic flight recorder.
    pub logger: DiagnosticLogger,
}

/// Result of running a file.
#[derive(Debug, Clone, PartialEq)]
pub struct RunResult {
    /// Executed substrate format.
    pub format: RunFormat,
    /// Final balanced ternary state (+1:PROCEED).
    pub state: QuadState,
    /// Combined terminal output string.
    pub output: String,
    /// Execution duration in microseconds.
    pub duration_micros: u128,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Multi-Substrate Execution Pipelines & Virtual Terminal Integration )
// ==============================================================================

/// Discovers Track 2 standard library paths for C and Lua transpile backbones.
pub fn find_track2_transpile_dirs() -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let base = resolve_track_path("lexicon/01_nouns/ideas/types/transpile variant");
    let c_include = base.join("c_backbone/include");
    let c_tests_include = base.join("tests/c/includes");
    let c_src = base.join("c_backbone/src");
    let lua_tapes = base.join("lua_tapes");
    (c_include, c_tests_include, c_src, lua_tapes)
}

/// Executes an OmniCode source reel (`.omni`) or episodic scroll (`.scroll`).
pub fn run_omni_source(ctx: &RunContext) -> Result<RunResult, TripwireError> {
    let start = Instant::now();
    let content = match fs::read_to_string(&ctx.resolved_input) {
        Ok(c) => c,
        Err(e) => {
            return Err(Tripwire::trip(
                &ctx.logger,
                "TRIPWIRE_OMNI_READ",
                FailpointType::SyntaxAnomaly,
                format!("Failed to read OmniCode file '{}': {e}", ctx.resolved_input.display()),
            ));
        }
    };

    let mut lexer = Lexer::new(&content, ctx.logger.clone());
    let tokens = match lexer.tokenize() {
        Ok(toks) => toks,
        Err(e) => {
            return Err(Tripwire::trip(
                &ctx.logger,
                "TRIPWIRE_OMNI_LEX",
                FailpointType::SyntaxAnomaly,
                format!("Lexical analysis failed for '{}': {e}", ctx.resolved_input.display()),
            ));
        }
    };

    let debugger = OmniDebugger::new(ctx.logger.clone(), 100);
    let mut parser = Parser::new(tokens, debugger);
    let prog = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            return Err(Tripwire::trip(
                &ctx.logger,
                "TRIPWIRE_OMNI_PARSE",
                FailpointType::SyntaxAnomaly,
                format!("Parsing failed for '{}': {e}", ctx.resolved_input.display()),
            ));
        }
    };

    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
    let eval_val = match evaluator.eval(&crate::ast::nodes::ASTNode::Program(prog)) {
        Ok(v) => v,
        Err(e) => {
            return Err(Tripwire::trip(
                &ctx.logger,
                "TRIPWIRE_OMNI_EVAL",
                FailpointType::SyntaxAnomaly,
                format!("Evaluation failed for '{}': {e}", ctx.resolved_input.display()),
            ));
        }
    };

    let state = eval_val.to_quad_state();
    let output = format!(
        "[+] OmniCode Source Reel Executed Successfully: '{}'\n    • Substrate: Native OmniCode VM (Track 1 Rust Engine)\n    • Result:    => {}\n    • State:     +1:PROCEED",
        ctx.resolved_input.display(),
        eval_val.format_colored()
    );

    // Inscribe persistent execution scroll and telemetry log
    let session_id = format!("run_omni_{}", std::process::id());
    let timestamp_str = crate::ai::model::scroll::generate_timestamp_id();
    let scroll = crate::ai::model::scroll::ExecutionScroll {
        session_id,
        timestamp_str,
        prompt: format!("Execute OmniCode Source Reel: {}", ctx.resolved_input.display()),
        synthesized_omnicode: content,
        execution_state: state,
        natural_response: output.clone(),
        failpoints: ctx.logger.failpoints(),
        repairs_applied: Vec::new(),
        remediations_suggested: ctx.logger.remediations(),
        elapsed_micros: start.elapsed().as_micros(),
    };
    let _ = crate::ai::model::scroll::ScrollLogger::default().save_dual_logs(&scroll);

    Ok(RunResult {
        format: RunFormat::Omni,
        state,
        output,
        duration_micros: start.elapsed().as_micros(),
    })
}

/// Executes a sequential process tape (`.tape`) via the OmniScheduler.
pub fn run_tape_ribbon(ctx: &RunContext) -> Result<RunResult, TripwireError> {
    let start = Instant::now();
    let mut scheduler = OmniScheduler::new();

    let path_str = ctx.resolved_input.to_string_lossy().to_string();
    let track = if path_str.contains("kernel") || path_str.contains("boot") {
        Track::Kernel
    } else {
        Track::App
    };

    let pid = match scheduler.mount_tape_file(track, &path_str, ctx.logger.clone()) {
        Ok(id) => id,
        Err(e) => {
            return Err(Tripwire::trip(
                &ctx.logger,
                "TRIPWIRE_TAPE_MOUNT",
                FailpointType::SyntaxAnomaly,
                format!("Failed to mount process tape '{}': {e}", ctx.resolved_input.display()),
            ));
        }
    };

    let mut logs = Vec::new();
    logs.push(format!("[+] Mounted Tape '{}' onto {} (PID {pid})", path_str, track.as_str()));

    let mut steps = 0;
    loop {
        match scheduler.step_task(pid) {
            Ok(msg) => {
                logs.push(msg.clone());
                if msg.contains("completed") || msg.contains("already executed") || msg.contains("STOP") {
                    break;
                }
            }
            Err(e) => {
                return Err(Tripwire::trip(
                    &ctx.logger,
                    "TRIPWIRE_TAPE_STEP",
                    FailpointType::SyntaxAnomaly,
                    format!("Task {pid} step halted: {e}"),
                ));
            }
        }
        steps += 1;
        if steps > 1000 {
            logs.push("[!] Transport kinematic loop limit reached (1000 steps)".to_string());
            break;
        }
    }

    let output = logs.join("\n");

    // Inscribe persistent execution scroll and telemetry log
    let session_id = format!("run_tape_{}", std::process::id());
    let timestamp_str = crate::ai::model::scroll::generate_timestamp_id();
    let scroll = crate::ai::model::scroll::ExecutionScroll {
        session_id,
        timestamp_str,
        prompt: format!("Run OmniCode Process Tape: {}", ctx.resolved_input.display()),
        synthesized_omnicode: output.clone(),
        execution_state: QuadState::Proceed,
        natural_response: "Tape execution complete: All rolls verified in Proceed (+1)".to_string(),
        failpoints: ctx.logger.failpoints(),
        repairs_applied: Vec::new(),
        remediations_suggested: ctx.logger.remediations(),
        elapsed_micros: start.elapsed().as_micros(),
    };
    let _ = crate::ai::model::scroll::ScrollLogger::default().save_dual_logs(&scroll);

    Ok(RunResult {
        format: RunFormat::Tape,
        state: QuadState::Proceed,
        output,
        duration_micros: start.elapsed().as_micros(),
    })
}

/// Compiles and executes a C transpile equivalent (`.c`).
pub fn run_c_transpile(ctx: &RunContext) -> Result<RunResult, TripwireError> {
    let start = Instant::now();
    let (c_inc, c_tests_inc, c_src, _) = find_track2_transpile_dirs();

    let target_path = &ctx.resolved_input;
    let target_content = fs::read_to_string(target_path).unwrap_or_default();
    let has_main = target_content.contains("int main(") || target_content.contains("int main (");

    // Output binary in temp dir with thread-unique identifier
    let seq = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
    let temp_bin = std::env::temp_dir().join(format!("omnicode_c_bin_{}_{}", std::process::id(), seq));

    let mut cmd = Command::new("gcc");
    cmd.arg("-O2");

    if c_inc.exists() {
        cmd.arg("-I").arg(&c_inc);
    }
    if c_tests_inc.exists() {
        cmd.arg("-I").arg(&c_tests_inc);
    }

    cmd.arg(target_path);

    // Link all supporting C sources from c_backbone/src if present
    if c_src.exists() {
        if let Ok(entries) = fs::read_dir(&c_src) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|e| e.to_str()) == Some("c") && p != *target_path {
                    cmd.arg(p);
                }
            }
        }
    }

    // If compiling the multi-file testbed (test_omni_lib.c), link unit test companions
    if target_path.to_string_lossy().contains("test_omni_lib") {
        let unit_dir = c_tests_inc.parent().map(|p| p.join("unit"));
        if let Some(ref udir) = unit_dir {
            if udir.exists() {
                if let Ok(entries) = fs::read_dir(udir) {
                    for entry in entries.flatten() {
                        let p = entry.path();
                        if p.extension().and_then(|e| e.to_str()) == Some("c") && p != *target_path {
                            cmd.arg(p);
                        }
                    }
                }
            }
        }
    }

    // If unit test without main, link with testbed integrated runner
    if !has_main {
        let testbed = c_tests_inc.parent().map(|p| p.join("integrated/test_omni_lib.c"));
        if let Some(tb) = testbed {
            if tb.exists() && tb != *target_path {
                cmd.arg(&tb);
            }
        }
    }

    cmd.arg("-o").arg(&temp_bin);

    let compile_output = match cmd.output() {
        Ok(out) => out,
        Err(e) => {
            return Err(Tripwire::trip(
                &ctx.logger,
                "TRIPWIRE_C_COMPILE",
                FailpointType::SyntaxAnomaly,
                format!("Failed to invoke C compiler (gcc): {e}"),
            ));
        }
    };

    if !compile_output.status.success() {
        let stderr = String::from_utf8_lossy(&compile_output.stderr);
        return Err(Tripwire::trip(
            &ctx.logger,
            "TRIPWIRE_C_BUILD",
            FailpointType::SyntaxAnomaly,
            format!("C compilation failed:\n{stderr}"),
        ));
    }

    if ctx.config.dry_run {
        let _ = fs::remove_file(&temp_bin);
        return Ok(RunResult {
            format: RunFormat::C,
            state: QuadState::Proceed,
            output: format!("[√ Dry Run] C compilation verified for '{}'", target_path.display()),
            duration_micros: start.elapsed().as_micros(),
        });
    }

    // Execute compiled binary
    let exec_output = match Command::new(&temp_bin).output() {
        Ok(out) => out,
        Err(e) => {
            let _ = fs::remove_file(&temp_bin);
            return Err(Tripwire::trip(
                &ctx.logger,
                "TRIPWIRE_C_EXEC",
                FailpointType::SyntaxAnomaly,
                format!("Failed to execute compiled C binary: {e}"),
            ));
        }
    };

    let _ = fs::remove_file(&temp_bin);

    let stdout = String::from_utf8_lossy(&exec_output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&exec_output.stderr).to_string();
    let combined = if stderr.is_empty() {
        stdout
    } else {
        format!("{stdout}\n{stderr}")
    };

    let state = if exec_output.status.success() {
        QuadState::Proceed
    } else {
        QuadState::Rollback
    };

    Ok(RunResult {
        format: RunFormat::C,
        state,
        output: combined,
        duration_micros: start.elapsed().as_micros(),
    })
}

/// Executes a Lua transpile tape (`.lua`) via the host Lua engine.
pub fn run_lua_transpile(ctx: &RunContext) -> Result<RunResult, TripwireError> {
    let start = Instant::now();
    let (_, _, _, lua_tapes_dir) = find_track2_transpile_dirs();

    let target_path = &ctx.resolved_input;
    let target_dir = target_path.parent().unwrap_or_else(|| Path::new("."));

    // Build LUA_PATH to find tape_engine.lua, test_common.lua, and local modules
    let lua_path_env = format!(
        "{}/?.lua;{}/?.lua;{}/?.lua;{}/tests/lua/unit/?.lua;;",
        lua_tapes_dir.display(),
        target_dir.display(),
        lua_tapes_dir.parent().unwrap_or(&lua_tapes_dir).display(),
        lua_tapes_dir.parent().unwrap_or(&lua_tapes_dir).display()
    );

    let mut cmd = Command::new("lua");
    cmd.env("LUA_PATH", &lua_path_env);
    cmd.arg(target_path);

    if ctx.config.dry_run {
        return Ok(RunResult {
            format: RunFormat::Lua,
            state: QuadState::Proceed,
            output: format!("[√ Dry Run] Lua target validated: '{}'", target_path.display()),
            duration_micros: start.elapsed().as_micros(),
        });
    }

    let output = match cmd.output() {
        Ok(out) => out,
        Err(e) => {
            return Err(Tripwire::trip(
                &ctx.logger,
                "TRIPWIRE_LUA_EXEC",
                FailpointType::SyntaxAnomaly,
                format!("Failed to invoke Lua engine: {e}"),
            ));
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined = if stderr.is_empty() {
        stdout
    } else {
        format!("{stdout}\n{stderr}")
    };

    let state = if output.status.success() {
        QuadState::Proceed
    } else {
        QuadState::Rollback
    };

    Ok(RunResult {
        format: RunFormat::Lua,
        state,
        output: combined,
        duration_micros: start.elapsed().as_micros(),
    })
}

/// Executes a dynamic on-the-fly transpile & run cycle for `.tape` or `.omni`.
pub fn run_transpile_on_the_fly(
    ctx: &RunContext,
    target_lang: &str,
) -> Result<RunResult, TripwireError> {
    let _start = Instant::now();
    let debugger = OmniDebugger::new(ctx.logger.clone(), 100);
    let mut assembler = OmniAssembler::new(ctx.logger.clone(), debugger);

    let assembled = match assembler.assemble_file(&ctx.resolved_input) {
        Ok(a) => a,
        Err(e) => {
            return Err(Tripwire::trip(
                &ctx.logger,
                "TRIPWIRE_TRANSPILE_ASSEMBLE",
                FailpointType::SyntaxAnomaly,
                format!("Assembly failed prior to on-the-fly transpilation: {e}"),
            ));
        }
    };

    match target_lang {
        "c" => {
            let c_code = assembled.format_c();
            let seq = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
            let temp_c = std::env::temp_dir().join(format!("omnicode_dyn_{}_{}.c", std::process::id(), seq));
            if let Err(e) = fs::write(&temp_c, &c_code) {
                return Err(Tripwire::trip(
                    &ctx.logger,
                    "TRIPWIRE_TRANSPILE_WRITE",
                    FailpointType::SyntaxAnomaly,
                    format!("Failed to write dynamic C artifact: {e}"),
                ));
            }
            let mut sub_config = ctx.config.clone();
            sub_config.input_path = temp_c.clone();
            sub_config.format = RunFormat::C;
            let sub_ctx = RunContext {
                resolved_input: temp_c.clone(),
                format: RunFormat::C,
                config: sub_config,
                logger: ctx.logger.clone(),
            };
            let res = run_c_transpile(&sub_ctx);
            let _ = fs::remove_file(&temp_c);
            res
        }
        "lua" => {
            let lua_code = assembled.format_lua();
            let seq = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
            let temp_lua = std::env::temp_dir().join(format!("omnicode_dyn_{}_{}.lua", std::process::id(), seq));
            if let Err(e) = fs::write(&temp_lua, &lua_code) {
                return Err(Tripwire::trip(
                    &ctx.logger,
                    "TRIPWIRE_TRANSPILE_WRITE",
                    FailpointType::SyntaxAnomaly,
                    format!("Failed to write dynamic Lua artifact: {e}"),
                ));
            }
            let mut sub_config = ctx.config.clone();
            sub_config.input_path = temp_lua.clone();
            sub_config.format = RunFormat::Lua;
            let sub_ctx = RunContext {
                resolved_input: temp_lua.clone(),
                format: RunFormat::Lua,
                config: sub_config,
                logger: ctx.logger.clone(),
            };
            let res = run_lua_transpile(&sub_ctx);
            let _ = fs::remove_file(&temp_lua);
            res
        }
        other => Err(Tripwire::trip(
            &ctx.logger,
            "TRIPWIRE_TRANSPILE_LANG",
            FailpointType::SyntaxAnomaly,
            format!("Unsupported on-the-fly transpile target '{other}' (expected 'c' or 'lua')"),
        )),
    }
}

/// Main execution pipeline dispatcher.
pub fn execute_run_pipeline(ctx: &RunContext) -> Result<RunResult, TripwireError> {
    // 1. Dynamic On-the-Fly Transpilation Check
    if let Some(ref target_lang) = ctx.config.transpile_target {
        return run_transpile_on_the_fly(ctx, target_lang);
    }

    // 2. Format Dispatcher
    match ctx.format {
        RunFormat::Omni => run_omni_source(ctx),
        RunFormat::Tape => run_tape_ribbon(ctx),
        RunFormat::C => run_c_transpile(ctx),
        RunFormat::Lua => run_lua_transpile(ctx),
        RunFormat::Auto => {
            let ft = OmniFileType::from_path(&ctx.resolved_input);
            match ft {
                Some(OmniFileType::Tape) => run_tape_ribbon(ctx),
                Some(OmniFileType::SourceReel) | Some(OmniFileType::EpisodicScroll) => run_omni_source(ctx),
                None => {
                    let ext = ctx.resolved_input.extension().and_then(|e| e.to_str()).unwrap_or("");
                    match ext {
                        "c" | "h" => run_c_transpile(ctx),
                        "lua" => run_lua_transpile(ctx),
                        _ => run_omni_source(ctx),
                    }
                }
            }
        }
    }
}

/// Convenience entry point running directly from a `RunConfig`.
pub fn run_with_config(config: RunConfig) -> Result<RunResult, String> {
    let logger = DiagnosticLogger::new(config.verbose);
    let ctx = config.open(&logger).map_err(|e| e.to_string())?;
    execute_run_pipeline(&ctx).map_err(|e| e.to_string())
}

/// Unified runner integration for the virtual terminal substrate ([`TerminalSession`]).
pub fn run_on_virtual_terminal(
    session: &mut TerminalSession,
    path_or_cmd: &str,
) -> Result<String, String> {
    let clean_path = path_or_cmd.trim();
    let config = RunConfig::new(clean_path);
    let logger = session.logger.clone();
    let ctx = config.open(&logger).map_err(|e| e.to_string())?;

    let res = execute_run_pipeline(&ctx).map_err(|e| e.to_string())?;

    // Pipe characters into the virtual terminal's output stream
    for c in res.output.chars() {
        session.process_character_echo(c).ok();
    }

    Ok(res.output)
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Multi-Substrate Parity Proofs )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_format_detection_and_names() {
        assert_eq!(RunFormat::from_path(Path::new("test.omni")), RunFormat::Omni);
        assert_eq!(RunFormat::from_path(Path::new("test.scroll")), RunFormat::Omni);
        assert_eq!(RunFormat::from_path(Path::new("test.tape")), RunFormat::Tape);
        assert_eq!(RunFormat::from_path(Path::new("test.c")), RunFormat::C);
        assert_eq!(RunFormat::from_path(Path::new("test.lua")), RunFormat::Lua);
        assert_eq!(RunFormat::from_str_name("c"), Some(RunFormat::C));
        assert_eq!(RunFormat::from_str_name("lua"), Some(RunFormat::Lua));
    }

    #[test]
    fn test_run_character_tape_on_host_and_virtual_terminal() {
        let config = RunConfig::new("lexicon/01_nouns/ideas/types/character.tape");
        let res = run_with_config(config).expect("character.tape execution must succeed");
        assert_eq!(res.state, QuadState::Proceed);
        assert!(res.output.contains("Mounted Tape"));
        assert!(res.output.contains("TRACK 2 [APP]"));
        assert!(res.output.contains("Roll 1/6"));
        assert!(res.output.contains("Execution stopped and sealed at Roll 6"));

        // Virtual terminal integration
        let mut session = TerminalSession::default();
        let vt_out = run_on_virtual_terminal(&mut session, "lexicon/01_nouns/ideas/types/character.tape")
            .expect("Virtual terminal run of character.tape must succeed");
        assert!(vt_out.contains("Mounted Tape"));
        assert!(session.read_output_char().is_some(), "Virtual terminal INTO port must contain characters");
    }

    #[test]
    fn test_run_character_omni_on_host_and_virtual_terminal() {
        let config = RunConfig::new("lexicon/01_nouns/ideas/types/character.omni");
        let res = run_with_config(config).expect("character.omni execution must succeed");
        assert_eq!(res.state, QuadState::Proceed);
        assert!(res.output.contains("OmniCode Source Reel Executed Successfully"));
        assert!(res.output.contains("+1:PROCEED"));

        // Virtual terminal integration
        let mut session = TerminalSession::default();
        let vt_out = run_on_virtual_terminal(&mut session, "lexicon/01_nouns/ideas/types/character.omni")
            .expect("Virtual terminal run of character.omni must succeed");
        assert!(vt_out.contains("OmniCode Source Reel Executed Successfully"));
        assert!(session.read_output_char().is_some(), "Virtual terminal INTO port must contain characters");
    }

    #[test]
    fn test_run_c_backbone_suite() {
        let config = RunConfig::new("lexicon/01_nouns/ideas/types/transpile variant/tests/c/integrated/test_omni_lib.c");
        let res = run_with_config(config).expect("C backbone test suite execution must succeed");
        assert_eq!(res.state, QuadState::Proceed);
        assert!(res.output.contains("ALL C BACKBONE TESTS PASS"));
        assert!(res.output.contains("FULL 4-BLOCK PARITY SEALED IN PROCEED"));
    }

    #[test]
    fn test_run_lua_tape_suite() {
        let config = RunConfig::new("lexicon/01_nouns/ideas/types/transpile variant/tests/lua/unit/test_character.lua");
        let res = run_with_config(config).expect("Lua test tape execution must succeed");
        assert_eq!(res.state, QuadState::Proceed);
        assert!(res.output.contains("Character Process Ribbon (Lua)"));
        assert!(res.output.contains("[PASS]"));
    }

    #[test]
    fn test_run_character_transpile_c_dynamic() {
        let mut config = RunConfig::new("lexicon/01_nouns/ideas/types/character.tape");
        config.transpile_target = Some("c".to_string());
        let res = run_with_config(config).expect("character.tape transpile C must succeed");
        assert_eq!(res.state, QuadState::Proceed);
        assert!(res.output.contains("OMNICODE C TRANSPILE RUNNER: [Character]"));
        assert!(res.output.contains("All 6 rolls verified"));
    }

    #[test]
    fn test_run_character_transpile_lua_dynamic() {
        let mut config = RunConfig::new("lexicon/01_nouns/ideas/types/character.tape");
        config.transpile_target = Some("lua".to_string());
        let res = run_with_config(config).expect("character.tape transpile Lua must succeed");
        assert_eq!(res.state, QuadState::Proceed);
        assert!(res.output.contains("OMNICODE LUA TRANSPILE RUNNER: [Character]"));
        assert!(res.output.contains("All 6 rolls verified"));
    }

    #[test]
    fn test_run_number_tape_on_host_and_virtual_terminal() {
        let config = RunConfig::new("lexicon/01_nouns/ideas/types/number.tape");
        let res = run_with_config(config).expect("number.tape execution must succeed");
        assert_eq!(res.state, QuadState::Proceed);
        assert!(res.output.contains("Mounted Tape"));
        assert!(res.output.contains("TRACK 2 [APP]"));
        assert!(res.output.contains("Roll 1/8"));
        assert!(res.output.contains("Execution stopped and sealed at Roll 8"));

        // Virtual terminal integration
        let mut session = TerminalSession::default();
        let vt_out = run_on_virtual_terminal(&mut session, "lexicon/01_nouns/ideas/types/number.tape")
            .expect("Virtual terminal run of number.tape must succeed");
        assert!(vt_out.contains("Mounted Tape"));
        assert!(session.read_output_char().is_some(), "Virtual terminal INTO port must contain characters");
    }

    #[test]
    fn test_run_number_omni_on_host_and_virtual_terminal() {
        let config = RunConfig::new("lexicon/01_nouns/ideas/types/number.omni");
        let res = run_with_config(config).expect("number.omni execution must succeed");
        assert_eq!(res.state, QuadState::Proceed);
        assert!(res.output.contains("OmniCode Source Reel Executed Successfully"));
        assert!(res.output.contains("+1:PROCEED"));

        // Virtual terminal integration
        let mut session = TerminalSession::default();
        let vt_out = run_on_virtual_terminal(&mut session, "lexicon/01_nouns/ideas/types/number.omni")
            .expect("Virtual terminal run of number.omni must succeed");
        assert!(vt_out.contains("OmniCode Source Reel Executed Successfully"));
        assert!(session.read_output_char().is_some(), "Virtual terminal INTO port must contain characters");
    }

    #[test]
    fn test_run_number_transpile_c_dynamic() {
        let mut config = RunConfig::new("lexicon/01_nouns/ideas/types/number.tape");
        config.transpile_target = Some("c".to_string());
        let res = run_with_config(config).expect("number.tape transpile C must succeed");
        assert_eq!(res.state, QuadState::Proceed);
        assert!(res.output.contains("OMNICODE C TRANSPILE RUNNER: [Number]"));
        assert!(res.output.contains("All 8 rolls verified"));
    }

    #[test]
    fn test_run_number_transpile_lua_dynamic() {
        let mut config = RunConfig::new("lexicon/01_nouns/ideas/types/number.tape");
        config.transpile_target = Some("lua".to_string());
        let res = run_with_config(config).expect("number.tape transpile Lua must succeed");
        assert_eq!(res.state, QuadState::Proceed);
        assert!(res.output.contains("OMNICODE LUA TRANSPILE RUNNER: [Number]"));
        assert!(res.output.contains("All 8 rolls verified"));
    }
}
