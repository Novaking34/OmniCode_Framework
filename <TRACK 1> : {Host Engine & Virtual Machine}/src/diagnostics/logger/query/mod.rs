// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Episodic Query Subsystem Facade & Track 0 Cross-Scale Diagnostics )
// ==============================================================================
//! # OmniCode Episodic Query Subsystem: Fast Retrieval & Silicon Correlation
//!
//! # Architectural Scope & Purpose:
//! The `query` module serves as the primary intelligence retrieval organ for the
//! entire `diagnostics::logger` subsystem. While `inscriber` writes paired streams
//! (`.log` telemetry and `.scroll` cognitive traces), and `telemetry` captures
//! microsecond rollframes and Track 0 genesis baselines, the `query` subsystem
//! transforms this historical ledger into an instantaneously interrogatable memory bank.
//!
//! # Subsystem Architecture:
//! - [`index`]: Structural representations of indexed sessions ([`IndexedSession`]),
//!   fluent multi-criteria query builders ([`SessionCriteria`]), statistical aggregates
//!   ([`QueryResult`], [`QuerySummary`]), and Track 0 bare-metal silicon logs ([`Track0LogRecord`]).
//! - [`search`]: High-performance [`EpisodicQueryEngine`] featuring multi-key $O(1)$
//!   hash indexing, chronological sorting, binary range slicing, inverted prompt
//!   keyword indexing, and Track 0 silicon flight recorder log discovery.
//!
//! # Scripture Witness:
//! - **Job 8:8 (KJV)**:
//!   "For enquire, I pray thee, of the former age, and prepare thyself to the search of their fathers."
//!   (Examining past execution history to diagnose errors and prevent regression).
//! - **Proverbs 25:2 (KJV)**:
//!   "It is the glory of God to conceal a thing: but the honour of kings is to search out a matter."
//!   (Systematic inquiry into internal state vectors, registers, and failpoints).
//! - **Jeremiah 29:13 (KJV)**:
//!   "And ye shall seek me, and find me, when ye shall search for me with all your heart."
//!   (Exhaustive, non-probabilistic diagnostic verification).
//!
//! # Why Query Capability is Essential:
//! In OmniCode, a log is not a discard dump. The cognitive AI (`AICortex`, `AILearner`)
//! and the autonomic immune system (`Restorator`, `Debugger`) must be able to query
//! past runs in $O(1)$ time to self-heal, prevent hallucination, and correlate high-level
//! software faults with bare-metal Track 0 silicon landmarks (`0x18000`, `0x108000`, `0x200000`).
// ==============================================================================

pub mod index;
pub mod search;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Public Type Re-exports & Subsystem Invariants )
// ==============================================================================

pub use index::{
    IndexedSession, QueryResult, QuerySummary, SessionCriteria, Track0LogRecord,
};
pub use search::EpisodicQueryEngine;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Facade Constructors, Quick-Search Utilities & Diagnostic Helpers )
// ==============================================================================

/// Constructs a fully populated [`EpisodicQueryEngine`] pointing to `storehouse_root`.
pub fn create_query_engine_at(storehouse_root: &std::path::Path) -> Result<EpisodicQueryEngine, String> {
    let mut engine = EpisodicQueryEngine::new();
    engine.index_directory(storehouse_root)?;
    Ok(engine)
}

/// Discovers an indexed session by ID or alias directly from a storage directory.
pub fn query_session_by_id(
    storehouse_root: &std::path::Path,
    session_id: &str,
) -> Option<IndexedSession> {
    let engine = create_query_engine_at(storehouse_root).ok()?;
    engine.find_by_session(session_id).cloned()
}

/// Diagnoses failpoints for a given session into a human-readable dossier.
pub fn diagnose_session(
    engine: &EpisodicQueryEngine,
    session_id: &str,
) -> Option<String> {
    engine.inspect_session_detail(session_id)
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Unit Test Verification Proofs & Query Invariant Seals )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_query_subsystem_re_exports_and_facade() {
        let engine = EpisodicQueryEngine::new();
        assert_eq!(engine.all_sessions().len(), 0);

        let criteria = SessionCriteria::new()
            .with_category("character")
            .with_limit(10);
        let results = engine.query(&criteria);
        assert_eq!(results.sessions.len(), 0);
        assert_eq!(results.summary.total_matched, 0);
    }

    #[test]
    fn test_facade_quick_search_nonexistent_returns_empty() {
        let empty_path = PathBuf::from("/nonexistent/omnicode/storage/scrolls");
        let result = create_query_engine_at(&empty_path).unwrap();
        assert_eq!(result.all_sessions().len(), 0);
    }
}
