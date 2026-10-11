// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Terminal Session State: In-Memory REPL Environment & Command History )
// ==============================================================================
//! OmniCode Terminal Session Engine
//!
//! Maintains interactive state, variable bindings, command history, and
//! dispatches instructions to the Task Scheduler or the OmniCode Runtime Evaluator.
//!
//! # Living Operating System Shell
//! Rather than executing commands through isolated subprocess forks, the OmniCode
//! Terminal Session evaluates rolls directly in persistent silicon memory via
//! [`Evaluator`] and [`Environment`]. It provides direct hardware interaction commands:
//! - `mount <path.tape>`: Mounts a tape ribbon onto an execution track.
//! - `run <path.tape>`: Mounts and advances a tape to completion.
//! - `step <pid>`: Manually advances a Tape Head by 1 roll.
//! - `rolls <pid>`: Displays the Tape Head pointer and all rolls.
//! - `mem`: Displays the ANSI Root_Storehouse utilization meter.
//! - `tracks`: Displays tasks organized across all 3 execution channels.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use super::scheduler::{OmniScheduler, SharedScheduler, TaskPriority, Track};
use crate::core::ternary::QuadState;
use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::runtime::environment::Environment;
use crate::runtime::evaluator::Evaluator;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Session Models, History Entries, and Execution Results )
// ==============================================================================

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub stdout: String,
    pub state: QuadState,
    pub duration_us: u128,
}

impl ExecutionResult {
    pub fn success(stdout: String, duration_us: u128) -> Self {
        Self {
            stdout,
            state: QuadState::Proceed,
            duration_us,
        }
    }

    pub fn await_state(stdout: String, duration_us: u128) -> Self {
        Self {
            stdout,
            state: QuadState::Await,
            duration_us,
        }
    }

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
// ( Terminal Session Implementation & Command Dispatcher )
// ==============================================================================

pub struct TerminalSession {
    pub history: Vec<String>,
    pub variables: HashMap<String, String>,
    pub scheduler: SharedScheduler,
    pub env: Environment,
    pub logger: DiagnosticLogger,
    pub debugger: OmniDebugger,
}

impl Default for TerminalSession {
    fn default() -> Self {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 10);
        let env = Environment::new();
        Self {
            history: Vec::new(),
            variables: HashMap::new(),
            scheduler: Arc::new(Mutex::new(OmniScheduler::new())),
            env,
            logger,
            debugger,
        }
    }
}

impl TerminalSession {
    pub fn new(scheduler: SharedScheduler) -> Self {
        let logger = DiagnosticLogger::new(false);
        let debugger = OmniDebugger::new(logger.clone(), 10);
        let env = Environment::new();
        Self {
            history: Vec::new(),
            variables: HashMap::new(),
            scheduler,
            env,
            logger,
            debugger,
        }
    }

    /// Dispatches a command line and returns an ExecutionResult.
    pub fn execute(&mut self, line: &str) -> ExecutionResult {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return ExecutionResult::success(String::new(), 0);
        }

        self.history.push(trimmed.to_string());
        let start = Instant::now();

        // 1. Process table
        if trimmed == "ps" || trimmed == "tasks" {
            let sched = self.scheduler.lock().unwrap();
            let table = sched.format_table();
            return ExecutionResult::success(table, start.elapsed().as_micros());
        }

        // 2. Multi-track channel summary
        if trimmed == "tracks" || trimmed == "channels" {
            let sched = self.scheduler.lock().unwrap();
            let summary = sched.track_summary();
            return ExecutionResult::success(summary, start.elapsed().as_micros());
        }

        // 3. Storehouse memory meter
        if trimmed == "mem" || trimmed == "ram" {
            let sched = self.scheduler.lock().unwrap();
            let summary = sched.mem_summary();
            return ExecutionResult::success(summary, start.elapsed().as_micros());
        }

        // 4. Mount a tape file onto Track 2 (App) or Track 0 (Kernel)
        if trimmed.starts_with("mount ") {
            let path = trimmed["mount ".len()..].trim();
            let mut sched = self.scheduler.lock().unwrap();
            let track = if path.contains("kernel") || path.contains("boot") {
                Track::Kernel
            } else {
                Track::App
            };
            match sched.mount_tape_file(track, path, self.logger.clone()) {
                Ok(pid) => return ExecutionResult::success(
                    format!("[+] Mounted Tape '{path}' onto {} with PID {pid}", track.as_str()),
                    start.elapsed().as_micros(),
                ),
                Err(e) => return ExecutionResult::failure(
                    format!("[!] Failed to mount tape: {e}"),
                    start.elapsed().as_micros(),
                ),
            }
        }

