// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode Terminal Task Scheduler: Cooperative Quad-State Process Dispatcher )
// ==============================================================================
//! OmniCode Terminal Task Scheduler
//!
//! Provides the primordial process and task scheduling engine for the OmniCode
//! Native Terminal and OS userspace shell.
//!
//! Features:
//! - Quad-State task lifecycles grounded in Balanced Radix-3 logic:
//!     * `+1 (Proceed)`: Active, executing forward vector (`-->`)
//!     * ` 0 (Await)`  : Yielded, paused, asynchronous wait / standby
//!     * `-1 (Rollback)`: Invariant fault, unwinding state (`<--`)
//!     * `NULL (Halt)`  : Terminated, absorbed, process exited
//! - Priority queues (High, Normal, Low, Background)
//! - Cooperative tick loops, task execution time tracking, and IPC-ready task ledger.

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::reel::MemoryReel;
use crate::core::tape::Tape;
use crate::core::ternary::QuadState;
use crate::core::vessel::Vessel;
use crate::diagnostics::logger::DiagnosticLogger;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Task Priorities, Tracks, Task Descriptor & Configuration )
// ==============================================================================

/// The 3 physical execution channels of the Multi-Track Scheduler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Track {
    /// Track 0: Hardware / Kernel Genesis / Core Daemons.
    Kernel = 0,
    /// Track 1: Terminal Shell & Console HUD.
    Terminal = 1,
    /// Track 2: Userspace & Worker Applications.
    App = 2,
}

impl Track {
    pub fn as_str(&self) -> &'static str {
        match self {
            Track::Kernel => "TRACK 0 [KERNEL]",
            Track::Terminal => "TRACK 1 [TERMINAL]",
            Track::App => "TRACK 2 [APP]",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    High = 3,
    Normal = 2,
    Low = 1,
    Background = 0,
}

impl TaskPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskPriority::High => "HIGH",
            TaskPriority::Normal => "NORMAL",
            TaskPriority::Low => "LOW",
            TaskPriority::Background => "BACKGROUND",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.trim().to_uppercase().as_str() {
            "HIGH" | "3" => TaskPriority::High,
            "LOW" | "1" => TaskPriority::Low,
            "BACKGROUND" | "IDLE" | "0" => TaskPriority::Background,
            _ => TaskPriority::Normal,
        }
    }
}

/// Descriptor representing an active, waiting, or halted process in the terminal.
#[derive(Debug, Clone)]
pub struct OmniTask {
    pub id: u64,
    pub name: String,
    pub command: String,
    pub priority: TaskPriority,
    pub track: Track,
    pub state: QuadState,
    pub created_at: u64,
    pub updated_at: u64,
    pub ticks: u64,
    pub log_output: Vec<String>,
    pub tape: Option<Tape>,
    pub partition: Option<Vessel>,
    pub reel: Option<MemoryReel>,
    pub loop_count: usize,
    pub max_loops: usize,
}

impl OmniTask {
    pub fn new(id: u64, name: String, command: String, priority: TaskPriority) -> Self {
        Self::with_track(id, name, command, priority, Track::App)
    }

    pub fn with_track(id: u64, name: String, command: String, priority: TaskPriority, track: Track) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id,
            name: name.clone(),
            command,
            priority,
            track,
            state: QuadState::Proceed,
            created_at: now,
            updated_at: now,
            ticks: 0,
            log_output: Vec::new(),
            tape: None,
            partition: None,
            reel: Some(MemoryReel::new(name, 64)),
            loop_count: 0,
            max_loops: 100,
        }
    }


    pub fn to_json(&self) -> String {
        let state_str = match self.state {
            QuadState::Proceed => "+1:PROCEED",
            QuadState::Await => "0:AWAIT",
            QuadState::Rollback => "-1:ROLLBACK",
            QuadState::Halt => "NULL:HALT",
        };

        let logs = self.log_output.join("\\n").replace('"', "\\\"");

        format!(
            r#"{{"id":{},"name":"{}","command":"{}","priority":"{}","track":"{}","state":"{}","ticks":{},"created_at":{},"logs":"{}"}}"#,
            self.id,
            self.name,
            self.command.replace('"', "\\\""),
            self.priority.as_str(),
            self.track.as_str(),
            state_str,
            self.ticks,
            self.created_at,
            logs
        )
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Scheduler Core: Cooperative Dispatcher, State Mutations & Tick Engine )
// ==============================================================================

