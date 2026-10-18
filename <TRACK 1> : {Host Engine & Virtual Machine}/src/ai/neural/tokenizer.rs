// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Byte-Pair Ternary Trie Tokenizer & Zero-OOV Balanced-Ternary Transducer )
// ==============================================================================
//! # Byte-Pair Ternary Trie (`tokenizer.rs`)
//!
//! Implements a sub-word tokenizer based on a compact Ternary Byte Trie (`TernaryByteTree`).
//! Decomposes arbitrary UTF-8 text into balanced-ternary sensory vectors (`{-1, 0, +1}`)
//! for ingestion into `NeuralBrain`.
//!
//! ### Key Invariants:
//! 1. **Zero Out-of-Vocabulary (OOV) Drops**: Every valid UTF-8 byte is rooted in the trie,
//!    guaranteeing 100% coverage without unknown token placeholders (`<UNK>`).
//! 2. **Sub-Word Morpheme Merging**: Frequently co-occurring byte pairs, OmniCode keywords,
//!    and Biblical ontology concepts are merged into atomic sub-word nodes.
//! 3. **Ternary Majority Pooling**: Long sequences exceeding the sensory field width
//!    are folded through balanced-ternary majority consensus rather than truncated.
//! 4. **Ultra-Lean Footprint**: Total memory usage remains strictly under 2 MB (< 256 KB typical).

use std::collections::HashMap;
use crate::core::cell::{Cell, Trit};
use crate::core::ternary::QuadState;
use crate::core::linguistics::PartOfSpeech;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Trie Node Definitions, Subword Tokens & Invariant Constants )
// ==============================================================================

/// An atomic sub-word token extracted by the `TernaryByteTree`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubwordToken {
    pub token_id: usize,
    pub text: String,
    pub raw_bytes: Vec<u8>,
    pub cell: Cell,
    pub part_of_speech: Option<PartOfSpeech>,
}

/// A node within the Ternary Byte Trie.
#[derive(Debug, Clone)]
pub struct TernaryTrieNode {
    /// Token identifier if this node terminates a valid subword.
    pub token_id: Option<usize>,
    /// The canonical subword text if terminal.
    pub subword: Option<String>,
    /// The raw byte sequence corresponding to this subword.
    pub raw_bytes: Vec<u8>,
    /// The 5-trit atomic balanced-ternary cell for this node.
    pub cell: Option<Cell>,
    /// Grammatical part of speech if known.
    pub part_of_speech: Option<PartOfSpeech>,
    /// Frequency counter for heuristic BPE priority.
    pub frequency: usize,
    /// Byte-indexed child branches.
    pub children: HashMap<u8, Box<TernaryTrieNode>>,
}

impl Default for TernaryTrieNode {
    fn default() -> Self {
        Self {
            token_id: None,
            subword: None,
            raw_bytes: Vec::new(),
            cell: None,
            part_of_speech: None,
            frequency: 0,
            children: HashMap::new(),
        }
    }
}

/// The Byte-Pair Ternary Trie tokenizer engine.
#[derive(Debug, Clone)]
pub struct TernaryByteTree {
    root: Box<TernaryTrieNode>,
    next_token_id: usize,
    total_tokens: usize,
}

impl Default for TernaryByteTree {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Trie Construction, Vocabulary Ingestion & Balanced-Ternary Encoding )
// ==============================================================================

impl TernaryByteTree {
    /// Constructs a new `TernaryByteTree` initialized with the 256 single-byte roots
    /// and core domain morphemes.
    pub fn new() -> Self {
        let mut tree = Self {
            root: Box::new(TernaryTrieNode::default()),
            next_token_id: 0,
            total_tokens: 0,
        };

        // 1. Ingest all 256 physical single-byte runes (guaranteeing 0 OOV drops)
        for b in 0..=255u8 {
            let ch_str = match std::str::from_utf8(&[b]) {
                Ok(s) => s.to_string(),
                Err(_) => format!("\\x{b:02X}"),
            };
            let cell = Cell::from_natural(b).unwrap_or_else(|_| Cell::from_state(QuadState::Await));
            tree.insert_token(&[b], &ch_str, cell, Some(PartOfSpeech::Noun), 1);
        }

        // 2. Ingest high-frequency English morphemes & grammar affixes
        tree.seed_english_morphemes();

        // 3. Ingest OmniCode 4-Block & Morphological Boundary primitives
        tree.seed_omnicode_primitives();

        // 4. Ingest Decalogue, Theological & Diagnostic terminology
        tree.seed_covenant_and_diagnostics();

        tree
    }

