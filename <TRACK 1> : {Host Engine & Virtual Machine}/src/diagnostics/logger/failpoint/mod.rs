// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Failpoint Subsystem Re-exports & Diagnostics Facade )
// ==============================================================================
//! Failpoint Pathology & Remediation Telemetry Module

pub mod types;
pub mod remediation;

pub use types::FailpointType;
pub use remediation::{FixKind, FixSuggestion};
