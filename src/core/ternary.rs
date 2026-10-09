// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Module Identity, Balanced Ternary Invariants, and Mathematical Basis )
// ==============================================================================
//! OmniCode Core: Balanced Ternary Quad-State Engine
//!
//! Grounded in Balanced Radix-3 logic with a terminal Null/Void state:
//!     T4 = { -1 (Rollback), 0 (Await), +1 (Proceed), NULL (Halt) }
//!
//! Algebraic properties:
//!     - Symmetry: NOT(x) = -x
//!     - Conjunction (:: AND): min(A, B) with NULL absorption
//!     - Disjunction (| OR): max(A, B) with NULL absorption
//!     - Adversative Caveat (! BUT): Action A constrained by Caveat B

use std::fmt;

// Invariant QuadState Constants
pub const ROLLBACK: QuadState = QuadState::Rollback;
pub const AWAIT: QuadState = QuadState::Await;
pub const PROCEED: QuadState = QuadState::Proceed;
pub const HALT: QuadState = QuadState::Halt;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Context Declarations, State Enum, and Constructor Initialization )
// ==============================================================================
/// The four fundamental ontological states of computation in OmniCode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuadState {
    /// -1: NO / Rollback. Triggers retroactive vector (`<--`), unwinds mutated state.
    Rollback,
    /// 0: OK / Await. Equilibrium, standby, asynchronous yield, pass-through.
    Await,
    /// +1: YES / Proceed. Advances forward vector (`-->`), commits state.
    Proceed,
    /// NULL: VOID / Halt. Bottom type (⊥), boundary trap, terminal exit.
    Halt,
}

impl QuadState {
    /// Parses a raw integer or string representation into a QuadState.
    ///
    /// # Failpoints
    /// Returns `Err` if the value does not belong to {-1, 0, 1, NULL}.
    pub fn from_raw(raw: &str) -> Result<Self, String> {
        match raw.trim().to_uppercase().as_str() {
            "-1" | "NO" | "ROLLBACK" => Ok(QuadState::Rollback),
            "0" | "OK" | "AWAIT" => Ok(QuadState::Await),
            "1" | "+1" | "YES" | "PROCEED" => Ok(QuadState::Proceed),
            "NULL" | "VOID" | "HALT" => Ok(QuadState::Halt),
            other => Err(format!(
                "FAILPOINT [InvalidQuadState]: '{other}' is not a valid balanced ternary state. \
                 Permitted states: -1, 0, +1, NULL"
            )),
        }
    }

    /// Returns the integer value, or None for Halt (NULL).
    pub fn as_int(&self) -> Option<i8> {
        match self {
            QuadState::Rollback => Some(-1),
            QuadState::Await => Some(0),
            QuadState::Proceed => Some(1),
            QuadState::Halt => None,
        }
    }

    /// Micro level (linguistic / logical / epistemic stance):
    /// - Proceed: "yes" (1)
    /// - Rollback: "no" (-1)
    /// - Await: "ok" (0)
    /// - Halt: "void" (NULL)
    pub fn micro(&self) -> &'static str {
        match self {
            QuadState::Proceed => "yes",
            QuadState::Rollback => "no",
            QuadState::Await => "ok",
            QuadState::Halt => "void",
        }
    }

    /// Macro level (systems / OS state-machine physics):
    /// - Proceed: "PROCEED" (commit state / advance PC)
    /// - Rollback: "ROLLBACK" (unwind snapshot / retry)
    /// - Await: "AWAIT" (async yield / wait queue)
    /// - Halt: "HALT" (boundary trap / terminal exit)
    pub fn macro_op(&self) -> &'static str {
        match self {
            QuadState::Proceed => "PROCEED",
            QuadState::Rollback => "ROLLBACK",
            QuadState::Await => "AWAIT",
            QuadState::Halt => "HALT",
        }
    }

    /// Bridges the Micro and Macro ontological perspective.
    pub fn description(&self) -> &'static str {
        match self {
            QuadState::Proceed => "+1 / yes (Micro: Affirmation) | PROCEED (Macro: Forward commit)",
            QuadState::Rollback => "-1 / no (Micro: Refutation) | ROLLBACK (Macro: Retroactive rewind)",
            QuadState::Await => "0 / ok (Micro: Neutral balance) | AWAIT (Macro: Async yield / Standby)",
            QuadState::Halt => "NULL / void (Micro: Absence) | HALT (Macro: Terminal trap / Exit)",
        }
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Algebraic Transformations: Inversion, Conjunction, Disjunction, Caveat )
// ==============================================================================
impl QuadState {
    /// Algebraic Inversion (`--` NOT).
    ///
    /// Rules:
    /// - `--(+1)` == `-1` (NOT Proceed is Rollback)
    /// - `--(-1)` == `+1` (NOT Rollback is Proceed)
    /// - `--(0)`  ==  `0` (NOT Await is Await - zero is symmetric balance)
    /// - `--(NULL)` == `NULL` (NOT Halt is Halt - void remains void)
    pub fn invert(&self) -> Self {
        match self {
            QuadState::Proceed => QuadState::Rollback,
            QuadState::Rollback => QuadState::Proceed,
            QuadState::Await => QuadState::Await,
            QuadState::Halt => QuadState::Halt,
        }
    }

