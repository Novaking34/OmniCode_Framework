// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Core Subsystem Identity, Module Architecture, and Invariant Re-exports )
// ==============================================================================
//! OmniCode Core Subsystem
//! Encapsulates the algebraic foundation: Ternary Quad-State, Boundaries, and Operators.

pub mod ternary;
pub mod boundaries;
pub mod operators;
pub mod filetype;
pub mod linguistics;
pub mod cell;
pub mod vessel;
pub mod tape;
pub mod word;
pub mod scene;
pub mod reel;
pub mod mmio;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Direct Core Access )
// ==============================================================================
pub use ternary::{QuadState, TransportAction, ROLLBACK, AWAIT, PROCEED, HALT};
pub use boundaries::BoundaryType;
pub use operators::{BeingRelation, DirectionalVector, Connective, Polarity, AffixPosition};
pub use filetype::OmniFileType;
pub use linguistics::{PartOfSpeech, GrammaticalClassification, GrammaticalEntity, PronounKind, PronounCase, PronounPerson, PronounNumber};
pub use cell::{Cell, Trit, BYTE_HALT, BYTE_AWAIT, BYTE_ROLLBACK, BYTE_PROCEED};
pub use vessel::{Vessel, Offset, Capacity, Count};
pub use tape::{Roll, Tape, TapeHead};
pub use word::{Word32Trit, Word64};
pub use scene::Scene;
pub use reel::MemoryReel;
pub use mmio::{PhysicalMemoryMap, VgaBuffer, VgaColor, VgaCell, SerialPort, VGA_BUFFER_BASE, COM1_SERIAL_PORT};




// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Subsystem Invariant Definitions )
// ==============================================================================
// Invariants and operators are actively materialized via re-exported submodules.


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification & Subsystem Status )
// ==============================================================================
// Core module namespace verified and closed.
