// =============================================================================
// BLOCK 1: METADATA & TYPOLOGY BLOCK
// ( OmniCode UEFI Subsystem - Core Specification & Protocol Bindings )
// =============================================================================
// Module:       OmniCode.Kernel.UEFI.Core
// Architecture: x86_64 Native Long Mode (PE32+ EFI Application)
// Standard:     Unified Extensible Firmware Interface (UEFI) Specification 2.10
// Invariant:    Freestanding C runtime with zero libc dependencies.
// =============================================================================

#ifndef OMNICODE_UEFI_CORE_H
#define OMNICODE_UEFI_CORE_H

// -----------------------------------------------------------------------------
// Fundamental Data Types (64-Bit System Architecture)
// -----------------------------------------------------------------------------
typedef unsigned long long UINT64;
typedef long long          INT64;
typedef unsigned int       UINT32;
typedef int                INT32;
typedef unsigned short     UINT16;
typedef short              INT16;
typedef unsigned char      UINT8;
typedef char               INT8;
typedef unsigned long long UINTN;
typedef long long          INTN;
typedef void              *EFI_HANDLE;
typedef UINTN              EFI_STATUS;
typedef UINT64             EFI_PHYSICAL_ADDRESS;
typedef UINT64             EFI_VIRTUAL_ADDRESS;

// -----------------------------------------------------------------------------
// Standard UEFI Status Codes
// -----------------------------------------------------------------------------
#define EFI_SUCCESS               0x0000000000000000ULL
#define EFI_LOAD_ERROR            0x8000000000000001ULL
#define EFI_INVALID_PARAMETER     0x8000000000000002ULL
#define EFI_UNSUPPORTED           0x8000000000000003ULL
#define EFI_BAD_BUFFER_SIZE       0x8000000000000004ULL
#define EFI_BUFFER_TOO_SMALL      0x8000000000000005ULL
#define EFI_NOT_READY             0x8000000000000006ULL
#define EFI_DEVICE_ERROR          0x8000000000000007ULL
#define EFI_OUT_OF_RESOURCES      0x8000000000000009ULL

// -----------------------------------------------------------------------------
// EFI Table Header
// -----------------------------------------------------------------------------
typedef struct {
    UINT64 Signature;
    UINT32 Revision;
    UINT32 HeaderSize;
    UINT32 CRC32;
    UINT32 Reserved;
} EFI_TABLE_HEADER;

// -----------------------------------------------------------------------------
// Simple Text Output Protocol
// -----------------------------------------------------------------------------
struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;

typedef EFI_STATUS (*EFI_TEXT_RESET)(
    struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *This,
    unsigned char ExtendedVerification
);

typedef EFI_STATUS (*EFI_TEXT_STRING)(
    struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *This,
    const short *String
);

typedef EFI_STATUS (*EFI_TEXT_CLEAR_SCREEN)(
    struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *This
);

typedef EFI_STATUS (*EFI_TEXT_SET_ATTRIBUTE)(
    struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *This,
    UINTN Attribute
);

typedef struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    EFI_TEXT_RESET         Reset;
    EFI_TEXT_STRING        OutputString;
    void                  *TestString;
    void                  *QueryMode;
    void                  *SetMode;
    EFI_TEXT_SET_ATTRIBUTE SetAttribute;
    EFI_TEXT_CLEAR_SCREEN  ClearScreen;
} EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;

// -----------------------------------------------------------------------------
// Memory Allocation & Descriptor Typology
// -----------------------------------------------------------------------------
typedef enum {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType
} EFI_ALLOCATE_TYPE;

typedef enum {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiMaxMemoryType
} EFI_MEMORY_TYPE;

typedef struct {
    UINT32               Type;
    EFI_PHYSICAL_ADDRESS PhysicalStart;
    EFI_VIRTUAL_ADDRESS  VirtualStart;
    UINT64               NumberOfPages;
    UINT64               Attribute;
} EFI_MEMORY_DESCRIPTOR;

// -----------------------------------------------------------------------------
// Boot Services Definition
// -----------------------------------------------------------------------------
typedef struct {
    EFI_TABLE_HEADER Hdr;

    // Task Priority Services
    void *RaiseTPL;
    void *RestoreTPL;

    // Memory Services
    EFI_STATUS (*AllocatePages)(
        EFI_ALLOCATE_TYPE Type,
        EFI_MEMORY_TYPE MemoryType,
        UINTN Pages,
        EFI_PHYSICAL_ADDRESS *Memory
    );
    EFI_STATUS (*FreePages)(EFI_PHYSICAL_ADDRESS Memory, UINTN Pages);
    EFI_STATUS (*GetMemoryMap)(
        UINTN *MemoryMapSize,
        EFI_MEMORY_DESCRIPTOR *MemoryMap,
        UINTN *MapKey,
        UINTN *DescriptorSize,
        UINT32 *DescriptorVersion
    );
    EFI_STATUS (*AllocatePool)(
        EFI_MEMORY_TYPE PoolType,
        UINTN Size,
        void **Buffer
    );
    EFI_STATUS (*FreePool)(void *Buffer);

    // Event & Timer Services
    void *CreateEvent;
    void *SetTimer;
    void *WaitForEvent;
    void *SignalEvent;
    void *CloseEvent;
    void *CheckEvent;

    // Protocol Handler Services
    void *InstallProtocolInterface;
    void *ReinstallProtocolInterface;
    void *UninstallProtocolInterface;
    void *HandleProtocol;
    void *Pcr;
    void *RegisterProtocolNotify;
    void *LocateHandle;
    void *LocateDevicePath;
    void *InstallConfigurationTable;

    // Image Services
    void *LoadImage;
    void *StartImage;
    void *Exit;
    void *UnloadImage;
    EFI_STATUS (*ExitBootServices)(EFI_HANDLE ImageHandle, UINTN MapKey);

    // Miscellaneous Services
    void *GetNextMonotonicCount;
    void *Stall;
    void *SetWatchdogTimer;
} EFI_BOOT_SERVICES;

// -----------------------------------------------------------------------------
// Master EFI System Table
// -----------------------------------------------------------------------------
typedef struct {
    EFI_TABLE_HEADER                 Hdr;
    short                           *FirmwareVendor;
    UINT32                           FirmwareRevision;
    EFI_HANDLE                       ConsoleInHandle;
    void                            *ConIn;
    EFI_HANDLE                       ConsoleOutHandle;
    EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *ConOut;
    EFI_HANDLE                       StandardErrorHandle;
    void                            *StdErr;
    void                            *RuntimeServices;
    EFI_BOOT_SERVICES               *BootServices;
    UINTN                            NumberOfTableEntries;
    void                            *ConfigurationTable;
} EFI_SYSTEM_TABLE;

#endif // OMNICODE_UEFI_CORE_H