#[derive(Debug)]
pub struct OmniScheduler {
    tasks: Vec<OmniTask>,
    next_id: u64,
    pub root_storehouse: Vessel,
}

impl Default for OmniScheduler {
    fn default() -> Self {
        let mut scheduler = Self {
            tasks: Vec::new(),
            next_id: 1,
            root_storehouse: Vessel::new_storehouse("Root_RAM", 4096),
        };
        // Initialize default OS background tasks on Track 0 (Kernel)
        scheduler.spawn_on_track("CoreDaemon", "omnicode kernel --daemon", TaskPriority::Background, Track::Kernel);
        scheduler.spawn_on_track("MemoryWatcher", "omnicode mem --watch", TaskPriority::Low, Track::Kernel);
        scheduler
    }
}

impl OmniScheduler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Spawns a new task in the scheduler on Track 2 (App) by default.
    pub fn spawn(&mut self, name: &str, command: &str, priority: TaskPriority) -> u64 {
        self.spawn_on_track(name, command, priority, Track::App)
    }

    /// Spawns a new task on a designated execution track (Kernel, Terminal, or App).
    pub fn spawn_on_track(&mut self, name: &str, command: &str, priority: TaskPriority, track: Track) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let mut task = OmniTask::with_track(id, name.to_string(), command.to_string(), priority, track);
        task.log_output.push(format!(
            "[*] Task spawned on {} with state +1:PROCEED (Priority: {})",
            track.as_str(),
            priority.as_str()
        ));
        self.tasks.push(task);
        id
    }

    /// Mounts a structured [`Tape`] onto an execution track, carving an isolated memory partition.
    pub fn mount_tape(&mut self, track: Track, mut tape: Tape) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let part_size = tape.head.required_partition_size;
        let partition_res = self.root_storehouse.partition(&tape.head.name, part_size);

        tape.bound_partition = Some(tape.head.name.clone());
        let priority = match track {
            Track::Kernel => TaskPriority::High,
            Track::Terminal => TaskPriority::Normal,
            Track::App => TaskPriority::Normal,
        };

        let mut task = OmniTask::with_track(
            id,
            tape.head.name.clone(),
            format!("tape:{}", tape.head.name),
            priority,
            track,
        );

        let part_info = match partition_res {
            Ok(p) => {
                let msg = format!("carved partition '{}' ({} cells)", p.name, p.capacity);
                task.partition = Some(p);
                msg
            }
            Err(e) => format!("partition warning: {e}"),
        };

        task.log_output.push(format!(
            "[+] Mounted Tape '{}' on {} ({}, {} rolls)",
            tape.head.name,
            track.as_str(),
            part_info,
            tape.roll_count()
        ));

        task.tape = Some(tape);
        self.tasks.push(task);
        id
    }

    /// Mounts and parses a `.tape` file from disk onto the specified track.
    pub fn mount_tape_file(&mut self, track: Track, path: impl AsRef<Path>, logger: DiagnosticLogger) -> Result<u64, String> {
        let tape = Tape::from_file(path, logger)?;
        Ok(self.mount_tape(track, tape))
    }

    /// Advances the scheduler clock by 1 tick, executing tasks in Proceed state.
    ///
    /// For tasks with mounted Tapes, steps the Tape Head forward roll-by-roll.
    pub fn tick(&mut self) -> usize {
        let mut active_executed = 0;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        for task in &mut self.tasks {
            if task.state == QuadState::Proceed {
                task.ticks += 1;
                task.updated_at = now;
                active_executed += 1;

                if let Some(ref mut tape) = task.tape {
                    if !tape.is_finished() {
                        if let Some(roll) = tape.active_roll() {
                            task.log_output.push(format!("==> [HEAD @ Roll {}] {}", tape.cursor + 1, roll.text));
                        }
                        tape.advance();
                        if tape.is_finished() {
                            task.log_output.push(format!(
                                "[√] Tape '{}' execution completed successfully (all {} rolls verified)",
                                tape.head.name,
                                tape.roll_count()
                            ));
                        }
                    }
                }
            }
        }
        active_executed
    }

    /// Manually steps the Tape Head on the given task by 1 roll, executing transport kinematics.
    pub fn step_task(&mut self, id: u64) -> Result<String, String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if let Some(ref mut tape) = task.tape {
                if !tape.is_finished() {
                    let cur_idx = tape.cursor + 1;
                    let roll_text = tape.active_roll().map(|r| r.text.clone()).unwrap_or_default();

                    // Check for STOP transport action
                    if roll_text.contains("STOP") || roll_text.contains("HALT") {
                        tape.cursor = tape.roll_count();
                        task.state = QuadState::Halt;
                        if let Some(ref mut reel) = task.reel {
                            reel.stop();
                        }
                        let msg = format!("[⏹ STOP] Task {id}: Execution stopped and sealed at Roll {cur_idx}: {roll_text}");
                        task.log_output.push(msg.clone());
                        return Ok(msg);
                    }

                    // Check for REWIND transport loop action
                    if roll_text.contains("REWIND") {
                        let cue_opt = if let Some(start) = roll_text.find('<') {
                            if let Some(end) = roll_text[start..].find('>') {
                                Some(roll_text[start + 1..start + end].trim().to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        if let Some(cue) = cue_opt {
                            if task.loop_count < task.max_loops {
                                if let Some(target_idx) = tape.find_landmark_index(&cue) {
                                    task.loop_count += 1;
                                    tape.rewind_to(target_idx);
                                    if let Some(ref mut reel) = task.reel {
                                        let _ = reel.rewind_to_landmark(&cue);
                                    }
                                    let msg = format!(
                                        "[⏮ REWIND] Task {id}: Track rewound to Landmark <{cue}> at Roll {} (Iteration {}/{})",
                                        target_idx + 1, task.loop_count, task.max_loops
                                    );
                                    task.log_output.push(msg.clone());
                                    return Ok(msg);
                                }
                            }
                        }
                    }

                    // Check for PAUSE transport action
                    if roll_text.contains("PAUSE") {
                        tape.advance();
                        task.state = QuadState::Await;
                        if let Some(ref mut reel) = task.reel {
                            reel.pause();
                        }
                        let msg = format!("[⏸ PAUSE] Task {id}: Track paused at Roll {cur_idx}: {roll_text}");
                        task.log_output.push(msg.clone());
                        return Ok(msg);
                    }

                    // Default: PLAY (forward advance)
                    tape.advance();
                    if let Some(ref mut reel) = task.reel {
                        reel.play();
                    }
                    let msg = format!("[▶ PLAY] Step PID {id}: Advanced to Roll {cur_idx}/{}: {roll_text}", tape.roll_count());
                    task.log_output.push(msg.clone());
                    Ok(msg)
                } else {
                    Ok(format!("[√] Task {id} Tape '{}' has already executed all {} rolls", task.name, tape.roll_count()))
                }
            } else {
                Err(format!("Task {id} ('{}') has no active mounted Tape ribbon", task.name))
            }
        } else {
            Err(format!("Task ID {id} not found"))
        }
    }

    /// Displays all rolls of a task's tape with the live `==> [HEAD]` pointer.
    pub fn rolls_for_task(&self, id: u64) -> Option<String> {
        let task = self.tasks.iter().find(|t| t.id == id)?;
        let tape = task.tape.as_ref()?;

        let mut out = String::new();
        out.push_str(&format!("TAPE RIBBON: [{}] (PID: {})\n", tape.head.name, task.id));
        out.push_str(&format!("  Channel:   {}\n", task.track.as_str()));
        out.push_str(&format!("  Tape Head: {}\n", tape.head.raw_text));
        out.push_str("-------------------------------------------------------------\n");

        for (i, roll) in tape.rolls.iter().enumerate() {
            if i == tape.cursor && !tape.is_finished() {
                out.push_str(&format!("==> [{:>2}] {}\n", i + 1, roll.text));
            } else if i < tape.cursor {
                out.push_str(&format!("  √ [{:>2}] {}\n", i + 1, roll.text));
            } else {
                out.push_str(&format!("    [{:>2}] {}\n", i + 1, roll.text));
            }
        }

        if tape.is_finished() {
            out.push_str("-------------------------------------------------------------\n");
            out.push_str("  [√] TAPE EXECUTION COMPLETE (Sealed with PROCEED)\n");
        }
        Some(out)
    }

    /// Returns memory storehouse utilization and active partition breakdown.
    pub fn mem_summary(&self) -> String {
        let total = self.root_storehouse.capacity;
        let allocated = self.root_storehouse.count;
        let free = total.saturating_sub(allocated);
        let pct = if total > 0 { (allocated * 100) / total } else { 0 };

        let mut out = String::new();
        out.push_str("=============================================================\n");
        out.push_str("OMNICODE ROOT STOREHOUSE & TASK PARTITIONS\n");
        out.push_str("=============================================================\n");
        out.push_str(&format!("Root_RAM:     {allocated} / {total} cells ({pct}% Allocated, {free} cells Free)\n"));

        // Visual progress bar
        let filled_bars = pct / 5;
        let empty_bars = 20 - filled_bars;
        out.push_str(&format!("RAM Meter:    [{}{}] {}%\n", "█".repeat(filled_bars), "░".repeat(empty_bars), pct));
        out.push_str("-------------------------------------------------------------\n");
        out.push_str("TASK PARTITION ALLOCATIONS:\n");

        let mut found_part = false;
        for t in &self.tasks {
            if let Some(ref p) = t.partition {
                found_part = true;
                out.push_str(&format!("  • PID {:<3} [{:<16}] Capacity: {:>4} cells (Track: {})\n", t.id, p.name, p.capacity, t.track.as_str()));
            }
        }
        if !found_part {
            out.push_str("  (No active task partitions carved yet)\n");
        }
        out.push_str("=============================================================\n");
        out
    }

    /// Returns a structured summary grouping tasks by Track.
    pub fn track_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("=============================================================\n");
        out.push_str("MULTI-TRACK EXECUTION CHANNELS\n");
        out.push_str("=============================================================\n");

        for track in [Track::Kernel, Track::Terminal, Track::App] {
            out.push_str(&format!("\n[ {} ]\n", track.as_str()));
            let track_tasks: Vec<&OmniTask> = self.tasks.iter().filter(|t| t.track == track).collect();
            if track_tasks.is_empty() {
                out.push_str("  • (No active processes on this track)\n");
            } else {
                for t in track_tasks {
                    let tape_tag = if t.tape.is_some() { "[TAPE]" } else { "      " };
                    let state_badge = match t.state {
                        QuadState::Proceed => "+1:PROCEED",
                        QuadState::Await => " 0:AWAIT  ",
                        QuadState::Rollback => "-1:ROLLBCK",
                        QuadState::Halt => "NULL:HALT ",
                    };
                    out.push_str(&format!("  • PID {:<3} {} {:<10} {:<16} ticks: {}\n", t.id, tape_tag, state_badge, t.name, t.ticks));
                }
            }
        }
        out
    }

    /// Transitions a task to 0:AWAIT (Standby/Paused).
    pub fn pause_task(&mut self, id: u64) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.state = QuadState::Await;
            task.log_output.push("[~] Task transitioned to 0:AWAIT (Suspended)".to_string());
            Ok(())
        } else {
            Err(format!("Task ID {id} not found"))
        }
    }

    /// Resumes a task back to +1:PROCEED (Running).
    pub fn resume_task(&mut self, id: u64) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.state = QuadState::Proceed;
            task.log_output.push("[+] Task resumed to +1:PROCEED".to_string());
            Ok(())
        } else {
            Err(format!("Task ID {id} not found"))
        }
    }

    /// Signals a rollback on a task (-1:ROLLBACK).
    pub fn rollback_task(&mut self, id: u64, reason: &str) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.state = QuadState::Rollback;
            task.log_output.push(format!("[-] Task rolled back to -1:ROLLBACK. Reason: {reason}"));
            Ok(())
        } else {
            Err(format!("Task ID {id} not found"))
        }
    }

    /// Halts and terminates a task (NULL:HALT).
    pub fn halt_task(&mut self, id: u64) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.state = QuadState::Halt;
            task.log_output.push("[!] Task terminated with state NULL:HALT".to_string());
            Ok(())
        } else {
            Err(format!("Task ID {id} not found"))
        }
    }

    /// Appends output log line to a task.
    pub fn log_task(&mut self, id: u64, line: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.log_output.push(line.to_string());
        }
    }

    /// Returns all tasks formatted as a JSON array.
    pub fn to_json(&self) -> String {
        let task_jsons: Vec<String> = self.tasks.iter().map(|t| t.to_json()).collect();
        format!("[{}]", task_jsons.join(","))
    }

    /// Returns a human-readable table of all running processes.
    pub fn format_table(&self) -> String {
        let mut out = String::new();
        out.push_str("PID   STATE        CHANNEL         PRIORITY    TICKS   COMMAND\n");
        out.push_str("-----------------------------------------------------------------------\n");
        for t in &self.tasks {
            let state_badge = match t.state {
                QuadState::Proceed => "+1 PROCEED",
                QuadState::Await => " 0 AWAIT  ",
                QuadState::Rollback => "-1 ROLLBCK",
                QuadState::Halt => "NULL HALT ",
            };
            out.push_str(&format!(
                "{:<5} {:<12} {:<15} {:<11} {:<7} {}\n",
                t.id,
                state_badge,
                t.track.as_str(),
                t.priority.as_str(),
                t.ticks,
                t.command
            ));
        }
        out
    }

    pub fn tasks(&self) -> &[OmniTask] {
        &self.tasks
    }

    pub fn get_task(&self, id: u64) -> Option<&OmniTask> {
        self.tasks.iter().find(|t| t.id == id)
    }

    pub fn get_task_mut(&mut self, id: u64) -> Option<&mut OmniTask> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Global Thread-Safe Scheduler Handle & Tests )
// ==============================================================================

pub type SharedScheduler = Arc<Mutex<OmniScheduler>>;

pub fn create_shared_scheduler() -> SharedScheduler {
    Arc::new(Mutex::new(OmniScheduler::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_lifecycle() {
        let mut sched = OmniScheduler::new();
        let pid = sched.spawn("TestProcess", "omnicode test", TaskPriority::High);

        assert_eq!(sched.tasks.iter().find(|t| t.id == pid).unwrap().state, QuadState::Proceed);

        sched.tick();
        assert_eq!(sched.tasks.iter().find(|t| t.id == pid).unwrap().ticks, 1);

        sched.pause_task(pid).unwrap();
        assert_eq!(sched.tasks.iter().find(|t| t.id == pid).unwrap().state, QuadState::Await);

        sched.resume_task(pid).unwrap();
        assert_eq!(sched.tasks.iter().find(|t| t.id == pid).unwrap().state, QuadState::Proceed);

        sched.halt_task(pid).unwrap();
        assert_eq!(sched.tasks.iter().find(|t| t.id == pid).unwrap().state, QuadState::Halt);
    }
}
