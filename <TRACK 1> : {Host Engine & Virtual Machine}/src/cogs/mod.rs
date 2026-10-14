// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal Cogs Module Identity & Mechanical Dynamic Primitives )
// ==============================================================================
//! OmniCode Cogs: Dynamic Mechanical Primitives & Decoupled Functions
//!
//! Reusable single-purpose dynamic utilities shared across gears:
//! - Dynamic host platform detection & environment adaptation (`platform`)
//! - Universal invariant tripwires and diagnostic interceptors (`tripwire`)
//! - Cross-track runtime path resolution

pub mod platform;
pub mod tripwire;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Cog Declarations & Re-exports )
// ==============================================================================
pub use platform::{HostPlatform, HostArch};
pub use tripwire::{Tripwire, TripwireError};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Cog Implementations & Shared Primitives )
// ==============================================================================

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification and Sealing )
// ==============================================================================
// Cogs subsystem verified and sealed.
