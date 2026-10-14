// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal Character Cursor, Source Reel Traversal & Coordinate Tracking )
// ==============================================================================
//! # Universal Character Stream Cursor (`char_stream.rs`)
//!
//! Provides pure, reusable Unicode character-stream navigation over OmniCode source reels.
//! Operates independently of grammatical syntax rules, providing deterministic cursor
//! advancement, coordinate tracking (1-indexed lines and columns), zero-allocation pattern
//! matching, and speculative backtracking.
//!
//! ## Scriptural Grounding:
//! - **Exodus 31:18**: *"And he gave unto Moses, when he had made an end of communing
//!   with him upon mount Sinai, two tables of testimony, tables of stone, written with
//!   the finger of God."*
//!   The character cursor reads the inscribed glyphs from the physical stone reel.
//! - **Proverbs 24:27**: *"Prepare thy work without, and make it fit for thyself in
//!   the field; and afterwards build thine house."*
//!   Precise coordinate tracking and stream traversal prepare clean raw glyphs before
//!   syntactic assembly.
//! - **Proverbs 22:28**: *"Remove not the ancient landmark, which thy fathers have set."*
//!   Preserves reel positions, line markers, and boundary checkpoints without loss.
//!
//! ## Universal Cursor Invariants:
//! 1. **1-Indexed Coordinates**: Lines and columns begin at 1 for human diagnostic clarity.
//! 2. **CRLF Normalization**: Both `\r\n` and `\n` uniformly advance line coordinates by exactly 1.
//! 3. **Zero-Heap Matching**: Compares string literals directly against character iterators.
//! 4. **Speculative Rewind**: Checkpointing allows lossless rollback for ambiguous prefixes.

use crate::parser::tokens::SourceLocation;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Checkpoint State & Character Cursor Structure )
// ==============================================================================

/// Immutable snapshot of cursor state for speculative lookahead and backtracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharCheckpoint {
    /// Character index position in reel.
    pub pos: usize,
    /// 1-indexed line coordinate.
    pub line: usize,
    /// 1-indexed column coordinate.
    pub col: usize,
    /// Byte offset within original source string.
    pub offset: usize,
}

/// Universal cursor over a sequence of Unicode characters with coordinate tracking.
#[derive(Debug, Clone)]
pub struct CharCursor<'a> {
    /// Borrowed raw source reel string.
    source: &'a str,
    /// Decoded Unicode characters representing the source reel tape.
    chars: Vec<char>,
    /// Current character index.
    pos: usize,
    /// Active line number (1-indexed).
    line: usize,
    /// Active column number (1-indexed).
    col: usize,
    /// Active byte offset within raw source text.
    offset: usize,
}

impl<'a> CharCursor<'a> {
    /// Constructs a new [`CharCursor`] over the provided UTF-8 source reel string.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            offset: 0,
        }
    }

    /// Returns the borrowed raw source reel string.
    #[inline]
    pub fn source(&self) -> &'a str {
        self.source
    }

    /// Current 1-indexed line number.
    #[inline]
    pub fn line(&self) -> usize {
        self.line
    }

    /// Current 1-indexed column number.
    #[inline]
    pub fn col(&self) -> usize {
        self.col
    }

    /// Current byte offset from start of source reel.
    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Captures the current cursor location as a [`SourceLocation`].
    #[inline]
    pub fn location(&self) -> SourceLocation {
        SourceLocation::new(self.line, self.col, self.offset)
    }

    /// Captures a lightweight checkpoint for backtracking.
    #[inline]
    pub fn checkpoint(&self) -> CharCheckpoint {
        CharCheckpoint {
            pos: self.pos,
            line: self.line,
            col: self.col,
            offset: self.offset,
        }
    }

    /// Restores the cursor state to a previously captured checkpoint.
    #[inline]
    pub fn restore(&mut self, cp: CharCheckpoint) {
        self.pos = cp.pos;
        self.line = cp.line;
        self.col = cp.col;
        self.offset = cp.offset;
    }

    /// Returns `true` if the cursor has reached or surpassed the end of the character reel.
    #[inline]
    pub fn is_at_end(&self) -> bool {
        self.pos >= self.chars.len()
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Universal Character Navigation, Lookahead & Scanning Primitives )
// ==============================================================================
impl<'a> CharCursor<'a> {
    /// Inspects the current character without advancing.
    #[inline]
    pub fn peek(&self) -> Option<char> {
        self.peek_at(0)
    }

    /// Inspects a character at `offset` positions ahead of current cursor without advancing.
    #[inline]
    pub fn peek_at(&self, offset: usize) -> Option<char> {
        let idx = self.pos + offset;
        if idx < self.chars.len() {
            Some(self.chars[idx])
        } else {
            None
        }
    }

