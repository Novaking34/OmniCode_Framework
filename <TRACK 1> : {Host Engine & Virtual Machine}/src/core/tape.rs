// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Tape Subsystem Identity, Ribbon Physical Topology & Invariants )
// ==============================================================================
//! OmniCode Core Subsystem: Physical Tape & Roll Execution Topology
//!
//! # Physical Computing Hierarchy
//! In the OmniCode Operating System, execution does not use raw unstructured threads.
//! It follows a strict 3-tier physical hierarchy:
//!
//! $$\text{Roll} \longrightarrow \text{Tape} \longrightarrow \text{Track}$$
//!
//! 1. **Roll (Instruction Line):** An atomic line of code executing an SVO transformation.
//! 2. **Tape (Process Ribbon):** A sequential chain of rolls led by a **Tape Head**
//!    (the covenant manifest declaring Who, What, When, Where, Why, and How).
//! 3. **Track (Subsystem Channel):** A concurrent execution lane on the scheduler
//!    (Track 0: Kernel, Track 1: Terminal, Track 2: Apps).
//!
//! # Tape Head Anatomy
//! The Tape Head is the first roll on the ribbon. It establishes:
//! - Identity & Name (`[Tape.<Name>]`)
//! - Principal & Caller (`WHO {Caller}`)
//! - Target Entity (`WHAT {Target}`)
//! - Lifecycle Trigger (`WHEN {Phase}`)
//! - Memory Partition Ceiling (`WHERE {partition} = [ <N> ]`)
//! - Teleological Purpose Aside (`WHY ( ... )`)
//! - Operational Vector (`HOW [ ... ]`)

use std::fs;
use std::path::Path;
use crate::core::ternary::QuadState;
use crate::diagnostics::logger::DiagnosticLogger;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Tape Data Structures, Roll Containers and Context )
// ==============================================================================

/// An atomic line of code on a Tape ribbon.
#[derive(Debug, Clone, PartialEq)]
pub struct Roll {
    /// 0-indexed position within the tape's rolls.
    pub index: usize,
    /// 1-indexed source line number.
    pub line_number: usize,
    /// Raw text content of the roll.
    pub text: String,
    /// Current execution state of this roll (+1: PROCEED, 0: AWAIT, -1: ROLLBACK, NULL: HALT).
    pub status: QuadState,
    /// Whether this roll is the leading Tape Head covenant.
    pub is_head: bool,
}

/// The leading covenant roll establishing the process manifest and memory bounds.
#[derive(Debug, Clone, PartialEq)]
pub struct TapeHead {
    /// Identifier or process name.
    pub name: String,
    /// Cardinal Clause: WHO (Caller / Principal).
    pub who: String,
    /// Cardinal Clause: WHAT (Process substance).
    pub what: String,
    /// Cardinal Clause: WHEN (Trigger precondition).
    pub when_clause: String,
    /// Cardinal Clause: WHERE (Target partition or spatial bounds).
    pub where_clause: String,
    /// Cardinal Clause: WHY (Teleological intent aside).
    pub why: String,
    /// Cardinal Clause: HOW (Directional vector and methodology).
    pub how: String,
    /// Required memory partition size in cells (carved from Storehouse via `v--`).
    pub required_partition_size: usize,
    /// Complete raw text of the Tape Head roll.
    pub raw_text: String,
}

impl Default for TapeHead {
    fn default() -> Self {
        Self {
            name: "Anonymous_Tape".to_string(),
            who: "System".to_string(),
            what: "Process".to_string(),
            when_clause: "Immediate".to_string(),
            where_clause: "Memory".to_string(),
            why: "Execution".to_string(),
            how: "Forward".to_string(),
            required_partition_size: 256,
            raw_text: String::new(),
        }
    }
}

/// A complete sequential process ribbon executed roll-by-roll.
#[derive(Debug, Clone)]
pub struct Tape {
    /// The leading covenant manifest.
    pub head: TapeHead,
    /// The sequence of subordinate execution rolls.
    pub rolls: Vec<Roll>,
    /// Active cursor pointing to the roll currently being executed.
    pub cursor: usize,
    /// Name of the isolated memory partition allocated for this tape.
    pub bound_partition: Option<String>,
    /// Overall execution quad-state.
    pub execution_state: QuadState,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Tape Parsing, Stepping, and Execution Management )
// ==============================================================================

