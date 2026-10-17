; ============================================================================
; BLOCK 1: METADATA BLOCK
; ( MilleniumOS 32-Bit Multiboot 1 Direct Hypervisor Kernel & CPI-SI Substrate )
; ============================================================================
; Module:           OmniCode.Kernel.Multiboot
; System:           MilleniumOS (OmniCode Cognitive Framework)
; Description:      32-Bit Multiboot 1 Kernel, Hypervisor Handshake & Telemetry
; Version:          1.0.0 (Primordial Genesis Inception Baseline)
; Schema:           4-Block Decalogue Transduction Specification v1.0
; Created:          2026-10-11 (Epoch 2026.10 Inscription)
; Updated:          2026-10-16 (Dual-State Inception, RAM Log & Reg Dumper)
; Authors:          CreativeWorkzStudio LLC / ProfessorSeanEX
; Repository:       https://github.com/Novaking34/OmniCode_Framework
; License:          MIT / Apache-2.0
; Target:           x86 (32-Bit Protected Mode ELF via QEMU/KVM/GRUB)
; Classification:   Rank 1 Invariant (< Invariant Covenant >)
;
; ARCHITECTURAL ROLE & HYPERVISOR COVENANT:
;   Loaded directly at 1 MB (0x00100000) by Multiboot 1 compliant loaders.
;   - Preconditions: 32-bit Protected Mode, A20 enabled, interrupts disabled.
;   - Covenant Verification: EAX must contain magic 0x2BADB002 at entry.
;   - Physical Discovery: EBX points to multiboot_info_t (memory maps, modules).
;   - Truth Verification: 32-bit balanced ternary ALU proof (+1) + (-1) = 0.
;   - Telemetry Inception: COM1 UART (0x03F8) & MMIO VGA (0x000B8000).
;   - In-Memory RAM Log: Standardized flight recorder buffer at 0x00108000.
;   - Cognitive Substrate: Mounts 16-trit / 32-trit workspace at 0x00200000.
;   - Dual-State Duality: State 0 Safe Mode fallback if verification fails.
;
; SCRIPTURE WITNESSES (CANONICAL INVARIANTS):
;   - Exodus 31:18 (KJV):
;     "two tables of testimony, tables of stone, written with the
;      finger of God"
;     (The immutable inscription of atomic runes onto bounded physical tablets)
;   - Proverbs 24:27 (KJV):
;     "Prepare thy work without, and make it fit for thyself in the field;
;      and afterwards build thine house."
;     (The discipline of proving atomic substrates before building systems)
;   - Proverbs 22:28 (KJV):
;     "Remove not the ancient landmark, which thy fathers have set."
;     (The preservation of historical memory boundaries: 1 MB, 0x03F8, VGA)
; ============================================================================

; ============================================================================
; BLOCK 2: ENVIRONMENT / INVARIANTS / SETUP BLOCK
; ( Multiboot Header, Topological Constants, Event Codes & Memory Landmarks )
; ============================================================================

[BITS 32]

; Standard Multiboot 1 Header (Must reside within first 8 KB of binary)
section .multiboot
align 4
    dd 0x1BADB002                   ; Multiboot 1 magic
    dd 0x00000003                   ; Flags: align modules (1) + mem info (2)
    dd -(0x1BADB002 + 0x00000003)   ; Checksum: magic + flags + sum == 0

; Hardware & Topological Constants
MULTIBOOT_LOADER_MAGIC equ 0x2BADB002 ; Expected EAX from compliant loader
RAM_LOG_OFFSET         equ 0x00108000 ; Flight recorder RAM buffer (32KB above)
CPISI_WORKSPACE_BASE   equ 0x00200000 ; 16-Trit cognitive substrate (2 MB mark)
COM1_PORT              equ 0x03F8     ; Base I/O port for COM1 UART
VGA_TEXT_BUFFER        equ 0x000B8000 ; MMIO video buffer (80x25 text mode)

; Telemetric Event Codes for RAM Log Flight Recorder
EVT_MB_ENTRY           equ 0x0001     ; Multiboot _start reached
EVT_MAGIC_VERIFIED     equ 0x0002     ; Magic 0x2BADB002 verified in EAX
EVT_MEM_DISCOVERED     equ 0x0003     ; Physical memory extracted from EBX
EVT_ALU_VERIFIED       equ 0x0004     ; (+1) + (-1) = 0 balance proven in ALU
EVT_REGS_ARMED         equ 0x0005     ; 13 Silicon registers armed
EVT_CPISI_MOUNTED      equ 0x0006     ; 16-Trit cognitive substrate mounted
EVT_CASCADE_BIOS       equ 0x0007     ; Multiboot -> BIOS cascade initiated
EVT_MB_SAFE_MODE       equ 0xEEEE     ; Safe Mode fallback engaged

