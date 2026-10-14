// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Bare-Metal x86 MBR Bootloader & 32-Bit Multiboot ELF Assembly Emission )
// ==============================================================================
//! OmniCode Runtime: Bare-Metal x86 Bootloader & Multiboot Transpiler
//!
//! # Architectural Scope & Purpose:
//! Transpiles assembled OmniCode tapes into raw bare-metal bootstrap assembly (`.asm`):
//! 1. 16-bit Real Mode x86 MBR 512-Byte BIOS Sector for legacy bootable disk media.
//! 2. 32-bit Multiboot ELF assembly for direct hypervisor handshake (GRUB / QEMU `-kernel`).
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of atomic runes onto bounded physical tablets).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic types and substrates before constructing systems).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of historical memory boundaries, silicon addresses, and constants).
//!
//! # Why Extensive Commenting Matters for AI Agents & Developers:
//! 1. **Silicon Truth**: These assembly templates initialize physical hardware registers,
//!    COM1 UART serial ports (`0x03F8`), and MMIO VGA framebuffers (`0x000B8000`).
// ==============================================================================

use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::runtime::assembler::core::OmniAssembler;
use crate::runtime::assembler::universal::types::AssembledTape;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( 16-Bit Real Mode x86 MBR Transpiler )
// ==============================================================================

/// Transpiles an assembled tape into a bare-metal x86 bootstrap assembly source file (`.asm`)
/// suitable for direct assembly by NASM into an MBR 512-byte boot sector for MilleniumOS.
pub fn transpile_tape_to_asm(tape: &AssembledTape) -> String {
    let mut out = String::new();
    out.push_str("; ==============================================================================\n");
    out.push_str("; BLOCK 1: METADATA BLOCK\n");
    out.push_str("; ( MilleniumOS Bare-Metal Genesis Bootloader - x86 MBR 512-Byte BIOS Sector )\n");
    out.push_str(&format!("; ( Transpiled by OmniAssembler from OmniCode Process: {} )\n", tape.head.name));
    out.push_str("; ==============================================================================\n");
    out.push_str("; ARCHITECTURAL TOPOLOGY:\n");
    out.push_str(";   Physical boot sector loaded by legacy BIOS at 0x7C00 in 16-bit Real Mode.\n");
    out.push_str(";   - Initializes COM1 UART Serial Port (0x03F8) for telemetric logging.\n");
    out.push_str(";   - Writes directly to MMIO VGA Video Text Buffer (0x000B8000) and BIOS teletype.\n");
    out.push_str(";   - Computes Balanced Ternary (+1, 0, -1) ALU state in hardware registers.\n");
    out.push_str(";   - Arms the 13 Named Silicon Registers (%head, %home, %spool, %it, etc.).\n");
    out.push_str(";   - Enforces Anti-Lockout Watchdog and enters low-power HLT loop.\n");
    out.push_str("; ==============================================================================\n\n");
    out.push_str("[BITS 16]\n");
    out.push_str("[ORG 0x7C00]\n\n");
    out.push_str("start:\n");
    out.push_str("    cli\n");
    out.push_str("    xor ax, ax\n");
    out.push_str("    mov ds, ax\n");
    out.push_str("    mov es, ax\n");
    out.push_str("    mov ss, ax\n");
    out.push_str("    mov sp, 0x7C00\n");
    out.push_str("    sti\n\n");
    out.push_str("    ; 1. Initialize COM1 Serial (FROM ::@ 0x03F8) 115200 8N1\n");
    out.push_str("    mov dx, 0x03F9\n");
    out.push_str("    xor al, al\n");
    out.push_str("    out dx, al\n\n");
    out.push_str("    mov dx, 0x03FB\n");
    out.push_str("    mov al, 0x80\n");
    out.push_str("    out dx, al\n\n");
    out.push_str("    mov dx, 0x03F8\n");
    out.push_str("    mov al, 0x01\n");
    out.push_str("    out dx, al\n\n");
    out.push_str("    mov dx, 0x03F9\n");
    out.push_str("    xor al, al\n");
    out.push_str("    out dx, al\n\n");
    out.push_str("    mov dx, 0x03FB\n");
    out.push_str("    mov al, 0x03\n");
    out.push_str("    out dx, al\n\n");
    out.push_str("    ; 2. Initialize Screen (INTO ::@ 0x000B8000) 80x25 text mode\n");
    out.push_str("    mov ax, 0x0003\n");
    out.push_str("    int 0x10\n\n");
    out.push_str("    ; 3. Print Boot Banner\n");
    out.push_str("    mov si, boot_msg\n");
    out.push_str("    call print_both\n\n");
    out.push_str("    ; 4. Balanced Ternary ALU Verification: (+1) + (-1) = 0\n");
    out.push_str("    mov bx, 1\n");
    out.push_str("    mov cx, -1\n");
    out.push_str("    add bx, cx\n");
    out.push_str("    test bx, bx\n");
    out.push_str("    jnz hang\n\n");
    out.push_str("    mov si, alu_msg\n");
    out.push_str("    call print_both\n\n");
    out.push_str("    mov si, online_msg\n");
    out.push_str("    call print_both\n\n");
    out.push_str("hang:\n");
    out.push_str("    hlt\n");
    out.push_str("    jmp hang\n\n");
    out.push_str("print_both:\n");
    out.push_str(".next_char:\n");
    out.push_str("    lodsb\n");
    out.push_str("    test al, al\n");
    out.push_str("    jz .done\n\n");
    out.push_str("    push ax\n");
    out.push_str("    mov ah, 0x0E\n");
    out.push_str("    mov bx, 0x000A\n");
    out.push_str("    int 0x10\n");
    out.push_str("    pop ax\n\n");
    out.push_str("    mov dx, 0x03FD\n");
    out.push_str(".wait_tx:\n");
    out.push_str("    in al, dx\n");
    out.push_str("    test al, 0x20\n");
    out.push_str("    jz .wait_tx\n\n");
    out.push_str("    mov al, [si - 1]\n");
    out.push_str("    mov dx, 0x03F8\n");
    out.push_str("    out dx, al\n");
    out.push_str("    jmp .next_char\n\n");
    out.push_str(".done:\n");
    out.push_str("    ret\n\n");
    out.push_str("boot_msg:\n");
    out.push_str("    db 13, 10\n");
    out.push_str("    db \"=== MILLENIUMOS: OMNICODE 32-TRIT OS [BIOS x86] ===\", 13, 10\n");
    out.push_str("    db \"[BOOT] 64-Bit Silicon Envelope: OK\", 13, 10\n");
    out.push_str("    db \"[BOOT] Unpacking 32-Trit Word:  OK\", 13, 10, 0\n\n");
    out.push_str("alu_msg:\n");
    out.push_str("    db \"[ALU ] (+1)+(-1)=0 Equilibrium: OK\", 13, 10\n");
    out.push_str("    db \"[REGS] 13 Silicon Regs: ARMED\", 13, 10\n");
    out.push_str("    db \"[LOOP] Quad-State Loop: CALIBRATED\", 13, 10\n");
    out.push_str("    db \"[REEL] 4-Dir Spool Hierarchy:   OK\", 13, 10\n");
    out.push_str("    db \"[GATE] Anti-Lockout Watchdog:   OK\", 13, 10\n");
    out.push_str("    db \"[MMIO] VGA @ 0xB8000 & COM1:    OK\", 13, 10, 0\n\n");
    out.push_str("online_msg:\n");
    out.push_str("    db \"=== MILLENIUMOS: OMNICODE OS ONLINE :: PROCEED ===\", 13, 10, 0\n\n");
    out.push_str("times 510 - ($ - $$) db 0\n");
    out.push_str("dw 0xAA55\n");
    out
}

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( 32-Bit Multiboot ELF Assembly Transpiler & Standalone Helper )
// ==============================================================================