        // 5. Mount and execute a tape file to completion
        if trimmed.starts_with("run ") && (trimmed.ends_with(".tape") || trimmed.ends_with(".omni")) {
            let path = trimmed["run ".len()..].trim();
            let mut sched = self.scheduler.lock().unwrap();
            let track = if path.contains("kernel") || path.contains("boot") {
                Track::Kernel
            } else {
                Track::App
            };
            match sched.mount_tape_file(track, path, self.logger.clone()) {
                Ok(pid) => {
                    let mut logs = Vec::new();
                    logs.push(format!("[+] Mounted Tape '{path}' onto {} (PID {pid})", track.as_str()));
                    loop {
                        match sched.step_task(pid) {
                            Ok(msg) => {
                                logs.push(msg.clone());
                                if msg.contains("completed") || msg.contains("already executed") {
                                    break;
                                }
                            }
                            Err(e) => {
                                logs.push(format!("[!] Error: {e}"));
                                break;
                            }
                        }
                    }
                    return ExecutionResult::success(logs.join("\n"), start.elapsed().as_micros());
                }
                Err(e) => return ExecutionResult::failure(
                    format!("[!] Failed to run tape: {e}"),
                    start.elapsed().as_micros(),
                ),
            }
        }

        // 6. Step Tape Head on a task
        if trimmed.starts_with("step ") {
            let pid_str = trimmed["step ".len()..].trim();
            if let Ok(pid) = pid_str.parse::<u64>() {
                let mut sched = self.scheduler.lock().unwrap();
                match sched.step_task(pid) {
                    Ok(msg) => return ExecutionResult::success(msg, start.elapsed().as_micros()),
                    Err(e) => return ExecutionResult::failure(format!("[!] Error: {e}"), start.elapsed().as_micros()),
                }
            } else {
                return ExecutionResult::failure("Usage: step <pid>".to_string(), start.elapsed().as_micros());
            }
        }

        // 7. Display rolls on a task
        if trimmed.starts_with("rolls ") {
            let pid_str = trimmed["rolls ".len()..].trim();
            if let Ok(pid) = pid_str.parse::<u64>() {
                let sched = self.scheduler.lock().unwrap();
                match sched.rolls_for_task(pid) {
                    Some(rolls_str) => return ExecutionResult::success(rolls_str, start.elapsed().as_micros()),
                    None => return ExecutionResult::failure(format!("[!] Task {pid} not found or has no mounted tape"), start.elapsed().as_micros()),
                }
            } else {
                return ExecutionResult::failure("Usage: rolls <pid>".to_string(), start.elapsed().as_micros());
            }
        }

        // 8. General task spawning
        if trimmed.starts_with("spawn ") {
            let cmd = trimmed["spawn ".len()..].trim();
            let mut sched = self.scheduler.lock().unwrap();
            let pid = sched.spawn("UserTask", cmd, TaskPriority::Normal);
            return ExecutionResult::success(
                format!("[+] Spawned process PID {pid}: '{cmd}' with state +1:PROCEED"),
                start.elapsed().as_micros(),
            );
        }

        // 9. Task controls
        if trimmed.starts_with("pause ") || trimmed.starts_with("await ") {
            let pid_str = trimmed.split_whitespace().nth(1).unwrap_or("");
            if let Ok(pid) = pid_str.parse::<u64>() {
                let mut sched = self.scheduler.lock().unwrap();
                match sched.pause_task(pid) {
                    Ok(()) => return ExecutionResult::await_state(format!("[~] Task {pid} transitioned to 0:AWAIT"), start.elapsed().as_micros()),
                    Err(e) => return ExecutionResult::failure(format!("[!] Error: {e}"), start.elapsed().as_micros()),
                }
            }
        }

        if trimmed.starts_with("resume ") {
            let pid_str = trimmed.split_whitespace().nth(1).unwrap_or("");
            if let Ok(pid) = pid_str.parse::<u64>() {
                let mut sched = self.scheduler.lock().unwrap();
                match sched.resume_task(pid) {
                    Ok(()) => return ExecutionResult::success(format!("[+] Task {pid} resumed to +1:PROCEED"), start.elapsed().as_micros()),
                    Err(e) => return ExecutionResult::failure(format!("[!] Error: {e}"), start.elapsed().as_micros()),
                }
            }
        }