impl Tape {
    /// Parses a native `.tape` source string into a structured [`Tape`].
    pub fn from_source(source: &str, logger: DiagnosticLogger) -> Result<Self, String> {
        let mut head_opt: Option<TapeHead> = None;
        let mut raw_roll_lines: Vec<(usize, String)> = Vec::new();

        let mut head_text_accumulator = String::new();
        let mut parsing_multiline_head = false;

        for (zero_idx, raw_line) in source.lines().enumerate() {
            let line_number = zero_idx + 1;
            let trimmed = raw_line.trim();

            // Skip comment boundaries (# ...)
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }

            // Detect Tape Head
            if head_opt.is_none() {
                if !parsing_multiline_head {
                    if trimmed.starts_with('[')
                        || trimmed.starts_with("= [Tape")
                        || trimmed.starts_with("Tape [")
                        || trimmed.starts_with("Tape <")
                    {
                        parsing_multiline_head = true;
                        head_text_accumulator.push_str(trimmed);
                        head_text_accumulator.push(' ');
                        // Check if closed on same line (only if not opening a block with '{')
                        if !trimmed.ends_with('{') && (trimmed.contains(":: <") || (!trimmed.contains("WHERE") && !trimmed.contains("WHO"))) {
                            parsing_multiline_head = false;
                            let head = Self::parse_tape_head(&head_text_accumulator, line_number)?;
                            logger.log("TAPE", format!("Discovered Tape Head: [{}]", head.name));
                            head_opt = Some(head);
                            head_text_accumulator.clear();
                        }
                    } else {
                        // Plain line without leading bracket treated as fallback roll
                        raw_roll_lines.push((line_number, trimmed.to_string()));
                    }
                } else {
                    // Accumulating multiline Tape Head
                    if trimmed == "}" {
                        // Closing brace of block-manifest Tape Head
                        parsing_multiline_head = false;
                        let head = Self::parse_tape_head(&head_text_accumulator, line_number)?;
                        logger.log("TAPE", format!("Discovered Tape Head: [{}]", head.name));
                        head_opt = Some(head);
                        head_text_accumulator.clear();
                        continue;
                    } else if trimmed.starts_with("WHERE")
                        || trimmed.starts_with("WHEN")
                        || trimmed.starts_with("WHY")
                        || trimmed.starts_with("HOW")
                        || trimmed.starts_with("WHO")
                        || trimmed.starts_with("WHAT")
                        || trimmed.starts_with("< Scripture")
                        || trimmed.starts_with("::")
                        || trimmed.contains(":: <")
                    {
                        head_text_accumulator.push_str(trimmed);
                        head_text_accumulator.push(' ');
                    } else {
                        // Current line is not part of the head; the multiline head is complete
                        parsing_multiline_head = false;
                        let head = Self::parse_tape_head(&head_text_accumulator, line_number.saturating_sub(1))?;
                        logger.log("TAPE", format!("Discovered Tape Head: [{}]", head.name));
                        head_opt = Some(head);
                        head_text_accumulator.clear();

                        raw_roll_lines.push((line_number, trimmed.to_string()));
                    }
                }
                continue;
            }

            raw_roll_lines.push((line_number, trimmed.to_string()));
        }

        // If multiline head was accumulating at EOF
        if head_opt.is_none() && !head_text_accumulator.is_empty() {
            let head = Self::parse_tape_head(&head_text_accumulator, 1)?;
            head_opt = Some(head);
        }

        let head = head_opt.unwrap_or_else(|| TapeHead {
            name: "Default_Tape".to_string(),
            raw_text: "[Default_Tape]".to_string(),
            ..Default::default()
        });

        // Group raw_roll_lines into cohesive, multiline-aware Rolls
        let mut rolls = Vec::new();
        let mut cur_roll_text = String::new();
        let mut cur_roll_line = 1;
        let mut bracket_balance: i32 = 0;

        let is_new_roll_start = |s: &str| -> bool {
            let upper = s.to_uppercase();
            upper.starts_with("ROLL_")
                || upper.starts_with("[ ROLL")
                || upper.starts_with("[ROLL")
        };

