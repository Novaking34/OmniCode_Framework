// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( 4-Directional Toroidal Memory Reel: Transport Kinematics & Scene Spool )
// ==============================================================================
//! OmniCode Core: 4-Directional Memory Reel
//!
//! Spools memory across two orthogonal mechanical axes:
//! 1. **Horizontal Ribbon Navigation:**
//!    - `push_forward(word)` (`-->`): Advances write head along the track.
//!    - `pull_back()` (`<--`): Recedes backward along the track to inspect/undo.
//! 2. **Vertical Spool Hierarchy Navigation:**
//!    - `sink_in(name, cap)` (`v--`): Cuts into a child scene (pushes stack frame).
//!    - `pop_out()` (`^--`): Cuts back to parent scene (pops stack frame).
//!
//! Governed by the Quad-State Transport Kinematics:
//! `PLAY` (+1), `PAUSE` (0), `REWIND` (-1), `STOP` (NULL).

use std::fmt;
use crate::core::scene::Scene;
use crate::core::ternary::TransportAction;
use crate::core::word::Word64;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( MemoryReel Struct Definition & Constructor )
// ==============================================================================

/// A 4-directional spooled memory stack and multi-track film canister.
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryReel {
    pub name: String,
    pub transport: TransportAction,
    pub scenes: Vec<Scene>,
    pub next_scene_id: usize,
}

impl MemoryReel {
    /// Creates a new MemoryReel initialized with a root master scene (depth 0).
    pub fn new(name: impl Into<String>, root_capacity: usize) -> Self {
        let root_scene = Scene::new(0, "MasterSet_0", root_capacity, 0, Some("genesis".to_string()));
        Self {
            name: name.into(),
            transport: TransportAction::Pause,
            scenes: vec![root_scene],
            next_scene_id: 1,
        }
    }

    /// Returns a reference to the currently active camera scene.
    pub fn active_scene(&self) -> &Scene {
        self.scenes.last().expect("Reel must always contain at least one scene")
    }

    /// Returns a mutable reference to the currently active camera scene.
    pub fn active_scene_mut(&mut self) -> &mut Scene {
        self.scenes.last_mut().expect("Reel must always contain at least one scene")
    }

    /// Returns the current spindle depth level (0 = Master set).
    pub fn depth(&self) -> usize {
        self.active_scene().depth
    }

