// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Cinematic Reel Kinematics & Transport Instructions )
// ==============================================================================
//! OmniCode Terminal: Cinematic Reel Kinematics
//!
//! # Architectural Scope & Purpose:
//! Evaluates procedural roll statements against the 4-directional [`MemoryReel`]:
//! - Transport Actions: `STOP`, `HALT`, `PAUSE`, `PLAY`.
//! - Spatial Spindle Kinematics: Sinking in (`v--`), Popping out (`^--`), Pulling back (`<--`).
//! - Hardware MMIO Video Projection: Screen / VGA writes at `0x000B8000`.
//! - Inscribing words forward into the reel (`-->`).
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**: Tables of testimony, written with the finger of God.
//! - **Proverbs 24:27 (KJV)**: Prepare thy work without, and make it fit for thyself in the field.
//! - **Proverbs 22:28 (KJV)**: Remove not the ancient landmark, which thy fathers have set.
// ==============================================================================

use crate::core::mmio::VgaColor;
use crate::core::word::Word64;

use super::hud::OmniStudio;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Kinematic Step Methods on OmniStudio )
// ==============================================================================

impl OmniStudio {
    /// Interprets a single tape roll instruction against the memory reel.
    pub fn step_roll(&mut self, roll_text: &str) -> String {
        // ======================================================================
        // BLOCK 3: BODY BLOCK
        // ( Motion Evaluation & Hardware MMIO Dispatch )
        // ======================================================================
        if roll_text.contains("STOP") || roll_text.contains("HALT") {
            self.reel.stop();
            return "Kinematics: Reel stopped and transport sealed (⏹ STOP)".to_string();
        }
        if roll_text.contains("PAUSE") {
            self.reel.pause();
            return "Kinematics: Track paused waiting on cue (⏸ PAUSE)".to_string();
        }
        if roll_text.contains("v--") {
            let scene_name = if let Some(pos) = roll_text.find("v--") {
                roll_text.get(pos + 3..).unwrap_or("").split_whitespace().next().unwrap_or("SubScene")
            } else {
                "SubScene"
            };
            self.reel.sink_in(scene_name, 16, None);
            return format!("Kinematics: Sunk into sub-scene <{}> (v-- Spindle Depth {})", scene_name, self.reel.depth());
        }
        if roll_text.contains("^--") {
            let _ = self.reel.pop_out();
            return format!("Kinematics: Popped out to parent scene (^-- Spindle Depth {})", self.reel.depth());
        }
        if roll_text.contains("<--") {
            self.reel.pull_back();
            return "Kinematics: Pulled back track head (<--)".to_string();
        }
        if roll_text.contains("Screen") || roll_text.contains("VGA") || roll_text.contains("CONSOLE") || roll_text.contains("0x000B8000") {
            let extracted = if let Some(first_quote) = roll_text.find('\'') {
                if let Some(second_quote) = roll_text.get(first_quote + 1..).and_then(|s| s.find('\'')) {
                    roll_text.get(first_quote + 1..first_quote + 1 + second_quote)
                } else {
                    None
                }
            } else if let Some(first_quote) = roll_text.find('"') {
                if let Some(second_quote) = roll_text.get(first_quote + 1..).and_then(|s| s.find('"')) {
                    roll_text.get(first_quote + 1..first_quote + 1 + second_quote)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(screen_msg) = extracted {
                self.vga.write_str(screen_msg, VgaColor::LightGreen, VgaColor::Black);
                self.vga.new_line();
                return format!("Hardware MMIO: Wrote '{screen_msg}' to Bare-Metal Video Buffer @ 0x000B8000");
            }
        }
        if roll_text.contains("-->") {
            let val = roll_text
                .split_whitespace()
                .find_map(|s| s.parse::<i64>().ok())
                .unwrap_or(1);
            let word = Word64::from_signed_int(val);
            let _ = self.reel.push_forward(word);
            return format!("Kinematics: Inscribed word ({val}) and pushed forward (-->)");
        }
        self.reel.play();
        "Kinematics: Tape head advancing smoothly (▶ PLAY)".to_string()
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Kinematics Subsystem Verification )
// ==============================================================================
// Reel kinematics layer verified and closed.