        for (line_num, line_str) in raw_roll_lines {
            let starts_new = is_new_roll_start(&line_str);

            if starts_new && !cur_roll_text.is_empty() && bracket_balance <= 0 {
                rolls.push(Roll {
                    index: rolls.len(),
                    line_number: cur_roll_line,
                    text: cur_roll_text.trim().to_string(),
                    status: QuadState::Proceed,
                    is_head: false,
                });
                cur_roll_text.clear();
                bracket_balance = 0;
            }

            if cur_roll_text.is_empty() {
                cur_roll_line = line_num;
            }

            for ch in line_str.chars() {
                match ch {
                    '[' | '{' => bracket_balance += 1,
                    ']' | '}' => bracket_balance -= 1,
                    _ => {}
                }
            }

            cur_roll_text.push_str(&line_str);
            cur_roll_text.push(' ');

            // If not an explicit roll_... format and bracket_balance == 0, flush standalone roll
            if !starts_new && !cur_roll_text.starts_with("roll_") && !cur_roll_text.to_uppercase().starts_with("[ ROLL") && bracket_balance <= 0 {
                rolls.push(Roll {
                    index: rolls.len(),
                    line_number: cur_roll_line,
                    text: cur_roll_text.trim().to_string(),
                    status: QuadState::Proceed,
                    is_head: false,
                });
                cur_roll_text.clear();
                bracket_balance = 0;
            }
        }

        if !cur_roll_text.trim().is_empty() {
            rolls.push(Roll {
                index: rolls.len(),
                line_number: cur_roll_line,
                text: cur_roll_text.trim().to_string(),
                status: QuadState::Proceed,
                is_head: false,
            });
        }

        Ok(Self {
            head,
            rolls,
            cursor: 0,
            bound_partition: None,
            execution_state: QuadState::Proceed,
        })
    }

    /// Loads and parses a `.tape` file from the filesystem.
    pub fn from_file(path: impl AsRef<Path>, logger: DiagnosticLogger) -> Result<Self, String> {
        let p = path.as_ref();
        let resolved = crate::core::filetype::resolve_track_path(p);
        let content = fs::read_to_string(&resolved)
            .map_err(|e| format!("Failed to read tape file {:?}: {e}", resolved))?;
        Self::from_source(&content, logger)
    }

    /// Parses the raw text of a Tape Head roll into structured cardinal clauses.
    fn parse_tape_head(text: &str, line_number: usize) -> Result<TapeHead, String> {
        let clean = text.trim();

        // 1. Extract Name (e.g., from `[ Process { State } ]`, `[Tape.Name]` or `[Cell]`)
        let name = if let Some(proc_idx) = clean.find("Process {") {
            if let Some(end_brace) = clean[proc_idx..].find('}') {
                clean[proc_idx + 9..proc_idx + end_brace].trim().to_string()
            } else {
                "Process".to_string()
            }
        } else if let Some(start) = clean.find('[') {
            if let Some(end) = clean[start..].find(']') {
                let inner = &clean[start + 1..start + end];
                inner.trim_start_matches("Tape.").trim().to_string()
            } else {
                format!("Tape_L{}", line_number)
            }
        } else {
            format!("Tape_L{}", line_number)
        };

        // 2. Extract Cardinal Clauses
        let who = extract_clause(clean, "WHO").unwrap_or_else(|| "System".to_string());
        let what = extract_clause(clean, "WHAT").unwrap_or_else(|| name.clone());
        let when_clause = extract_clause(clean, "WHEN").unwrap_or_else(|| "Opening".to_string());
        let where_clause = extract_clause(clean, "WHERE").unwrap_or_else(|| "Memory".to_string());
        let why = extract_parenthetical(clean, "WHY").unwrap_or_else(|| "Operational task".to_string());
        let how = extract_clause(clean, "HOW").unwrap_or_else(|| "+1 : PROCEED".to_string());

        // 3. Extract required partition size from WHERE clause
        let mut required_partition_size = 256;
        if clean.contains("WHERE") {
            for token in clean.split_whitespace() {
                if let Ok(num) = token.trim_matches(|c: char| !c.is_ascii_digit()).parse::<usize>() {
                    if num > 0 {
                        required_partition_size = num;
                        break;
                    }
                }
            }
        }

        Ok(TapeHead {
            name,
            who,
            what,
            when_clause,
            where_clause,
            why,
            how,
            required_partition_size,
            raw_text: clean.to_string(),
        })
    }

    /// Returns the roll currently under the Tape Head cursor.
    pub fn active_roll(&self) -> Option<&Roll> {
        self.rolls.get(self.cursor)
    }

    /// Advances the Tape Head cursor to the next roll, returning the newly focused roll.
    pub fn advance(&mut self) -> Option<&Roll> {
        if self.cursor < self.rolls.len() {
            self.cursor += 1;
        }
        self.active_roll()
    }

    /// Returns true if all subordinate rolls have been traversed.
    pub fn is_finished(&self) -> bool {
        self.cursor >= self.rolls.len()
    }

    /// Resets the Tape Head back to the first subordinate roll.
    pub fn reset(&mut self) {
        self.cursor = 0;
        for r in &mut self.rolls {
            r.status = QuadState::Proceed;
        }
        self.execution_state = QuadState::Proceed;
    }

    /// Rewinds the Tape Head cursor back to an explicit roll index.
    pub fn rewind_to(&mut self, target_index: usize) {
        if target_index < self.rolls.len() {
            self.cursor = target_index;
        }
    }

    /// Searches for a roll containing a scene landmark cue `< landmark >` or name.
    pub fn find_landmark_index(&self, landmark: &str) -> Option<usize> {
        let trimmed = landmark.trim();
        for (idx, roll) in self.rolls.iter().enumerate() {
            if roll.text.contains(&format!("< {trimmed} >"))
                || roll.text.contains(&format!("<{trimmed}>"))
                || roll.text.contains(&format!("::{trimmed}"))
                || roll.text.contains(trimmed)
            {
                return Some(idx);
            }
        }
        None
    }

    /// Total number of subordinate rolls on the tape.
    pub fn roll_count(&self) -> usize {
        self.rolls.len()
    }
}


