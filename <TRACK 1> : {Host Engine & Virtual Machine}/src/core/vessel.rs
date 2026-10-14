// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Vessel Subsystem Identity, Memory Containers & Boundary Invariants )
// ==============================================================================
//! OmniCode Core: Memory Vessel & Storehouse Subsystem
//!
//! Codifies the foundational storage nouns of OmniCode:
//!     - Units: Trit, Cell, Word
//!     - Vessels: Vessel, Storehouse, Partition
//!     - Coordinates: Offset, Origin, Landmark
//!     - Measures: Capacity, Count
//!     - Substance: Value, State
//!
//! Governed by Invariant Boundary Physics: An access beyond `Capacity`
//! produces an immediate `-1: ROLLBACK` (Caveat Veto), preserving memory integrity.

use std::fmt;
use crate::core::cell::Cell;
use crate::core::linguistics::{GrammaticalClassification, GrammaticalEntity, PartOfSpeech};
use crate::core::ternary::QuadState;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Vessel, Partition, and Coordinate Struct Definitions )
// ==============================================================================

/// A bounded container of memory cells.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vessel {
    /// Canonical identifier / name of the vessel.
    pub name: String,
    /// Physical or logical origin address / base coordinate.
    pub origin: usize,
    /// Maximum number of cells this vessel is allocated to hold.
    pub capacity: usize,
    /// Number of cells currently written or active.
    pub count: usize,
    /// Dense cell array (where each cell packs to exactly 1 byte in physical RAM).
    pub cells: Vec<Cell>,
    /// Whether this vessel acts as an ambient system Storehouse.
    pub is_storehouse: bool,
    /// If this vessel is a Partition, references the parent Storehouse name.
    pub parent: Option<String>,
}

/// Coordinate offset within a vessel (`Vessel[Offset]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Offset(pub usize);

/// Measurement extent of a vessel's total bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Capacity(pub usize);

/// Measurement count of active cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Count(pub usize);


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Vessel Allocation, Partitioning, Bounds Checking, and Cell Read/Write )
// ==============================================================================

impl Vessel {
    /// Allocates a new bounded Vessel with the given capacity.
    /// Initialized with empty equilibrium cells (`AWAIT`).
    pub fn new(name: &str, capacity: usize) -> Self {
        Self {
            name: name.to_string(),
            origin: 0,
            capacity,
            count: 0,
            cells: vec![Cell::equilibrium(); capacity],
            is_storehouse: false,
            parent: None,
        }
    }

    /// Allocates an ambient system Storehouse.
    pub fn new_storehouse(name: &str, capacity: usize) -> Self {
        let mut v = Self::new(name, capacity);
        v.is_storehouse = true;
        v
    }

    /// Grants an entrusted, bounded Partition from this Storehouse.
    ///
    /// # Invariants
    /// - The partition capacity cannot exceed the remaining unallocated capacity.
    /// - Breaches within the partition are contained and cannot overrun the parent.
    pub fn partition(&mut self, partition_name: &str, partition_capacity: usize) -> Result<Self, String> {
        let available = self.capacity.saturating_sub(self.count);
        if partition_capacity > available {
            return Err(format!(
                "FAILPOINT [StorehouseExhaustion]: Requested partition '{partition_name}' of {partition_capacity} cells, \
                 but Storehouse '{}' only has {available} cells remaining (Capacity: {}, Count: {}).",
                self.name, self.capacity, self.count
            ));
        }

        let partition_origin = self.origin + self.count;
        self.count += partition_capacity;

        Ok(Self {
            name: partition_name.to_string(),
            origin: partition_origin,
            capacity: partition_capacity,
            count: 0,
            cells: vec![Cell::equilibrium(); partition_capacity],
            is_storehouse: false,
            parent: Some(self.name.clone()),
        })
    }

    /// Writes a Cell into this vessel at the specified Offset.
    ///
    /// # Invariant Physics
    /// If `offset >= capacity`, boundary physics triggers an immediate `-1: ROLLBACK`!
    pub fn write(&mut self, offset: usize, value: Cell) -> Result<QuadState, String> {
        if offset >= self.capacity {
            return Ok(QuadState::Rollback);
        }

        self.cells[offset] = value;
        if offset >= self.count {
            self.count = offset + 1;
        }

        Ok(QuadState::Proceed)
    }

    /// Reads a Cell from this vessel at the specified Offset.
    ///
    /// # Invariant Physics
    /// If `offset >= capacity`, returns `Cell::from_state(QuadState::Rollback)`.
    pub fn read(&self, offset: usize) -> Cell {
        if offset >= self.capacity {
            Cell::from_state(QuadState::Rollback)
        } else {
            self.cells[offset]
        }
    }

    /// Packs the entire vessel into a dense array of physical 8-bit bytes (1 byte per cell).
    pub fn pack_bytes(&self) -> Vec<u8> {
        self.cells.iter().map(|c| c.pack()).collect()
    }

    /// Reconstructs a Vessel directly from raw physical bytes (e.g. from RAM, MMIO, or disk).
    pub fn unpack_bytes(name: &str, bytes: &[u8]) -> Self {
        let cells: Vec<Cell> = bytes.iter().map(|&b| Cell::unpack(b)).collect();
        let capacity = cells.len();
        Self {
            name: name.to_string(),
            origin: 0,
            capacity,
            count: capacity,
            cells,
            is_storehouse: false,
            parent: None,
        }
    }
}

