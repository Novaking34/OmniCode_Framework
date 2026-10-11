# Volume 2: Logic & Algebra

## Document 02: Relational Connectives & Operators

---

### 1. Relational Connectives

OmniCode incorporates natural linguistic and relational operators into its formal grammar:

#### A. The Adversative Conjunction (`!` BUT)

In traditional Boolean logic, "A but B" is flattened into a naive "A and B". In OmniCode, **`!` (BUT)** introduces an explicit **adversative constraint / caveat**:
$$A \text{ ! } B \iff A \text{ executes, BUT must satisfy caveat } B$$
If caveat $B$ evaluates to `-1` (Rollback) or `NULL` (Halt), the execution of $A$ is immediately unwound and suppressed.

#### B. Cumulative Conjunction (`::` AND)

Represents co-presence and mutual satisfaction. Both sides must hold or execute simultaneously.

#### C. Relational Projection (`:` IN RELATION TO / TO BE AS)

Maps an entity to a perspective, category, or role.

* Used for typing: `x : Integer` ("x in relation to Integer").
* Used for contextual roles: `user : Admin` ("user in relation to Admin role").

#### D. Disjunction (`|` OR)

Represents branching alternatives and non-deterministic choice.

#### E. Negation (`--` NOT)

Inverts the quad-state value across the zero balance line.

---

### 2. Relations of Being (Ontological Identity)

| Operator | Plain Language | Mathematical Concept | Meaning |
| :---: | :--- | :--- | :--- |
| **`=`** | **IS / TO BE** | Identity ($\equiv$) | Exact equivalence in value, shape, and essence. |
| **`!=`** | **IS NOT / TO BE NOT** | Non-identity ($\not\equiv$) | Distinctness; exclusion. |
| **`≈`** or **`~=`** | **TO BE LIKE / Liken To** | Homomorphism / Analogy | Structural compatibility; behavioral similarity without identical essence. |

---

### 3. Directional Flow Operators

| Operator | Plain Language | Vector Direction | Role |
| :---: | :--- | :---: | :--- |
| **`-->`** | **Act Upon** | Forward ($\rightarrow$) | Causation, pipeline, execution step, event dispatch. |
| **`<--`** | **Act Again** | Backward ($\leftarrow$) | Feedback, retry, undo, rollback, reactive subscription. |
| **`^--`** | **Which Builds To** | Upward ($\uparrow$) | Layering up, abstraction, synthesis, composition. |
| **`v--`** | **Which Breaks Down From** | Downward ($\downarrow$) | Decomposition, reduction, destructuring, compilation to IR. |
