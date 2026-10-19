// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Core Subsystem Identity, Module Architecture, and Invariant Re-exports )
// =============================================================================
//! OmniCode Core Subsystem
//! Encapsulates the algebraic foundation: Ternary Quad-State, Boundaries,
//! and Operators.

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
pub mod scripture_coord;


// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations for Direct Core Access )
// =============================================================================
pub use ternary::{
    QuadState, TransportAction, AWAIT, HALT, PROCEED, ROLLBACK,
};
pub use boundaries::BoundaryType;
pub use operators::{
    AffixPosition, BeingRelation, Connective, DirectionalVector, Polarity,
};
pub use filetype::OmniFileType;
pub use linguistics::{
    GrammaticalClassification, GrammaticalEntity, PartOfSpeech,
    PronounCase, PronounKind, PronounNumber, PronounPerson,
};
pub use cell::{
    Cell, Trit, BYTE_AWAIT, BYTE_HALT, BYTE_PROCEED, BYTE_ROLLBACK,
};
pub use vessel::{Capacity, Count, Offset, Vessel};
pub use tape::{Roll, Tape, TapeHead};
pub use word::{Word32Trit, Word64};
pub use scene::Scene;
pub use reel::MemoryReel;
pub use mmio::{
    PhysicalMemoryMap, SerialPort, VgaBuffer, VgaCell, VgaColor,
    COM1_SERIAL_PORT, VGA_BUFFER_BASE,
};
pub use scripture_coord::{
    audit_scripture_coordinate_engine, BookSpec, DualWitnessRole, Point3D,
    ScriptureAuditReport, ScriptureCoordinate, ScriptureCoordinateEngine,
    ScriptureVerse, ShapeCycle, ShapeCycleStage, SpatialBlock, SpatialCircle,
    Testament, Triangulation, VariantVerseEntry, CANONICAL_BIBLE_BOOKS,
    WEB_VARIANT_INDEX,
};




// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Subsystem Invariant Definitions )
// =============================================================================
// Invariants and operators are actively materialized via re-exported
// submodules.


// =============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification & Subsystem Status )
// =============================================================================
// Core module namespace verified and closed.