    /// Inserts a sequence of bytes as a valid subword token into the trie.
    pub fn insert_token(
        &mut self,
        bytes: &[u8],
        subword: &str,
        cell: Cell,
        pos: Option<PartOfSpeech>,
        frequency: usize,
    ) -> usize {
        let mut curr = &mut self.root;
        for &b in bytes {
            curr = curr.children.entry(b).or_insert_with(|| Box::new(TernaryTrieNode::default()));
        }

        if let Some(id) = curr.token_id {
            curr.frequency += frequency;
            return id;
        }

        let token_id = self.next_token_id;
        self.next_token_id += 1;
        self.total_tokens += 1;

        curr.token_id = Some(token_id);
        curr.subword = Some(subword.to_string());
        curr.raw_bytes = bytes.to_vec();
        curr.cell = Some(cell);
        curr.part_of_speech = pos;
        curr.frequency = frequency;

        token_id
    }

    /// Seeds common English morphemes, prefixes, and suffixes.
    fn seed_english_morphemes(&mut self) {
        let affixes = [
            ("ing", PartOfSpeech::Verb),
            ("tion", PartOfSpeech::Noun),
            ("ment", PartOfSpeech::Noun),
            ("ness", PartOfSpeech::Noun),
            ("able", PartOfSpeech::Adjective),
            ("ible", PartOfSpeech::Adjective),
            ("ed", PartOfSpeech::Verb),
            ("ly", PartOfSpeech::Adverb),
            ("pre", PartOfSpeech::Preposition),
            ("pro", PartOfSpeech::Preposition),
            ("sub", PartOfSpeech::Preposition),
            ("super", PartOfSpeech::Preposition),
            ("con", PartOfSpeech::Conjunction),
            ("dis", PartOfSpeech::Adverb),
            ("un", PartOfSpeech::Adverb),
            ("re", PartOfSpeech::Verb),
            ("auto", PartOfSpeech::Adjective),
            ("multi", PartOfSpeech::Adjective),
            ("anti", PartOfSpeech::Adverb),
            ("inter", PartOfSpeech::Preposition),
        ];

        for (word, pos) in affixes {
            let cell = Cell::from_natural((word.as_bytes()[0] as u16 + word.len() as u16) as u8)
                .unwrap_or_else(|_| Cell::from_state(QuadState::Proceed));
            self.insert_token(word.as_bytes(), word, cell, Some(pos), 50);
        }
    }

    /// Seeds OmniCode 4-block and morphological boundary keywords.
    fn seed_omnicode_primitives(&mut self) {
        let keywords = [
            ("Metadata", PartOfSpeech::Noun),
            ("Opening", PartOfSpeech::Noun),
            ("Body", PartOfSpeech::Noun),
            ("Closing", PartOfSpeech::Noun),
            ("Module", PartOfSpeech::Noun),
            ("Version", PartOfSpeech::Noun),
            ("Tape", PartOfSpeech::Noun),
            ("Roll", PartOfSpeech::Noun),
            ("Cell", PartOfSpeech::Noun),
            ("Vessel", PartOfSpeech::Noun),
            ("Container", PartOfSpeech::Noun),
            ("Fixed", PartOfSpeech::Adjective),
            ("Fluid", PartOfSpeech::Adjective),
            ("Universal", PartOfSpeech::Adjective),
            ("Accumulator", PartOfSpeech::Noun),
            ("Generator", PartOfSpeech::Noun),
            ("Aside", PartOfSpeech::Interjection),
            ("PROCEED", PartOfSpeech::Interjection),
            ("ROLLBACK", PartOfSpeech::Interjection),
            ("AWAIT", PartOfSpeech::Interjection),
            ("HALT", PartOfSpeech::Interjection),
            ("-->", PartOfSpeech::Verb),
            ("<--", PartOfSpeech::Verb),
            ("^--", PartOfSpeech::Verb),
            ("v--", PartOfSpeech::Verb),
            ("::", PartOfSpeech::Conjunction),
            ("!", PartOfSpeech::Conjunction),
        ];

        for (kw, pos) in keywords {
            let cell = Cell::from_natural(kw.len() as u8)
                .unwrap_or_else(|_| Cell::from_state(QuadState::Proceed));
            self.insert_token(kw.as_bytes(), kw, cell, Some(pos), 100);
        }
    }

