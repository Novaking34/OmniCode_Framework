// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Multi-Track Cooperative Process Scheduler Engine & Lifecycle Dispatcher )
// ==============================================================================
//! OmniCode Terminal: Multi-Track Process Scheduler Engine
//!
//! # Architectural Scope & Purpose:
//! Provides the primary process and task scheduling engine for the OmniCode
//! Native Terminal and OS userspace shell:
//! - Balanced Radix-3 process lifecycles (+1:Proceed, 0:Await, -1:Rollback, NULL:Halt).
//! - Multi-track execution: Track 0 (Kernel), Track 1 (Terminal), Track 2 (App).
//! - Root_Storehouse partition carving for mounted tape process ribbons.
//! - Cooperative `tick` clock and manual tape roll stepping (`step_task`).
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of process state onto physical tables of memory).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving cooperative task scheduling before dispatching shell interfaces).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of historical execution channel boundaries across kernel, terminal, and apps).
// ==============================================================================

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::tape::Tape;
use crate::core::ternary::QuadState;
use crate::core::vessel::Vessel;
use crate::diagnostics::logger::DiagnosticLogger;

use super::tasks::{OmniTask, TaskPriority, Track};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( OmniScheduler Struct Definition, Constructor & Defaults )
// ==============================================================================

/// Primary multi-track cooperative process scheduler.
#[derive(Debug)]
pub struct OmniScheduler {
    tasks: Vec<OmniTask>,
    next_id: u64,
    /// Root storehouse from which task RAM partitions are carved.
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

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Scheduler Core: Process Dispatcher, Partition Carving & Tick Engine )
// ==============================================================================

impl OmniScheduler {
    /// Constructs a new scheduler initialized with default kernel daemons.
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
                    let is_unconditional_stop = roll_text.contains("STOP") || (roll_text.contains("HALT") && !roll_text.contains("HALT WHEN"));
                    if is_unconditional_stop {
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

    /// Returns a reference to all tasks in the scheduler.
    pub fn tasks(&self) -> &[OmniTask] {
        &self.tasks
    }

    /// Finds a task by ID.
    pub fn get_task(&self, id: u64) -> Option<&OmniTask> {
        self.tasks.iter().find(|t| t.id == id)
    }

    /// Finds a mutable reference to a task by ID.
    pub fn get_task_mut(&mut self, id: u64) -> Option<&mut OmniTask> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Global Thread-Safe Scheduler Handle & Verification Tests )
// ==============================================================================

/// Thread-safe shared scheduler pointer wrapped in `Arc<Mutex<OmniScheduler>>`.
pub type SharedScheduler = Arc<Mutex<OmniScheduler>>;

/// Factory function creating a thread-safe shared scheduler instance.
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
