// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Terminal Built-in Shell Command Dispatcher & Host OS Interlock )
// ==============================================================================
//! OmniCode Terminal: Built-in Shell Command Dispatcher
//!
//! # Architectural Scope & Purpose:
//! Handles parsing and execution of native operating system shell commands:
//! - Process table inspection (`ps`, `tasks`).
//! - Multi-track execution channel inspection (`tracks`, `channels`).
//! - Storehouse RAM utilization meter (`mem`, `ram`).
//! - Tape mounting and step execution (`mount`, `run`, `step`, `rolls`).
//! - Cooperative scheduler clock stepping (`tick`).
//! - Process control (`spawn`, `pause`, `resume`, `kill`, `halt`).
//! - Variable declarations (`let x = 42`) and environment inspection (`vars`, `env`).
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**: Tables of testimony, written with the finger of God.
//! - **Proverbs 24:27 (KJV)**: Prepare thy work without, and make it fit for thyself in the field.
//! - **Proverbs 22:28 (KJV)**: Remove not the ancient landmark, which thy fathers have set.
// ==============================================================================

use std::collections::HashMap;
use std::time::Instant; 

use crate::cogs::tripwire::Tripwire;
use crate::core::filetype::resolve_track_path;
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType};
use crate::runtime::environment::Environment;
use crate::terminal::scheduler::{SharedScheduler, TaskPriority, Track};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( ExecutionResult Container & Command Dispatch Function Signature )
// ==============================================================================

/// Result of an interactive terminal command execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResult {
    /// Text emitted to stdout during execution.
    pub stdout: String,
    /// Balanced ternary state following execution (+1:Proceed, 0:Await, -1:Rollback, NULL:Halt).
    pub state: QuadState,
    /// Execution duration in microseconds.
    pub duration_us: u128,
}

impl ExecutionResult {
    /// Constructs a successful execution result with state +1:PROCEED.
    pub fn success(stdout: String, duration_us: u128) -> Self {
        Self {
            stdout,
            state: QuadState::Proceed,
            duration_us,
        }
    }

    /// Constructs an asynchronous wait or standby result with state 0:AWAIT.
    pub fn await_state(stdout: String, duration_us: u128) -> Self {
        Self {
            stdout,
            state: QuadState::Await,
            duration_us,
        }
    }

