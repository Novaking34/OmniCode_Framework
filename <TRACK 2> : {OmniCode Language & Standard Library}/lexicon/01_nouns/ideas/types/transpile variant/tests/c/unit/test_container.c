/* ==============================================================================
 * BLOCK 1: METADATA & TEST TOPOLOGY
 * ( Unit Test: < Lexicon { Types of Ideas } > & [ Test { Container } ] )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Validates the generic memory capsule type < Container > in isolation:
 *   - Bounded slice allocation and element encapsulation
 *   - Law of Containment verification (Occupancy <= Capacity)
 *   - Overflow detection and tripwire interlock triggering
 *   - Standard 64-byte vessel envelope
 *   - Full 4-block lifecycle execution (open, execute, close)
 *   - Witness: Scripture 2 Kings 4:4 KJV
 *     "And when thou art come in, thou shalt shut the door upon thee and upon
 *      thy sons, and shalt pour out into all those vessels, and thou shalt set
 *      aside that which is full."
 *   - Semantics: { MemoryCapsule :: BoundedSlice :: ContainmentLaw }
 * ============================================================================== */

#include "test_common.h"

/* ==============================================================================
 * BLOCK 2: OPENING CONTEXT & FIXTURE SETUP
 * ============================================================================== */

static int test_container_basic_lifecycle(void) {
    uint32_t storage[8];
    omni_container_t c;
    omni_container_init(&c, storage, sizeof(uint32_t), 8);

    OMNI_TEST_ASSERT(c.capacity == 8, "Container capacity must be 8");
    OMNI_TEST_ASSERT(c.occupancy == 0, "Container occupancy must initially be 0");
    OMNI_TEST_ASSERT(c.is_enclosed == true, "Container must be enclosed");
    OMNI_TEST_ASSERT(omni_container_is_empty(&c) == true, "Container must report empty");

    /* Push 4 elements */
    for (uint32_t i = 1; i <= 4; ++i) {
        omni_quad_state_t push_st = omni_container_push(&c, &i);
        OMNI_TEST_ASSERT(push_st == OMNI_STATE_PROCEED, "Push must PROCEED");
    }
    OMNI_TEST_ASSERT(c.occupancy == 4, "Occupancy must be 4");

    /* Verify random access get */
    for (size_t i = 0; i < 4; ++i) {
        uint32_t *val = (uint32_t *)omni_container_get(&c, i);
        OMNI_TEST_ASSERT(val != NULL, "Get must return non-null pointer");
        OMNI_TEST_ASSERT(*val == (uint32_t)(i + 1), "Stored element must match pushed value");
    }

    /* Out of bounds get must return NULL */
    OMNI_TEST_ASSERT(omni_container_get(&c, 10) == NULL, "Out of bounds get must return NULL");

    /* Test random access set */
    uint32_t new_val = 999;
    omni_quad_state_t set_st = omni_container_set(&c, 1, &new_val);
    OMNI_TEST_ASSERT(set_st == OMNI_STATE_PROCEED, "Set must PROCEED");
    uint32_t *read_back = (uint32_t *)omni_container_get(&c, 1);
    OMNI_TEST_ASSERT(read_back != NULL && *read_back == 999, "Read back value at index 1 must be 999");

    /* Clear */
    omni_container_clear(&c);
    OMNI_TEST_ASSERT(c.occupancy == 0, "Occupancy must be 0 after clear");
    OMNI_TEST_ASSERT(omni_container_is_empty(&c) == true, "Container must report empty after clear");

    return 0;
}

