// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Terminal Hardware MMIO Ports, Directional Streams & Invariant Constants )
// ==============================================================================
//! OmniCode Terminal: Hardware MMIO Ports & Directional Stream Substrate
//!
//! # Architectural Scope & Purpose:
//! Defines the physical silicon MMIO port addresses and ring buffer streams
//! connecting the host hardware environment to the OmniCode Virtual Machine:
//! - `FROM ::@ 0x03F8`: COM1 UART Serial Input port (Host Keyboard / Stdin).
//! - `INTO ::@ 0x000B8000`: Video Text Buffer / Console Output port (Host Screen / Stdout).
//! - Directional FIFO streaming containers ([`TerminalInputStream`], [`TerminalOutputStream`]).
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of physical data across bounded hardware channels).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic hardware port substrates before constructing user interfaces).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of historical MMIO addresses: UART 0x03F8 and VGA 0x000B8000).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **MMIO Port Invariance**: Explicit mapping of physical hardware addresses guarantees
//!    isomorphic behavior whether executing on the Linux host CLI, inside QEMU bare-metal,
//!    or via the local Web GUI server.
// ==============================================================================

use std::collections::VecDeque;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Hardware Port Constants & Stream Struct Definitions )
// ==============================================================================

/// Hardware MMIO Address for COM1 UART Serial Input (`FROM ::@ 0x03F8`).
pub const UART_COM1_INPUT_PORT: usize = 0x03F8;

/// Hardware MMIO Address for Video Graphics Array Text Buffer (`INTO ::@ 0x000B8000`).
pub const VGA_TEXT_OUTPUT_PORT: usize = 0x000B8000;

/// Default ring buffer capacity for terminal stream queues (1,024 characters).
pub const DEFAULT_TERMINAL_BUFFER_CAPACITY: usize = 1024;

/// Directional FIFO input stream connected to the hardware `FROM` port (`0x03F8`).
#[derive(Debug, Clone)]
pub struct TerminalInputStream {
    buffer: VecDeque<char>,
    max_capacity: usize,
}

/// Directional FIFO output stream connected to the hardware `INTO` port (`0x000B8000`).
#[derive(Debug, Clone)]
pub struct TerminalOutputStream {
    buffer: VecDeque<char>,
    max_capacity: usize,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Directional Stream Methods & Buffer Operations )
// ==============================================================================

impl Default for TerminalInputStream {
    fn default() -> Self {
        Self::new(DEFAULT_TERMINAL_BUFFER_CAPACITY)
    }
}

impl TerminalInputStream {
    /// Creates a new directional input stream with the designated buffer capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            max_capacity: capacity,
        }
    }

    /// Pushes a single character into the input buffer (simulating a hardware UART arrival).
    pub fn push_char(&mut self, c: char) {
        if self.buffer.len() >= self.max_capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(c);
    }

    /// Pushes an entire string slice into the input buffer.
    pub fn push_str(&mut self, s: &str) {
        for c in s.chars() {
            self.push_char(c);
        }
    }

    /// Pops the next available character from the input queue.
    pub fn pop_char(&mut self) -> Option<char> {
        self.buffer.pop_front()
    }

    /// Returns `true` if the input buffer has no pending characters.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Returns the number of characters currently buffered.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Clears all pending characters from the stream.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Default for TerminalOutputStream {
    fn default() -> Self {
        Self::new(DEFAULT_TERMINAL_BUFFER_CAPACITY)
    }
}

impl TerminalOutputStream {
    /// Creates a new directional output stream with the designated buffer capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            max_capacity: capacity,
        }
    }

    /// Pushes a projected character into the output buffer (simulating a VGA text write).
    pub fn push_char(&mut self, c: char) {
        if self.buffer.len() >= self.max_capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(c);
    }

    /// Pushes an entire string slice into the output buffer.
    pub fn push_str(&mut self, s: &str) {
        for c in s.chars() {
            self.push_char(c);
        }
    }

    /// Pops the next projected character from the output queue.
    pub fn pop_char(&mut self) -> Option<char> {
        self.buffer.pop_front()
    }

    /// Flushes all pending output characters into a single allocated String.
    pub fn flush_to_string(&mut self) -> String {
        let s: String = self.buffer.iter().collect();
        self.buffer.clear();
        s
    }

    /// Returns `true` if the output buffer has no pending characters.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Returns the number of characters currently buffered.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Clears all pending characters from the stream.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Test Verification Proofs & Stream Invariants )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_mmio_port_constants() {
        assert_eq!(UART_COM1_INPUT_PORT, 0x03F8);
        assert_eq!(VGA_TEXT_OUTPUT_PORT, 0x000B8000);
    }

    #[test]
    fn test_directional_input_stream_lifecycle() {
        let mut input = TerminalInputStream::new(16);
        assert!(input.is_empty());
        input.push_str("GLYPH");
        assert_eq!(input.len(), 5);
        assert_eq!(input.pop_char(), Some('G'));
        assert_eq!(input.pop_char(), Some('L'));
        assert_eq!(input.len(), 3);
        input.clear();
        assert!(input.is_empty());
    }

    #[test]
    fn test_directional_output_stream_flush() {
        let mut output = TerminalOutputStream::new(16);
        output.push_str("ECHO_OK");
        assert_eq!(output.len(), 7);
        let flushed = output.flush_to_string();
        assert_eq!(flushed, "ECHO_OK");
        assert!(output.is_empty());
    }
}
