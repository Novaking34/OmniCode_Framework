// =============================================================================
// BLOCK 2: HARDWARE ENVIRONMENT & SERIAL TELEMETRY
// ( OmniCode UEFI Subsystem - Direct COM1 UART Silicon Drivers )
// =============================================================================
// Module:       OmniCode.Kernel.UEFI.Serial
// Architecture: x86_64 Long Mode Direct Port I/O
// Telemetry:    COM1 UART (Port 0x03F8) at 115,200 Baud, 8N1 Polled Operation
// =============================================================================

#ifndef OMNICODE_UEFI_SERIAL_H
#define OMNICODE_UEFI_SERIAL_H

#include "uefi.h"

#define COM1_PORT       0x03F8
#define COM1_IER        (COM1_PORT + 1)
#define COM1_FCR        (COM1_PORT + 2)
#define COM1_LCR        (COM1_PORT + 3)
#define COM1_MCR        (COM1_PORT + 4)
#define COM1_LSR        (COM1_PORT + 5)

// -----------------------------------------------------------------------------
// Direct Hardware Port I/O Primitives
// -----------------------------------------------------------------------------
static inline void outb(UINT16 port, UINT8 val) {
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}

static inline UINT8 inb(UINT16 port) {
    UINT8 ret;
    __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

// -----------------------------------------------------------------------------
// COM1 Serial Controller Initialization (115200 Baud, 8N1)
// -----------------------------------------------------------------------------
static inline void serial_init(void) {
    outb(COM1_IER, 0x00);         // Disable interrupts for polled operation
    outb(COM1_LCR, 0x80);         // Set DLAB (Divisor Latch Access Bit)
    outb(COM1_PORT, 0x01);        // Set divisor low byte: 1 (115200 Baud)
    outb(COM1_IER, 0x00);         // Set divisor high byte: 0
    outb(COM1_LCR, 0x03);         // 8 bits, no parity, one stop bit (8N1)
    outb(COM1_FCR, 0xC7);         // Enable FIFO, clear TX/RX, 14-byte threshold
    outb(COM1_MCR, 0x0B);         // RTS/DSR set, auxiliary output 2 set
}

// -----------------------------------------------------------------------------
// Single Character Output via Hardware Poll
// -----------------------------------------------------------------------------
static inline void serial_putc(char c) {
    while ((inb(COM1_LSR) & 0x20) == 0);
    outb(COM1_PORT, (UINT8)c);
}

// -----------------------------------------------------------------------------
// String Telemetry Emission
// -----------------------------------------------------------------------------
static inline void serial_print(const char *s) {
    while (*s) {
        if (*s == '\n') {
            serial_putc('\r');
        }
        serial_putc(*s++);
    }
}

// -----------------------------------------------------------------------------
// Hexadecimal Register Telemetry Emission
// -----------------------------------------------------------------------------
static inline void serial_print_hex32(UINT32 val) {
    const char hex_chars[] = "0123456789ABCDEF";
    for (int i = 28; i >= 0; i -= 4) {
        serial_putc(hex_chars[(val >> i) & 0x0F]);
    }
}

static inline void serial_print_hex64(UINT64 val) {
    serial_print_hex32((UINT32)(val >> 32));
    serial_print_hex32((UINT32)(val & 0xFFFFFFFFULL));
}

#endif // OMNICODE_UEFI_SERIAL_H