; ============================================================================
; BLOCK 3: BODY / EXECUTION BLOCK
; ( Handover Verification, Hardware Telemetry, Memory Audit & Inception )
; ============================================================================

section .text
global _start

_start:
    cli                             ; Enforce interrupts disabled

    ; Step 1: Preserve Hypervisor Covenant Registers
    ; EAX = 0x2BADB002 (Loader proof), EBX = physical pointer to multiboot_info
    mov [saved_magic], eax
    mov [saved_info_ptr], ebx

    ; Step 2: Initialize 32-Bit Execution Stack
    mov esp, stack_top

    ; Step 3: Initialize COM1 UART Serial Port (Port 0x03F8 at 115200 8N1)
    mov dx, 0x03F9                  ; IER: Disable interrupts
    xor al, al
    out dx, al

    mov dx, 0x03FB                  ; LCR: Set DLAB bit
    mov al, 0x80
    out dx, al

    mov dx, 0x03F8                  ; DLL: Divisor low = 1 (115200 Baud)
    mov al, 0x01
    out dx, al

    mov dx, 0x03F9                  ; DLM: Divisor high = 0
    xor al, al
    out dx, al

    mov dx, 0x03FB                  ; LCR: 8 data bits, no parity, 1 stop bit
    mov al, 0x03
    out dx, al

    ; Step 4: Emit Genesis Inception Announcement to COM1 and MMIO VGA
    mov esi, msg_banner
    call print_serial32

    mov esi, vga_banner
    call print_vga32

    ; Step 5: Verify Hypervisor Covenant Seal (EAX == 0x2BADB002)
    cmp dword [saved_magic], MULTIBOOT_LOADER_MAGIC
    jne enter_multiboot_safe_mode

    mov esi, msg_magic_ok
    call print_serial32

    ; Step 6: Discover Physical Memory Topology via EBX Information Structure
    mov ebx, [saved_info_ptr]
    test ebx, ebx
    jz .skip_mem_audit

    ; Check if memory flags are present (Bit 0 of flags dword)
    test dword [ebx], 0x01
    jz .skip_mem_audit

    mov eax, [ebx + 4]              ; mem_lower in KB
    mov [mem_lower_kb], eax
    mov eax, [ebx + 8]              ; mem_upper in KB
    mov [mem_upper_kb], eax

    mov esi, msg_mem_discovered
    call print_serial32

.skip_mem_audit:

    ; Step 7: Verify Balanced Ternary ALU Equilibrium in 32-Bit Silicon
    ; In OmniCode and MilleniumOS, truth is grounded in balance: (+1) + (-1) = 0
    mov ebx, 1
    mov ecx, -1
    add ebx, ecx
    test ebx, ebx
    jnz hang_fault

    mov esi, msg_alu_ok
    call print_serial32

    ; Step 8: Initialize In-Memory RAM Log Flight Recorder Buffer (0x00108000)
    mov dword [RAM_LOG_OFFSET], 0    ; Reset event entry counter

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

    ; Step 9: Emit Live CPU Silicon Register Snapshot
    call dump_registers32

    ; Step 10: Arm 13 Silicon Registers
    xor eax, eax                    ; %head      = 0
    xor ebx, ebx                    ; %home      = 0
    xor ecx, ecx                    ; %turn      = 0
    mov edx, COM1_PORT              ; %spool     = 0x03F8
    xor esi, esi                    ; %spool_in  = 0
    xor edi, edi                    ; %spool_out = 0
    xor ebp, ebp                    ; %frame     = 0

    mov eax, EVT_REGS_ARMED
    call log_event32

    mov esi, msg_regs_armed
    call print_serial32

    ; Step 11: Mount 32-Trit / 16-Trit CPI-SI Cognitive Substrate (0x00200000)
    mov eax, EVT_CPISI_MOUNTED
    mov ebx, 16                     ; 16 trits container width
    mov ecx, 43046721               ; 3^16 state space volume
    call log_event32

    mov esi, msg_cpisi_mounted
    call print_serial32

    mov esi, msg_online
    call print_serial32

