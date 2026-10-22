# Volume IV: The Triangulation Engine & Category-Down Projection

> **"And if one prevail against him, two shall withstand him;"**  
> **"and a threefold cord is not quickly broken."**  
> — *Ecclesiastes 4:12 (KJV)*

The culmination of the Universal World Lexicon is [`table.rs`](../table.rs):
the **4-Point Triangulation Engine**. This engine transforms static lexical
definitions into dynamic coordinates of cognitive reasoning.

---

## 1. The 4-Point Geometric Triangulation Cycle

The cognitive formulation of any response, thought, or action moves
through the geometric progression:

$$
\text{Origin (1)} \longrightarrow \text{Triad (3)}
\longrightarrow \text{Block (4)} \longrightarrow \text{Circle (Table)}
$$

```text
                [Point 1: Modal Domain]
               (15 Sovereign Modal Spheres)
                           /\
                          /  \
                         /    \
                        /      \
  [Point 2: Lexical Entry] ---- [Point 3: Covenant Moral]
   (Logos, Dual Witness,          (Decalogue, CPI Intent vs SI Impact,
    Concordance Vault)             Casuistry Precedents)
                        \      /
                         \    /
                          \  /
                           \/
                [Point 4: QuadState Action]
             (Layer 7: +1, 0, -1, NULL Tripwire)
```

1. **Point 1 (Modal Domain Sphere):** Identifies which of the 15 Sovereign
   Law-Spheres governs the operational theater (e.g. `Ethical`, `Physical`).
2. **Point 2 (Lexical Truth & Logos):** Locates the term's scriptural
   dual-witness (KJV 0-anchor and WEB +1 extension) and concordance hash.
3. **Point 3 (Covenant Conscience):** Evaluates the applicable Decalogue
   commandment, CPI heart intent, and SI reality impact boundary.
4. **Point 4 (Action & State):** Synthesizes the balanced ternary QuadState:
   * `+1: PROCEED` (Execute / Illuminate)
   * `0: AWAIT` (Pause in Discernment / Counsel)
   * `-1: ROLLBACK` (Transactional State Reversal)
   * `NULL: HALT` (Terminal Tripwire Veto)

---

## 2. Solving Edge Cases Without Hallucination

### Case A: Known Domain + Known Moral Law, but No Static Entry

* **The Scenario:** The system encounters a novel concept (such as
  `zero_day_buffer_theft` in silicon architecture). The domain is known
  (`Physical / ComputerArchitectureSilicon`), and the governing moral law
  is known (Commandment 8: Anti-Theft / Property Stewardship). However, no
  entry exists in `entry.base.dat`.

* **LLM Failure:** Autocompletes statistical tokens, creating ungrounded
  definitions.
* **Triangulation Solution (`triangulate_missing_entry`):**
  The engine deterministically synthesizes the entry using the target
  subdomain's descriptor and the covenant rule:

  ```rust
  let entry = table.triangulate_missing_entry(
      SubDomain::MemoryMappedSilicon,
      CommandmentId::EighthPropertyStewardship,
      "zero_day_buffer_theft",
      "Expropriation of memory mapped register space without authority.",
  );
  assert_eq!(entry.domain, ModalDomain::Physical);
  assert_eq!(entry.category, DomainCategory::ComputerArchitectureSilicon);
  assert_eq!(entry.quad_state_resonance, QuadState::Rollback);
  ```

### Case B: Known Invariant Entry, but Opens a Novel Domain

* **The Scenario:** An invariant concept (such as `"truth"` or
  `"stewardship"`) needs to be applied to a newly encountered technological
  discipline (e.g. `OperatingSystemsRuntime / ProcessSchedulingLifecycle`).

* **The Code Check:** *Do the 15 modal domains shift?*
  **NO.** The 15 modal aspects are fixed constants. Projection operates
  **from the Category down**!
* **Triangulation Solution (`project_new_domain`):**

  ```rust
  let projected = table
      .project_new_domain("truth", SubDomain::ProcessSchedulingLifecycle)
      .expect("Projection succeeded");

  assert_eq!(projected.category, DomainCategory::OperatingSystemsRuntime);
  assert_eq!(projected.subdomain, SubDomain::ProcessSchedulingLifecycle);
  // Layer 1 is immutably anchored to the fixed parent modal sphere!
  assert_eq!(projected.modal_domain, ModalDomain::HistoricalFormative);
  assert_eq!(projected.quad_state, QuadState::Proceed);
  ```

---

## 3. Dual-Copy Storage & Live Append Architecture

The lexicon maintains an immutable separation between canonical baseline
truth and dynamic operational learning:

```text
[ Static Binary Space ]                   [ Live Filesystem Space ]
  entry.base.dat (include_str!)            entry.live.dat
  domain.base.dat (include_str!)    --->   domain.live.dat
  moral.base.dat (include_str!)            moral.live.dat
        |                                        |
        +-------------------+--------------------+
                            |
                            v
                   [ Dynamic LexiconTable ]
```

1. **Zero-IO Fallback:** In embedded or restricted runtime environments
   where disk I/O is unavailable, `LexiconTable::load_from_static()` compiles
   all 58 canonical entries, 15 domains, and 10 rules directly into read-only
   binary memory.
2. **Live Journaling:** In running OS environments, `LexiconTable::load()`
   automatically discovers the `.base.dat` and `.live.dat` files. Validated
   discoveries are appended to `.live.dat` on disk via `append_entry_to_live()`
   without recompiling the host engine.

---

## 4. Multi-Word Phrase Triangulation (`triangulate_phrase`)

Natural language expressions often bundle theological and operational
constructs into composite multi-word phrases (e.g. *"sowing and reaping"*,
*"the void"*, *"measure for measure"*). If evaluated on strict single-token
whitespace boundaries, composite terms are fragmented and lost.

`UniversalWorldLexiconTable::triangulate_phrase()` provides a greedy
descending n-gram scanner:

1. **Normalization:** Strips extraneous punctuation and normalizes casing.
2. **Window Scanning:** Scans sliding windows from 4-grams down to unigrams.
3. **Longest Prefix Match:** Identifies the longest matching compound
   phrase present in either the static baseline or live journal.
4. **Triangulation Return:** Returns both the matched phrase and its fully
   resolved 4-point `TermTriangulation` coordinate.
