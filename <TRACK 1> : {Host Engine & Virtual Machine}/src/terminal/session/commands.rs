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
    // 0. Primordial Call & Response (Echo input to output stream)
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

        match crate::terminal::universal::character_echo::CharacterEchoEngine::echo_stream(payload) {
            Ok(echoed) => {
                return Some(ExecutionResult::success(echoed, start.elapsed().as_micros()));
            }
            Err(e) => {
                let err = Tripwire::trip(
                    logger,
                    "TERMINAL_ECHO",
                    FailpointType::SyntaxAnomaly,
                    format!("Character echo failed: {e}"),
                );
                return Some(ExecutionResult::failure(
                    format!("[!] {err}"),
                    start.elapsed().as_micros(),
                ));
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

    if trimmed == "help" {
        let help = "OMNICODE NATIVE OPERATING SYSTEM SHELL:
  ps, tasks          List scheduled tasks and active processes
  tracks             Display the 3 execution channels (Kernel, Terminal, Apps)
  mem, ram           Display ANSI storehouse RAM meter and partition allocations
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
