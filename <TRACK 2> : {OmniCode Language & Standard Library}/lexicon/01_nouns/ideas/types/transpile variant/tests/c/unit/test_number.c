/* ==============================================================================
 * BLOCK 1: METADATA & TEST TOPOLOGY
 * ( Unit Test: < Lexicon { Types of Ideas } > & [ Test { Number } ] )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Validates the scalar invariant type < Number > in isolation:
 *   - Bijective round-trip radix-3 transduction (64 bits <=> 32 balanced trits)
 *   - Dual-rail bit-plane packed representation and O(1) rail-swap negation
 *   - Instant O(1) NULL collision trap detection
 *   - Dynamic packing topologies: 2-2-2-2, 2-1-2-1-2, 1-2-1-2-2, 2-2-1-2-1,
 *     3-3-2, 3-2-3, 2-3-3
 *   - ALU Zero-Sum Equilibrium Law: (+1) plus (-1) <=> 0
 *   - Balanced arithmetic addition, subtraction, and inverse cancellation
 *   - Strict single-word boundary laws
 *   - Full 4-block lifecycle execution (open, execute, close)
 *   - Witness: Scripture Psalm 147:4 KJV
 *     "He telleth the number of the stars; he calleth them all by their names."
 *   - Semantics: { DualRailBitPlanes :: KinematicTopologies :: ALUZeroSum }
 * ============================================================================== */

#include "test_common.h"

/* ==============================================================================
 * BLOCK 2: OPENING CONTEXT & FIXTURE SETUP
 * ============================================================================== */

static int test_number_bijective_roundtrip(void) {
    const int64_t test_values[] = {
        0, 1, -1, 2, -2, 4096, -4096, 65536, -65536, 1000000, -1000000
    };
    const size_t num_values = sizeof(test_values) / sizeof(test_values[0]);

    for (size_t i = 0; i < num_values; ++i) {
        int64_t original = test_values[i];
        omni_number_t n = omni_number_from_scalar(original);

        OMNI_TEST_ASSERT(n.is_enclosed == true, "Number must be enclosed");
        OMNI_TEST_ASSERT(n.scalar == original, "Number scalar must match original");

        /* Verify all 32 trits are in {-1, 0, +1} */
        for (size_t t = 0; t < OMNI_NUM_TRITS_PER_WORD; ++t) {
            OMNI_TEST_ASSERT(n.trits[t] >= -1 && n.trits[t] <= 1,
                             "Trit must be in {-1, 0, +1}");
        }

        /* Verify reconstruction from trits */
        int64_t reconstructed = omni_number_to_scalar(n.trits);
        OMNI_TEST_ASSERT(reconstructed == original,
                         "Reconstructed scalar must equal original value");
    }
    return 0;
}

static int test_number_dual_rail_bitplanes(void) {
    /* Test Zero: All trits 00 -> packed is 0 */
    omni_number_t zero = omni_number_from_scalar(0);
    OMNI_TEST_ASSERT(omni_number_trits_is_equilibrium(zero.packed_trits) == true,
                     "Packed zero must be in equilibrium (0ULL)");
    OMNI_TEST_ASSERT(omni_number_trits_has_null(zero.packed_trits) == false,
                     "Packed zero must not have NULL traps");

    /* Test Positive Unity (+1): First trit is 01, others 00 */
    omni_number_t pos_one = omni_number_from_scalar(1);
    OMNI_TEST_ASSERT((pos_one.packed_trits & 0x03) == OMNI_TRIT_PAIR_POS,
                     "First trit of +1 must be 01 (positive rail active)");

    /* Test Negative Unity (-1): First trit is 10, others 00 */
    omni_number_t neg_one = omni_number_from_scalar(-1);
    OMNI_TEST_ASSERT((neg_one.packed_trits & 0x03) == OMNI_TRIT_PAIR_NEG,
                     "First trit of -1 must be 10 (negative rail active)");

    /* Test Instant Bit-Plane Negation (+1 <-> -1) */
    uint64_t negated_pos = omni_number_trits_negate(pos_one.packed_trits);
    OMNI_TEST_ASSERT((negated_pos & 0x03) == OMNI_TRIT_PAIR_NEG,
                     "Negating +1 packed trits must produce -1 (10)");
    OMNI_TEST_ASSERT(negated_pos == neg_one.packed_trits,
                     "Negated +1 packed word must equal -1 packed word");

    /* Test NULL Trap Detection: artificially inject 11 into trit 0 */
    uint64_t corrupted = pos_one.packed_trits | 0x03;
    OMNI_TEST_ASSERT(omni_number_trits_has_null(corrupted) == true,
                     "Packed word with 11 must be flagged as NULL trap");

    return 0;
}

