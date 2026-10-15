// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniStudio: Cinematic Projector, Set Props & Spindle HUD )
// ==============================================================================
//! OmniCode Terminal: Cinematic Projector & Memory HUD
//!
//! # Architectural Scope & Purpose:
//! Provides a real-time visual projection HUD of the 4-directional Memory Reel,
//! showing active tracks, scenes, rolls, and set props (64-bit / 32-trit cells):
//! 1. **Projector Head:** Active Reel, Channel Track, Transport Status (`PLAY`, `PAUSE`, `REWIND`, `STOP`).
//! 2. **Spindle Depth Meter:** Current Scene nesting level (`v--` depth).
//! 3. **The Script (Active Roll):** Current instruction line rolling through the gate.
//! 4. **Set Props (Memory Grid):** Bounded cells holding physical 64-bit words.
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of physical data into visual display volumes).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic projector frames before launching graphical studios).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of physical memory cell addresses and clapper cue landmarks).
// ==============================================================================

use crate::core::mmio::VgaBuffer;
use crate::core::reel::MemoryReel;
use crate::core::tape::Tape;
use crate::core::ternary::TransportAction;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( OmniStudio Struct Definition and Configuration )
// ==============================================================================

/// Cinematic Projector rendering interactive terminal HUD frames of memory reel execution.
pub struct OmniStudio {
    /// Active 4-directional memory reel.
    pub reel: MemoryReel,
    /// Canonical execution track display label.
    pub active_track: &'static str,
    /// Hardware MMIO VGA text buffer emulator.
    pub vga: VgaBuffer,
}

impl OmniStudio {
    /// Creates a new Studio Projector initialized with a named memory reel.
    pub fn new(reel_name: impl Into<String>, capacity: usize) -> Self {
        Self {
            reel: MemoryReel::new(reel_name, capacity),
            active_track: "TRACK 2 [APP]",
            vga: VgaBuffer::new(),
        }
    }

    /// Sets the active channel track display label.
    pub fn set_track(&mut self, track: &'static str) {
        self.active_track = track;
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Projector Frame Rendering & HUD Formatting )
// ==============================================================================

impl OmniStudio {
    /// Renders a complete cinematic projector HUD frame for a tape and its active roll.
    pub fn render_frame(&self, tape: &Tape, roll_idx: usize, message: &str) -> String {
        let mut out = String::new();
        let transport_badge = match self.reel.transport {
            TransportAction::Play => "\x1b[1;32m[▶ PLAY]\x1b[0m",
            TransportAction::Pause => "\x1b[1;33m[⏸ PAUSE]\x1b[0m",
            TransportAction::Rewind => "\x1b[1;35m[⏮ REWIND]\x1b[0m",
            TransportAction::Stop => "\x1b[1;31m[⏹ STOP]\x1b[0m",
        };

        let scene = self.reel.active_scene();
        let landmark_str = scene.landmark.as_deref().unwrap_or("none");

        out.push_str("\x1b[1;36m╔═══════════════════════════════════════════════════════════════════════════════╗\x1b[0m\n");
        out.push_str(&format!(
            "\x1b[1;36m║\x1b[0m  \x1b[1mOMNICODE CINEMA STUDIO\x1b[0m | REEL: \x1b[1;33m[{}]\x1b[0m | {}  \x1b[1;36m║\x1b[0m\n",
            self.reel.name, transport_badge
        ));
        out.push_str(&format!(
            "\x1b[1;36m║\x1b[0m  CHANNEL: {:<18} | SPINDLE DEPTH: {} | SCENE: <{}>  \x1b[1;36m║\x1b[0m\n",
            self.active_track, scene.depth, scene.name
        ));
        out.push_str(&format!(
            "\x1b[1;36m║\x1b[0m  CLAPPER CUE: <{:<16}> | HEAD POSITION: @0x{:04X}               \x1b[1;36m║\x1b[0m\n",
            landmark_str, scene.head_pos
        ));
        out.push_str("\x1b[1;36m╠═══════════════════════════════════════════════════════════════════════════════╣\x1b[0m\n");

        // Script Gate: Active Roll
        let roll_count = tape.roll_count();
        let roll_text = tape.rolls.get(roll_idx).map(|r| r.text.as_str()).unwrap_or("EOF");
        out.push_str(&format!(
            "\x1b[1;36m║\x1b[0m  \x1b[1;32mSCRIPT GATE [Roll {}/{}]:\x1b[0m {:<48} \x1b[1;36m║\x1b[0m\n",
            roll_idx + 1, roll_count, roll_text
        ));
        out.push_str(&format!(
            "\x1b[1;36m║\x1b[0m  {:<76} \x1b[1;36m║\x1b[0m\n",
            message
        ));
        out.push_str("\x1b[1;36m╠═══════════════════════════════════════════════════════════════════════════════╣\x1b[0m\n");

        // Set Props (Active Memory Cells)
        out.push_str("\x1b[1;36m║\x1b[0m  \x1b[1;33mSET PROPS (MEMORY CELL GRID):\x1b[0m                                                \x1b[1;36m║\x1b[0m\n");
        let display_cells = 4.min(scene.capacity);
        for i in 0..display_cells {
            if let Some(word) = scene.read(i) {
                let focus_ptr = if i == scene.head_pos { "▶" } else { " " };
                out.push_str(&format!(
                    "\x1b[1;36m║\x1b[0m   {} [0x{:04X}]: 0x{:016X} | Val: {:<5} | State: {:<7} \x1b[1;36m║\x1b[0m\n",
                    focus_ptr, i, word.raw, word.payload().to_i64(), word.state().macro_op()
                ));
            }
        }

        out.push_str("\x1b[1;36m╚═══════════════════════════════════════════════════════════════════════════════╝\x1b[0m\n");
        if self.vga.cursor_col > 0 || self.vga.cursor_row > 0 {
            out.push_str(&self.vga.render_screen(4));
        }
        out
    }

    /// Projects an entire tape sequentially with terminal cinematic frames.
    pub fn project_tape(&mut self, tape: &Tape) {
        println!("\x1b[1;32m▶ Mounting & Projecting Reel [{}] on {}\x1b[0m\n", self.reel.name, self.active_track);
        for (idx, roll) in tape.rolls.iter().enumerate() {
            let msg = self.step_roll(&roll.text);
            let frame = self.render_frame(tape, idx, &msg);
            print!("{frame}");
        }
        println!("\x1b[1;32m✓ Projection Complete: Tape sealed in {}\x1b[0m\n", tape.execution_state.macro_op());
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::logger::DiagnosticLogger;

    #[test]
    fn test_studio_render_frame() {
        let logger = DiagnosticLogger::new(false);
        let tape = Tape::from_source(
            r#"
            [Tape.TestProjection] : WHO {Kernel} WHAT {Test} --> DRIVE {Cinema}
            roll_1 [ 42 --> Cell_0 @ 0 ]
            roll_2 [ commit_status [ STOP ] ]
            "#,
            logger,
        ).expect("Parse tape");

        let studio = OmniStudio::new("TestReel", 16);
        let frame = studio.render_frame(&tape, 0, "Test kinematically advancing");
        assert!(frame.contains("OMNICODE CINEMA STUDIO"));
        assert!(frame.contains("TestReel"));
        assert!(frame.contains("SET PROPS"));
    }
}
