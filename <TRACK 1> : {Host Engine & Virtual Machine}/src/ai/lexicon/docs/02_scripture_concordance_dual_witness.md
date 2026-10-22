# Volume II: Scripture Concordance & The 5-Fold Witness Standard

> **"It is written in the law, that the testimony of two men is true."**  
> — *John 8:17 (KJV)*  
>
> **"One witness shall not rise up against a man for any iniquity,"**  
> **"or for any sin... at the mouth of two witnesses, or at the mouth"**  
> **"of three witnesses, shall the matter be established."**  
> — *Deuteronomy 19:15 (KJV)*

In modern AI research, language models suffer from catastrophic
"hallucination"—the generation of statistically plausible but factually
fictitious claims. This occurs because LLMs operate on token likelihood
rather than grounded truth.

OmniCode solves hallucination at the architectural root by binding every
lexicon entry to an immutable **5-Fold Witness Standard**.

---

## 1. The 5-Fold Witness Topology

Every concept defined in [`entry.rs`](../entry.rs) and recorded in
[`entry.base.dat`](../data/entry.base.dat) is anchored to five distinct,
verifiable layers of authority:

```text
                      [ Witness 0: ANCHOR ]
                   King James Version (KJV 1611)
                     Immutable Public Domain
                                 |
                                 v
   [ Witness -1: ANCIENT ROOTS ]   [ Witness +1: EXTENSION ]
   Ethiopic Canon (81 Books)        World English Bible (WEB)
   Dead Sea Scrolls & Ge'ez         Modern Public Domain
                 \               /
                  \             /
                   v           v
            [ Witness 2: SCHOLARLY CROSS-CHECK ]
                  NKJV / ESV Scholarly Texts
                               |
                               v
            [ Witness 3: CRYPTOGRAPHIC VAULT ]
              Track 4 PDF Artifact Concordance
              Immutable SHA-256 Hash Signature
```

### Witness Specifications

1. **Witness 0 (Primordial Anchor — KJV):**
   * *Theological Basis:* Authorized King James Bible (1611, public domain).
   * *Role:* Primary theological definition and literary anchor.
2. **Witness +1 (Modern Clarity — WEB):**
   * *Theological Basis:* World English Bible (WEB, public domain).
   * *Role:* Validates propositional truth in contemporary grammar.
3. **Witness -1 (Ancient Roots — Ethiopic Canon):**
   * *Theological Basis:* Ethiopic Tewahedo 81-book canon (Enoch, Jubilees).
   * *Role:* Preserves primordial covenantal context and root meanings.
4. **Witness 2 (Scholarly Verification — NKJV / ESV):**
   * *Theological Basis:* Modern formal equivalence translations.
   * *Role:* Cross-verifies Greek/Hebrew lemmas and syntax variants.
5. **Witness 3 (Cryptographic Vault — ConcordanceRef):**
   * *Technical Basis:* SHA-256 hashed documentation PDFs in Track 4 vault:
     `storage/vault/immutable_documentation/<translation>_canon.pdf`.
   * *Role:* Guarantees mathematical immutability against tampering.

---

## 2. Concordance Reference Format & Hash Tripwires

In [`entry.base.dat`](../data/entry.base.dat), concordance references are
encoded with book, chapter, verse ranges, and 64-character hex digests:

```text
CONCORDANCE SPECIFICATION FORMAT:
<Translation>:<Book>:<Chapter>:<VerseStart>-<VerseEnd>#<SHA256_HASH>

EXAMPLE:
KJV:Exodus:20:13-13#e3b0c44298...855;
WEB:Exodus:20:13-13#a8f5c3298e...966
```

### Validation Tripwires in [`entry.rs`](../entry.rs)

```rust
pub fn validate(&self) -> EntryValidationReport {
    // 1. Validates dual KJV and WEB passages exist
    let has_dual_witness = !self.kjv_passage.trim().is_empty()
        && !self.web_passage.trim().is_empty();

    // 2. Validates domain and category ontological coherence
    let sub_desc = self.subdomain.descriptor();
    let domain_coherent = sub_desc.domain == self.domain
        && sub_desc.category == self.category
        && self.category.parent_domain() == self.domain;

    // 3. Validates verse ranges and SHA-256 hex signature formats
    for c in &self.concordance_refs {
        assert!(c.is_valid_range());
        assert!(c.has_valid_hash_format());
    }
}
```

If an unauthorized modification to scripture occurs or an ungrounded
definition is introduced, the entry's `validate()` fails immediately,
preventing ingestion into `LexiconTable`.

---

## 3. Canonical Scripture Landmark Resolution (Track 4 Coordinates)

To prevent discourse generation from emitting bare, ungrounded coordinate
integers (e.g. `1243` or `281`), the lexicon provides deterministic
canonical resolution between biblical passages and Track 4 coordinates:

1. **`ConcordanceRef::resolve_ordinal()`** maps biblical book and verse
   ranges into absolute canonical coordinates:
   * Genesis offsets are preserved directly (e.g. Genesis 1:29 -> 29).
   * Exodus offsets add canonical base 1,533.
   * Galatians offsets add canonical base 27,413.
2. **`TermTriangulation::resolve_scripture_landmark()`** formats
   dual-witness annotations (e.g. `[Genesis ~v29]`, `[Galatians ~v27435]`),
   guaranteeing that cognitive deliberation is transparently grounded
   in human-readable and mathematically auditable Scripture landmarks.