impl GrammaticalEntity for Vessel {
    fn classify(&self) -> GrammaticalClassification {
        let role = if self.is_storehouse {
            "System Storehouse; root reservoir of physical or virtual memory cells"
        } else if self.parent.is_some() {
            "Entrusted Partition; bounded memory slice derived from a storehouse"
        } else {
            "Bounded memory vessel; holds sequential cells under invariant law"
        };

        GrammaticalClassification::new(
            PartOfSpeech::Noun,
            Some(PartOfSpeech::Preposition),
            None,
            role,
            "Contiguous memory block / Buffer / Page frame",
        )
    }
}

impl GrammaticalEntity for Offset {
    fn classify(&self) -> GrammaticalClassification {
        GrammaticalClassification::new(
            PartOfSpeech::Preposition,
            Some(PartOfSpeech::Noun),
            None,
            "Spatial coordinate index within a vessel: Vessel[Offset]",
            "Array index / Memory byte offset",
        )
    }
}

impl GrammaticalEntity for Capacity {
    fn classify(&self) -> GrammaticalClassification {
        GrammaticalClassification::new(
            PartOfSpeech::Noun,
            Some(PartOfSpeech::Adjective),
            None,
            "Total allocated capacity measure of a vessel",
            "Buffer size / Maximum allocation limit",
        )
    }
}

impl GrammaticalEntity for Count {
    fn classify(&self) -> GrammaticalClassification {
        GrammaticalClassification::new(
            PartOfSpeech::Noun,
            Some(PartOfSpeech::Adjective),
            None,
            "Active or written element count within a vessel",
            "Active buffer length / Populated element count",
        )
    }
}

impl fmt::Display for Vessel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = if self.is_storehouse {
            "Storehouse"
        } else if self.parent.is_some() {
            "Partition"
        } else {
            "Vessel"
        };
        write!(
            f,
            "{kind} < {} > [ Capacity: {}, Count: {}, Origin: 0x{:X} ]",
            self.name, self.capacity, self.count, self.origin
        )
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Vessel Invariant Tests, Bounds Safety & Partition Verification )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vessel_creation_and_bounds_checking() {
        let mut vessel = Vessel::new("Scratchpad", 10);
        assert_eq!(vessel.capacity, 10);
        assert_eq!(vessel.count, 0);

        // In-bounds write
        let cell42 = Cell::from_natural(42).unwrap();
        let res = vessel.write(0, cell42).unwrap();
        assert_eq!(res, QuadState::Proceed);
        assert_eq!(vessel.count, 1);

        // Read back
        let read_cell = vessel.read(0);
        assert_eq!(read_cell.to_natural(), Some(42));
        assert_eq!(read_cell.state, QuadState::Proceed);

        // Out-of-bounds write -> Automatic ROLLBACK (-1)
        let oob_res = vessel.write(15, cell42).unwrap();
        assert_eq!(oob_res, QuadState::Rollback);

        // Out-of-bounds read -> Automatic ROLLBACK cell
        let oob_read = vessel.read(15);
        assert_eq!(oob_read.state, QuadState::Rollback);
    }

    #[test]
    fn test_storehouse_partitioning() {
        let mut storehouse = Vessel::new_storehouse("Main_RAM", 1024);
        assert!(storehouse.is_storehouse);

        // Partition 256 cells
        let mut partition = storehouse.partition("Process_1_Stack", 256).unwrap();
        assert_eq!(partition.capacity, 256);
        assert_eq!(partition.origin, 0);
        assert_eq!(storehouse.count, 256);

        // Write inside partition
        let val = Cell::from_natural(99).unwrap();
        assert_eq!(partition.write(255, val).unwrap(), QuadState::Proceed);
        assert_eq!(partition.count, 256);

        // Boundary breach inside partition triggers rollback
        assert_eq!(partition.write(256, val).unwrap(), QuadState::Rollback);

        // Storehouse exhaustion
        let err = storehouse.partition("Excessive", 1000);
        assert!(err.is_err());
    }

    #[test]
    fn test_dense_byte_packing_round_trip() {
        let mut vessel = Vessel::new("PackedVessel", 4);
        vessel.write(0, Cell::from_natural(10).unwrap()).unwrap();
        vessel.write(1, Cell::from_natural(20).unwrap()).unwrap();
        vessel.write(2, Cell::from_state(QuadState::Rollback)).unwrap();
        vessel.write(3, Cell::from_state(QuadState::Halt)).unwrap();

        let bytes = vessel.pack_bytes();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes[0], 10);
        assert_eq!(bytes[1], 20);
        assert_eq!(bytes[2], crate::core::cell::BYTE_ROLLBACK);
        assert_eq!(bytes[3], crate::core::cell::BYTE_HALT);

        let reconstructed = Vessel::unpack_bytes("Reconstructed", &bytes);
        assert_eq!(reconstructed.read(0).to_natural(), Some(10));
        assert_eq!(reconstructed.read(1).to_natural(), Some(20));
        assert_eq!(reconstructed.read(2).state, QuadState::Rollback);
        assert_eq!(reconstructed.read(3).state, QuadState::Halt);
    }
}
