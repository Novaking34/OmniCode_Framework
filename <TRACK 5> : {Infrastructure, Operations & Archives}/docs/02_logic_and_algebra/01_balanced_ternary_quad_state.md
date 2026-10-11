# Volume 2: Logic & Algebra

## Document 01: Balanced Ternary & The Quad-State Engine

---

### 1. The Quad-State Truth Space & Micro/Macro Duality

OmniCode replaces Boolean logic (`0` and `1`) with a 4-state system grounded in **Balanced Ternary Logic** with a terminal **Null State**:

$$\mathbb{T}_4 = \{ -1, \; 0, \; +1, \; \text{NULL} \}$$

The engine operates across a fundamental **Micro / Macro Duality**:

| State Symbol | Micro Level (Epistemic / Stance) | Macro Level (Systems / Physics) | Directional Vector | OS / Machine Meaning |
| :---: | :--- | :--- | :---: | :--- |
| **`+1`** | **`yes`** (Affirmation / Condition holds) | **`PROCEED`** (Monotonic commit) | `-->` / `->` | Advance PC, commit transaction frame |
| **`0`** | **`ok`** (Neutral equilibrium / Balance) | **`AWAIT`** (Standby / Async yield) | `---` | Yield thread, wait queue, idle / `HLT` |
| **`-1`** | **`no`** (Refutation / Disagreement) | **`ROLLBACK`** (Retroactive unwind) | `<--` / `<-` | Unwind memory snapshot, retry vector |
| **`NULL`** | **`void`** (Absence / Non-existence) | **`HALT`** (Boundary trap / Bottom ⊥) | `-x-` | Trap handler, kernel panic, abort |

```text
    Micro (Linguistic):     [ no ]      ---      [ ok ]      ---      [ yes ]       -x-  [ void ]
                              │                    │                    │                     │
    Value:                   -1                    0                   +1                   NULL
                              │                    │                    │                     │
    Macro (Systems):      ROLLBACK    <--        AWAIT       -->     PROCEED        -x-     HALT
```

---

### 2. Algebraic Symmetry and Negation (`--`)

In binary logic, negation flips $0 \leftrightarrow 1$.
In OmniCode, negation is pure algebraic reflection around the zero equilibrium point:

$$\operatorname{NOT}(x) = -x$$

* $-(+1) = -1$ (NOT Proceed is **Rollback**)
* $-(-1) = +1$ (NOT Rollback is **Proceed**)
* $-(0) = 0$ (NOT Equilibrium is **Equilibrium**)
* $-(\text{NULL}) = \text{NULL}$ (NOT Void is **Void**)

---

### 3. Truth Tables for Fundamental Connectives

#### Conjunction (`::` AND)

Evaluates to the greatest lower bound (minimum):
$$\operatorname{AND}(A, B) = \min(A, B) \quad (\text{with NULL acting as an absorbing trap})$$

#### Disjunction (`|` OR)

Evaluates to the least upper bound (maximum):
$$\operatorname{OR}(A, B) = \max(A, B) \quad (\text{with NULL acting as an absorbing trap})$$

| $A$ | $B$ | $A :: B$ (AND) | $A \mid B$ (OR) | $--A$ (NOT) |
| :---: | :---: | :---: | :---: | :---: |
| $+1$ | $+1$ | $+1$ | $+1$ | $-1$ |
| $+1$ | $0$ | $0$ | $+1$ | $-1$ |
| $+1$ | $-1$ | $-1$ | $+1$ | $-1$ |
| $0$ | $0$ | $0$ | $0$ | $0$ |
| $0$ | $-1$ | $-1$ | $0$ | $0$ |
| $-1$ | $-1$ | $-1$ | $-1$ | $+1$ |
| $\text{NULL}$ | $*$ | $\text{NULL}$ | $\text{NULL}$ | $\text{NULL}$ |

---

### 4. Direct Mapping to Runtime Mechanics

1. **State `+1` (Proceed):** Advances the program counter, commits current state mutations, and passes through the forward arrow `-->`.
2. **State `0` (Await):** Suspends execution without failure, yielding CPU or awaiting asynchronous event resolution.
3. **State `-1` (Rollback):** Reverts all mutations executed within the current boundary and invokes the backward arrow `<--`.
4. **State `NULL` (Halt):** Immediate abort. Used for fatal assertions, memory out-of-bounds, or program exit.