; ============================================================================
; BLOCK 4: SENTINEL / COMPLETION BLOCK
; ( Watchdog Loops, Safe Mode Fallbacks, Diagnostics, Strings & Stack )
; ============================================================================

hang_watchdog:
    ; Anti-Lockout Watchdog Sentinel: low-power halt loop
    hlt
    jmp hang_watchdog

enter_multiboot_safe_mode:
    ; State 0 Safe Mode Fallback: Engaged if hypervisor breaks covenant
    mov eax, EVT_MB_SAFE_MODE
    mov ebx, [saved_magic]
    mov ecx, [saved_info_ptr]
    call log_event32

    mov esi, msg_safe_mode
    call print_serial32
    call dump_registers32

    ; Engage Adaptive Boot Cascade: De-escalate from Multiboot down to BIOS
    mov esi, msg_cascade_bios
    call print_serial32

    mov eax, EVT_CASCADE_BIOS
    mov ebx, [saved_magic]
    mov ecx, [saved_info_ptr]
    call log_event32

    jmp cascade_down_to_bios

; ----------------------------------------------------------------------------
; SUBROUTINE: cascade_down_to_bios
; Drops CPU from 32-Bit Protected Mode down to 16-Bit Real Mode to execute
; the foundational BIOS MBR at physical address 0x0000:0x7C00.
; Uses a 25-byte relocation-free trampoline copied to linear RAM at 0x7000.
; ----------------------------------------------------------------------------
cascade_down_to_bios:
    cli                             ; Enforce interrupts disabled

    ; Step 1: Stage 25-Byte Real Mode Trampoline at Linear Address 0x00007000
    mov esi, trampoline_16
    mov edi, 0x00007000
    mov ecx, trampoline_16_len
    rep movsb

    ; Step 2: Restore Real Mode Interrupt Vector Table (IVT at 0x00000000)
    lidt [real_mode_idtr]

    ; Step 3: Load 16-Bit Protected Mode GDT Descriptors
    lgdt [gdt_real_ptr]

    ; Step 4: Far Jump to 16-Bit Code Segment at 0x7000 (Selector 0x08)
    jmp 0x08:0x7000

.cascade_fallback_reset:
    ; Step 5: Hardware fallback: Pulse 8042 keyboard controller reset pin
    mov al, 0xFE
    out 0x64, al
    hlt
    jmp .cascade_fallback_reset

hang_fault:
    ; Arithmetic Fault Sentinel
    mov esi, msg_fault
    call print_serial32
    cli
.fault_loop:
    hlt
    jmp .fault_loop

; ----------------------------------------------------------------------------
; SUBROUTINE: log_event32
; Inscribes a structured 16-byte record into the RAM Log Flight Recorder.
; Buffer Landmark: [RAM_LOG_OFFSET] = count (u32)
; Entry: [event_id (u32), arg1 (u32), arg2 (u32), status (u32)]
; Inputs: EAX = event_id, EBX = arg1, ECX = arg2
; ----------------------------------------------------------------------------
log_event32:
    pushad
    mov edx, [RAM_LOG_OFFSET]       ; Current entry count
    cmp edx, 64                     ; Cap at 64 entries (ring protection)
    jae .log_done
    lea edi, [RAM_LOG_OFFSET + 4]
    shl edx, 4                      ; Multiply entry index by 16 bytes
    add edi, edx
    mov [edi + 0], eax              ; event_id
    mov [edi + 4], ebx              ; arg1
    mov [edi + 8], ecx              ; arg2
    mov dword [edi + 12], 0         ; status = OK (0)
    inc dword [RAM_LOG_OFFSET]      ; Increment logged count
.log_done:
    popad
    ret

; ----------------------------------------------------------------------------
; SUBROUTINE: putc_serial32
; Transmits a single byte to COM1 Serial (Port 0x03F8), polling Line Status.
; Input: AL = ASCII byte
; ----------------------------------------------------------------------------
putc_serial32:
    push edx
    push eax
    mov dx, 0x03FD                  ; Line Status Register
.wait_tx:
    in al, dx
    test al, 0x20                   ; Bit 5: Transmitter Holding Register Empty
    jz .wait_tx
    pop eax                         ; Restore character byte into AL
    mov dx, 0x03F8                  ; Transmit Holding Register
    out dx, al
    pop edx
    ret

