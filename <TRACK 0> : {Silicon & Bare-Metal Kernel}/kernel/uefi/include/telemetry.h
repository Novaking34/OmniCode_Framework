// =============================================================================
// BLOCK 3: FLIGHT RECORDER & HARDWARE DIAGNOSTICS
// ( OmniCode UEFI Subsystem - In-Memory RAM Logger & Register Telemetry )
// =============================================================================
// Module:       OmniCode.Kernel.UEFI.Telemetry
// Architecture: x86_64 Long Mode RAM Logging & CPU Register Snapshot
// Invariant:    Fixed landmark 0x00200000 (2 MB) 16-byte structured records.
// =============================================================================

#ifndef OMNICODE_UEFI_TELEMETRY_H
#define OMNICODE_UEFI_TELEMETRY_H

#include "uefi.h"
#include "serial.h"

// -----------------------------------------------------------------------------
// In-Memory RAM Log Flight Recorder Landmarks
// -----------------------------------------------------------------------------
#define UEFI_RAM_LOG_LANDMARK   0x00200000ULL
#define UEFI_RAM_LOG_MAX_ENTRIES 64

#define EVT_UEFI_ENTRY          0x0001
#define EVT_BOOT_SERVICES       0x0002
#define EVT_MEM_DISCOVERED      0x0003
#define EVT_ALU_VERIFIED        0x0004
#define EVT_REGS_ARMED          0x0005
#define EVT_CPISI_MOUNT         0x0006
#define EVT_CASCADE_DOWN        0x0007
#define EVT_CASCADE_UP          0x0008
#define EVT_SAFE_MODE           0xEEEE

typedef struct {
    UINT32 event_id;
    UINT32 arg1;
    UINT32 arg2;
    UINT32 status;
} __attribute__((packed)) UefiLogEntry;

typedef struct {
    UINT32 count;
    UINT32 reserved;
    UefiLogEntry entries[UEFI_RAM_LOG_MAX_ENTRIES];
} __attribute__((packed)) UefiFlightRecorder;

// Inscribes an atomic 16-byte record into the RAM Log Flight Recorder
static inline void uefi_log_event(
    UINT32 event_id,
    UINT32 arg1,
    UINT32 arg2,
    UINT32 status
) {
    volatile UefiFlightRecorder *rec =
        (volatile UefiFlightRecorder *)UEFI_RAM_LOG_LANDMARK;

    UINT32 count = rec->count;
    if (count < UEFI_RAM_LOG_MAX_ENTRIES) {
        rec->entries[count].event_id = event_id;
        rec->entries[count].arg1 = arg1;
        rec->entries[count].arg2 = arg2;
        rec->entries[count].status = status;
        rec->count = count + 1;
    }
}

// -----------------------------------------------------------------------------
// 64-Bit Silicon Register Snapshot
// -----------------------------------------------------------------------------
typedef struct {
    UINT64 rax;
    UINT64 rbx;
    UINT64 rcx;
    UINT64 rdx;
    UINT64 rsi;
    UINT64 rdi;
    UINT64 rbp;
    UINT64 rsp;
    UINT64 r8;
    UINT64 r9;
    UINT64 r10;
    UINT64 r11;
    UINT64 r12;
    UINT64 r13;
    UINT64 cr0;
    UINT64 cr3;
} CpuRegisters64;

// Captures a snapshot of current CPU state and streams hex values over serial
static inline void uefi_dump_registers(void) {
    CpuRegisters64 regs;

    __asm__ volatile (
        "movq %%rax, %0\n\t"
        "movq %%rbx, %1\n\t"
        "movq %%rcx, %2\n\t"
        "movq %%rdx, %3\n\t"
        "movq %%rsi, %4\n\t"
        "movq %%rdi, %5\n\t"
        "movq %%rbp, %6\n\t"
        "movq %%rsp, %7\n\t"
        "movq %%r8,  %8\n\t"
        "movq %%r9,  %9\n\t"
        "movq %%r10, %10\n\t"
        "movq %%r11, %11\n\t"
        "movq %%r12, %12\n\t"
        "movq %%r13, %13\n\t"
        : "=m"(regs.rax), "=m"(regs.rbx), "=m"(regs.rcx), "=m"(regs.rdx),
          "=m"(regs.rsi), "=m"(regs.rdi), "=m"(regs.rbp), "=m"(regs.rsp),
          "=m"(regs.r8),  "=m"(regs.r9),  "=m"(regs.r10), "=m"(regs.r11),
          "=m"(regs.r12), "=m"(regs.r13)
        :
        : "memory"
    );

    __asm__ volatile (
        "movq %%cr0, %%rax\n\t"
        "movq %%rax, %0\n\t"
        "movq %%cr3, %%rax\n\t"
        "movq %%rax, %1\n\t"
        : "=m"(regs.cr0), "=m"(regs.cr3)
        :
        : "rax", "memory"
    );

    serial_print("[DBG ] REGS: RAX=0x");
    serial_print_hex64(regs.rax);
    serial_print(" RBX=0x");
    serial_print_hex64(regs.rbx);
    serial_print(" RCX=0x");
    serial_print_hex64(regs.rcx);
    serial_print(" RDX=0x");
    serial_print_hex64(regs.rdx);
    serial_print("\n[DBG ] REGS: RSP=0x");
    serial_print_hex64(regs.rsp);
    serial_print(" RBP=0x");
    serial_print_hex64(regs.rbp);
    serial_print(" CR0=0x");
    serial_print_hex64(regs.cr0);
    serial_print(" CR3=0x");
    serial_print_hex64(regs.cr3);
    serial_print("\n");
}

#endif // OMNICODE_UEFI_TELEMETRY_H