    /// Returns the total number of scenes currently on the reel spindle.
    pub fn scene_count(&self) -> usize {
        self.scenes.len()
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( The 4 Mechanical Motions: Push Forward, Pull Back, Sink In, Pop Out )
// ==============================================================================

impl MemoryReel {
    // --------------------------------------------------------------------------
    // Axis 1: Horizontal Track Motions (--> and <--)
    // --------------------------------------------------------------------------

    /// **Push Forward (`-->`):**
    /// Advances the track head, inscribes the physical word at the new position,
    /// and sets transport to `PLAY`.
    pub fn push_forward(&mut self, word: Word64) -> Result<usize, String> {
        let scene = self.active_scene_mut();
        let target_pos = scene.head_pos;

        if target_pos >= scene.capacity {
            return Err(format!(
                "FAILPOINT [ReelCapacityReached]: Track head at @0x{:04X} exceeded scene capacity {}",
                target_pos, scene.capacity
            ));
        }

        scene.write(target_pos, word)?;
        scene.head_pos += 1;
        self.transport = TransportAction::Play;
        Ok(target_pos)
    }

    /// **Pull Back (`<--`):**
    /// Recedes the track head backward along the ribbon to inspect antecedent props
    /// or perform transactional retreat without destroying future frame cells.
    pub fn pull_back(&mut self) -> Option<Word64> {
        let scene = self.active_scene_mut();
        if scene.head_pos > 0 {
            scene.head_pos -= 1;
            scene.read(scene.head_pos)
        } else {
            scene.read(0)
        }
    }

    // --------------------------------------------------------------------------
    // Axis 2: Vertical Spool Hierarchy Motions (v-- and ^--)
    // --------------------------------------------------------------------------

    /// **Sink In (`v--`):**
    /// Cuts into a subordinate scene (pushes a new stack frame onto the spindle).
    /// The parent scene remains frozen and untouched above it.
    pub fn sink_in(
        &mut self,
        scene_name: impl Into<String>,
        capacity: usize,
        landmark: Option<String>,
    ) -> usize {
        let parent_depth = self.depth();
        let scene_id = self.next_scene_id;
        self.next_scene_id += 1;

        let child_scene = Scene::new(scene_id, scene_name, capacity, parent_depth + 1, landmark);
        self.scenes.push(child_scene);
        self.depth()
    }

    /// **Pop Out (`^--`):**
    /// Completes the subordinate scene, pops the stack frame off the spindle,
    /// and restores the camera framing to the supervisory parent set.
    pub fn pop_out(&mut self) -> Result<Scene, String> {
        if self.scenes.len() <= 1 {
            return Err("FAILPOINT [RootScenePop]: Cannot pop out of the master root scene (Depth 0)".to_string());
        }
        let completed = self.scenes.pop().expect("Scene pop must succeed");
        Ok(completed)
    }

    // --------------------------------------------------------------------------
    // Transport Kinematics Controls: Play, Pause, Rewind, Stop
    // --------------------------------------------------------------------------

    /// Sets transport to `PLAY` (+1 / PROCEED).
    pub fn play(&mut self) -> TransportAction {
        self.transport = TransportAction::Play;
        self.transport
    }

    /// Sets transport to `PAUSE` (0 / AWAIT).
    pub fn pause(&mut self) -> TransportAction {
        self.transport = TransportAction::Pause;
        self.transport
    }

    /// **Rewinds (`<--` / -1 / ROLLBACK):**
    /// Spools the track head backward to a designated scene landmark cue.
    pub fn rewind_to_landmark(&mut self, target_cue: &str) -> Result<usize, String> {
        let scene = self.active_scene_mut();
        if let Some(ref landmark) = scene.landmark {
            if landmark == target_cue {
                scene.head_pos = 0;
                self.transport = TransportAction::Rewind;
                return Ok(0);
            }
        }
        Err(format!(
            "FAILPOINT [LandmarkNotFound]: Active scene '{}' does not match landmark cue '<{target_cue}>'",
            scene.name
        ))
    }

    /// Sets transport to `STOP` (NULL / HALT).
    pub fn stop(&mut self) -> TransportAction {
        self.transport = TransportAction::Stop;
        self.transport
    }

    /// Generates a cinematic ASCII status dashboard of the memory reel.
    pub fn inspect(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("═════════════════════════════════════════════════════════════\n"));
        out.push_str(&format!("REEL: [{}] | TRANSPORT: [{:?}]\n", self.name, self.transport));
        out.push_str(&format!("SPINDLE DEPTH: {} | TOTAL SCENES: {}\n", self.depth(), self.scene_count()));
        out.push_str(&format!("-------------------------------------------------------------\n"));
        for (i, sc) in self.scenes.iter().enumerate() {
            let active_marker = if i == self.scenes.len() - 1 { "▶ [ACTIVE]" } else { "  [FROZEN]" };
            out.push_str(&format!("  {} Level {}: {}\n", active_marker, sc.depth, sc));
        }
        out.push_str(&format!("═════════════════════════════════════════════════════════════\n"));
        out
    }
}

impl fmt::Display for MemoryReel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Reel [{} | Transport: {:?} | Depth: {} | Active: {}]",
            self.name, self.transport, self.depth(), self.active_scene().name
        )
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Verification Proof )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::word::Word32Trit;
    use crate::core::ternary::QuadState;
    use crate::core::operators::Polarity;
    use crate::core::boundaries::BoundaryType;

    #[test]
    fn test_reel_4_directional_motions() {
        let mut reel = MemoryReel::new("CinemaReel", 32);
        assert_eq!(reel.depth(), 0);
        assert_eq!(reel.scene_count(), 1);

        // 1. Push Forward (-->) on master set
        let w1 = Word64::new(Word32Trit::from_i64(10), QuadState::Proceed, Polarity::Concrete, BoundaryType::Denotation, 0);
        let pos1 = reel.push_forward(w1).expect("Push forward 1");
        assert_eq!(pos1, 0);
        assert_eq!(reel.transport, TransportAction::Play);

        let w2 = Word64::new(Word32Trit::from_i64(20), QuadState::Proceed, Polarity::Concrete, BoundaryType::Denotation, 0);
        let pos2 = reel.push_forward(w2).expect("Push forward 2");
        assert_eq!(pos2, 1);

        // 2. Pull Back (<--) along master track
        let back_word = reel.pull_back().expect("Pull back");
        assert_eq!(back_word.payload().to_i64(), 20);

        // 3. Sink In (v--) into nested sub-scene
        let depth = reel.sink_in("CloseUp_Scene", 16, Some("closeup_take".to_string()));
        assert_eq!(depth, 1);
        assert_eq!(reel.depth(), 1);
        assert_eq!(reel.scene_count(), 2);
        assert_eq!(reel.active_scene().name, "CloseUp_Scene");

        // Write in nested sub-scene
        let w_sub = Word64::new(Word32Trit::from_i64(999), QuadState::Proceed, Polarity::Concrete, BoundaryType::Denotation, 0);
        reel.push_forward(w_sub).expect("Push in sub-scene");
        assert_eq!(reel.active_scene().read(0).unwrap().payload().to_i64(), 999);

        // 4. Pop Out (^--) back to master wide shot
        let popped = reel.pop_out().expect("Pop out to master");
        assert_eq!(popped.name, "CloseUp_Scene");
        assert_eq!(reel.depth(), 0);
        assert_eq!(reel.active_scene().name, "MasterSet_0");
        assert_eq!(reel.scene_count(), 1);

        // Verify master set props were unaffected by sub-scene
        assert_eq!(reel.active_scene().read(0).unwrap().payload().to_i64(), 10);
        assert_eq!(reel.active_scene().read(1).unwrap().payload().to_i64(), 20);
    }

    #[test]
    fn test_reel_rewind_to_landmark() {
        let mut reel = MemoryReel::new("TakeReel", 32);
        reel.sink_in("TakeScene", 16, Some("scene_cue".to_string()));

        let w = Word64::new(Word32Trit::from_i64(42), QuadState::Proceed, Polarity::Concrete, BoundaryType::Denotation, 0);
        reel.push_forward(w).unwrap();
        reel.push_forward(w).unwrap();
        assert_eq!(reel.active_scene().head_pos, 2);

        // Rewind to landmark
        let reset_pos = reel.rewind_to_landmark("scene_cue").expect("Rewind to cue");
        assert_eq!(reset_pos, 0);
        assert_eq!(reel.active_scene().head_pos, 0);
        assert_eq!(reel.transport, TransportAction::Rewind);
    }
}