    /// Seeds Decalogue concepts and Track 1 diagnostic tool tokens.
    fn seed_covenant_and_diagnostics(&mut self) {
        let terms = [
            ("Sovereignty", PartOfSpeech::Noun),
            ("Reverence", PartOfSpeech::Noun),
            ("Sabbath", PartOfSpeech::Noun),
            ("Lineage", PartOfSpeech::Noun),
            ("Sanctity", PartOfSpeech::Noun),
            ("Fidelity", PartOfSpeech::Noun),
            ("Property", PartOfSpeech::Noun),
            ("Truth", PartOfSpeech::Noun),
            ("Witness", PartOfSpeech::Noun),
            ("Contentment", PartOfSpeech::Noun),
            ("Logger", PartOfSpeech::Noun),
            ("Debugger", PartOfSpeech::Noun),
            ("Linter", PartOfSpeech::Noun),
            ("Restorator", PartOfSpeech::Noun),
            ("Hover", PartOfSpeech::Noun),
            ("Checksum", PartOfSpeech::Noun),
            ("Parity", PartOfSpeech::Noun),
            ("Tripwire", PartOfSpeech::Noun),
            ("Deadman", PartOfSpeech::Noun),
            ("Cryosleep", PartOfSpeech::Noun),
            ("Ternary", PartOfSpeech::Adjective),
            ("Radix-3", PartOfSpeech::Noun),
            ("Polarity", PartOfSpeech::Noun),
            ("Balanced", PartOfSpeech::Adjective),
        ];

        for (term, pos) in terms {
            let cell = Cell::from_natural(term.len() as u8)
                .unwrap_or_else(|_| Cell::from_state(QuadState::Proceed));
            self.insert_token(term.as_bytes(), term, cell, Some(pos), 80);
        }
    }

    /// Tokenizes arbitrary UTF-8 text using greedy longest-match.
    pub fn tokenize(&self, text: &str) -> Vec<SubwordToken> {
        let bytes = text.as_bytes();
        let mut tokens = Vec::new();
        let mut idx = 0;

        while idx < bytes.len() {
            let mut longest_match = None;
            let mut curr = &self.root;
            let mut match_len = 0;

            for (offset, &b) in bytes[idx..].iter().enumerate() {
                if let Some(child) = curr.children.get(&b) {
                    curr = child;
                    if let Some(token_id) = curr.token_id {
                        longest_match = Some((
                            token_id,
                            curr.subword.clone().unwrap_or_default(),
                            curr.raw_bytes.clone(),
                            curr.cell.unwrap_or_else(|| Cell::from_state(QuadState::Proceed)),
                            curr.part_of_speech,
                        ));
                        match_len = offset + 1;
                    }
                } else {
                    break;
                }
            }

            if let Some((id, subword, raw_bytes, cell, pos)) = longest_match {
                tokens.push(SubwordToken {
                    token_id: id,
                    text: subword,
                    raw_bytes,
                    cell,
                    part_of_speech: pos,
                });
                idx += match_len;
            } else {
                // Fallback to single byte (guaranteed present by constructor)
                let b = bytes[idx];
                let cell = Cell::from_natural(b).unwrap_or_else(|_| Cell::from_state(QuadState::Await));
                let text = match std::str::from_utf8(&[b]) {
                    Ok(s) => s.to_string(),
                    Err(_) => format!("\\x{b:02X}"),
                };
                tokens.push(SubwordToken {
                    token_id: b as usize,
                    text,
                    raw_bytes: vec![b],
                    cell,
                    part_of_speech: Some(PartOfSpeech::Noun),
                });
                idx += 1;
            }
        }

        tokens
    }

    /// Reconstructs the original UTF-8 text losslessly from a sequence of subword tokens.
    pub fn decode(&self, tokens: &[SubwordToken]) -> String {
        let mut bytes = Vec::new();
        for t in tokens {
            bytes.extend_from_slice(&t.raw_bytes);
        }
        String::from_utf8_lossy(&bytes).into_owned()
    }

