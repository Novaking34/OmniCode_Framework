// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Universal Block Boundary Detector & Semantic Slice Extractor )
// ==============================================================================
//! Universal Block Boundary Detector
//!
//! Locates and slices the four canonical blocks across both `.omni` Source Reels
//! and `.tape` Process Ribbons using dual-layer detection:
//! 1. Comment Header `#` Delimiters (`# BLOCK 1`, `# BLOCK 2`, `# =================`).
//! 2. Semantic Code Keywords (`Opening`, `Body`, `Closing`, `= [Tape]:`, `[ ROLL 1 ]`).
//!
//! # Foundational Invariant:
//! Any code appearing prior to the `Opening` scope (or before the first operational
//! roll in `.tape`) is deterministically categorized as Block 1 (Metadata).

#[derive(Debug, Clone)]
pub struct BlockSlice {
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct FileBlockSlices {
    pub metadata: BlockSlice,
    pub opening: BlockSlice,
    pub body: BlockSlice,
    pub closing: BlockSlice,
}

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( BlockDetector Definition )
// ==============================================================================

pub struct BlockDetector;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Extraction Logic for Reels and Tapes )
// ==============================================================================

impl BlockDetector {
    /// Detects and extracts all four block slices from source code.
    pub fn detect_blocks(content: &str, is_tape: bool) -> FileBlockSlices {
        if is_tape {
            Self::detect_tape_blocks(content)
        } else {
            Self::detect_omni_blocks(content)
        }
    }

    /// Detects the 4-block lifecycle in `.omni` Source Reels.
    fn detect_omni_blocks(content: &str) -> FileBlockSlices {
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        let mut opening_line = None;
        let mut body_line = None;
        let mut closing_line = None;

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;
            let trimmed = line.trim();
            let upper = trimmed.to_uppercase();

            // Detect Opening Block boundary
            if opening_line.is_none() {
                if upper.contains("BLOCK 2")
                    || upper.contains("OPENING BLOCK")
                    || trimmed.starts_with("Opening {")
                    || trimmed.starts_with("Opening\n")
                    || trimmed == "Opening"
                {
                    opening_line = Some(line_num);
                    continue;
                }
            }

            // Detect Body Block boundary
            if opening_line.is_some() && body_line.is_none() {
                if upper.contains("BLOCK 3")
                    || upper.contains("BODY BLOCK")
                    || trimmed.starts_with("Body [")
                    || trimmed.starts_with("Body {")
                    || trimmed == "Body"
                {
                    body_line = Some(line_num);
                    continue;
                }
            }

            // Detect Closing Block boundary
            if body_line.is_some() && closing_line.is_none() {
                if upper.contains("BLOCK 4")
                    || upper.contains("CLOSING BLOCK")
                    || trimmed.starts_with("Closing {")
                    || trimmed.starts_with("Closing\n")
                    || trimmed.starts_with("Closing (")
                    || trimmed == "Closing"
                {
                    closing_line = Some(line_num);
                }
            }
        }

        let l_open = opening_line.unwrap_or(total_lines + 1);
        let l_body = body_line.unwrap_or(total_lines + 1);
        let l_close = closing_line.unwrap_or(total_lines + 1);

        // Invariant: Everything prior to Opening is Metadata (Block 1)
        let meta_end = if l_open > 1 && l_open <= total_lines {
            // If opening line is a comment header like '# BLOCK 2', backtrack slightly
            l_open - 1
        } else {
            total_lines
        };

        let meta_slice = Self::slice_lines(&lines, 1, meta_end);
        let open_slice = if l_open <= total_lines {
            let end = if l_body > l_open && l_body <= total_lines { l_body - 1 } else { total_lines };
            Self::slice_lines(&lines, l_open, end)
        } else {
            BlockSlice { start_line: 0, end_line: 0, content: String::new() }
        };

        let body_slice = if l_body <= total_lines {
            let end = if l_close > l_body && l_close <= total_lines { l_close - 1 } else { total_lines };
            Self::slice_lines(&lines, l_body, end)
        } else {
            BlockSlice { start_line: 0, end_line: 0, content: String::new() }
        };

        let close_slice = if l_close <= total_lines {
            Self::slice_lines(&lines, l_close, total_lines)
        } else {
            BlockSlice { start_line: 0, end_line: 0, content: String::new() }
        };

        FileBlockSlices {
            metadata: meta_slice,
            opening: open_slice,
            body: body_slice,
            closing: close_slice,
        }
    }

    /// Detects the Tape Head vs. Tape Rolls lifecycle in `.tape` Process Ribbons.
    fn detect_tape_blocks(content: &str) -> FileBlockSlices {
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        let mut rolls_indices: Vec<(usize, String)> = Vec::new();
        let mut head_end_line = None;

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;
            let trimmed = line.trim();
            let upper = trimmed.to_uppercase();

            if upper.starts_with("[ ROLL") || upper.starts_with("ROLL_") {
                if head_end_line.is_none() {
                    head_end_line = Some(if line_num > 1 { line_num - 1 } else { 1 });
                }
                rolls_indices.push((line_num, trimmed.to_string()));
            }
        }

        let head_end = head_end_line.unwrap_or(total_lines);
        let meta_slice = Self::slice_lines(&lines, 1, head_end);

        if rolls_indices.is_empty() {
            return FileBlockSlices {
                metadata: meta_slice,
                opening: BlockSlice { start_line: 0, end_line: 0, content: String::new() },
                body: BlockSlice { start_line: 0, end_line: 0, content: String::new() },
                closing: BlockSlice { start_line: 0, end_line: 0, content: String::new() },
            };
        }

        // Roll 1 serves as the initial setup / opening condition
        let (first_roll_line, _) = rolls_indices[0];
        let (last_roll_line, ref last_roll_text) = rolls_indices[rolls_indices.len() - 1];

        let is_last_commit = last_roll_text.to_uppercase().contains("COMMIT STATUS")
            || last_roll_text.to_uppercase().contains("COMMIT_STATUS");

        let open_end = if rolls_indices.len() > 1 {
            rolls_indices[1].0 - 1
        } else if is_last_commit {
            first_roll_line
        } else {
            total_lines
        };

        let open_slice = Self::slice_lines(&lines, first_roll_line, open_end);

        let body_slice = if rolls_indices.len() > 2 {
            let body_start = rolls_indices[1].0;
            let body_end = if is_last_commit { last_roll_line - 1 } else { total_lines };
            Self::slice_lines(&lines, body_start, body_end)
        } else if rolls_indices.len() == 2 && !is_last_commit {
            Self::slice_lines(&lines, rolls_indices[1].0, total_lines)
        } else {
            BlockSlice { start_line: 0, end_line: 0, content: String::new() }
        };

        let close_slice = if is_last_commit {
            Self::slice_lines(&lines, last_roll_line, total_lines)
        } else {
            BlockSlice { start_line: 0, end_line: 0, content: String::new() }
        };

        FileBlockSlices {
            metadata: meta_slice,
            opening: open_slice,
            body: body_slice,
            closing: close_slice,
        }
    }

    fn slice_lines(lines: &[&str], start: usize, end: usize) -> BlockSlice {
        if start == 0 || end == 0 || start > end || start > lines.len() {
            return BlockSlice { start_line: 0, end_line: 0, content: String::new() };
        }
        let clamped_end = end.min(lines.len());
        let slice = lines[(start - 1)..clamped_end].join("\n");
        BlockSlice {
            start_line: start,
            end_line: clamped_end,
            content: slice,
        }
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Status )
// ==============================================================================
// BlockDetector sealed.
