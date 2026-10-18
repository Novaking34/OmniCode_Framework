// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( BoundaryFrame Model, Delimiter Stack Tracking & Morphological Verification )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "Remove not the ancient landmark, which thy fathers have set."
//   — Proverbs 22:28 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   The boundary tracking organ of the OmniDebugger.
//   Acts as the focused eye inspecting morphological enclosures:
//     - Tracks opening delimiters: `<` (Invariant), `[` (Denotation),
//       `{` (Connotation), `(` (Aside).
//     - Verifies delimiter pairs upon closing, including asymmetric pairs:
//       `[ ... }` (Bottom-Up Emergence) and `{ ... ]` (Top-Down Imposition).
//     - Intercepts boundary leaks (unclosed at EOF) and mismatches,
//       synthesizing exact FixSuggestions for the Restorator.
// ==============================================================================

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Imports, Stack Data Models & Delimiter Matcher )
// ==============================================================================
use crate::core::boundaries::BoundaryType;
use crate::diagnostics::logger::{DiagnosticLogger, FailpointType, FixKind, FixSuggestion};

/// Stack frame recording an unclosed boundary delimiter's opening context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryFrame {
    /// The opening delimiter character: `<`, `[`, `{`, or `(`.
    pub open_char: char,
    /// 1-based source line where the boundary opened.
    pub line: usize,
    /// 1-based source column where the delimiter occurred.
    pub col: usize,
}

impl BoundaryFrame {
    /// Constructs a new [`BoundaryFrame`].
    pub fn new(open_char: char, line: usize, col: usize) -> Self {
        Self { open_char, line, col }
    }

    /// Returns the canonical matching closing character for standard enclosures.
    pub fn matching_closing_char(&self) -> char {
        match self.open_char {
            '<' => '>',
            '[' => ']',
            '{' => '}',
            _ => ')',
        }
    }
}

/// Focused observer tracking the active LIFO stack of nested boundaries.
#[derive(Debug, Clone, Default)]
pub struct BoundaryTracker {
    /// Active LIFO stack of unclosed boundary frames.
    pub stack: Vec<BoundaryFrame>,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Gaze: Push Delimiter, Pop & Verify Pair, Assert Enclosures Sealed )
// ==============================================================================
impl BoundaryTracker {
    /// Creates a new empty [`BoundaryTracker`].
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Returns the current nesting depth of the boundary stack.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Inspects whether the boundary stack is currently empty.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Pushes an opening delimiter onto the active stack and logs the focus point.
    pub fn push(&mut self, open: char, line: usize, col: usize, logger: &DiagnosticLogger) {
        let frame = BoundaryFrame::new(open, line, col);
        let current_depth = self.stack.len() + 1;
        logger.log(
            "BOUNDARY",
            format!("Gaze Focus: Pushing delimiter '{open}' at L{line}:C{col} (depth: {current_depth})"),
        );
        self.stack.push(frame);
    }

    /// Pops a closing boundary delimiter and validates the pair against the stack top.
    ///
    /// # Failpoints Intercepted
    /// - **Empty Stack**: Traps unmatched closing delimiter as [`FailpointType::BoundaryMismatch`].
    /// - **Mismatched Delimiter**: Traps illegal pair crossing, logs a [`FixSuggestion`] pointing
    ///   to the correct partner, and queues remediation.
    pub fn pop(
        &mut self,
        close: char,
        line: usize,
        col: usize,
        logger: &DiagnosticLogger,
    ) -> Result<BoundaryType, (String, Option<FixSuggestion>)> {
        if self.stack.is_empty() {
            let msg = format!("Unmatched closing delimiter '{close}' at L{line}:C{col} (stack is empty)");
            logger.failpoint(FailpointType::BoundaryMismatch, "DEBUGGER", &msg);
            return Err((format!("FAILPOINT: {msg}"), None));
        }

        let frame = self.stack.pop().unwrap();
        match BoundaryType::from_delimiters(frame.open_char, close) {
            Ok(btype) => {
                logger.log(
                    "BOUNDARY",
                    format!(
                        "Gaze Focus: Successfully resolved boundary {} '{}{}' (remaining depth: {})",
                        btype, frame.open_char, close, self.stack.len()
                    ),
                );
                Ok(btype)
            }
            Err(err) => {
                let msg = format!(
                    "Boundary mismatch: opened with '{}' (L{}:C{}) but closed with '{}' at L{}:C{}",
                    frame.open_char, frame.line, frame.col, close, line, col
                );
                let matching = frame.matching_closing_char();
                let suggestion = FixSuggestion {
                    kind: FixKind::FixBoundaryMismatch,
                    line,
                    col,
                    target_token: close.to_string(),
                    suggested_patch: matching.to_string(),
                    rationale: format!(
                        "Replace mismatched closing delimiter '{close}' with matching '{matching}' to seal boundary opened with '{}' at L{}:C{}",
                        frame.open_char, frame.line, frame.col
                    ),
                };

                logger.failpoint_with_remediation(
                    FailpointType::BoundaryMismatch,
                    "DEBUGGER",
                    &msg,
                    suggestion.clone(),
                );

                Err((err, Some(suggestion)))
            }
        }
    }