        if trimmed.starts_with("kill ") || trimmed.starts_with("halt ") {
            let pid_str = trimmed.split_whitespace().nth(1).unwrap_or("");
            if let Ok(pid) = pid_str.parse::<u64>() {
                let mut sched = self.scheduler.lock().unwrap();
                match sched.halt_task(pid) {
                    Ok(()) => return ExecutionResult::success(format!("[!] Task {pid} halted (NULL:HALT)"), start.elapsed().as_micros()),
                    Err(e) => return ExecutionResult::failure(format!("[!] Error: {e}"), start.elapsed().as_micros()),
                }
            }
        }

        if trimmed == "tick" {
            let mut sched = self.scheduler.lock().unwrap();
            let count = sched.tick();
            return ExecutionResult::success(format!("[*] Advanced scheduler clock 1 tick. Executed {count} active tasks."), start.elapsed().as_micros());
        }

        if trimmed == "vars" || trimmed == "env" {
            let mut out = String::from("ACTIVE SESSION STATE:\n");
            let mut has_entries = false;
            for (k, v) in self.env.bindings() {
                out.push_str(&format!("  {} = {:?}\n", k, v));
                has_entries = true;
            }
            for (k, v) in &self.variables {
                out.push_str(&format!("  let {} = {}\n", k, v));
                has_entries = true;
            }
            if !has_entries {
                return ExecutionResult::success("No active session bindings declared.".to_string(), start.elapsed().as_micros());
            }
            return ExecutionResult::success(out, start.elapsed().as_micros());
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
            return ExecutionResult::success(help.to_string(), start.elapsed().as_micros());
        }

        // Variable assignment: let x = 42
        if trimmed.starts_with("let ") && trimmed.contains('=') {
            let parts: Vec<&str> = trimmed["let ".len()..].splitn(2, '=').collect();
            if parts.len() == 2 {
                let name = parts[0].trim().to_string();
                let val = parts[1].trim().to_string();
                self.variables.insert(name.clone(), val.clone());
                return ExecutionResult::success(format!("Variable '{name}' bound to: {val}"), start.elapsed().as_micros());
            }
        }

        // 10. Direct In-Memory Roll Evaluation (OmniCode Native Execution)
        // If the statement looks like OmniCode syntax, execute directly without subprocess overhead!
        if trimmed.contains("-->") || trimmed.contains("<--") || trimmed.contains("v--") || trimmed.contains("--^")
            || trimmed.starts_with("Storehouse") || (trimmed.contains('[') && trimmed.contains(']'))
            || (trimmed.contains('<') && trimmed.contains('>'))
            || trimmed.starts_with("status")
        {
            let mut lexer = Lexer::new(trimmed, self.logger.clone());
            if let Ok(tokens) = lexer.tokenize() {
                let mut parser = Parser::new(tokens, self.debugger.clone());
                if let Ok(prog) = parser.parse_program() {
                    let mut evaluator = Evaluator::new(self.env.clone(), self.debugger.clone());
                    match evaluator.eval(&crate::ast::nodes::ASTNode::Program(prog)) {
                        Ok(val) => {
                            self.env = evaluator.env; // Persist environment state!
                            return ExecutionResult::success(
                                format!("=> {:?} (state: +1:PROCEED)", val),
                                start.elapsed().as_micros(),
                            );
                        }
                        Err(e) => {
                            return ExecutionResult::failure(
                                format!("Execution Failpoint: {e}"),
                                start.elapsed().as_micros(),
                            );
                        }
                    }
                }
            }
        }

        // 11. Fallback: CLI Subcommand Execution
        let clean_cmd = if trimmed.starts_with("omnicode ") {
            trimmed.to_string()
        } else {
            format!("omnicode {trimmed}")
        };

        let exe = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("omnicode"));
        let args: Vec<&str> = clean_cmd["omnicode ".len()..].split_whitespace().collect();
        let output = std::process::Command::new(&exe)
            .args(&args)
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                let combined = if !stderr.is_empty() {
                    format!("{stdout}\n{stderr}")
                } else {
                    stdout
                };
                if out.status.success() {
                    ExecutionResult::success(combined, start.elapsed().as_micros())
                } else {
                    ExecutionResult::failure(combined, start.elapsed().as_micros())
                }
            }
            Err(e) => ExecutionResult::failure(format!("Failed to execute command: {e}"), start.elapsed().as_micros()),
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Terminal Session Sealing & Verification )
// ==============================================================================
// Terminal session module sealed.
