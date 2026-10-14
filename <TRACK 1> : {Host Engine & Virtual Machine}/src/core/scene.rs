// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Scene Memory Frame: Set, Props & Landmark Clapper )
// ==============================================================================
//! OmniCode Core: Scene Memory Frame
//!
//! Represents a unified memory stack frame / cinematic set where instructions
//! (rolls) operate upon local props (memory cells):
//! - **Set:** Bounded memory partition slice.
//! - **Props:** Physical 64-bit / 32-trit memory cells.
//! - **Clapper Landmark:** Scene cue point for retakes and rewind operations.

use std::fmt;
use crate::core::word::Word64;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Scene Struct Definition & Constructor )
// ==============================================================================

/// A memory stack frame and camera set in the OmniCode cinema architecture.
#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    pub id: usize,
    pub name: String,
    pub depth: usize,
    pub landmark: Option<String>,
    pub capacity: usize,
    pub head_pos: usize,
    pub cells: Vec<Word64>,
}

impl Scene {
    /// Creates a new Scene set with bounded cell capacity.
    pub fn new(
        id: usize,
        name: impl Into<String>,
        capacity: usize,
        depth: usize,
        landmark: Option<String>,
    ) -> Self {
        let cap = capacity.max(8);
        Self {
            id,
            name: name.into(),
            depth,
            landmark,
            capacity: cap,
            head_pos: 0,
            cells: vec![Word64::default(); cap],
        }
    }

    /// Returns the number of allocated cells in this scene.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns true if this scene has zero capacity.
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Reads a memory cell prop at an explicit offset.
    pub fn read(&self, offset: usize) -> Option<Word64> {
        self.cells.get(offset).copied()
    }

    /// Inscribes a memory cell prop at an explicit offset with bounds enforcement.
    pub fn write(&mut self, offset: usize, word: Word64) -> Result<(), String> {
        if offset >= self.capacity {
            return Err(format!(
                "FAILPOINT [SceneBoundsOverflow]: Scene '{}' capacity is {}, but attempted write at offset {}",
                self.name, self.capacity, offset
            ));
        }
        self.cells[offset] = word;
        Ok(())
    }

    /// Reads the prop directly under the active scene head.
    pub fn current(&self) -> Option<Word64> {
        self.read(self.head_pos)
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Scene Display Formatting )
// ==============================================================================

impl fmt::Display for Scene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let landmark_str = self.landmark.as_deref().unwrap_or("none");
        write!(
            f,
            "Scene [{} | ID: {} | Depth: {} | Landmark: <{}> | Head: @0x{:04X} | Cells: {}/{}]",
            self.name, self.id, self.depth, landmark_str, self.head_pos, self.cells.len(), self.capacity
        )
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::word::Word32Trit;
    use crate::core::ternary::QuadState;
    use crate::core::operators::Polarity;
    use crate::core::boundaries::BoundaryType;

    #[test]
    fn test_scene_creation_and_bounded_write() {
        let mut scene = Scene::new(0, "TestSet", 16, 0, Some("take_1".to_string()));
        assert_eq!(scene.name, "TestSet");
        assert_eq!(scene.depth, 0);
        assert_eq!(scene.capacity, 16);

        let word = Word64::new(Word32Trit::from_i64(77), QuadState::Proceed, Polarity::Concrete, BoundaryType::Denotation, 1);
        scene.write(3, word).expect("Write to offset 3");
        assert_eq!(scene.read(3).unwrap().payload().to_i64(), 77);

        // Bounds overflow rejection
        assert!(scene.write(16, word).is_err());
    }
}
