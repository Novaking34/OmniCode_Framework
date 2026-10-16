// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Assembly Pre-flight Subsystem Coordinator )
// ==============================================================================
//! Assembly Pre-flight Linter Subsystem
//!
//! Enforces hardware invariants, memory partition sizing, MMIO mappings,
//! and delimiter stack integrity before machine lowering.

pub mod assembly;

pub use assembly::AssemblyLinter;

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Status )
// ==============================================================================
// Preflight subsystem sealed.