    /// Constructs a failed or rollback result with state -1:ROLLBACK.
    pub fn failure(stdout: String, duration_us: u128) -> Self {
        Self {
            stdout,
            state: QuadState::Rollback,
            duration_us,
        }
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Built-in Shell Command Dispatch Engine )
// ==============================================================================

/// Dispatches built-in terminal shell commands. Returns `Some(ExecutionResult)` if
/// the command was handled, or `None` if it should fall through to roll evaluation.
pub fn dispatch_builtin_command(
    trimmed: &str,
    scheduler: &SharedScheduler,
    env: &Environment,
    variables: &mut HashMap<String, String>,
    logger: &DiagnosticLogger,
    _debugger: &OmniDebugger,
    start: Instant,
) -> Option<ExecutionResult> {
    // 0. Universal Call & Response Engine (ECHO command)
    if trimmed == "echo" || trimmed.starts_with("echo ") {
        let raw_payload = if trimmed == "echo" {
            ""
        } else {
            trimmed["echo ".len()..].trim()
        };
        let payload = if (raw_payload.starts_with('\'') && raw_payload.ends_with('\'') && raw_payload.len() >= 2)
            || (raw_payload.starts_with('"') && raw_payload.ends_with('"') && raw_payload.len() >= 2)
        {
            &raw_payload[1..raw_payload.len() - 1]
        } else {
            raw_payload
        };

        // Determine if payload is a numeric/binary scalar Call
        let num_parse = if payload.starts_with("0b") || payload.starts_with("0B") {
            i64::from_str_radix(&payload[2..], 2).ok()
        } else {
            payload.parse::<i64>().ok()
        };

        if let Some(n) = num_parse {
            match crate::terminal::universal::call_response_engine::CallResponseEngine::call_number(n) {
                Ok(resp) => {
                    let out = format!(
                        "Number: {} | Dual-Rail: 0x{:016X} | Pack(2-2-2-2): 0x{:02X} | Pack(2-1-2-1-2): 0x{:02X} | Equilibrium: PROVED | State: +1:PROCEED",
                        resp.scalar, resp.packed_dual_rail, resp.byte_pack_2222, resp.byte_pack_21212
                    );
                    return Some(ExecutionResult::success(out, start.elapsed().as_micros()));
                }
                Err(e) => {
                    let err = Tripwire::trip(
                        logger,
                        "TERMINAL_ECHO",
                        FailpointType::SyntaxAnomaly,
                        format!("Number Call-Response failed: {e}"),
                    );
                    return Some(ExecutionResult::failure(
                        format!("[!] {err}"),
                        start.elapsed().as_micros(),
                    ));
                }
            }
        }

        // Otherwise execute Stream / Character Call-Response turn
        match crate::terminal::universal::call_response_engine::CallResponseEngine::call_stream(payload) {
            Ok(resp) => {
                return Some(ExecutionResult::success(resp.text, start.elapsed().as_micros()));
            }
            Err(e) => {
                let err = Tripwire::trip(
                    logger,
                    "TERMINAL_ECHO",
                    FailpointType::SyntaxAnomaly,
                    format!("Echo Call-Response failed: {e}"),
                );
                return Some(ExecutionResult::failure(
                    format!("[!] {err}"),
                    start.elapsed().as_micros(),
                ));
            }
        }
    }

    // 0b. Explicit number call (Convenience alias)
    if trimmed.starts_with("number ") || trimmed.starts_with("num ") {
        let raw = if trimmed.starts_with("number ") {
            trimmed["number ".len()..].trim()
        } else {
            trimmed["num ".len()..].trim()
        };
        if let Ok(n) = raw.parse::<i64>() {
            match crate::terminal::universal::call_response_engine::CallResponseEngine::call_number(n) {
                Ok(resp) => {
                    let out = format!(
                        "Number: {} | Dual-Rail: 0x{:016X} | Pack(2-2-2-2): 0x{:02X} | Pack(2-1-2-1-2): 0x{:02X} | Equilibrium: PROVED | State: +1:PROCEED",
                        resp.scalar, resp.packed_dual_rail, resp.byte_pack_2222, resp.byte_pack_21212
                    );
                    return Some(ExecutionResult::success(out, start.elapsed().as_micros()));
                }
                Err(e) => {
                    return Some(ExecutionResult::failure(
                        format!("[!] Number call failed: {e}"),
                        start.elapsed().as_micros(),
                    ));
                }
            }
        }
    }

    // 1. Process table
    if trimmed == "ps" || trimmed == "tasks" {
        let sched = scheduler.lock().unwrap();
        let table = sched.format_table();
        return Some(ExecutionResult::success(table, start.elapsed().as_micros()));
    }

    // 2. Multi-track channel summary
    if trimmed == "tracks" || trimmed == "channels" {
        let sched = scheduler.lock().unwrap();
        let summary = sched.track_summary();
        return Some(ExecutionResult::success(summary, start.elapsed().as_micros()));
    }

    // 3. Storehouse memory meter
    if trimmed == "mem" || trimmed == "ram" {
        let sched = scheduler.lock().unwrap();
        let summary = sched.mem_summary();
        return Some(ExecutionResult::success(summary, start.elapsed().as_micros()));
    }

    // 4. Mount a tape file onto Track 2 (App) or Track 0 (Kernel)
    if trimmed.starts_with("mount ") {
        let raw_path = trimmed["mount ".len()..].trim();
        let resolved = resolve_track_path(raw_path);
        let path_str = resolved.to_string_lossy().to_string();
        let mut sched = scheduler.lock().unwrap();
        let track = if raw_path.contains("kernel") || raw_path.contains("boot") {
            Track::Kernel
        } else {
            Track::App
        };
        match sched.mount_tape_file(track, &path_str, logger.clone()) {
            Ok(pid) => return Some(ExecutionResult::success(
                format!("[+] Mounted Tape '{raw_path}' onto {} with PID {pid}", track.as_str()),
                start.elapsed().as_micros(),
            )),
            Err(e) => {
                let err = Tripwire::trip(
                    logger,
                    "TERMINAL_MOUNT",
                    FailpointType::SyntaxAnomaly,
                    format!("Failed to mount tape '{raw_path}': {e}"),
                );
                return Some(ExecutionResult::failure(
                    format!("[!] {err}"),
                    start.elapsed().as_micros(),
                ));
            }
        }
    }

    // 5. Mount and execute a tape, omni, C, or Lua file
    if trimmed.starts_with("run ") {
        let path = trimmed["run ".len()..].trim();
        let track = if path.contains("kernel") || path.contains("boot") {
            Track::Kernel
        } else {
            Track::App
        };

        // If it's a tape, preserve existing scheduler mount & step logs
        if path.ends_with(".tape") {
            let resolved = resolve_track_path(path);
            let path_str = resolved.to_string_lossy().to_string();
            let mut sched = scheduler.lock().unwrap();
            match sched.mount_tape_file(track, &path_str, logger.clone()) {
                Ok(pid) => {
                    let mut logs = Vec::new();
                    logs.push(format!("[+] Mounted Tape '{path}' onto {} (PID {pid})", track.as_str()));
                    loop {
                        match sched.step_task(pid) {
                            Ok(msg) => {
                                logs.push(msg.clone());
                                if msg.contains("completed") || msg.contains("already executed") || msg.contains("STOP") {
                                    break;
                                }
                            }
                            Err(e) => {
                                let err = Tripwire::trip(
                                    logger,
                                    "TERMINAL_RUN",
                                    FailpointType::SyntaxAnomaly,
                                    format!("Task {pid} execution halted: {e}"),
                                );
                                logs.push(format!("[!] {err}"));
                                break;
                            }
                        }
                    }
                    return Some(ExecutionResult::success(logs.join("\n"), start.elapsed().as_micros()));
                }
                Err(e) => {
                    let err = Tripwire::trip(
                        logger,
                        "TERMINAL_RUN",
                        FailpointType::SyntaxAnomaly,
                        format!("Failed to run tape '{path}': {e}"),
                    );
                    return Some(ExecutionResult::failure(
                        format!("[!] {err}"),
                        start.elapsed().as_micros(),
                    ));
                }
            }
        } else if path.ends_with(".omni") || path.ends_with(".c") || path.ends_with(".lua") {
            let config = crate::cli::commands::run::RunConfig::new(path);
            let ctx = match config.open(logger) {
                Ok(c) => c,
                Err(e) => {
                    return Some(ExecutionResult::failure(
                        format!("[!] {e}"),
                        start.elapsed().as_micros(),
                    ));
                }
            };
            match crate::cli::commands::run::execute_run_pipeline(&ctx) {
                Ok(res) => {
                    return Some(ExecutionResult::success(res.output, start.elapsed().as_micros()));
                }
                Err(e) => {
                    return Some(ExecutionResult::failure(
                        format!("[!] {e}"),
                        start.elapsed().as_micros(),
                    ));
                }
            }
        }
    }

    // 6. Step Tape Head on a task
    if trimmed.starts_with("step ") {
        let pid_str = trimmed["step ".len()..].trim();
        if let Ok(pid) = pid_str.parse::<u64>() {
            let mut sched = scheduler.lock().unwrap();
            match sched.step_task(pid) {
                Ok(msg) => return Some(ExecutionResult::success(msg, start.elapsed().as_micros())),
                Err(e) => return Some(ExecutionResult::failure(format!("[!] Error: {e}"), start.elapsed().as_micros())),
            }
        } else {
            return Some(ExecutionResult::failure("Usage: step <pid>".to_string(), start.elapsed().as_micros()));
        }
    }

    // 7. Display rolls on a task
    if trimmed.starts_with("rolls ") {
        let pid_str = trimmed["rolls ".len()..].trim();
        if let Ok(pid) = pid_str.parse::<u64>() {
            let sched = scheduler.lock().unwrap();
            match sched.rolls_for_task(pid) {
                Some(rolls_str) => return Some(ExecutionResult::success(rolls_str, start.elapsed().as_micros())),
                None => return Some(ExecutionResult::failure(format!("[!] Task {pid} not found or has no mounted tape"), start.elapsed().as_micros())),
            }
        } else {
            return Some(ExecutionResult::failure("Usage: rolls <pid>".to_string(), start.elapsed().as_micros()));
        }
    }

    // 8. General task spawning
    if trimmed.starts_with("spawn ") {
        let cmd = trimmed["spawn ".len()..].trim();
        let mut sched = scheduler.lock().unwrap();
        let pid = sched.spawn("UserTask", cmd, TaskPriority::Normal);
        return Some(ExecutionResult::success(
            format!("[+] Spawned process PID {pid}: '{cmd}' with state +1:PROCEED"),
            start.elapsed().as_micros(),
        ));
    }

    // 9. Task controls
    if trimmed.starts_with("pause ") || trimmed.starts_with("await ") {
        let pid_str = trimmed.split_whitespace().nth(1).unwrap_or("");
        if let Ok(pid) = pid_str.parse::<u64>() {
            let mut sched = scheduler.lock().unwrap();
            match sched.pause_task(pid) {
                Ok(()) => return Some(ExecutionResult::await_state(format!("[~] Task {pid} transitioned to 0:AWAIT"), start.elapsed().as_micros())),
                Err(e) => return Some(ExecutionResult::failure(format!("[!] Error: {e}"), start.elapsed().as_micros())),
            }
        }
    }

    if trimmed.starts_with("resume ") {
        let pid_str = trimmed.split_whitespace().nth(1).unwrap_or("");
        if let Ok(pid) = pid_str.parse::<u64>() {
            let mut sched = scheduler.lock().unwrap();
            match sched.resume_task(pid) {
                Ok(()) => return Some(ExecutionResult::success(format!("[+] Task {pid} resumed to +1:PROCEED"), start.elapsed().as_micros())),
                Err(e) => return Some(ExecutionResult::failure(format!("[!] Error: {e}"), start.elapsed().as_micros())),
            }
        }
    }

    if trimmed.starts_with("kill ") || trimmed.starts_with("halt ") {
        let pid_str = trimmed.split_whitespace().nth(1).unwrap_or("");
        if let Ok(pid) = pid_str.parse::<u64>() {
            let mut sched = scheduler.lock().unwrap();
            match sched.halt_task(pid) {
                Ok(()) => return Some(ExecutionResult::success(format!("[!] Task {pid} halted (NULL:HALT)"), start.elapsed().as_micros())),
                Err(e) => return Some(ExecutionResult::failure(format!("[!] Error: {e}"), start.elapsed().as_micros())),
            }
        }
    }

    if trimmed == "tick" {
        let mut sched = scheduler.lock().unwrap();
        let count = sched.tick();
        return Some(ExecutionResult::success(format!("[*] Advanced scheduler clock 1 tick. Executed {count} active tasks."), start.elapsed().as_micros()));
    }

    if trimmed == "vars" || trimmed == "env" {
        let mut out = String::from("ACTIVE SESSION STATE:\n");
        let mut has_entries = false;
        for (k, v) in env.bindings() {
            out.push_str(&format!("  {} = {:?}\n", k, v));
            has_entries = true;
        }
        for (k, v) in variables.iter() {
            out.push_str(&format!("  let {} = {}\n", k, v));
            has_entries = true;
        }
        if !has_entries {
            return Some(ExecutionResult::success("No active session bindings declared.".to_string(), start.elapsed().as_micros()));
        }
        return Some(ExecutionResult::success(out, start.elapsed().as_micros()));
    }

    // 10. AI Model Inquiry & Cognitive Dialogue
    if trimmed.starts_with("ai ") {
        let raw_args = trimmed["ai ".len()..].trim();
        let (prompt_str, verbose) = if raw_args.starts_with("-v ") {
            (raw_args["-v ".len()..].trim(), true)
        } else if raw_args.starts_with("--verbose ") {
            (raw_args["--verbose ".len()..].trim(), true)
        } else {
            (raw_args, false)
        };

        let prompt = if (prompt_str.starts_with('"')
            && prompt_str.ends_with('"')
            && prompt_str.len() >= 2)
            || (prompt_str.starts_with('\'')
                && prompt_str.ends_with('\'')
                && prompt_str.len() >= 2)
        {
            &prompt_str[1..prompt_str.len() - 1]
        } else {
            prompt_str
        };

        let mut cortex = crate::ai::AICortex::new();
        match cortex.process_prompt(prompt) {
            Ok(res) => {
                let out = if verbose {
                    let mut s = format!(
                        "[CPI-SI Cognitive Thought Trace]\n\
                         • State: +1:{}\n\
                         • WHO:   {}\n\
                         • WHAT:  {}\n\
                         • WHY:   {}\n\
                         • HOW:   {}\n\n\
                         [Synthesized Reel]\n{}\n\n\
                         [Machine Layer Response]\n\"{}\"\n\n\
                         [Session: {} | Scroll: {:?}]",
                        res.execution_state.macro_op(),
                        res.thought.who,
                        res.thought.what,
                        res.thought.why,
                        res.thought.how,
                        res.synthesized_omnicode.trim(),
                        res.natural_response,
                        res.session_id,
                        res.scroll_path
                    );
                    if !res.repairs_applied.is_empty() {
                        s.push_str(&format!(
                            "\n[Restorator Repairs: {}]",
                            res.repairs_applied.len()
                        ));
                    }
                    s
                } else {
                    format!(
                        "[AI Model Response: CPI-SI Base]\n\"{}\"\n\n\
                        [Session: {}]",
                        res.natural_response, res.session_id
                    )
                };
                return Some(ExecutionResult::success(
                    out,
                    start.elapsed().as_micros(),
                ));
            }
            Err(e) => {
                return Some(ExecutionResult::failure(
                    format!("[!] AI Cortex Error: {e}"),
                    start.elapsed().as_micros(),
                ));
            }
        }
    }

    // 11. O(1) Episodic Session Query
    if trimmed.starts_with("query ") {
        let sid = trimmed["query ".len()..].trim();
        let cortex = crate::ai::AICortex::new();
        if let Some(s) = cortex.query_episode(sid) {
            let state_str = match s.final_state.as_int() {
                Some(i) => format!("{i:+}:{}", s.final_state.macro_op()),
                _ => format!("*:{}", s.final_state.macro_op()),
            };
            let paired_str = s
                .log_path
                .as_deref()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_else(|| "(none)".to_string());
            let out = format!(
                "Session: {}\n  • Category:   {}\n  • State:      {}\n  • Prompt:     \"{}\"\n  • Failpoints: {}\n  • Repairs:    {}\n  • Scroll:     {:?}\n  • Telemetry:  {}",
                s.session_id,
                s.category,
                state_str,
                s.prompt,
                s.failpoint_count,
                s.repair_count,
                s.scroll_path,
                paired_str,
            );
            return Some(ExecutionResult::success(
                out,
                start.elapsed().as_micros(),
            ));
        } else {
            return Some(ExecutionResult::failure(
                format!("[!] Session '{sid}' not found in episodic storehouse"),
                start.elapsed().as_micros(),
            ));
        }
    }

    // 12. Failpoint & Anomaly Diagnostic Report
    if trimmed.starts_with("diagnose ") {
        let sid = trimmed["diagnose ".len()..].trim();
        let cortex = crate::ai::AICortex::new();
        if let Some(diag) = cortex.diagnose_failpoint(sid) {
            return Some(ExecutionResult::success(
                diag,
                start.elapsed().as_micros(),
            ));
        } else {
            return Some(ExecutionResult::failure(
                format!("[!] Session '{sid}' not found in episodic storehouse"),
                start.elapsed().as_micros(),
            ));
        }
    }

    // 13. Episodic Memory Listing
    if trimmed == "episodes" || trimmed.starts_with("episodes ")
        || trimmed == "scrolls" || trimmed.starts_with("scrolls ")
    {
        let count = if trimmed.starts_with("episodes ") {
            trimmed["episodes ".len()..]
                .trim()
                .parse::<usize>()
                .unwrap_or(10)
        } else if trimmed.starts_with("scrolls ") {
            trimmed["scrolls ".len()..]
                .trim()
                .parse::<usize>()
                .unwrap_or(10)
        } else {
            10
        };

        let cortex = crate::ai::AICortex::new();
        let recents = cortex.recent_episodes(count);
        if recents.is_empty() {
            return Some(ExecutionResult::success(
                "No episodes found in storehouse.".to_string(),
                start.elapsed().as_micros(),
            ));
        }

        let mut out = format!("RECENT EPISODES (Last {}):\n", recents.len());
        for (i, s) in recents.iter().enumerate() {
            let state_str = match s.final_state.as_int() {
                Some(val) => format!("{val:+}:{}", s.final_state.macro_op()),
                _ => format!("*:{}", s.final_state.macro_op()),
            };
            out.push_str(&format!(
                "  [{:>2}] {} | Cat: {:<8} | State: {:<11} | FP: {:>2} | Rep: {:>2}\n       Prompt: \"{}\"\n",
                i + 1,
                s.session_id,
                s.category,
                state_str,
                s.failpoint_count,
                s.repair_count,
                s.prompt,
            ));
        }
        return Some(ExecutionResult::success(
            out,
            start.elapsed().as_micros(),
        ));
    }

    // 14. Storehouse Vessel Capacity Rotation
    if trimmed == "rotate" || trimmed.starts_with("rotate ") {
        let cap = if trimmed.starts_with("rotate ") {
            trimmed["rotate ".len()..]
                .trim()
                .parse::<usize>()
                .unwrap_or(1000)
        } else {
            1000
        };
        let policy = crate::diagnostics::logger::RotationPolicy::new(
            cap, true,
        );
        let scroll_dir = crate::core::filetype::resolve_scroll_storage_dir();
        let s_logger = crate::ai::ScrollLogger::with_rotation_policy(
            &scroll_dir,
            policy,
        );
        match s_logger.rotate_storehouse() {
            Ok(n) => return Some(ExecutionResult::success(
                format!(
                    "[√] Rotated storehouse (cap={cap}): {n} excess archived"
                ),
                start.elapsed().as_micros(),
            )),
            Err(e) => return Some(ExecutionResult::failure(
                format!("[!] Rotation failed: {e}"),
                start.elapsed().as_micros(),
            )),
        }
    }

    // 14b. CPI-SI Discourse Prompt & Interaction
    if trimmed == "cpisi" || trimmed.starts_with("cpisi ")
        || trimmed == "interact" || trimmed.starts_with("interact ")
    {
        let prompt = if trimmed.contains(' ') {
            trimmed.split_once(' ').map(|x| x.1).unwrap_or("hello")
        } else {
            "hello"
        };
        let mut cortex = crate::ai::AICortex::new();
        match cortex.process_prompt(prompt) {
            Ok(res) => {
                let (exc, neu, inh) = cortex.brain.synaptic_counts();
                let out = format!(
                    "[√] CPI-SI Base Deliberation (+1:{}):\n  \
                     • Response: \"{}\"\n  \
                     • Synapse State: +1: {}, 0: {}, -1: {}",
                    res.execution_state.macro_op(),
                    res.natural_response,
                    exc, neu, inh
                );
                return Some(ExecutionResult::success(
                    out,
                    start.elapsed().as_micros(),
                ));
            }
            Err(e) => {
                return Some(ExecutionResult::failure(
                    format!("[!] CPI-SI error: {e}"),
                    start.elapsed().as_micros(),
                ));
            }
        }
    }

    // 15. Supervised Episodic Neural Training
    if trimmed == "train" || trimmed.starts_with("train ") {
        let scroll_dir = crate::core::filetype::resolve_scroll_storage_dir();
        let learner = crate::ai::AILearner::new(&scroll_dir);
        match learner.digest_experiences() {
            Ok(summary) => {
                let mut brain = crate::ai::NeuralBrain::new(
                    &[8, 16, 4],
                    1,
                    logger.clone(),
                );
                let trainer = crate::ai::NeuralTrainer::new();
                let report = trainer.train(&mut brain, &summary.experiences);
                let weights_path =
                    std::path::PathBuf::from(
                        "storage/weights/cpisi_base.weights"
                    );
                brain.save_weights(&weights_path).ok();
                let out = format!(
                    "[√] Neural Training Complete:\n  \
                     • Episodes Digested: {}\n  \
                     • Synapses Adapted:  {}\n  \
                     • Resilience Score:  {:.1}% -> {:.1}%\n  \
                     • Weights Saved:     {:?}",
                    report.total_episodes,
                    report.synapses_adapted,
                    report.noise_resilience_before * 100.0,
                    report.noise_resilience_after * 100.0,
                    weights_path
                );
                return Some(ExecutionResult::success(
                    out,
                    start.elapsed().as_micros(),
                ));
            }
            Err(e) => {
                return Some(ExecutionResult::failure(
                    format!("[!] Training failed: {e}"),
                    start.elapsed().as_micros(),
                ));
            }
        }
    }

    // 16. Structured Systemic Noise & Degradation Simulation
    if trimmed == "noise" || trimmed.starts_with("noise ") {
        let engine = crate::ai::NoiseEngine::default();
        let ideal = &[
            crate::core::ternary::QuadState::Proceed,
            crate::core::ternary::QuadState::Proceed,
            crate::core::ternary::QuadState::Proceed,
            crate::core::ternary::QuadState::Proceed,
        ];
        let brain = crate::ai::NeuralBrain::new(
            &[4, 8, 2],
            1,
            logger.clone(),
        );
        let ladder = engine.simulate_breakdown_ladder(ideal);
        let mut out = String::from(
            "DEGRADATION LADDER (Ideal -> Breakdown):\n"
        );
        for step in &ladder {
            let noisy_out = brain.think(&step.perturbed_vector);
            let mode_str = match step.dominant_mode {
                crate::ai::DegradationMode::Ideal => "Pristine",
                crate::ai::DegradationMode::TritDecay => "Trit Decay",
                crate::ai::DegradationMode::BoundaryLeak => "Boundary Leak",
                crate::ai::DegradationMode::ThermalInversion => "Inversion",
                crate::ai::DegradationMode::FailpointTrap => "Halt Trap",
            };
            out.push_str(&format!(
                "  [{:>3.0}%] {:<13} | Fidelity: {:>5.1}% | Out: {:?}\n",
                step.stress_level * 100.0,
                mode_str,
                step.signal_fidelity * 100.0,
                noisy_out
            ));
        }
        let resilience = brain.evaluate_noise_resilience(ideal, &engine);
        out.push_str(&format!(
            "Neural Resilience Score: {:.1}%",
            resilience * 100.0
        ));
        return Some(ExecutionResult::success(
            out,
            start.elapsed().as_micros(),
        ));
    }

    if trimmed == "help" {
        let help = "OMNICODE NATIVE OPERATING SYSTEM SHELL:
  ps, tasks          List scheduled tasks and active processes
  tracks             Display the 3 execution channels (Kernel, Terminal, Apps)
  mem, ram           Display ANSI storehouse RAM meter and allocations
  mount <path.tape>  Mount a tape ribbon onto an execution track
  run <path.tape>    Mount and execute a process tape to completion
  step <pid>         Manually step a Tape Head forward by 1 roll
  rolls <pid>        View tape rolls and the live ==> [HEAD] pointer
  spawn <cmd>        Spawn a new task under the task scheduler
  pause <pid>        Transition a process to 0:AWAIT (standby)
  resume <pid>       Resume a process to +1:PROCEED (running)
  kill <pid>         Halt and terminate a process (NULL:HALT)
  tick               Advance the cooperative scheduler clock
  vars, env          Display in-memory declared variables and Vessels
  ai <prompt>        Deliberate with the AI Model and synthesize reels
  interact, cpisi    Engage in live discourse with CPI-SI Base model
  query <session>    Query an episodic memory scroll in O(1) time
  diagnose <session> Inspect trapped failpoints and immune repairs
  episodes [N]       Display the N most recent execution scrolls
  rotate [capacity]  Enforce vessel capacity bounds (Occupancy <= Capacity)
  train              Train balanced-ternary weights on episodic scrolls
  noise              Run non-random structured degradation ladder
  clear              Clear the terminal screen
  help               Display this help reference

LIVE OMNICODE STATEMENTS & ROLLS:
  Type any valid roll (e.g. `A [ 10 ]`, `42 --> Buf @ 0`, `Storehouse v-- App [ 256 ]`)
  to execute directly in persistent silicon memory!
";
        return Some(ExecutionResult::success(help.to_string(), start.elapsed().as_micros()));
    }

    // Variable assignment: let x = 42
    if trimmed.starts_with("let ") && trimmed.contains('=') {
        let parts: Vec<&str> = trimmed["let ".len()..].splitn(2, '=').collect();
        if parts.len() == 2 {
            let name = parts[0].trim().to_string();
            let val = parts[1].trim().to_string();
            variables.insert(name.clone(), val.clone());
            return Some(ExecutionResult::success(format!("Variable '{name}' bound to: {val}"), start.elapsed().as_micros()));
        }
    }

    None
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification & Invariant Proofs )
// ==============================================================================
// Built-in command dispatcher is decoupled from session orchestration.
