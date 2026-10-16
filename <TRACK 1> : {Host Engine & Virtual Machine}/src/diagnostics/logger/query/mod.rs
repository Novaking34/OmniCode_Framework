// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Episodic Query Subsystem Re-exports )
// ==============================================================================
//! Episodic Query Subsystem: Indexing & Fast Retrieval

pub mod index;
pub mod search;

pub use index::IndexedSession;
pub use search::EpisodicQueryEngine;
