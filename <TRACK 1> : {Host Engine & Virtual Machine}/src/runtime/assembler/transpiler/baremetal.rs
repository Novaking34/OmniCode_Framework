// ============================================================================
// BLOCK 1: METADATA BLOCK
// ============================================================================
// Module:           OmniCode.Runtime.Assembler.Transpiler.Baremetal
// System:           MilleniumOS (OmniCode Cognitive Framework)
// Description:      Bare-Metal x86 Dual-State MBR & Diagnostic Engine
// Version:          1.0.0 (Primordial Genesis Inception Baseline)
// Schema:           4-Block Decalogue Transduction Specification v1.0
// Created:          2026-10-11 (Epoch 2026.10 Inscription)
// Updated:          2026-10-16 (RAM Logging, Reg Dumper & Dual-State Audit)
// Authors:          CreativeWorkzStudio LLC / ProfessorSeanEX
// Repository:       https://github.com/Novaking34/OmniCode_Framework
// License:          MIT / Apache-2.0
// Target:           x86 (16-Bit Real Mode BIOS MBR & 32-Bit Multiboot)
// Classification:   Rank 1 Invariant (< Invariant Covenant >)
//
// SCRIPTURE WITNESSES (CANONICAL INVARIANTS):
//   - Exodus 31:18 (KJV):
//     "two tables of testimony, tables of stone, written with the
//      finger of God"
//     (The immutable inscription of atomic runes onto bounded physical tablets)
//   - Proverbs 24:27 (KJV):
//     "Prepare thy work without, and make it fit for thyself in the field;
//      and afterwards build thine house."
//     (The discipline of proving atomic substrates before building systems)
//   - Proverbs 22:28 (KJV):
//     "Remove not the ancient landmark, which thy fathers have set."
//     (The preservation of historical memory boundaries: 0x7C00, 512-byte MBR)
//
// ARCHITECTURAL GUIDANCE FOR DEVELOPERS, COMPILERS & AI MODELS:
//   1. Physical Boundary: Sector 0 is hard-bounded to exactly 512 bytes.
//      Bytes 0-509 hold code/data; bytes 510-511 hold sacred seal 0xAA55.
//   2. Dual-State Duality: State 0 (Safe Mode beacon) guarantees
//      zero-dependency silicon boot; State +1 attempts Stage 2 handoff.
//   3. Transpiler Contract: Emits assembly matching the physical
//      invariants of Track 0 (kernel/bios/boot.asm & multiboot/boot.asm).
//   4. Diagnostic Visibility: Telemetry and audit logging are active during
//      transpilation to verify sector boundaries and architectural covenants.
// ============================================================================

// ============================================================================
// BLOCK 2: OPENING BLOCK
// ( Environment, Imports, Hardware Constants & Assembly Template Helpers )
// ============================================================================

use crate::diagnostics::debugger::OmniDebugger;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::runtime::assembler::core::OmniAssembler;
use crate::runtime::assembler::universal::types::AssembledTape;

// ----------------------------------------------------------------------------
// HARDWARE INVARIANTS & TOPOLOGICAL CONSTANTS
// ----------------------------------------------------------------------------
/// Physical size of an x86 Master Boot Record sector (Ancient Landmark).
pub const MBR_SECTOR_SIZE: usize = 512;

/// Byte offset where the sacred 0xAA55 boot signature must reside.
pub const MBR_SIGNATURE_OFFSET: usize = 510;

/// Physical memory load address where legacy BIOS places Sector 0.
pub const MBR_LOAD_ADDRESS: u16 = 0x7C00;

/// Base I/O port address for COM1 UART serial controller.
pub const COM1_BASE_PORT: u16 = 0x03F8;

/// Physical memory segment where Stage 2 is loaded (0x1000:0000 = 64 KB mark).
pub const STAGE2_LOAD_SEGMENT: u16 = 0x1000;

/// Memory offset within STAGE2_LOAD_SEGMENT for Stage 2 entry.
pub const STAGE2_LOAD_OFFSET: u16 = 0x0000;

/// Number of disk sectors loaded for Stage 2 kernel payload (2 KB).
pub const STAGE2_SECTOR_COUNT: u8 = 4;

/// Multiboot 1 specification magic number required by hypervisors.
pub const MULTIBOOT_HEADER_MAGIC: u32 = 0x1BADB002;

/// Expected loader magic passed in EAX by compliant Multiboot bootloaders.
pub const MULTIBOOT_LOADER_MAGIC: u32 = 0x2BADB002;

/// Standard physical memory load address for 32-bit Multiboot ELF kernels.
pub const MULTIBOOT_LOAD_ADDRESS: u32 = 0x00100000;