    /// Tests whether the character at `offset` satisfies the given predicate.
    #[inline]
    pub fn peek_matches<F: Fn(char) -> bool>(&self, offset: usize, predicate: F) -> bool {
        self.peek_at(offset).map_or(false, predicate)
    }

    /// Finds the next non-whitespace character starting from `start_offset` ahead.
    #[inline]
    pub fn peek_next_non_whitespace(&self, start_offset: usize) -> Option<char> {
        let mut off = start_offset;
        while let Some(ch) = self.peek_at(off) {
            if !ch.is_whitespace() {
                return Some(ch);
            }
            off += 1;
        }
        None
    }

    /// Advances the cursor by one character, updating line, column, and byte offset.
    ///
    /// Uniformly normalizes `\r\n` (CRLF) and `\n` to prevent spurious line jumps.
    pub fn advance(&mut self) -> Option<char> {
        if self.pos >= self.chars.len() {
            return None;
        }

        let ch = self.chars[self.pos];
        self.pos += 1;
        self.offset += ch.len_utf8();

        if ch == '\r' {
            // Check for CRLF sequence
            if self.pos < self.chars.len() && self.chars[self.pos] == '\n' {
                self.pos += 1;
                self.offset += 1; // '\n' is 1 byte
            }
            self.line += 1;
            self.col = 1;
        } else if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }

        Some(ch)
    }

    /// Conditionally consumes the current character if it matches `expected`.
    #[inline]
    pub fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Matches and consumes the exact target string at cursor without heap allocations.
    pub fn match_str(&mut self, target: &str) -> bool {
        let target_len = target.chars().count();
        if self.pos + target_len > self.chars.len() {
            return false;
        }

        for (i, target_ch) in target.chars().enumerate() {
            if self.chars[self.pos + i] != target_ch {
                return false;
            }
        }

        for _ in 0..target_len {
            self.advance();
        }
        true
    }

    /// Repeatedly consumes characters while `predicate` returns `true`, accumulating them.
    pub fn consume_while<F: Fn(char) -> bool>(&mut self, predicate: F) -> String {
        let mut result = String::new();
        while let Some(ch) = self.peek() {
            if predicate(ch) {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    /// Consumes contiguous whitespace characters (spaces, tabs, newlines).
    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Consumes a single-line comment initiated by `comment_char` up to the next newline.
    pub fn skip_line_comment(&mut self, comment_char: char) -> bool {
        if self.peek() == Some(comment_char) {
            while let Some(ch) = self.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                self.advance();
            }
            true
        } else {
            false
        }
    }

    /// Universally consumes whitespace and comments starting with `comment_char`.
    pub fn skip_whitespace_and_comments(&mut self, comment_char: char) {
        loop {
            self.skip_whitespace();
            if !self.skip_line_comment(comment_char) {
                break;
            }
        }
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Universal Character Stream Verification Proofs & Traversal Oracles )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_linear_navigation_and_crlf() {
        let mut cursor = CharCursor::new("A\r\nB\nC");
        assert_eq!(cursor.line(), 1);
        assert_eq!(cursor.col(), 1);

        assert_eq!(cursor.advance(), Some('A'));
        assert_eq!(cursor.line(), 1);
        assert_eq!(cursor.col(), 2);

        // CRLF normalized advance
        assert_eq!(cursor.advance(), Some('\r'));
        assert_eq!(cursor.line(), 2);
        assert_eq!(cursor.col(), 1);

        assert_eq!(cursor.advance(), Some('B'));
        assert_eq!(cursor.advance(), Some('\n'));
        assert_eq!(cursor.line(), 3);
        assert_eq!(cursor.col(), 1);

        assert_eq!(cursor.advance(), Some('C'));
        assert_eq!(cursor.advance(), None);
        assert!(cursor.is_at_end());
    }

    #[test]
    fn test_zero_allocation_match_str() {
        let mut cursor = CharCursor::new("--> INSCRIBE");
        assert!(cursor.match_str("-->"));
        assert_eq!(cursor.col(), 4);

        cursor.skip_whitespace();
        assert!(cursor.match_str("INSCRIBE"));
        assert!(cursor.is_at_end());
    }

    #[test]
    fn test_checkpoint_and_backtracking() {
        let mut cursor = CharCursor::new("Character` <= [ 1 ]");
        let cp = cursor.checkpoint();

        let ident = cursor.consume_while(|c| c.is_alphanumeric() || c == '`');
        assert_eq!(ident, "Character`");

        // Rollback to origin
        cursor.restore(cp);
        assert_eq!(cursor.line(), 1);
        assert_eq!(cursor.col(), 1);
        assert_eq!(cursor.peek(), Some('C'));
    }

    #[test]
    fn test_whitespace_and_line_comments() {
        let mut cursor = CharCursor::new("   # This is a comment\n42");
        cursor.skip_whitespace_and_comments('#');
        assert_eq!(cursor.peek(), Some('4'));
        assert_eq!(cursor.line(), 2);
    }
}