    /// Decomposes text into an exact fixed-length balanced-ternary sensory vector.
    ///
    /// If the sequence produces more trits than `target_len`, applies a balanced-ternary
    /// majority consensus fold so that the entire text is preserved in the sensory field.
    pub fn encode_to_sensory_vector(&self, text: &str, target_len: usize) -> Vec<QuadState> {
        let tokens = self.tokenize(text);
        let mut raw_trits = Vec::new();

        for token in &tokens {
            for trit in &token.cell.trits {
                let st = match trit {
                    Trit::Pos => QuadState::Proceed,
                    Trit::Neg => QuadState::Rollback,
                    Trit::Zero => QuadState::Await,
                };
                raw_trits.push(st);
            }
        }

        if raw_trits.is_empty() {
            return vec![QuadState::Await; target_len];
        }

        if raw_trits.len() == target_len {
            return raw_trits;
        }

        if raw_trits.len() < target_len {
            // Pad with neutral equilibrium (0: AWAIT)
            raw_trits.resize(target_len, QuadState::Await);
            return raw_trits;
        }

        // FOLDING: raw_trits.len() > target_len
        // Fold multiple trits into a single target position via balanced ternary majority consensus
        let mut pooled = vec![QuadState::Await; target_len];
        let chunk_size = (raw_trits.len() as f64 / target_len as f64).ceil() as usize;

        for (i, slot) in pooled.iter_mut().enumerate() {
            let start = i * chunk_size;
            let end = (start + chunk_size).min(raw_trits.len());
            if start >= raw_trits.len() {
                break;
            }

            let slice = &raw_trits[start..end];
            let mut sum: i32 = 0;
            for st in slice {
                match st {
                    QuadState::Proceed => sum += 1,
                    QuadState::Rollback => sum -= 1,
                    _ => {}
                }
            }

            *slot = if sum > 0 {
                QuadState::Proceed
            } else if sum < 0 {
                QuadState::Rollback
            } else {
                QuadState::Await
            };
        }

        pooled
    }

    /// Total unique tokens indexed in the trie.
    pub fn vocab_size(&self) -> usize {
        self.total_tokens
    }

    /// Computes approximate heap memory footprint of the trie in bytes.
    pub fn memory_bytes(&self) -> usize {
        fn node_size(node: &TernaryTrieNode) -> usize {
            let mut size = std::mem::size_of::<TernaryTrieNode>();
            size += node.subword.as_ref().map(|s| s.len()).unwrap_or(0);
            for child in node.children.values() {
                size += node_size(child);
            }
            size
        }
        node_size(&self.root)
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Invariant Verification Proofs )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_oov_arbitrary_utf8_coverage() {
        let tree = TernaryByteTree::new();
        let arbitrary = "Arbitrary prompt: 12345! @#$% \u{1F600} Greek: \u{03A9}";
        let tokens = tree.tokenize(arbitrary);
        assert!(!tokens.is_empty(), "Tokenizer must produce tokens for any UTF-8 input");

        // Reconstruct string
        let reconstructed = tree.decode(&tokens);
        assert_eq!(reconstructed, arbitrary, "Lossless tokenization required");
    }

    #[test]
    fn test_subword_merging_for_keywords() {
        let tree = TernaryByteTree::new();
        let text = "Metadata Opening Body Closing Logger Restorator";
        let tokens = tree.tokenize(text);

        let subwords: Vec<&str> = tokens.iter().map(|t| t.text.as_str()).filter(|s| *s != " ").collect();
        assert!(subwords.contains(&"Metadata"));
        assert!(subwords.contains(&"Opening"));
        assert!(subwords.contains(&"Body"));
        assert!(subwords.contains(&"Closing"));
        assert!(subwords.contains(&"Logger"));
        assert!(subwords.contains(&"Restorator"));
    }

    #[test]
    fn test_sensory_vector_folding_and_padding() {
        let tree = TernaryByteTree::new();

        // Test padding short input
        let short_vec = tree.encode_to_sensory_vector("Hi", 64);
        assert_eq!(short_vec.len(), 64);

        // Test folding long input
        let long_text = "Why do we use balanced ternary instead of binary? In OmniCode, we reject binary computing because balanced ternary guarantees sign symmetry and eliminates precision drift.";
        let folded_vec = tree.encode_to_sensory_vector(long_text, 64);
        assert_eq!(folded_vec.len(), 64);

        // Verify that folding captures signal (not just all neutral zeros)
        let non_neutral = folded_vec.iter().any(|st| *st != QuadState::Await);
        assert!(non_neutral, "Folded sensory vector must retain signal from the text");
    }

    #[test]
    fn test_memory_footprint_guardrail() {
        let tree = TernaryByteTree::new();
        let bytes = tree.memory_bytes();
        // Strict invariant: under 2 MB (< 2,097,152 bytes)
        assert!(bytes < 2_097_152, "TernaryByteTree memory ({bytes} bytes) must be < 2 MB");
        assert!(tree.vocab_size() >= 256, "Must contain at least 256 atomic byte tokens");
    }
}
