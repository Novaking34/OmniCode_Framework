// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Physical Memory-Mapped I/O: Bare-Metal VGA Text Buffer & Serial Console )
// ==============================================================================
//! OmniCode Physical MMIO Subsystem
//!
//! Provides the bare-metal hardware interaction layer for the bootable machine:
//!
//! # Physical Memory Architecture
//! - **0x000B8000 .. 0x000B8FA0**: Standard VGA text-mode video buffer (80 cols × 25 rows = 2000 cells).
//!   Each cell consists of an ASCII character code byte + color attribute byte.
//! - **0x03F8 .. 0x03FF**: COM1 UART 16550 Serial Port for low-level kernel telemetry.
//! - **0x00100000 .. 0x00FFFFFF**: Extended 16MB RAM partition for kernel and reels.
//!
//! When instructions on `Track 0 [KERNEL]` inscribe physical words to memory,
//! addresses in the MMIO range drive hardware video output and serial telemetry.

use crate::core::word::Word64;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( VGA Data Structures, Colors, and Serial Port Models )
// ==============================================================================

pub const VGA_BUFFER_BASE: u64 = 0x000B8000;
pub const VGA_COLS: usize = 80;
pub const VGA_ROWS: usize = 25;
pub const VGA_CELL_COUNT: usize = VGA_COLS * VGA_ROWS;

pub const COM1_SERIAL_PORT: u64 = 0x03F8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VgaCell {
    pub ascii: u8,
    pub attribute: u8,
}

impl Default for VgaCell {
    fn default() -> Self {
        Self {
            ascii: b' ',
            attribute: (VgaColor::Black as u8) << 4 | (VgaColor::LightGray as u8),
        }
    }
}

pub struct VgaBuffer {
    pub cells: [VgaCell; VGA_CELL_COUNT],
    pub cursor_col: usize,
    pub cursor_row: usize,
}

impl Default for VgaBuffer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SerialPort {
    pub buffer: Vec<u8>,
}

impl Default for SerialPort {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PhysicalMemoryMap {
    pub vga: VgaBuffer,
    pub serial: SerialPort,
    pub ram_cells: Vec<Word64>,
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( MMIO Driver Methods, Video Screen Rendering, and Address Dispatch )
// ==============================================================================

impl VgaBuffer {
    /// Creates an empty VGA text buffer cleared with light gray on black.
    pub fn new() -> Self {
        Self {
            cells: [VgaCell::default(); VGA_CELL_COUNT],
            cursor_col: 0,
            cursor_row: 0,
        }
    }

    /// Clears the video text buffer.
    pub fn clear(&mut self) {
        self.cells = [VgaCell::default(); VGA_CELL_COUNT];
        self.cursor_col = 0;
        self.cursor_row = 0;
    }

    /// Inscribes a single character byte at the current cursor location.
    pub fn write_char(&mut self, ch: u8, fg: VgaColor, bg: VgaColor) {
        if ch == b'\n' {
            self.new_line();
            return;
        }

        let attr = ((bg as u8) << 4) | (fg as u8);
        let idx = self.cursor_row * VGA_COLS + self.cursor_col;
        if idx < VGA_CELL_COUNT {
            self.cells[idx] = VgaCell { ascii: ch, attribute: attr };
        }

        self.cursor_col += 1;
        if self.cursor_col >= VGA_COLS {
            self.new_line();
        }
    }

    /// Writes an ASCII string to the video buffer.
    pub fn write_str(&mut self, s: &str, fg: VgaColor, bg: VgaColor) {
        for b in s.bytes() {
            self.write_char(b, fg, bg);
        }
    }

    /// Advances the cursor to the beginning of the next row, scrolling if needed.
    pub fn new_line(&mut self) {
        self.cursor_col = 0;
        if self.cursor_row + 1 < VGA_ROWS {
            self.cursor_row += 1;
        } else {
            // Scroll buffer up by 1 line
            for row in 1..VGA_ROWS {
                for col in 0..VGA_COLS {
                    let from_idx = row * VGA_COLS + col;
                    let to_idx = (row - 1) * VGA_COLS + col;
                    self.cells[to_idx] = self.cells[from_idx];
                }
            }
            // Clear last row
            let blank = VgaCell::default();
            for col in 0..VGA_COLS {
                let last_idx = (VGA_ROWS - 1) * VGA_COLS + col;
                self.cells[last_idx] = blank;
            }
        }
    }

