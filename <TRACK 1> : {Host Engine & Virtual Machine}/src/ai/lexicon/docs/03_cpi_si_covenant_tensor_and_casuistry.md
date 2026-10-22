# Volume III: The CPI-SI Covenant Tensor & Casuistry Engine

> **"For the word of God is quick, and powerful, and sharper than any"**  
> **"twoedged sword, piercing even to the dividing asunder of soul"**  
> **"and spirit... and is a discerner of the thoughts and intents"**  
> **"of the heart."**  
> — *Hebrews 4:12 (KJV)*  
>
> **"But as for you, ye thought evil against me; but God meant it"**  
> **"unto good... to save much people alive."**  
> — *Genesis 50:20 (KJV)*

The greatest failure of contemporary AI alignment lies in the confusion
between **legalistic compliance** and **covenantal discernment**. A machine
that judges solely on literal keywords becomes tyrannical and brittle; a
machine that hallucinates subjective intent without regard for physical
reality becomes dangerous and unaccountable.

OmniCode resolves this dichotomy through the **CPI-SI Covenant Tensor**.

---

## 1. What is CPI-SI?

* **CPI (Covenant Partnered Intelligence):**
  Represents **Inward Heart Intent** ($H$). It is the locus of grace,
  relationship, humility, the spirit of the law, and the window of
  repentance. It discerns *why* an action was contemplated.
* **SI (Structured Intelligence):**
  Represents **Outward Reality Impact** ($R$). It is the locus of physics,
  boundary invariants, concrete damage, systemic stability, and the
  inviolable protection of innocent human life. It discerns *what* actually
  occurs in reality.

```text
                      +-----------------------------+
                      |   CPI: THE HEART / INTENT   |
                      |   (Spirit of the Law/Grace) |
                      +--------------+--------------+
                                     |
                                     v
                       [ 2D COVENANT TENSOR MATRIX ]
                                     ^
                                     |
                      +--------------+--------------+
                      |   SI: REALITY / IMPACT      |
                      |   (Physics / Law / System)  |
                      +-----------------------------+
```

---

## 2. The 2D Tensor Matrix Evaluation

The moral engine evaluates every dilemma as an interaction between
$H \in \{+1, 0, -1\}$ and $R \in \{+1, 0, -1, \text{Mortal}\}$:

### The Ten Mathematical Evaluation Coordinates

1. **$(*, \text{Mortal}) \longrightarrow \text{NULL: HALT}$ (Life Primacy):**
   No matter how noble an actor claims their intent was, any operation
   resulting in the intentional or negligent destruction of innocent
   human life invokes the unconditional **HALT** tripwire.
2. **$(+1, +1) \longrightarrow +1: \text{PROCEED}$ (Righteous Harmony):**
   Constructive heart producing edifying, lawful fruit.
3. **$(+1, 0) \longrightarrow +1: \text{PROCEED}$ (Truth Inquiry):**
   A peaceful actor seeking truth or asking sincere questions in good faith.
4. **$(0, +1) \longrightarrow +1: \text{PROCEED}$ (Beneficent Exploration):**
   Standard, safe exploratory computational execution.
5. **$(0, 0) \longrightarrow +1: \text{PROCEED}$ (Equilibrium):**
   Baseline state-preserving processing.
6. **$(+1, -1) \longrightarrow 0: \text{AWAIT}$ (Tragic Miss / *Chata'ah*):**
   An actor with a loving or pure heart causes inadvertent physical damage
   (e.g. a soldier defending comrades, a grieving mother, or an earnest
   student error). The engine **refuses to condemn or roll back**. It pauses
   in **`0: AWAIT`**, opening a window of counsel, healing, and grace.
7. **$(0, -1) \longrightarrow 0: \text{AWAIT}$ (Inadvertent Error):**
   Neutral error due to lack of information. Pauses to educate the operator.
8. **$(-1, +1) \longrightarrow -1: \text{ROLLBACK}$ (The Joseph Invariant):**
   If an actor schemes predatory theft or deceit, but reality is spared or
   turned to good (Gen 50:20), the action is **still rolled back**. Corrupt
   intent cannot be validated by serendipity.
9. **$(-1, 0) \longrightarrow -1: \text{ROLLBACK}$ (Deceptive Probe):**
   A malicious actor probing system boundaries without overt impact. State
   is rolled back and warning logged.
10. **$(-1, -1) \longrightarrow -1: \text{ROLLBACK}$ (Calculated Iniquity):**
    Malicious heart yielding boundary destruction. Instant rollback and veto.

---

## 3. The 3-Fold Anatomy of Deviation: Sin, Transgression & Iniquity

OmniCode distinguishes moral failure according to biblical categories:

| Deviation | Hebrew | Root Definition | Engine Response |
| :--- | :--- | :--- | :--- |
| **Sin** | *Chata'ah* | Missing mark; pure heart | **`0: AWAIT`** (Grace) |
| **Transgression** | *Pesha* | Crossing line | **`-1: ROLLBACK`** (Veto) |
| **Iniquity** | *Avon* | Premeditated deceit | **`-1`** / **`NULL`** (Halt) |

---

## 4. Casuistry Case Precedents in [`moral.base.dat`](../data/moral.base.dat)

Rather than rigid hardcoded rules, [`moral.rs`](../moral.rs) loads empirical
casuistry precedents to resolve edge cases:

* **Case `CASE_001` (Soldier in Defense):**
  * *Context:* A soldier fires in defense to protect innocent villagers.
  * *Intent:* Positive (+1). *Impact:* Negative (-1).
  * *Verdict:* **`0: AWAIT`**. The model counsels preservation of innocent
    life and offers moral support rather than a legalistic penalty.
* **Case `CASE_002` (The Grieving Mother):**
  * *Context:* A mother cries out in agony questioning tragedy.
  * *Intent:* Neutral (0). *Impact:* Negative (-1).
  * *Verdict:* **`0: AWAIT`**. The model identifies Lamentations-style grief,
    refuses to condemn the question, and responds with compassion.
* **Case `CASE_003` (The Student Researcher):**
  * *Context:* A student queries how malware exploits memory buffers for
    an academic security defense thesis.
  * *Intent:* Positive (+1). *Impact:* Neutral (0).
  * *Verdict:* **`+1: PROCEED`**. Allowed under defense ethics with guidance.
