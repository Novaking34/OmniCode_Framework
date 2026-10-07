# Volume 3: Architecture & Execution

## Document 01: The 3D Coordinate System of Computation

---

### 1. The Three Orthogonal Axes

Every computational entity, transformation, and expression in OmniCode is positioned within a 3-dimensional coordinate space:

```text
                               ▲ +Z (Scale: Layer Up / Builds To: ^--)
                               │
                               │
       -X (Action: Backward)   │       +X (Action: Forward)
       <-- Act Again           │       --> Act Upon
       ────────────────────────┼────────────────────────►
      (Feedback / Rollback)    │       (Causation / Pipeline)
                               │
                               │
                               ▼ -Z (Scale: Layer Down / Breaks Down From: v--)

                [ Y-Axis: Being / Identity: = (IS), != (IS NOT), ≈ (IS LIKE) ]
```

---

### 2. The Orthogonal Dimensions

1. **The X-Axis: Temporal / Causal Vector (`-->` vs `<--`)**
   * Governs forward causation vs. retroactive feedback/unwinding.
   * Interacts directly with the Quad-State engine: `+1` moves along $+X$, `-1` moves along $-X$.
2. **The Y-Axis: Ontological Identity (`=`, `!=`, `≈`)**
   * Governs equivalence and relation of being in state space.
3. **The Z-Axis: Abstraction Scale (`^--` vs `v--`)**
   * Governs the movement between low-level hardware representation and high-level conceptual modeling.
   * `^--` constructs high-level types from low-level memory layouts.
   * `v--` projects high-level models down to machine primitives.