static int test_container_overflow_tripwire(void) {
    uint8_t tiny_storage[2];
    omni_container_t c;
    omni_container_init(&c, tiny_storage, sizeof(uint8_t), 2);

    uint8_t a = 10, b = 20, c_overflow = 30;
    OMNI_TEST_ASSERT(omni_container_push(&c, &a) == OMNI_STATE_PROCEED, "First push must PROCEED");
    OMNI_TEST_ASSERT(omni_container_push(&c, &b) == OMNI_STATE_PROCEED, "Second push must PROCEED");
    OMNI_TEST_ASSERT(omni_container_is_full(&c) == true, "Container must report full at capacity 2");

    /* Third push MUST fail and return ROLLBACK (-1) */
    omni_quad_state_t overflow_res = omni_container_push(&c, &c_overflow);
    OMNI_TEST_ASSERT(overflow_res == OMNI_STATE_ROLLBACK, "Overflow push must return ROLLBACK (-1)");
    OMNI_TEST_ASSERT(c.occupancy == 2, "Occupancy must remain capped at capacity");

    return 0;
}

static int test_container_containment_law(void) {
    uint8_t storage[4];
    omni_container_t c;
    omni_container_init(&c, storage, sizeof(uint8_t), 4);

    OMNI_TEST_ASSERT(omni_container_verify_containment(&c) == true, "Valid container must pass Containment Law");

    /* Violate containment law manually */
    omni_container_t corrupted = c;
    corrupted.occupancy = 10; /* Exceeds capacity 4 */
    OMNI_TEST_ASSERT(omni_container_verify_containment(&corrupted) == false, "Overflowed occupancy must violate Containment Law");

    return 0;
}

static int test_byte_vessel_envelope(void) {
    omni_byte_vessel_t vessel;
    omni_byte_vessel_init(&vessel);

    OMNI_TEST_ASSERT(vessel.container.capacity == OMNI_CONTAINER_EXEMPLAR_VESSEL, "Byte vessel capacity must be 64");
    OMNI_TEST_ASSERT(vessel.container.element_size == 1, "Element size must be 1 byte");
    OMNI_TEST_ASSERT(vessel.container.occupancy == 0, "Byte vessel must initially be empty");

    return 0;
}

static int test_container_lifecycle_rolls(void) {
    uint16_t storage[16];
    uint16_t payload[4] = {100, 200, 300, 400};

    omni_container_context_t ctx;
    omni_container_open(&ctx, storage, sizeof(uint16_t), 16, payload, 4);

    OMNI_TEST_ASSERT(ctx.tripwire == OMNI_TRIPWIRE_ARMED, "Tripwire must be ARMED upon open");
    OMNI_TEST_ASSERT(ctx.payload_count == 4, "Payload count must be 4");

    omni_quad_state_t exec_status = omni_container_execute(&ctx);
    OMNI_TEST_ASSERT(exec_status == OMNI_STATE_PROCEED, "Container execute must PROCEED (+1)");
    OMNI_TEST_ASSERT(ctx.vessel.occupancy == 4, "Vessel occupancy must match payload count");

    omni_quad_state_t close_status = omni_container_close(&ctx);
    OMNI_TEST_ASSERT(close_status == OMNI_STATE_PROCEED, "Container close must PROCEED (+1)");
    OMNI_TEST_ASSERT(ctx.tripwire == OMNI_TRIPWIRE_DISARMED, "Tripwire must be DISARMED upon clean close");

    return 0;
}

/* ==============================================================================
 * BLOCK 3: BODY PRIMITIVES & TEST SUITE RUNNER
 * ============================================================================== */

int test_container_suite(void) {
    printf("[SUITE: Container Invariant Type]\n");
    OMNI_TEST_RUN(test_container_basic_lifecycle, "Basic Allocation, Push & Pop");
    OMNI_TEST_RUN(test_container_overflow_tripwire, "Overflow Tripwire & Rollback");
    OMNI_TEST_RUN(test_container_containment_law, "Core Containment Law");
    OMNI_TEST_RUN(test_byte_vessel_envelope, "64-Byte Vessel Envelope");
    OMNI_TEST_RUN(test_container_lifecycle_rolls, "4-Block Lifecycle Rolls");
    return 0;
}