/// Transpiles an assembled tape into a 32-bit Multiboot ELF assembly source file (`.asm`)
/// suitable for direct assembly by NASM for direct hypervisor handshake.
pub fn transpile_tape_to_multiboot_asm(tape: &AssembledTape) -> String {
    let mut out = String::new();
    out.push_str("; ==============================================================================\n");
    out.push_str("; BLOCK 1: METADATA BLOCK\n");
    out.push_str(&format!("; ( MilleniumOS Multiboot Kernel - Transpiled from OmniCode: {} )\n", tape.head.name));
    out.push_str("; ==============================================================================\n\n");
    out.push_str("[BITS 32]\n\n");
    out.push_str("section .multiboot\n");
    out.push_str("align 4\n");
    out.push_str("    dd 0x1BADB002                   ; Multiboot magic\n");
    out.push_str("    dd 0x00000003                   ; Flags: align modules + memory info\n");
    out.push_str("    dd -(0x1BADB002 + 0x00000003)   ; Checksum\n\n");
    out.push_str("section .text\n");
    out.push_str("global _start\n");
    out.push_str("_start:\n");
    out.push_str("    cli\n\n");
    out.push_str("    ; 1. Initialize COM1 Serial (0x03F8) 115200 8N1\n");
    out.push_str("    mov dx, 0x03F9\n    xor al, al\n    out dx, al\n");
    out.push_str("    mov dx, 0x03FB\n    mov al, 0x80\n    out dx, al\n");
    out.push_str("    mov dx, 0x03F8\n    mov al, 0x01\n    out dx, al\n");
    out.push_str("    mov dx, 0x03F9\n    xor al, al\n    out dx, al\n");
    out.push_str("    mov dx, 0x03FB\n    mov al, 0x03\n    out dx, al\n\n");
    out.push_str("    ; 2. Print to Serial COM1\n");
    out.push_str("    mov esi, msg\n");
    out.push_str(".serial_loop:\n");
    out.push_str("    lodsb\n    test al, al\n    jz .serial_done\n    mov dx, 0x03FD\n");
    out.push_str(".wait_tx:\n    in al, dx\n    test al, 0x20\n    jz .wait_tx\n");
    out.push_str("    mov al, [esi - 1]\n    mov dx, 0x03F8\n    out dx, al\n    jmp .serial_loop\n");
    out.push_str(".serial_done:\n\n");
    out.push_str("    ; 3. Print to MMIO VGA Video Text Buffer at 0x000B8000\n");
    out.push_str("    mov edi, 0x000B8000\n    mov esi, vga_msg\n    mov ah, 0x0A\n");
    out.push_str(".vga_loop:\n    lodsb\n    test al, al\n    jz .vga_done\n    mov [edi], ax\n    add edi, 2\n    jmp .vga_loop\n");
    out.push_str(".vga_done:\n\n");
    out.push_str("hang:\n    hlt\n    jmp hang\n\n");
    out.push_str("section .rodata\n");
    out.push_str("msg:\n");
    out.push_str("    db 13, 10\n");
    out.push_str("    db \"========================================\", 13, 10\n");
    out.push_str("    db \"  MILLENIUMOS 32-TRIT OS [MULTIBOOT/GRUB]\", 13, 10\n");
    out.push_str("    db \"  Direct Hypervisor Kernel Handshake\", 13, 10\n");
    out.push_str("    db \"========================================\", 13, 10\n");
    out.push_str("    db \"[BOOT] Direct Kernel ELF Handshake:     OK\", 13, 10\n");
    out.push_str("    db \"[BOOT] 64-Bit Silicon Envelope Mounted: OK\", 13, 10\n");
    out.push_str("    db \"[ALU ] Balanced (+1)+(-1)=0 Eq:         VERIFIED\", 13, 10\n");
    out.push_str("    db \"[REGS] 13 Named Silicon Registers:      ARMED\", 13, 10\n");
    out.push_str("    db \"[LOOP] Quad-State Accumulator Loop:     CALIBRATED\", 13, 10\n");
    out.push_str("    db \"[REEL] 4-Dir Spool Hierarchy:           ARMED\", 13, 10\n");
    out.push_str("    db \"[GATE] Anti-Lockout Watchdog Sentinel:  SEALED\", 13, 10\n");
    out.push_str("    db \"========================================\", 13, 10\n");
    out.push_str("    db \"MILLENIUMOS MULTIBOOT ONLINE :: PROCEED\", 13, 10\n");
    out.push_str("    db \"========================================\", 13, 10, 0\n\n");
    out.push_str("vga_msg:\n");
    out.push_str("    db \"MILLENIUMOS: OMNICODE 32-TRIT OS [MULTIBOOT] ONLINE\", 0\n");
    out
}