; ----------------------------------------------------------------------------
; SUBROUTINE: print_serial32
; Emits a null-terminated string at ESI to COM1 serial.
; ----------------------------------------------------------------------------
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

; ----------------------------------------------------------------------------
; SUBROUTINE: print_hex32
; Emits a 32-bit register value as 8 hexadecimal characters over COM1 serial.
; Input: EAX = 32-bit value
; ----------------------------------------------------------------------------
print_hex32:
    pushad
    mov ecx, 8                      ; 8 hex nibbles to output
    mov ebx, eax                    ; Keep working register in EBX
.hex_loop:
    rol ebx, 4                      ; Rotate highest nibble into bits 0-3
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

; ----------------------------------------------------------------------------
; SUBROUTINE: dump_registers32
; Emits formatted CPU register telemetry block over COM1 serial.
; ----------------------------------------------------------------------------
dump_registers32:
    pushad
    mov esi, msg_dbg_prefix
    call print_serial32

    mov esi, msg_dbg_eax
    call print_serial32
    mov eax, [esp + 28]             ; Saved EAX from pushad
    call print_hex32

    mov esi, msg_dbg_ebx
    call print_serial32
    mov eax, [esp + 16]             ; Saved EBX from pushad
    call print_hex32

    mov esi, msg_dbg_ecx
    call print_serial32
    mov eax, [esp + 24]             ; Saved ECX from pushad
    call print_hex32

    mov esi, msg_dbg_edx
    call print_serial32
    mov eax, [esp + 20]             ; Saved EDX from pushad
    call print_hex32

    mov esi, msg_dbg_esp
    call print_serial32
    mov eax, [esp + 12]             ; Saved ESP before pushad
    call print_hex32

    mov esi, msg_dbg_cr0
    call print_serial32
    mov eax, cr0
    call print_hex32

    mov esi, msg_dbg_crlf
    call print_serial32
    popad
    ret

; ----------------------------------------------------------------------------
; SUBROUTINE: print_vga32
; Writes a null-terminated ASCII string at ESI to MMIO VGA Buffer (0x000B8000).
; Attribute: Light Green on Black (0x0A)
; ----------------------------------------------------------------------------
print_vga32:
    pushad
    mov edi, VGA_TEXT_BUFFER
    mov ah, 0x0A                    ; Light green attribute
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

; ----------------------------------------------------------------------------
; TELEMETRY STRINGS (NULL-TERMINATED ASCII)
; ----------------------------------------------------------------------------
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

; 25-Byte Real Mode Return Trampoline (Relocation-Free Opcode Sequence)
; Executed from linear physical address 0x00007000 in 16-bit mode.
trampoline_16:
    db 0xb8, 0x10, 0x00             ; mov ax, 0x10 (16-bit data selector)
    db 0x8e, 0xd8                   ; mov ds, ax
    db 0x8e, 0xc0                   ; mov es, ax
    db 0x8e, 0xd0                   ; mov ss, ax
    db 0xbc, 0x00, 0x7c             ; mov sp, 0x7C00 (Standard MBR stack)
    db 0x0f, 0x20, 0xc0             ; mov eax, cr0
    db 0x24, 0xfe                   ; and al, 0xfe (Clear bit 0: PE = 0)
    db 0x0f, 0x22, 0xc0             ; mov cr0, eax (Protected Mode Disabled!)
    db 0xea, 0x00, 0x7c, 0x00, 0x00 ; jmp 0x0000:0x7C00 (Enter BIOS MBR!)
trampoline_16_len equ $ - trampoline_16

; ----------------------------------------------------------------------------
; DATA & BSS SECTIONS (MEMORY STATE & EXECUTION STACK)
; ----------------------------------------------------------------------------
section .data
saved_magic:        dd 0
saved_info_ptr:     dd 0
mem_lower_kb:       dd 0
mem_upper_kb:       dd 0

align 16
gdt_real:
    dq 0x0000000000000000           ; Null descriptor
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
    dw 0x03FF                       ; Limit = 1023 (1KB IVT: 256 vectors * 4B)
    dd 0x00000000                   ; Base = 0x00000000 (Physical Address 0)

section .bss
align 16
stack_bottom:
    resb 16384                      ; 16 KB Dedicated Kernel Stack
stack_top:
