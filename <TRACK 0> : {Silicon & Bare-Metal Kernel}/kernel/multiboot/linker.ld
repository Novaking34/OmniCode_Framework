/* ============================================================================
 * BLOCK 1: METADATA BLOCK
 * ( MilleniumOS 32-Bit Multiboot 1 Linker Script & Memory Topology )
 * ============================================================================
 * Module:           OmniCode.Kernel.Multiboot.Linker
 * System:           MilleniumOS (OmniCode Cognitive Framework)
 * Description:      ELF32 Linker Script, Memory Segmentation & W^X Enforcement
 * Version:          1.1.0 (Page-Aligned W^X Isolation & Exported Boundaries)
 * Schema:           4-Block Decalogue Transduction Specification v1.0
 * Created:          2026-10-11 (Epoch 2026.10 Inscription)
 * Updated:          2026-10-16 (PHDRS Segments, ALIGN(4K) & Dynamic Bounds)
 * Authors:          CreativeWorkzStudio LLC / ProfessorSeanEX
 * Repository:       https://github.com/Novaking34/OmniCode_Framework
 * License:          MIT / Apache-2.0
 * Target:           GNU ld (elf_i386) for 32-Bit Protected Mode ELF
 * Classification:   Rank 1 Invariant (< Invariant Covenant >)
 *
 * SCRIPTURE WITNESSES (CANONICAL INVARIANTS):
 *   - Proverbs 22:28 (KJV):
 *     "Remove not the ancient landmark, which thy fathers have set."
 *     (Preserving canonical memory landmarks: 1 MB base, page boundaries)
 *   - 1 Corinthians 14:40 (KJV):
 *     "Let all things be done decently and in order."
 *     (Strict 4KB page alignment and distinct permissions for memory segments)
 *   - Nehemiah 4:18 (KJV):
 *     "For the builders, every one had his sword girded by his side,
 *      and so builded."
 *     (W^X: Executable code is read-only; mutable data is never executable)
 * ============================================================================
 */

/* ============================================================================
 * BLOCK 2: ENVIRONMENT & PROGRAM HEADERS (PHDRS)
 * ( Explicit Segment Permissions: Enforcing W^X Security )
 * ============================================================================
 */

ENTRY(_start)

PHDRS
{
    boot   PT_LOAD FLAGS(5);  /* Segment 00: Read + Execute (Multiboot Header) */
    text   PT_LOAD FLAGS(5);  /* Segment 01: Read + Execute (Kernel Instructions) */
    rodata PT_LOAD FLAGS(4);  /* Segment 02: Read-Only (Constants, Strings) */
    data   PT_LOAD FLAGS(6);  /* Segment 03: Read + Write (Data, BSS, Stack) */
}

/* ============================================================================
 * BLOCK 3: BODY / SECTION TOPOLOGY & BOUNDARIES
 * ( 1 MB Physical Origin, 4KB Page Alignment & Dynamic Symbols )
 * ============================================================================
 */

SECTIONS
{
    /* Canonical 1 MB (0x00100000) Physical Base Landmark */
    . = 1M;
    _kernel_start = .;

    /* Multiboot Header (Must reside within first 8 KB of file) */
    .multiboot : ALIGN(4K) {
        KEEP(*(.multiboot))
    } :boot

    /* Executable Instructions (Read-Only + Execute) */
    .text : ALIGN(4K) {
        _text_start = .;
        *(.text)
        _text_end = .;
    } :text

    /* Read-Only Data (Lookup Tables, Banners & Format Strings) */
    .rodata : ALIGN(4K) {
        _rodata_start = .;
        *(.rodata)
        _rodata_end = .;
    } :rodata

    /* Initialized Writable Data */
    .data : ALIGN(4K) {
        _data_start = .;
        *(.data)
        _data_end = .;
    } :data

    /* Uninitialized Data (BSS) & Dedicated Kernel Stack */
    .bss : ALIGN(4K) {
        _bss_start = .;
        *(COMMON)
        *(.bss)
        _bss_end = .;
    } :data

    /* Compute Absolute Kernel End & Static Memory Allocation Size */
    . = ALIGN(4K);
    _kernel_end = .;
    _kernel_size = _kernel_end - _kernel_start;

    /* ========================================================================
     * BLOCK 4: DISCARD / SENTINEL BLOCK
     * ( Stripping Extraneous Metadata for Deterministic Binary Footprint )
     * ========================================================================
     */
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note.*)
    }
}