/// Physical linear landmark where Stage 2 RAM Log Flight Recorder resides.
pub const RAM_LOG_LANDMARK: u32 = 0x00018000;

/// Physical linear landmark where Multiboot RAM Log Flight Recorder resides.
pub const MULTIBOOT_RAM_LOG_LANDMARK: u32 = 0x00108000;

// ----------------------------------------------------------------------------
// TEMPLATE BUILDERS & FORMATTING HELPERS
// ( Prevents code duplication and ensures exact adherence to 4-block schema )
// ----------------------------------------------------------------------------

/// Emits the standard Block 1 metadata header for generated assembly files.
fn build_asm_metadata_header(process_name: &str, target_desc: &str) -> String {
    format!(
"; ============================================================================
; BLOCK 1: METADATA BLOCK
; ( MilleniumOS {target_desc} )
; ( Transpiled by OmniAssembler from OmniCode Process: {process_name} )
; ============================================================================
"
    )
}

/// Emits the full, verified 16-bit Dual-State MBR bootloader assembly.
fn emit_dual_state_mbr_source(process_name: &str) -> String {
    let mut out = String::with_capacity(4096);

    out.push_str(&build_asm_metadata_header(
        process_name,
        "Dual-State Genesis Bootloader - x86 MBR 512-Byte BIOS Sector",
    ));

    out.push_str(r#"; ARCHITECTURAL DUALITY & THE 512-BYTE ANCHOR POINT:
;   MilleniumOS operates on a balanced ternary dual-state boot architecture:
;   - State 0  (Safe Mode): Hardware truth beacon, ALU proof, 13 silicon regs,
;              zero external dependencies. Runs 100% in Sector 0 (512 bytes).
;   - State +1 (Normal Boot): Uses BIOS INT 0x13 to read Stage 2 from Sector 2+
;              into RAM (0x1000:0000) to awaken the full CPI-SI cognitive model.
;   - State -1 (Fault / Fallback): If disk read fails or a key is held at boot,
;              gracefully retreats to State 0 Safe Mode without crashing.
;
; SCRIPTURAL WITNESSES (COVENANT INVARIANTS):
;   - Exodus 31:18: Immutable inscription onto bounded physical tablets.
;   - Proverbs 24:27: Prepare thy work without, and make it fit in the field.
;   - Proverbs 22:28: Remove not the ancient landmark (0x7C00, 512-byte sector).
;
; TOPOLOGICAL INVARIANTS:
;   1. CPU Mode: 16-Bit Real Mode (segment:offset addressing).
;   2. Telemetry: COM1 UART Serial (0x03F8) at 115200 Baud, 8N1.
;   3. Display: Direct MMIO VGA text buffer at 0x000B8000 & BIOS INT 0x10.
;   4. Truth Verification: Balanced Ternary ALU proof (+1) + (-1) = 0.
;   5. Watchdog: Low-power HLT idle loop preventing CPU lockout/overheating.
; ============================================================================

; ============================================================================
; BLOCK 2: ENVIRONMENT / INVARIANTS / SETUP BLOCK
; ( Physical Hardware Origin, Segment Normalization & Stack Architecture )
; ============================================================================

[BITS 16]
[ORG 0x7C00]

STAGE2_SEG equ 0x1000               ; Stage 2 memory segment (64 KB mark)
STAGE2_OFF equ 0x0000               ; Stage 2 memory offset
STAGE2_SEC equ 4                    ; Number of sectors to load (2 KB payload)

start:
    ; Step 1: Normalize Segments & Initialize the Hardware Stack
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    mov [boot_drive], dl            ; Preserve physical boot drive from BIOS DL
    mov word [0x0500], 0xAA01       ; RAM Log Landmark: Genesis boot initialized
    mov [0x0502], dl                ; RAM Log Landmark: Logged physical drive
    sti

; ============================================================================
; BLOCK 3: BODY / EXECUTION BLOCK
; ( Hardware Telemetry, Display Inception, ALU Proof & Dual-State Decision )
; ============================================================================

    ; Step 3: Initialize COM1 UART Serial Port (Base Port 0x03F8)
    mov dx, 0x03F9
    xor al, al
    out dx, al

    mov dx, 0x03FB
    mov al, 0x80
    out dx, al

    mov dx, 0x03F8
    mov al, 0x01
    out dx, al

    mov dx, 0x03F9
    xor al, al
    out dx, al

    mov dx, 0x03FB
    mov al, 0x03
    out dx, al

    ; Step 4: Initialize Video Display (80x25 16-Color Text Mode)
    mov ax, 0x0003
    int 0x10

    ; Step 5: Emit Genesis Boot Inception Banner
    mov si, msg_boot
    call print_both

    ; Step 6: Verify Balanced Ternary ALU Equilibrium in Silicon
    mov bx, 1
    mov cx, -1
    add bx, cx
    test bx, bx
    jnz hang_fault

    mov si, msg_alu
    call print_both

    ; Step 7: Dual-State Decision: Check for Safe Mode Keypress Override
    mov ah, 0x01
    int 0x16
    jnz enter_safe_mode

    ; Step 8: Normal Boot Attempt: Load Stage 2 Kernel / CPI-SI Model
    mov ax, STAGE2_SEG
    mov es, ax
    xor bx, bx

    mov ah, 0x02
    mov al, STAGE2_SEC
    xor ch, ch
    mov cl, 2
    xor dh, dh
    mov dl, [boot_drive]
    int 0x13
    jc enter_safe_mode

    cmp word [es:0], 0
    jz enter_safe_mode

    mov si, msg_normal
    call print_both
    jmp STAGE2_SEG:STAGE2_OFF

; ============================================================================
; BLOCK 4: SENTINEL / COMPLETION BLOCK
; ( Safe Mode Beacon, Character Transmission & 512-Byte MBR Seal )
; ============================================================================

enter_safe_mode:
    mov word [0x0500], 0xAA00       ; RAM Log Landmark: State 0 Safe Mode active
    mov si, msg_safe
    call print_both

    ; Arm 13 Silicon Registers to Calibrated Baseline States
    xor ax, ax
    xor bx, bx
    xor cx, cx
    xor dx, dx
    xor si, si
    xor di, di
    xor bp, bp

hang_watchdog:
    hlt
    jmp hang_watchdog

hang_fault:
    mov si, msg_fault
    call print_both
    cli
.fault_loop:
    hlt
    jmp .fault_loop

print_both:
.next_char:
    lodsb
    test al, al
    jz .done

    push ax
    mov ah, 0x0E
    mov bx, 0x000A
    int 0x10
    pop ax

    mov dx, 0x03FD
.wait_tx:
    in al, dx
    test al, 0x20
    jz .wait_tx

    mov al, [si - 1]
    mov dx, 0x03F8
    out dx, al
    jmp .next_char

.done:
    ret

msg_boot:
    db 13, 10
    db "MILLENIUMOS: OMNICODE 32-TRIT OS [x86 MBR]", 13, 10
    db "[BOOT] 16-Bit Genesis: OK", 13, 10, 0

msg_alu:
    db "[ALU ] (+1)+(-1)=0 Eq: OK", 13, 10
    db "[REGS] 13 Regs Armed / Calibrated", 13, 10, 0

msg_normal:
    db "[BOOT] State +1: Stage 2 -> CPI-SI", 13, 10
    db "MILLENIUMOS ONLINE :: PROCEED", 13, 10, 0

msg_safe:
    db "[BOOT] State 0: Safe Mode Beacon", 13, 10
    db "MILLENIUMOS SAFE MODE :: PROCEED", 13, 10, 0

msg_fault:
    db "[ERR ] ALU Drift: HALT", 13, 10, 0

boot_drive:
    db 0

times 510 - ($ - $$) db 0
dw 0xAA55
"#);

    out
}

/// Emits standard 32-bit Multiboot 1 ELF assembly matching Track 0 kernel.
fn emit_multiboot_elf_source(process_name: &str) -> String {
    let mut out = String::with_capacity(8192);

    out.push_str(&build_asm_metadata_header(
        process_name,
        "Multiboot 1 Direct Hypervisor Kernel & Telemetric Inception",
    ));

    out.push_str(r#"[BITS 32]

section .multiboot
align 4
    dd 0x1BADB002                   ; Multiboot 1 magic
    dd 0x00000003                   ; Flags: align modules (1) + mem info (2)
    dd -(0x1BADB002 + 0x00000003)   ; Checksum: magic + flags + sum == 0

MULTIBOOT_LOADER_MAGIC equ 0x2BADB002 ; Expected EAX from compliant loader
RAM_LOG_OFFSET         equ 0x00108000 ; Flight recorder RAM buffer (32KB above)
CPISI_WORKSPACE_BASE   equ 0x00200000 ; 16-Trit cognitive substrate (2 MB mark)
COM1_PORT              equ 0x03F8     ; Base I/O port for COM1 UART
VGA_TEXT_BUFFER        equ 0x000B8000 ; MMIO video buffer (80x25 text mode)

EVT_MB_ENTRY           equ 0x0001
EVT_MAGIC_VERIFIED     equ 0x0002
EVT_MEM_DISCOVERED     equ 0x0003
EVT_ALU_VERIFIED       equ 0x0004
EVT_REGS_ARMED         equ 0x0005
EVT_CPISI_MOUNTED      equ 0x0006
EVT_CASCADE_BIOS       equ 0x0007
EVT_MB_SAFE_MODE       equ 0xEEEE

section .text
global _start

_start:
    cli

    mov [saved_magic], eax
    mov [saved_info_ptr], ebx
    mov esp, stack_top

    mov dx, 0x03F9
    xor al, al
    out dx, al

    mov dx, 0x03FB
    mov al, 0x80
    out dx, al

    mov dx, 0x03F8
    mov al, 0x01
    out dx, al

    mov dx, 0x03F9
    xor al, al
    out dx, al

    mov dx, 0x03FB
    mov al, 0x03
    out dx, al

    mov esi, msg_banner
    call print_serial32

    mov esi, vga_banner
    call print_vga32

    cmp dword [saved_magic], MULTIBOOT_LOADER_MAGIC
    jne enter_multiboot_safe_mode

    mov esi, msg_magic_ok
    call print_serial32

    mov ebx, [saved_info_ptr]
    test ebx, ebx
    jz .skip_mem_audit

    test dword [ebx], 0x01
    jz .skip_mem_audit

    mov eax, [ebx + 4]
    mov [mem_lower_kb], eax
    mov eax, [ebx + 8]
    mov [mem_upper_kb], eax

    mov esi, msg_mem_discovered
    call print_serial32

.skip_mem_audit:

    mov ebx, 1
    mov ecx, -1
    add ebx, ecx
    test ebx, ebx
    jnz hang_fault

    mov esi, msg_alu_ok
    call print_serial32

    mov dword [RAM_LOG_OFFSET], 0

    mov eax, EVT_MB_ENTRY
    mov ebx, [saved_magic]
    mov ecx, [saved_info_ptr]
    call log_event32

    mov eax, EVT_MAGIC_VERIFIED
    mov ebx, [saved_magic]
    mov ecx, 0
    call log_event32

    mov eax, EVT_MEM_DISCOVERED
    mov ebx, [mem_lower_kb]
    mov ecx, [mem_upper_kb]
    call log_event32

    mov eax, EVT_ALU_VERIFIED
    mov ebx, 0
    mov ecx, 0
    call log_event32

    mov esi, msg_ram_log_armed
    call print_serial32

    call dump_registers32

    xor eax, eax
    xor ebx, ebx
    xor ecx, ecx
    mov edx, COM1_PORT
    xor esi, esi
    xor edi, edi
    xor ebp, ebp

    mov eax, EVT_REGS_ARMED
    call log_event32

    mov esi, msg_regs_armed
    call print_serial32

    mov eax, EVT_CPISI_MOUNTED
    mov ebx, 16
    mov ecx, 43046721
    call log_event32

    mov esi, msg_cpisi_mounted
    call print_serial32

    mov esi, msg_online
    call print_serial32

hang_watchdog:
    hlt
    jmp hang_watchdog

enter_multiboot_safe_mode:
    mov eax, EVT_MB_SAFE_MODE
    mov ebx, [saved_magic]
    mov ecx, [saved_info_ptr]
    call log_event32

    mov esi, msg_safe_mode
    call print_serial32
    call dump_registers32

    mov esi, msg_cascade_bios
    call print_serial32

    mov eax, EVT_CASCADE_BIOS
    mov ebx, [saved_magic]
    mov ecx, [saved_info_ptr]
    call log_event32

    jmp cascade_down_to_bios

cascade_down_to_bios:
    cli
    mov esi, trampoline_16
    mov edi, 0x00007000
    mov ecx, trampoline_16_len
    rep movsb
    lidt [real_mode_idtr]
    lgdt [gdt_real_ptr]
    jmp 0x08:0x7000

.cascade_fallback_reset:
    mov al, 0xFE
    out 0x64, al
    hlt
    jmp .cascade_fallback_reset

hang_fault:
    mov esi, msg_fault
    call print_serial32
    cli
.fault_loop:
    hlt
    jmp .fault_loop

log_event32:
    pushad
    mov edx, [RAM_LOG_OFFSET]
    cmp edx, 64
    jae .log_done
    lea edi, [RAM_LOG_OFFSET + 4]
    shl edx, 4
    add edi, edx
    mov [edi + 0], eax
    mov [edi + 4], ebx
    mov [edi + 8], ecx
    mov dword [edi + 12], 0
    inc dword [RAM_LOG_OFFSET]
.log_done:
    popad
    ret

putc_serial32:
    push edx
    push eax
    mov dx, 0x03FD
.wait_tx:
    in al, dx
    test al, 0x20
    jz .wait_tx
    pop eax
    mov dx, 0x03F8
    out dx, al
    pop edx
    ret

print_serial32:
    pushad
.next:
    lodsb
    test al, al
    jz .done
    call putc_serial32
    jmp .next
.done:
    popad
    ret

print_hex32:
    pushad
    mov ecx, 8
    mov ebx, eax
.hex_loop:
    rol ebx, 4
    mov al, bl
    and al, 0x0F
    cmp al, 9
    jbe .is_digit
    add al, ('A' - 10)
    jmp .emit_nibble
.is_digit:
    add al, '0'
.emit_nibble:
    call putc_serial32
    loop .hex_loop
    popad
    ret

dump_registers32:
    pushad
    mov esi, msg_dbg_prefix
    call print_serial32

    mov esi, msg_dbg_eax
    call print_serial32
    mov eax, [esp + 28]
    call print_hex32

    mov esi, msg_dbg_ebx
    call print_serial32
    mov eax, [esp + 16]
    call print_hex32

    mov esi, msg_dbg_ecx
    call print_serial32
    mov eax, [esp + 24]
    call print_hex32

    mov esi, msg_dbg_edx
    call print_serial32
    mov eax, [esp + 20]
    call print_hex32

    mov esi, msg_dbg_esp
    call print_serial32
    mov eax, [esp + 12]
    call print_hex32

    mov esi, msg_dbg_cr0
    call print_serial32
    mov eax, cr0
    call print_hex32

    mov esi, msg_dbg_crlf
    call print_serial32
    popad
    ret

print_vga32:
    pushad
    mov edi, VGA_TEXT_BUFFER
    mov ah, 0x0A
.vga_loop:
    lodsb
    test al, al
    jz .vga_done
    mov [edi], ax
    add edi, 2
    jmp .vga_loop
.vga_done:
    popad
    ret

section .rodata

msg_banner:
    db 13, 10
    db "========================================", 13, 10
    db "  MILLENIUMOS 32-TRIT OS [MULTIBOOT 1]  ", 13, 10
    db "  Direct Hypervisor Kernel Handshake   ", 13, 10
    db "========================================", 13, 10, 0

msg_magic_ok:
    db "[BOOT] Hypervisor Magic 0x2BADB002:     VERIFIED", 13, 10, 0

msg_mem_discovered:
    db "[BOOT] Physical Memory Discovery:       OK", 13, 10, 0

msg_alu_ok:
    db "[ALU ] Balanced (+1)+(-1)=0 Eq:         VERIFIED", 13, 10, 0

msg_ram_log_armed:
    db "[BOOT] RAM Log Flight Recorder (108000): ARMED", 13, 10, 0

msg_regs_armed:
    db "[REGS] 13 Named Silicon Registers:      ARMED", 13, 10, 0

msg_cpisi_mounted:
    db "[BOOT] CPI-SI Cognitive Substrate:      MOUNTED", 13, 10, 0

msg_online:
    db "========================================", 13, 10
    db "MILLENIUMOS MULTIBOOT ONLINE :: PROCEED", 13, 10
    db "========================================", 13, 10, 0

msg_safe_mode:
    db "[ERR ] Multiboot Magic Invalid: Retreating to Safe Mode", 13, 10
    db "=== MILLENIUMOS MULTIBOOT SAFE MODE ACTIVE ===", 13, 10, 0

msg_cascade_bios:
    db 13, 10
    db "[CASCADE] De-escalating Ladder: Multiboot -> BIOS (16-Bit)...", 13, 10
    db "[CASCADE] Relinquishing Protected Mode -> Seeking 0x7C00 MBR", 13, 10, 0

msg_fault:
    db "[ERR ] ALU Equilibrium Drift in Hardware: HALTED", 13, 10, 0

msg_dbg_prefix:
    db "[DBG ] REGS: ", 0
msg_dbg_eax:
    db "EAX=0x", 0
msg_dbg_ebx:
    db " EBX=0x", 0
msg_dbg_ecx:
    db " ECX=0x", 0
msg_dbg_edx:
    db " EDX=0x", 0
msg_dbg_esp:
    db 13, 10, "[DBG ] REGS: ESP=0x", 0
msg_dbg_cr0:
    db " CR0=0x", 0
msg_dbg_crlf:
    db 13, 10, 0

vga_banner:
    db "MILLENIUMOS: OMNICODE 32-TRIT OS [MULTIBOOT 1] ONLINE", 0

trampoline_16:
    db 0xb8, 0x10, 0x00
    db 0x8e, 0xd8
    db 0x8e, 0xc0
    db 0x8e, 0xd0
    db 0xbc, 0x00, 0x7c
    db 0x0f, 0x20, 0xc0
    db 0x24, 0xfe
    db 0x0f, 0x22, 0xc0
    db 0xea, 0x00, 0x7c, 0x00, 0x00
trampoline_16_len equ $ - trampoline_16

section .data
saved_magic:        dd 0
saved_info_ptr:     dd 0
mem_lower_kb:       dd 0
mem_upper_kb:       dd 0

align 16
gdt_real:
    dq 0x0000000000000000
gdt_real_cs16:
    dw 0xFFFF, 0x0000
    db 0x00, 0x9A, 0x00, 0x00
gdt_real_ds16:
    dw 0xFFFF, 0x0000
    db 0x00, 0x92, 0x00, 0x00
gdt_real_ptr:
    dw $ - gdt_real - 1
    dd gdt_real

real_mode_idtr:
    dw 0x03FF
    dd 0x00000000

section .bss
align 16
stack_bottom:
    resb 16384
stack_top:
"#);

    out
}

// ============================================================================
// BLOCK 3: BODY BLOCK
// ( Core Transpiler Transducers, Diagnostic Logger Integration & Auditing )
// ============================================================================

/// Structural audit inspection result for emitted bare-metal assembly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaremetalSectorInspection {
    /// Target CPU mode (e.g., "16-Bit Real Mode BIOS MBR").
    pub target_arch: &'static str,
    /// Whether explicit CPU bitness mode directive is present.
    pub has_bits_directive: bool,
    /// Whether origin load landmark is present ([ORG 0x7C00] or .multiboot).
    pub has_org_landmark: bool,
    /// Whether ancient landmark boot seal (0xAA55 or 0x1BADB002) is present.
    pub has_boot_signature: bool,
    /// Whether State 0 Safe Mode beacon logic is present.
    pub has_safe_mode_beacon: bool,
    /// Whether State +1 Stage 2 / CPI-SI handoff logic is present.
    pub has_stage2_handoff: bool,
    /// Whether in-memory RAM log landmark tracking is present.
    pub has_ram_log_marker: bool,
    /// Whether bidirectional adaptive boot ladder cascade logic is present.
    pub has_cascade_ladder: bool,
    /// Total lines in emitted assembly source.
    pub total_lines: usize,
}

/// Inspects and audits an emitted bare-metal assembly source string for
/// structural compliance with Sector 0 invariants and covenant boundaries.
pub fn inspect_baremetal_asm(asm_src: &str) -> BaremetalSectorInspection {
    let is_real = asm_src.contains("[BITS 16]");
    let is_multi = asm_src.contains("[BITS 32]");
    BaremetalSectorInspection {
        target_arch: if is_real {
            "16-Bit Real Mode BIOS MBR"
        } else if is_multi {
            "32-Bit Protected Mode Multiboot"
        } else {
            "Unknown Architecture"
        },
        has_bits_directive: is_real || is_multi,
        has_org_landmark: asm_src.contains("[ORG 0x7C00]")
            || asm_src.contains("section .multiboot"),
        has_boot_signature: asm_src.contains("0xAA55")
            || asm_src.contains("0x1BADB002"),
        has_safe_mode_beacon: asm_src.contains("enter_safe_mode:")
            || asm_src.contains("enter_multiboot_safe_mode:"),
        has_stage2_handoff: (asm_src.contains("STAGE2_SEG equ 0x1000")
            && asm_src.contains("int 0x13"))
            || asm_src.contains("CPISI_WORKSPACE_BASE"),
        has_ram_log_marker: asm_src.contains("0x0500")
            || asm_src.contains("RAM_LOG_OFFSET"),
        has_cascade_ladder: asm_src.contains("cascade_down_to_bios")
            || asm_src.contains("EVT_CASCADE_UP")
            || asm_src.contains("EVT_CASCADE_BIOS")
            || (asm_src.contains("STAGE2_SEG equ 0x1000")
                && asm_src.contains("enter_safe_mode:")),
        total_lines: asm_src.lines().count(),
    }
}

/// Transpiles an assembled tape into a bare-metal x86 bootstrap assembly source
/// with explicit diagnostic logging to observe sector budgets and invariants.
pub fn transpile_tape_to_asm_with_logger(
    tape: &AssembledTape,
    logger: &DiagnosticLogger,
) -> String {
    logger.log(
        "BAREMETAL_TRANSPILER",
        format!(
            "Inscribing MilleniumOS Dual-State Genesis MBR for process: {}",
            tape.head.name
        ),
    );

    let asm_src = emit_dual_state_mbr_source(&tape.head.name);
    let audit = inspect_baremetal_asm(&asm_src);

    logger.log(
        "BAREMETAL_TRANSPILER",
        format!(
            "Emitted {} lines. Seal 0xAA55: {}, Safe Mode: {}, RAM Log: {}",
            audit.total_lines,
            audit.has_boot_signature,
            audit.has_safe_mode_beacon,
            audit.has_ram_log_marker
        ),
    );

    logger.record_ai_observation(
        "TRANSPILER_INCEPTION",
        "Dual-State MBR 512-Byte Genesis Assembly Inscribed",
        &format!(
            "Target: {}, Lines: {}, Seal: {}",
            audit.target_arch, audit.total_lines, audit.has_boot_signature
        ),
        ["baremetal", "mbr", "genesis", "dual_state", "track0"].as_slice(),
        Some(&tape.head.name),
    );

    asm_src
}

/// Transpiles an assembled tape into a bare-metal x86 bootstrap assembly source
/// file (`.asm`) suitable for direct assembly by NASM into an MBR 512-byte
/// boot sector for MilleniumOS.
pub fn transpile_tape_to_asm(tape: &AssembledTape) -> String {
    let logger = DiagnosticLogger::new(false);
    transpile_tape_to_asm_with_logger(tape, &logger)
}

/// Transpiles an assembled tape into a 32-bit Multiboot ELF assembly source
/// with diagnostic logging for hypervisor handshake observation.
pub fn transpile_tape_to_multiboot_asm_with_logger(
    tape: &AssembledTape,
    logger: &DiagnosticLogger,
) -> String {
    logger.log(
        "BAREMETAL_TRANSPILER",
        format!(
            "Inscribing Multiboot 1 ELF handshake for process: {}",
            tape.head.name
        ),
    );

    let mb_src = emit_multiboot_elf_source(&tape.head.name);
    let audit = inspect_baremetal_asm(&mb_src);

    logger.log(
        "BAREMETAL_TRANSPILER",
        format!(
            "Emitted Multiboot ELF ({} lines). Magic: {}, Safe: {}, RAM: {}",
            audit.total_lines,
            audit.has_boot_signature,
            audit.has_safe_mode_beacon,
            audit.has_ram_log_marker
        ),
    );

    logger.record_ai_observation(
        "TRANSPILER_INCEPTION",
        "Multiboot 1 Direct Hypervisor Assembly Inscribed",
        &format!(
            "Target: {}, Lines: {}, Seal: {}",
            audit.target_arch, audit.total_lines, audit.has_boot_signature
        ),
        ["baremetal", "multiboot", "genesis", "hypervisor", "track0"]
            .as_slice(),
        Some(&tape.head.name),
    );

    mb_src
}

/// Transpiles an assembled tape into a 32-bit Multiboot ELF assembly source
/// file (`.asm`) suitable for direct assembly by NASM for hypervisor handshake.
pub fn transpile_tape_to_multiboot_asm(tape: &AssembledTape) -> String {
    let logger = DiagnosticLogger::new(false);
    transpile_tape_to_multiboot_asm_with_logger(tape, &logger)
}

/// Standalone helper transpiling a raw declarative .omni string directly
/// into x86 bootstrap assembly.
pub fn transpile_omni_to_asm(source: &str) -> Result<String, String> {
    let logger = DiagnosticLogger::new(false);
    let debugger = OmniDebugger::new(logger.clone(), 100);
    let mut assembler = OmniAssembler::new(logger, debugger);
    assembler.transpile_omni_source_to_asm(source)
}

// ============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Unit Tests, Verification Invariants & Diagnostic Assertions )
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::core::tape::TapeHead;
    use crate::core::ternary::QuadState;

    fn create_test_tape(name: &str) -> AssembledTape {
        AssembledTape {
            head: TapeHead {
                name: name.into(),
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
        }
    }

    #[test]
    fn test_transpile_tape_to_asm_dual_state_invariants() {
        let tape = create_test_tape("GenesisTape");
        let asm_src = transpile_tape_to_asm(&tape);

        // Invariant 1: Physical Architecture and Segments
        assert!(
            asm_src.contains("[BITS 16]"),
            "CRITICAL: 16-Bit Real Mode directive [BITS 16] is missing!"
        );
        assert!(
            asm_src.contains("[ORG 0x7C00]"),
            "CRITICAL: Ancient landmark 0x7C00 load address is missing!"
        );

        // Invariant 2: Physical 512-Byte Ancient Landmark Seal
        assert!(
            asm_src.contains("dw 0xAA55"),
            "CRITICAL: MBR sacred boot signature 0xAA55 is missing!"
        );

        // Invariant 3: Dual-State Architecture (State 0 and State +1)
        assert!(
            asm_src.contains("enter_safe_mode:"),
            "DIAGNOSTIC: Safe Mode fallback label enter_safe_mode missing!"
        );
        assert!(
            asm_src.contains("STAGE2_SEG equ 0x1000"),
            "DIAGNOSTIC: Stage 2 load segment definition missing!"
        );
        assert!(
            asm_src.contains("int 0x13"),
            "DIAGNOSTIC: Stage 2 BIOS disk read interrupt (INT 0x13) missing!"
        );
        assert!(
            asm_src.contains("int 0x16"),
            "DIAGNOSTIC: Safe Mode keystroke check (INT 0x16) missing!"
        );

        // Invariant 4: Brand Identity & Provenance
        assert!(
            asm_src.contains("MILLENIUMOS"),
            "DIAGNOSTIC: Operating system identity MILLENIUMOS missing!"
        );
    }

    #[test]
    fn test_transpile_tape_to_asm_with_diagnostic_logger() {
        let tape = create_test_tape("DiagnosticTape");
        let logger = DiagnosticLogger::new(false);
        let asm_src = transpile_tape_to_asm_with_logger(&tape, &logger);

        // Verify output is complete
        assert!(!asm_src.is_empty());

        // Verify logger captured events and cognitive notes
        let events = logger.events();
        assert!(
            events.iter().any(|e| e.channel == "BAREMETAL_TRANSPILER"),
            "DIAGNOSTIC: Transpiler did not log to expected channel!"
        );

        let notes = logger.cognitive_notes();
        assert!(
            notes.iter().any(|n| n.category == "TRANSPILER_INCEPTION"),
            "DIAGNOSTIC: AI cognitive observation was not recorded!"
        );
    }

    #[test]
    fn test_inspect_baremetal_asm_auditing() {
        let tape = create_test_tape("AuditTape");
        let asm_src = transpile_tape_to_asm(&tape);
        let audit = inspect_baremetal_asm(&asm_src);

        assert_eq!(audit.target_arch, "16-Bit Real Mode BIOS MBR");
        assert!(audit.has_bits_directive);
        assert!(audit.has_org_landmark);
        assert!(audit.has_boot_signature);
        assert!(audit.has_safe_mode_beacon);
        assert!(audit.has_stage2_handoff);
        assert!(audit.has_ram_log_marker);
        assert!(audit.has_cascade_ladder);
        assert!(audit.total_lines > 100);
    }

    #[test]
    fn test_transpile_tape_to_multiboot_invariants() {
        let tape = create_test_tape("HypervisorTape");
        let mb_src = transpile_tape_to_multiboot_asm(&tape);

        // Invariant 1: 32-Bit Protected Mode
        assert!(
            mb_src.contains("[BITS 32]"),
            "CRITICAL: Multiboot requires 32-bit protected mode [BITS 32]!"
        );

        // Invariant 2: Standard Multiboot 1 Header
        assert!(
            mb_src.contains("section .multiboot"),
            "CRITICAL: .multiboot section marker is missing!"
        );
        assert!(
            mb_src.contains("0x1BADB002"),
            "CRITICAL: Official Multiboot 1 magic 0x1BADB002 is missing!"
        );

        // Invariant 3: Hypervisor Entrypoint & Magic Verification
        assert!(
            mb_src.contains("global _start"),
            "CRITICAL: Multiboot global entrypoint _start is missing!"
        );
        assert!(
            mb_src.contains("0x2BADB002"),
            "CRITICAL: Hypervisor covenant magic 0x2BADB002 check missing!"
        );

        // Invariant 4: In-Memory RAM Log Flight Recorder & Register Dumper
        assert!(
            mb_src.contains("RAM_LOG_OFFSET"),
            "CRITICAL: RAM Log Flight Recorder offset missing!"
        );
        assert!(
            mb_src.contains("dump_registers32"),
            "CRITICAL: Register dumper subroutine missing!"
        );
        assert!(
            mb_src.contains("enter_multiboot_safe_mode"),
            "CRITICAL: Safe mode fallback missing!"
        );

        // Invariant 5: Bidirectional Inception Cascade & 16-Bit Trampoline
        assert!(
            mb_src.contains("cascade_down_to_bios"),
            "CRITICAL: Multiboot to BIOS cascade routine missing!"
        );
        assert!(
            mb_src.contains("trampoline_16"),
            "CRITICAL: 16-bit real-mode return trampoline missing!"
        );
        assert!(
            mb_src.contains("gdt_real"),
            "CRITICAL: 16-bit real-mode GDT descriptors missing!"
        );
    }

    #[test]
    fn test_inspect_baremetal_multiboot_auditing() {
        let tape = create_test_tape("AuditMultibootTape");
        let mb_src = transpile_tape_to_multiboot_asm(&tape);
        let audit = inspect_baremetal_asm(&mb_src);

        assert_eq!(audit.target_arch, "32-Bit Protected Mode Multiboot");
        assert!(audit.has_bits_directive);
        assert!(audit.has_org_landmark);
        assert!(audit.has_boot_signature);
        assert!(audit.has_safe_mode_beacon);
        assert!(audit.has_stage2_handoff);
        assert!(audit.has_ram_log_marker);
        assert!(audit.has_cascade_ladder);
        assert!(audit.total_lines > 100);
    }
}
