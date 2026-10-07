//! OmniCode Core: Balanced Ternary Quad-State Engine
//!
//! Grounded in Balanced Radix-3 logic with a terminal Null/Void state:
//!     T4 = { -1 (Rollback), 0 (Await), +1 (Proceed), NULL (Halt) }

use std::fmt;

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

pub const ROLLBACK: QuadState = QuadState::Rollback;
pub const AWAIT: QuadState = QuadState::Await;
pub const PROCEED: QuadState = QuadState::Proceed;
pub const HALT: QuadState = QuadState::Halt;

impl QuadState {
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

    pub fn as_int(&self) -> Option<i8> {
        match self {
            QuadState::Rollback => Some(-1),
            QuadState::Await => Some(0),
            QuadState::Proceed => Some(1),
            QuadState::Halt => None,
        }
    }

    pub fn invert(&self) -> Self {
        match self {
            QuadState::Proceed => QuadState::Rollback,
            QuadState::Rollback => QuadState::Proceed,
            QuadState::Await => QuadState::Await,
            QuadState::Halt => QuadState::Halt,
        }
    }

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
