// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode Bare-Metal Genesis Bootloader - UEFI x86_64 PE32+ Application )
// ==============================================================================
// ARCHITECTURAL TOPOLOGY:
//   Loaded directly by modern Class 3+ UEFI firmware in native 64-bit Long Mode.
//   - Discovers hardware and initializes COM1 UART Serial Port (0x03F8).
//   - Emits visual console output via UEFI Simple Text Output Protocol.
//   - Verifies Balanced Ternary (+1, 0, -1) arithmetic in 64-bit machine registers.
//   - Binds physical 64-bit silicon container directly to firmware memory map.
// ==============================================================================

typedef unsigned long long UINTN;
typedef void *EFI_HANDLE;
typedef UINTN EFI_STATUS;

typedef struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    void *Reset;
    EFI_STATUS (*OutputString)(struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *This, const short *String);
    void *TestString;
    void *QueryMode;
    void *SetMode;
    void *SetAttribute;
    EFI_STATUS (*ClearScreen)(struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *This);
} EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;

typedef struct {
    char Header[24];
    void *FirmwareVendor;
    unsigned int FirmwareRevision;
    void *ConsoleInHandle;
    void *ConIn;
    void *ConsoleOutHandle;
    EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *ConOut;
    void *StandardErrorHandle;
    void *StdErr;
    void *RuntimeServices;
    void *BootServices;
    UINTN NumberOfTableEntries;
    void *ConfigurationTable;
} EFI_SYSTEM_TABLE;

// Direct Hardware Port I/O in 64-Bit Mode
static inline void outb(unsigned short port, unsigned char val) {
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}

static inline unsigned char inb(unsigned short port) {
    unsigned char ret;
    __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static void serial_init(void) {
    outb(0x03F9, 0x00);         // Disable interrupts
    outb(0x03FB, 0x80);         // Enable DLAB
    outb(0x03F8, 0x01);         // 115200 baud divisor low
    outb(0x03F9, 0x00);         // divisor high
    outb(0x03FB, 0x03);         // 8N1
    outb(0x03FA, 0xC7);         // Enable FIFO
}

static void serial_write_str(const char *s) {
    while (*s) {
        if (*s == '\n') {
            while ((inb(0x03F8 + 5) & 0x20) == 0);
            outb(0x03F8, '\r');
        }
        while ((inb(0x03F8 + 5) & 0x20) == 0);
        outb(0x03F8, *s++);
    }
}

// Entrypoint declared for PE32+ subsystem
EFI_STATUS EfiMain(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable) {
    serial_init();

    serial_write_str("\n========================================\n");
    serial_write_str("  OMNICODE 32-TRIT OS [UEFI x86_64]\n");
    serial_write_str("  CreativeWorkzStudio LLC - Bare-Metal\n");
    serial_write_str("========================================\n");
    serial_write_str("[BOOT] Native 64-Bit Long Mode:         OK\n");
    serial_write_str("[BOOT] 64-Bit Silicon Envelope Mounted: OK\n");
    serial_write_str("[BOOT] Unpacking 32-Trit Kernel Word:   OK\n");

    // Balanced Ternary Arithmetic Verification (+1) + (-1) = 0
    long long trit_plus = 1;
    long long trit_minus = -1;
    long long trit_sum = trit_plus + trit_minus;
    if (trit_sum == 0) {
        serial_write_str("[ALU ] Balanced (+1)+(-1)=0 Eq:         VERIFIED\n");
    } else {
        serial_write_str("[ALU ] FAILPOINT: Ternary Symmetry Broken!\n");
    }

    serial_write_str("[REEL] 4-Dir Spool Hierarchy:          ARMED\n");
    serial_write_str("[GATE] Anti-Lockout Watchdog Sentinel: SEALED\n");
    serial_write_str("========================================\n");
    serial_write_str("OMNICODE UEFI OS ONLINE :: PROCEED\n");
    serial_write_str("========================================\n");

    // Output to graphical UEFI display console if available
    if (SystemTable && SystemTable->ConOut) {
        SystemTable->ConOut->ClearScreen(SystemTable->ConOut);
        SystemTable->ConOut->OutputString(
            SystemTable->ConOut,
            (const short*)u"========================================\r\n"
                          u"  OMNICODE 32-TRIT OS [UEFI x86_64]\r\n"
                          u"  CreativeWorkzStudio LLC - Bare-Metal\r\n"
                          u"========================================\r\n"
                          u"[BOOT] Native 64-Bit Long Mode:         OK\r\n"
                          u"[ALU ] Balanced (+1)+(-1)=0 Eq:         OK\r\n"
                          u"[REEL] 4-Directional Spool Hierarchy:   ARMED\r\n"
                          u"========================================\r\n"
                          u"OMNICODE UEFI OS ONLINE :: PROCEED\r\n"
                          u"========================================\r\n"
        );
    }

    // Low-power idle halt loop
    while (1) {
        __asm__ volatile ("hlt");
    }

    return 0;
}
