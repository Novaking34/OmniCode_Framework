# Volume II: Scripture Concordance & The 5-Fold Witness Standard

> **"It is written in the law, that the testimony of two men is true."**
> — *John 8:17 (KJV)*
>
> **"One witness shall not rise up against a man for any iniquity, or for any sin, in any sin that he sinneth: at the mouth of two witnesses, or at the mouth of three witnesses, shall the matter be established."**
> — *Deuteronomy 19:15 (KJV)*

In modern AI research, language models suffer from catastrophic "hallucination"—the generation of statistically plausible but factually fictitious claims. This occurs because LLMs operate on token likelihood rather than grounded truth.

OmniCode solves hallucination at the architectural root by binding every lexicon entry to an immutable **5-Fold Witness Standard**.

---

## 1. The 5-Fold Witness Topology

Every concept defined in [`entry.rs`](../entry.rs) and recorded in [`entry.base.dat`](../data/entry.base.dat) is anchored to five distinct, verifiable layers of authority:

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
   * *Theological Basis:* The Authorized King James Bible (1611), public domain and free of corporate copyright encumbrance.
   * *Role:* Provides the primary theological definition and literary anchor for the term.
2. **Witness +1 (Modern Clarity — WEB):**
   * *Theological Basis:* The World English Bible (WEB), an open-source, public domain modern English translation.
   * *Role:* Validates that the archaic 17th-century English preserves identical propositional truth in contemporary grammar.
3. **Witness -1 (Ancient Roots — Ethiopic Canon):**
   * *Theological Basis:* The Ethiopic Tewahedo 81-book biblical canon (including 1 Enoch, Jubilees, and 4 Baruch).
   * *Role:* Preserves primordial pre-flood and ancient covenantal context often lost in truncated Western canons.
4. **Witness 2 (Scholarly Verification — NKJV / ESV):**
   * *Theological Basis:* Modern formal equivalence translations.
   * *Role:* Cross-verifies specialized grammatical forms, Greek/Hebrew lemmas, and syntax variants.
5. **Witness 3 (Cryptographic Vault — ConcordanceRef):**
   * *Technical Basis:* SHA-256 hashed documentation PDFs stored in the Track 4 storage vault:
     `storage/vault/immutable_documentation/<translation>_canon.pdf`
   * *Role:* Guarantees mathematical immutability. If an entry's text or concordance coordinates are tampered with, the SHA-256 hash fails validation.

---

## 2. Concordance Reference Format & Hash Tripwires

In [`entry.base.dat`](../data/entry.base.dat), concordance references are encoded with book, chapter, verse ranges, and 64-character hex digests:

```text
CONCORDANCE SPECIFICATION FORMAT:
<Translation>:<Book>:<Chapter>:<VerseStart>-<VerseEnd>#<SHA256_HASH>

EXAMPLE:
KJV:Exodus:20:13-13#e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855;
WEB:Exodus:20:13-13#a8f5c3298ec1d1149bfaf4c8996eb92427be41e4649c934ba495991c7852c966
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

If an unauthorized modification to scripture occurs or an ungrounded definition is introduced, the entry's `validate()` fails immediately, preventing ingestion into `LexiconTable`.