/// Helper function to extract clause content bounded by braces `{...}` or brackets `[...]`.
fn extract_clause(text: &str, keyword: &str) -> Option<String> {
    if let Some(pos) = text.find(keyword) {
        let rest = &text[pos + keyword.len()..];
        // Look for { ... }
        if let Some(open) = rest.find('{') {
            if let Some(close) = rest[open..].find('}') {
                return Some(rest[open + 1..open + close].trim().to_string());
            }
        }
        // Look for [ ... ]
        if let Some(open) = rest.find('[') {
            if let Some(close) = rest[open..].find(']') {
                return Some(rest[open + 1..open + close].trim().to_string());
            }
        }
    }
    None
}

/// Helper function to extract aside content bounded by parentheses `(...)`.
fn extract_parenthetical(text: &str, keyword: &str) -> Option<String> {
    if let Some(pos) = text.find(keyword) {
        let rest = &text[pos + keyword.len()..];
        if let Some(open) = rest.find('(') {
            if let Some(close) = rest[open..].find(')') {
                return Some(rest[open + 1..open + close].trim().to_string());
            }
        }
    }
    None
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests, Verification & Subsystem Sealing )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tape_parsing_and_head_extraction() {
        let source = r#"
            # Definition Roll / Tape Head
            [Tape.TaskWorker] : WHO {User} WHAT {Worker} --> DISPATCH {Worker}
                WHEN {Opening}
                WHERE {partition} = [ 512 ]
                WHY ( Secure background task execution )
                HOW [ +1 : PROCEED ]
                :: < covenant > ( verified )

            # Subordinate Rolls
            roll_1 [ 100 --> Buf @ 0 ] ( Initial write )
            roll_2 [ Buf[0] = 100 ! [ +1 ] ] ( Integrity verify )
            roll_3 [ commit_status [ PROCEED ] ] ( Tape sealed )
        "#;

        let logger = DiagnosticLogger::new(false);
        let mut tape = Tape::from_source(source, logger).expect("Parsing tape");

        assert_eq!(tape.head.name, "TaskWorker");
        assert_eq!(tape.head.who, "User");
        assert_eq!(tape.head.what, "Worker");
        assert_eq!(tape.head.when_clause, "Opening");
        assert_eq!(tape.head.why, "Secure background task execution");
        assert_eq!(tape.head.required_partition_size, 512);

        assert_eq!(tape.roll_count(), 3);
        assert_eq!(tape.cursor, 0);

        let r1 = tape.active_roll().unwrap();
        assert!(r1.text.contains("100 --> Buf @ 0"));

        tape.advance();
        assert_eq!(tape.cursor, 1);
        let r2 = tape.active_roll().unwrap();
        assert!(r2.text.contains("Buf[0] = 100"));

        tape.advance();
        assert_eq!(tape.cursor, 2);

        tape.advance();
        assert!(tape.is_finished());
    }
}
