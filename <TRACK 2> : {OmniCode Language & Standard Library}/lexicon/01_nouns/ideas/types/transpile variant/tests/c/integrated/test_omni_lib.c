/* ==============================================================================
 * BLOCK 1: METADATA & INTEGRATION TEST TOPOLOGY
 * ( Test Orchestrator: < Lexicon { Types of Ideas } > & [ Multi-File Testbed ] )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Primary test orchestrator for the OmniCode Standard Library C Backbone.
 *   Executes multi-file unit test suites followed by integrated cross-type pipelines:
 *   1. Unit Suite: Character (Exodus 31:18 KJV)
 *   2. Unit Suite: Number (Psalm 147:4 KJV)
 *   3. Unit Suite: Container (2 Kings 4:4 KJV)
 *   4. Unit Suite: Site (Genesis 28:16 KJV)
 *   5. Unit Suite: State (Ecclesiastes 3:1 KJV)
 *   6. Integrated Composition: Container<Character> Traversed via State Loop
 *   - Witness: Scripture Proverbs 24:27 KJV
 *     "Prepare thy work without, and make it fit for thyself in the field;
 *      and afterwards build thine house."
 *   - Semantics: { MultiFileTestbed :: CrossTypeIntegration :: DetentVerification }
 * ============================================================================== */

#include "test_common.h"
#include <stdio.h>

/* Global test failure accumulator */
int g_omni_test_failures = 0;

/* ==============================================================================
 * BLOCK 2: OPENING CONTEXT & INTEGRATED PIPELINE DEFINITIONS
 * ============================================================================== */

static int run_cross_type_integration_pipeline(void) {
    printf("[SUITE: Cross-Type Composition Pipeline]\n");
    printf("  -> Composing Container<Character> (\"The Text Substrate\") ...\n");

    #define TEST_TEXT_CAPACITY 32
    omni_character_t text_storage[TEST_TEXT_CAPACITY];
    omni_container_t text_ribbon;
    omni_container_init(&text_ribbon, text_storage, sizeof(omni_character_t), TEST_TEXT_CAPACITY);

    const char *source_str = "OMNICODE";
    for (size_t i = 0; source_str[i] != '\0'; ++i) {
        omni_character_t ch = omni_character_from_glyph((uint8_t)source_str[i]);
        omni_quad_state_t push_res = omni_container_push(&text_ribbon, &ch);
        OMNI_TEST_ASSERT(push_res == OMNI_STATE_PROCEED, "Container push must succeed without overflow");
    }

    OMNI_TEST_ASSERT(text_ribbon.occupancy == 8, "Text ribbon occupancy must be 8");
    printf("  -> Capacity: %zu, Occupancy: %zu [PASS]\n", text_ribbon.capacity, text_ribbon.occupancy);

    /* Traversal via Canonical State Loop */
    printf("  -> Driving Traversal via Canonical State Loop:\n     Transduced Output: \"");
    omni_state_t traversal_state = omni_state_create(OMNI_STATE_PROCEED, (int64_t)text_ribbon.occupancy);

    while (omni_state_is_proceeding(&traversal_state)) {
        size_t idx = (size_t)traversal_state.registers.it;
        omni_character_t *curr = (omni_character_t *)omni_container_get(&text_ribbon, idx);
        if (curr) {
            putchar((char)curr->glyph);
        }
        omni_state_step_loop(&traversal_state.registers);
    }
    printf("\"\n");

    OMNI_TEST_ASSERT(omni_state_is_halted(&traversal_state) == true, "Traversal must reach HALT:NULL detent");
    OMNI_TEST_ASSERT(traversal_state.registers.it == 8, "Final IT must equal 8");
    OMNI_TEST_ASSERT(traversal_state.registers.turn == 8, "Final TURN must equal 8");
    printf("     Final Detent: HALT:NULL (IT=8, TURN=8) [PASS]\n");

    /* Route Site to VGA Buffer MMIO */
    omni_site_t vga_site = omni_site_from_address(OMNI_SITE_VGA_BUFFER);
    OMNI_TEST_ASSERT(vga_site.routing == OMNI_ROUTING_HARDWARE_MMIO, "VGA site must resolve to HARDWARE_MMIO");
    printf("  -> Virtual Hardware MMIO Routing: 0x%lX -> HARDWARE_MMIO (::@) [PASS]\n",
           (unsigned long)vga_site.coordinate);

    return 0;
}

/* ==============================================================================
 * BLOCK 3: BODY PRIMITIVES & MAIN TEST RUNNER
 * ============================================================================== */

int main(void) {
    printf("==============================================================================\n");
    printf("   OMNICODE STANDARD LIBRARY C BACKBONE: MULTI-FILE TEST HARNESS\n");
    printf("==============================================================================\n\n");

    /* Run Unit Test Suites */
    if (test_character_suite() != 0) goto failure;
    printf("\n");

    if (test_number_suite() != 0) goto failure;
    printf("\n");

    if (test_container_suite() != 0) goto failure;
    printf("\n");

    if (test_site_suite() != 0) goto failure;
    printf("\n");

    if (test_state_suite() != 0) goto failure;
    printf("\n");

    /* Run Cross-Type Integration Pipeline */
    if (run_cross_type_integration_pipeline() != 0) goto failure;
    printf("\n");

/* ==============================================================================
 * BLOCK 4: CLOSING BLOCK & FINAL DETENT REPORT
 * ============================================================================== */

    printf("==============================================================================\n");
    printf("   [ALL C BACKBONE TESTS PASS]: FULL 4-BLOCK PARITY SEALED IN PROCEED (+1)\n");
    printf("==============================================================================\n");
    return 0;

failure:
    printf("\n==============================================================================\n");
    printf("   [TEST SUITE FAILURE]: %d assertion failure(s) recorded!\n", g_omni_test_failures);
    printf("==============================================================================\n");
    return 1;
}