    /// Asserts that all opened boundaries have been cleanly sealed at EOF.
    ///
    /// If unclosed delimiters remain on the stack, logs [`FailpointType::BoundaryLeak`]
    /// with a concrete [`FixSuggestion`] indicating the required closing tokens.
    pub fn assert_all_closed(
        &self,
        logger: &DiagnosticLogger,
    ) -> Result<(), (String, FixSuggestion)> {
        if let Some(frame) = self.stack.last() {
            let msg = format!(
                "Boundary leak: delimiter '{}' opened at L{}:C{} was never closed",
                frame.open_char, frame.line, frame.col
            );
            let matching = frame.matching_closing_char();
            let suggestion = FixSuggestion {
                kind: FixKind::CloseBoundary,
                line: frame.line,
                col: frame.col,
                target_token: frame.open_char.to_string(),
                suggested_patch: format!("\n{matching}"),
                rationale: format!(
                    "Append missing closing delimiter '{matching}' for boundary opened with '{}' at L{}:C{}",
                    frame.open_char, frame.line, frame.col
                ),
            };

            logger.failpoint_with_remediation(
                FailpointType::BoundaryLeak,
                "DEBUGGER",
                &msg,
                suggestion.clone(),
            );

            return Err((format!("FAILPOINT: {msg}"), suggestion));
        }
        Ok(())
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Boundary Stack Unit Tests & Affirmative Invariant Sealing )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundary_tracker_push_and_pop_pairs() {
        let logger = DiagnosticLogger::new(false);
        let mut tracker = BoundaryTracker::new();

        tracker.push('<', 1, 1, &logger);
        assert_eq!(tracker.depth(), 1);

        let res = tracker.pop('>', 1, 10, &logger);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), BoundaryType::Invariant);
        assert!(tracker.is_empty());
    }

    #[test]
    fn test_boundary_tracker_asymmetric_delimiters() {
        let logger = DiagnosticLogger::new(false);
        let mut tracker = BoundaryTracker::new();

        // Top-Down Imposition: { ... ]
        tracker.push('{', 1, 1, &logger);
        let res = tracker.pop(']', 1, 8, &logger);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), BoundaryType::TopDown);

        // Bottom-Up Emergence: [ ... }
        tracker.push('[', 2, 1, &logger);
        let res2 = tracker.pop('}', 2, 8, &logger);
        assert!(res2.is_ok());
        assert_eq!(res2.unwrap(), BoundaryType::BottomUp);
    }

    #[test]
    fn test_boundary_tracker_mismatch_generates_remediation() {
        let logger = DiagnosticLogger::new(false);
        let mut tracker = BoundaryTracker::new();

        tracker.push('<', 5, 1, &logger);
        let res = tracker.pop(']', 5, 20, &logger);
        assert!(res.is_err());

        let (_err, suggestion) = res.err().unwrap();
        assert!(suggestion.is_some());
        let s = suggestion.unwrap();
        assert_eq!(s.kind, FixKind::FixBoundaryMismatch);
        assert_eq!(s.suggested_patch, ">");
    }

    #[test]
    fn test_boundary_tracker_leak_detection() {
        let logger = DiagnosticLogger::new(false);
        let mut tracker = BoundaryTracker::new();

        tracker.push('[', 10, 4, &logger);
        let res = tracker.assert_all_closed(&logger);
        assert!(res.is_err());

        let (msg, suggestion) = res.err().unwrap();
        assert!(msg.contains("Boundary leak"));
        assert_eq!(suggestion.kind, FixKind::CloseBoundary);
        assert_eq!(suggestion.suggested_patch, "\n]");
    }
}