static int test_number_dynamic_packing_topologies(void) {
    uint8_t out[5];

    /* 1. Test 2-2-2-2 (Dense: 4 fields of 2 bits) */
    uint8_t p_2222 = omni_number_pack_byte(1, 2, 0, 3, 0, OMNI_PACK_2_2_2_2);
    omni_number_unpack_byte(p_2222, out, OMNI_PACK_2_2_2_2);
    OMNI_TEST_ASSERT(out[0] == 1 && out[1] == 2 && out[2] == 0 && out[3] == 3,
                     "2-2-2-2 round-trip unpack must match input");

    /* 2. Test 2-1-2-1-2 (Palindromic Equilibrium: 00/0/00/0/00) */
    uint8_t p_pal = omni_number_pack_byte(3, 1, 2, 0, 1, OMNI_PACK_2_1_2_1_2);
    omni_number_unpack_byte(p_pal, out, OMNI_PACK_2_1_2_1_2);
    OMNI_TEST_ASSERT(out[0] == 3 && out[1] == 1 && out[2] == 2 &&
                     out[3] == 0 && out[4] == 1,
                     "2-1-2-1-2 round-trip unpack must match input");

    /* 3. Test 1-2-1-2-2 (Prefix Carry) */
    uint8_t p_pre = omni_number_pack_byte(1, 3, 0, 2, 1, OMNI_PACK_1_2_1_2_2);
    omni_number_unpack_byte(p_pre, out, OMNI_PACK_1_2_1_2_2);
    OMNI_TEST_ASSERT(out[0] == 1 && out[1] == 3 && out[2] == 0 &&
                     out[3] == 2 && out[4] == 1,
                     "1-2-1-2-2 round-trip unpack must match input");

    /* 4. Test 2-2-1-2-1 (Postfix Carry) */
    uint8_t p_post = omni_number_pack_byte(2, 1, 1, 3, 0, OMNI_PACK_2_2_1_2_1);
    omni_number_unpack_byte(p_post, out, OMNI_PACK_2_2_1_2_1);
    OMNI_TEST_ASSERT(out[0] == 2 && out[1] == 1 && out[2] == 1 &&
                     out[3] == 3 && out[4] == 0,
                     "2-2-1-2-1 round-trip unpack must match input");

    /* 5. Test 3-3-2 (3-Space Directional Partition) */
    uint8_t p_332 = omni_number_pack_byte(7, 5, 2, 0, 0, OMNI_PACK_3_3_2);
    omni_number_unpack_byte(p_332, out, OMNI_PACK_3_3_2);
    OMNI_TEST_ASSERT(out[0] == 7 && out[1] == 5 && out[2] == 2,
                     "3-3-2 round-trip unpack must match input");

    /* 6. Test 3-2-3 (3-Space Directional Partition) */
    uint8_t p_323 = omni_number_pack_byte(6, 3, 5, 0, 0, OMNI_PACK_3_2_3);
    omni_number_unpack_byte(p_323, out, OMNI_PACK_3_2_3);
    OMNI_TEST_ASSERT(out[0] == 6 && out[1] == 3 && out[2] == 5,
                     "3-2-3 round-trip unpack must match input");

    /* 7. Test 2-3-3 (3-Space Directional Partition) */
    uint8_t p_233 = omni_number_pack_byte(3, 7, 4, 0, 0, OMNI_PACK_2_3_3);
    omni_number_unpack_byte(p_233, out, OMNI_PACK_2_3_3);
    OMNI_TEST_ASSERT(out[0] == 3 && out[1] == 7 && out[2] == 4,
                     "2-3-3 round-trip unpack must match input");

    return 0;
}

static int test_number_equilibrium_law(void) {
    /* ALU Equilibrium Law: (+1) plus (-1) <=> 0 */
    bool eq_holds = omni_number_verify_equilibrium();
    OMNI_TEST_ASSERT(eq_holds == true, "Core ALU Zero-Sum Equilibrium Law must hold");
    return 0;
}