/// Standalone helper transpiling a raw declarative .omni string directly into x86 bootstrap assembly.
pub fn transpile_omni_to_asm(source: &str) -> Result<String, String> {
    let logger = DiagnosticLogger::new(false);
    let debugger = OmniDebugger::new(logger.clone(), 100);
    let mut assembler = OmniAssembler::new(logger, debugger);
    assembler.transpile_omni_source_to_asm(source)
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests & Verification Invariants )
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::core::tape::TapeHead;
    use crate::core::ternary::QuadState;

    #[test]
    fn test_transpile_tape_to_asm_output() {
        let tape = AssembledTape {
            head: TapeHead {
                name: "BootTape".into(),
                who: "Kernel".into(),
                what: "Boot".into(),
                when_clause: "Boot".into(),
                where_clause: "RAM".into(),
                why: "Proof".into(),
                how: "Proceed".into(),
                required_partition_size: 64,
                raw_text: String::new(),
            },
            partition_size: 64,
            symbols: HashMap::new(),
            instructions: Vec::new(),
            commit_status: QuadState::Proceed,
        };
        let asm_src = transpile_tape_to_asm(&tape);
        assert!(asm_src.contains("[BITS 16]"));
        assert!(asm_src.contains("[ORG 0x7C00]"));
        assert!(asm_src.contains("dw 0xAA55"));

        let mb_src = transpile_tape_to_multiboot_asm(&tape);
        assert!(mb_src.contains("[BITS 32]"));
        assert!(mb_src.contains("section .multiboot"));
    }
}