    /// Cumulative Conjunction (`::` AND).
    ///
    /// Evaluates to minimum with NULL acting as an absorbing trap.
    pub fn and(&self, other: Self) -> Self {
        if *self == QuadState::Halt || other == QuadState::Halt {
            return QuadState::Halt;
        }
        let a = self.as_int().unwrap();
        let b = other.as_int().unwrap();
        match a.min(b) {
            -1 => QuadState::Rollback,
            0 => QuadState::Await,
            1 => QuadState::Proceed,
            _ => unreachable!(),
        }
    }

    /// Disjunction (`|` OR).
    ///
    /// Evaluates to maximum with NULL acting as an absorbing trap.
    pub fn or(&self, other: Self) -> Self {
        if *self == QuadState::Halt || other == QuadState::Halt {
            return QuadState::Halt;
        }
        let a = self.as_int().unwrap();
        let b = other.as_int().unwrap();
        match a.max(b) {
            -1 => QuadState::Rollback,
            0 => QuadState::Await,
            1 => QuadState::Proceed,
            _ => unreachable!(),
        }
    }

    /// Adversative Caveat (`!` BUT).
    ///
    /// `action.but(caveat)`: Action proceeds ONLY if caveat does not veto.
    /// - Caveat = Rollback (-1) -> forces Rollback (-1)
    /// - Caveat = Halt (NULL)   -> forces Halt (NULL)
    /// - Caveat = Await (0)     -> degrades Proceed to Await (0)
    /// - Caveat = Proceed (+1)  -> action executes unhindered
    pub fn but(&self, caveat: Self) -> Self {
        match caveat {
            QuadState::Halt => QuadState::Halt,
            QuadState::Rollback => QuadState::Rollback,
            QuadState::Await => {
                if *self == QuadState::Proceed {
                    QuadState::Await
                } else {
                    *self
                }
            }
            QuadState::Proceed => *self,
        }
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Verification, Formatter Proofs, Display Formatting, and Export )
// ==============================================================================
impl fmt::Display for QuadState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuadState::Rollback => write!(f, "-1:ROLLBACK"),
            QuadState::Await => write!(f, "0:AWAIT"),
            QuadState::Proceed => write!(f, "+1:PROCEED"),
            QuadState::Halt => write!(f, "NULL:HALT"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quad_state_micro_and_macro() {
        // Micro labels (linguistic stance)
        assert_eq!(QuadState::Proceed.micro(), "yes");
        assert_eq!(QuadState::Rollback.micro(), "no");
        assert_eq!(QuadState::Await.micro(), "ok");
        assert_eq!(QuadState::Halt.micro(), "void");

        // Macro labels (systems execution physics)
        assert_eq!(QuadState::Proceed.macro_op(), "PROCEED");
        assert_eq!(QuadState::Rollback.macro_op(), "ROLLBACK");
        assert_eq!(QuadState::Await.macro_op(), "AWAIT");
        assert_eq!(QuadState::Halt.macro_op(), "HALT");

        // Parsing micro and macro strings
        assert_eq!(QuadState::from_raw("yes").unwrap(), QuadState::Proceed);
        assert_eq!(QuadState::from_raw("no").unwrap(), QuadState::Rollback);
        assert_eq!(QuadState::from_raw("ok").unwrap(), QuadState::Await);
        assert_eq!(QuadState::from_raw("void").unwrap(), QuadState::Halt);

        assert_eq!(QuadState::from_raw("PROCEED").unwrap(), QuadState::Proceed);
        assert_eq!(QuadState::from_raw("ROLLBACK").unwrap(), QuadState::Rollback);
        assert_eq!(QuadState::from_raw("AWAIT").unwrap(), QuadState::Await);
        assert_eq!(QuadState::from_raw("HALT").unwrap(), QuadState::Halt);
    }
}

