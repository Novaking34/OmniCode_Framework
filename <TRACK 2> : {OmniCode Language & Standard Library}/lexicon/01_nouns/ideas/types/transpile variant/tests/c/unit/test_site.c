/* ==============================================================================
 * BLOCK 1: METADATA & TEST TOPOLOGY
 * ( Unit Test: < Lexicon { Types of Ideas } > & [ Test { Site } ] )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Validates the physical silicon address type < Site > in isolation:
 *   - Null pointer address rejection and safety tripwire
 *   - Hardware Routing Law (MMIO ports vs RAM spaces)
 *   - Relative displacement and pointer offset arithmetic
 *   - Volatile memory semantics for bare-metal registers
 *   - Full 4-block lifecycle execution (open, execute, close)
 *   - Witness: Scripture Genesis 28:16 KJV
 *     "And Jacob awaked out of his sleep, and he said, Surely the Lord is in
 *      this place; and I knew it not."
 *   - Semantics: { SiliconCoordinates :: HardwareRoutingLaw :: NonNullInterlock }
 * ============================================================================== */

#include "test_common.h"

/* ==============================================================================
 * BLOCK 2: OPENING CONTEXT & FIXTURE SETUP
 * ============================================================================== */

static int test_site_routing_classification(void) {
    /* VGA buffer coordinate (0xB8000) must route to MMIO */
    omni_site_t vga_site = omni_site_from_address(OMNI_SITE_VGA_BUFFER);
    OMNI_TEST_ASSERT(vga_site.routing == OMNI_ROUTING_HARDWARE_MMIO, "VGA buffer must classify as HARDWARE_MMIO");
    OMNI_TEST_ASSERT(omni_site_is_valid(&vga_site) == true, "VGA site must be valid");

    /* COM1 serial port (0x3F8) must route to MMIO */
    omni_site_t com1_site = omni_site_from_address(OMNI_SITE_COM1);
    OMNI_TEST_ASSERT(com1_site.routing == OMNI_ROUTING_HARDWARE_MMIO, "COM1 port must classify as HARDWARE_MMIO");

    /* Standard MBR memory location (0x7C00) must route to RAM */
    omni_site_t mbr_site = omni_site_from_address(OMNI_SITE_MBR_BOOT);
    OMNI_TEST_ASSERT(mbr_site.routing == OMNI_ROUTING_RAM_PHYSICAL, "MBR origin must classify as RAM_PHYSICAL");

    return 0;
}

static int test_site_null_tripwire(void) {
    /* Null address (0x0) must be invalid */
    omni_site_t null_site = omni_site_from_address(OMNI_SITE_NULL);
    OMNI_TEST_ASSERT(omni_site_is_valid(&null_site) == false, "Null site must be invalid");

    /* Executing lifecycle with NULL coordinate must trip the interlock and ROLLBACK */
    omni_site_context_t ctx;
    omni_site_open(&ctx, 0);
    ctx.input_coordinate = 0; /* Force null */
    omni_quad_state_t exec_st = omni_site_execute(&ctx);
    OMNI_TEST_ASSERT(exec_st == OMNI_STATE_ROLLBACK, "Null address execute must trigger ROLLBACK (-1)");
    OMNI_TEST_ASSERT(ctx.tripwire == OMNI_TRIPWIRE_TRIPPED, "Tripwire must be TRIPPED on null address");

    return 0;
}

static int test_site_offset_and_relational(void) {
    uint8_t dummy_buffer[16];
    uintptr_t base_addr = (uintptr_t)&dummy_buffer[0];

    omni_site_t base_site = omni_site_from_address(base_addr);
    omni_site_t offset_site = omni_site_offset(&base_site, 8);

    OMNI_TEST_ASSERT(offset_site.coordinate == base_addr + 8, "Displaced site coordinate must be base + 8");
    OMNI_TEST_ASSERT(omni_site_eq(&base_site, &base_site) == true, "Identical sites must evaluate equal");
    OMNI_TEST_ASSERT(omni_site_eq(&base_site, &offset_site) == false, "Different sites must evaluate not equal");

    return 0;
}

static int test_site_volatile_read_write(void) {
    uint8_t buffer[4] = {0, 0, 0, 0};
    uintptr_t addr = (uintptr_t)&buffer[0];
    omni_site_t site = omni_site_from_address(addr);

    /* Write via volatile primitives */
    omni_site_write_u8(&site, 0, 0xAB);
    omni_site_write_u8(&site, 1, 0xCD);

    /* Read back via volatile primitives */
    uint8_t r0 = omni_site_read_u8(&site, 0);
    uint8_t r1 = omni_site_read_u8(&site, 1);

    OMNI_TEST_ASSERT(r0 == 0xAB, "Volatile read at offset 0 must match written 0xAB");
    OMNI_TEST_ASSERT(r1 == 0xCD, "Volatile read at offset 1 must match written 0xCD");

    return 0;
}

static int test_site_lifecycle_rolls(void) {
    omni_site_context_t ctx;
    omni_site_open(&ctx, OMNI_SITE_VGA_BUFFER);

    OMNI_TEST_ASSERT(ctx.tripwire == OMNI_TRIPWIRE_ARMED, "Tripwire must be ARMED upon open");

    omni_quad_state_t exec_status = omni_site_execute(&ctx);
    OMNI_TEST_ASSERT(exec_status == OMNI_STATE_PROCEED, "Site execute must PROCEED (+1)");
    OMNI_TEST_ASSERT(ctx.site.routing == OMNI_ROUTING_HARDWARE_MMIO, "VGA site routing must be HARDWARE_MMIO");

    omni_quad_state_t close_status = omni_site_close(&ctx);
    OMNI_TEST_ASSERT(close_status == OMNI_STATE_PROCEED, "Site close must PROCEED (+1)");
    OMNI_TEST_ASSERT(ctx.tripwire == OMNI_TRIPWIRE_DISARMED, "Tripwire must be DISARMED upon clean close");

    return 0;
}

/* ==============================================================================
 * BLOCK 3: BODY PRIMITIVES & TEST SUITE RUNNER
 * ============================================================================== */

int test_site_suite(void) {
    printf("[SUITE: Site Invariant Type]\n");
    OMNI_TEST_RUN(test_site_routing_classification, "Hardware Routing Law (MMIO vs RAM)");
    OMNI_TEST_RUN(test_site_null_tripwire, "Null Pointer Rejection & Tripwire Interlock");
    OMNI_TEST_RUN(test_site_offset_and_relational, "Offset Arithmetic & Relational Equality");
    OMNI_TEST_RUN(test_site_volatile_read_write, "Volatile Memory Read / Write Semantics");
    OMNI_TEST_RUN(test_site_lifecycle_rolls, "4-Block Lifecycle Rolls");
    return 0;
}