static int test_number_addition_and_subtraction(void) {
    omni_number_t a = omni_number_from_scalar(4000);
    omni_number_t b = omni_number_from_scalar(96);
    omni_number_t sum = omni_number_add(a, b);

    OMNI_TEST_ASSERT(sum.scalar == 4096, "4000 + 96 must equal 4096");
    OMNI_TEST_ASSERT(sum.is_enclosed == true, "Sum must be enclosed");

    /* Test inverse cancellation via subtraction: a - a == 0 */
    omni_number_t zero = omni_number_sub(sum, sum);
    OMNI_TEST_ASSERT(zero.scalar == 0, "4096 - 4096 must equal 0");
    OMNI_TEST_ASSERT(omni_number_trits_is_equilibrium(zero.packed_trits) == true,
                     "Zero difference must be in packed equilibrium");

    /* Test negation: -4096 */
    omni_number_t neg_sum = omni_number_neg(sum);
    OMNI_TEST_ASSERT(neg_sum.scalar == -4096, "Negated 4096 must equal -4096");
    OMNI_TEST_ASSERT(omni_number_add(sum, neg_sum).scalar == 0,
                     "Summing with negated counterpart must yield 0");

    return 0;
}

static int test_number_bounds_and_enclosure(void) {
    omni_number_t valid_num = omni_number_from_scalar(100);
    OMNI_TEST_ASSERT(omni_number_verify_bounds(&valid_num) == true,
                     "Valid number must pass bounds check");

    omni_number_t corrupted = valid_num;
    corrupted.is_enclosed = false;
    OMNI_TEST_ASSERT(omni_number_verify_bounds(&corrupted) == false,
                     "Unenclosed number must fail bounds check");

    /* Test null trap tripping */
    omni_number_t trap_num = valid_num;
    trap_num.packed_trits |= 0x03; /* Inject 11 */
    OMNI_TEST_ASSERT(omni_number_verify_bounds(&trap_num) == false,
                     "Number with NULL collision trap must fail bounds check");

    return 0;
}

static int test_number_lifecycle_rolls(void) {
    omni_number_context_t ctx;
    omni_number_open(&ctx, 4096, OMNI_TARGET_ACCUMULATOR);

    OMNI_TEST_ASSERT(ctx.tripwire == OMNI_TRIPWIRE_ARMED,
                     "Tripwire must be ARMED upon open");
    OMNI_TEST_ASSERT(ctx.input_quantity == 4096,
                     "Input quantity must match 4096");

    /* Verify individual operational rolls */
    omni_number_inscribe(&ctx, 4096);
    OMNI_TEST_ASSERT(ctx.number.scalar == 4096, "Roll 1 Inscribe must set scalar");

    omni_number_transduce_context(&ctx);
    OMNI_TEST_ASSERT(!omni_number_trits_has_null(ctx.number.packed_trits),
                     "Roll 2 Transduce must produce trap-free trits");

    OMNI_TEST_ASSERT(omni_number_guard(&ctx) == OMNI_STATE_PROCEED,
                     "Roll 3 Guard assertion must PROCEED");

    OMNI_TEST_ASSERT(omni_number_calculate(&ctx) == OMNI_STATE_PROCEED,
                     "Roll 4 Calculate zero-sum balance must PROCEED");

    OMNI_TEST_ASSERT(omni_number_sequence(&ctx.number, NULL) == true,
                     "Roll 5 Sequence transition must succeed");

    /* Unified lifecycle execution */
    omni_quad_state_t exec_status = omni_number_execute(&ctx);
    OMNI_TEST_ASSERT(exec_status == OMNI_STATE_PROCEED,
                     "Number execute must PROCEED (+1)");
    OMNI_TEST_ASSERT(ctx.number.is_enclosed == true,
                     "Number must be enclosed after execute");

    omni_quad_state_t close_status = omni_number_close(&ctx);
    OMNI_TEST_ASSERT(close_status == OMNI_STATE_PROCEED,
                     "Number close must PROCEED (+1)");
    OMNI_TEST_ASSERT(ctx.tripwire == OMNI_TRIPWIRE_DISARMED,
                     "Tripwire must be DISARMED upon clean close");

    return 0;
}

/* ==============================================================================
 * BLOCK 3: BODY PRIMITIVES & TEST SUITE RUNNER
 * ============================================================================== */

int test_number_suite(void) {
    printf("[SUITE: Number Invariant Type]\n");
    OMNI_TEST_RUN(test_number_bijective_roundtrip, "Bijective Radix-3 Transduction");
    OMNI_TEST_RUN(test_number_dual_rail_bitplanes, "Dual-Rail Bit-Planes & Negation");
    OMNI_TEST_RUN(test_number_dynamic_packing_topologies, "Dynamic Packing Topologies");
    OMNI_TEST_RUN(test_number_equilibrium_law, "ALU Zero-Sum Equilibrium Law");
    OMNI_TEST_RUN(test_number_addition_and_subtraction, "Balanced Addition & Subtraction");
    OMNI_TEST_RUN(test_number_bounds_and_enclosure, "Single-Word Enclosure Bounds");
    OMNI_TEST_RUN(test_number_lifecycle_rolls, "4-Block Lifecycle Rolls");
    return 0;
}
