pub mod ternary;
pub mod boundaries;
pub mod operators;

pub use ternary::{QuadState, ROLLBACK, AWAIT, PROCEED, HALT};
pub use boundaries::BoundaryType;
pub use operators::{BeingRelation, DirectionalVector, Connective};