    /// Renders the 80x25 video buffer as a framed ASCII string for terminal display.
    pub fn render_screen(&self, max_rows: usize) -> String {
        let mut out = String::new();
        out.push_str("\x1b[1;34m┌────────────────────────────────── [BARE-METAL VGA CONSOLE @ 0x000B8000] ──────────────────────────────────┐\x1b[0m\n");
        let rows_to_show = max_rows.min(VGA_ROWS);
        for row in 0..rows_to_show {
            out.push_str("\x1b[1;34m│\x1b[0m ");
            for col in 0..VGA_COLS {
                let cell = self.cells[row * VGA_COLS + col];
                let ch = if (32..=126).contains(&cell.ascii) {
                    cell.ascii as char
                } else {
                    ' '
                };
                out.push(ch);
            }
            out.push_str(" \x1b[1;34m│\x1b[0m\n");
        }
        out.push_str("\x1b[1;34m└──────────────────────────────────────────────────────────────────────────────────────────────────────────┘\x1b[0m\n");
        out
    }
}

impl SerialPort {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn write_byte(&mut self, b: u8) {
        self.buffer.push(b);
    }

    pub fn write_str(&mut self, s: &str) {
        self.buffer.extend_from_slice(s.as_bytes());
    }

    pub fn read_output(&self) -> String {
        String::from_utf8_lossy(&self.buffer).to_string()
    }
}

impl PhysicalMemoryMap {
    pub fn new(ram_capacity: usize) -> Self {
        Self {
            vga: VgaBuffer::new(),
            serial: SerialPort::new(),
            ram_cells: vec![Word64::zero(); ram_capacity],
        }
    }

    /// Dispatches a 64-bit word write to RAM or hardware MMIO peripherals.
    pub fn write(&mut self, address: u64, word: Word64) -> Result<(), String> {
        if (VGA_BUFFER_BASE..VGA_BUFFER_BASE + (VGA_CELL_COUNT as u64)).contains(&address) {
            let offset = (address - VGA_BUFFER_BASE) as usize;
            let ch = (word.payload().to_i64() as u8).max(32);
            let attr = ((word.raw >> 8) & 0xFF) as u8;
            self.vga.cells[offset] = VgaCell {
                ascii: ch,
                attribute: if attr == 0 { 0x07 } else { attr },
            };
            return Ok(());
        }

        if address == COM1_SERIAL_PORT {
            let b = (word.payload().to_i64() & 0xFF) as u8;
            self.serial.write_byte(b);
            return Ok(());
        }

        let idx = address as usize;
        if idx < self.ram_cells.len() {
            self.ram_cells[idx] = word;
            Ok(())
        } else {
            Err(format!("FAILPOINT [PhysicalBusFault]: Address 0x{:08X} unmapped on physical bus", address))
        }
    }

    /// Reads a 64-bit word from RAM or MMIO.
    pub fn read(&self, address: u64) -> Option<Word64> {
        if (VGA_BUFFER_BASE..VGA_BUFFER_BASE + (VGA_CELL_COUNT as u64)).contains(&address) {
            let offset = (address - VGA_BUFFER_BASE) as usize;
            let cell = self.vga.cells[offset];
            return Some(Word64::from_signed_int(cell.ascii as i64));
        }

        let idx = address as usize;
        self.ram_cells.get(idx).copied()
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests )
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vga_write_and_scroll() {
        let mut vga = VgaBuffer::new();
        vga.write_str("OMNICODE BOOT", VgaColor::White, VgaColor::Black);
        assert_eq!(vga.cells[0].ascii, b'O');
        assert_eq!(vga.cells[1].ascii, b'M');
        assert_eq!(vga.cells[12].ascii, b'T');
        assert_eq!(vga.cursor_col, 13);
        assert_eq!(vga.cursor_row, 0);

        vga.new_line();
        assert_eq!(vga.cursor_col, 0);
        assert_eq!(vga.cursor_row, 1);
    }

    #[test]
    fn test_physical_memory_map_mmio_dispatch() {
        let mut mem = PhysicalMemoryMap::new(1024);

        // Write to VGA MMIO
        let word_char = Word64::from_signed_int(b'A' as i64);
        mem.write(VGA_BUFFER_BASE, word_char).expect("VGA write");
        assert_eq!(mem.vga.cells[0].ascii, b'A');

        // Write to Serial COM1
        let word_serial = Word64::from_signed_int(b'!' as i64);
        mem.write(COM1_SERIAL_PORT, word_serial).expect("Serial write");
        assert_eq!(mem.serial.read_output(), "!");

        // Write to RAM
        mem.write(42, Word64::from_signed_int(999)).expect("RAM write");
        assert_eq!(mem.read(42).unwrap().payload().to_i64(), 999);
    }
}
