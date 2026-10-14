/* ==============================================================================
 * BLOCK 1: METADATA & TEST TOPOLOGY
 * ( Unit Test: < Lexicon { Types of Ideas } > & [ Test { Number } ] )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Validates the scalar invariant type < Number > in isolation:
 *   - Bijective round-trip radix-3 transduction (64 bits <=> 32 balanced trits)
 *   - ALU Zero-Sum Equilibrium Law: (+1) plus (-1) <=> 0
 *   - Balanced arithmetic addition
 *   - Strict single-word boundary laws
 *   - Full 4-block lifecycle execution (open, execute, close)
 *   - Witness: Scripture Psalm 147:4 KJV
 *     "He telleth the number of the stars; he calleth them all by their names."
 *   - Semantics: { ScalarFidelity :: Radix3Equilibrium :: ZeroSumBalance }
 * ============================================================================== */

#include "test_common.h"

/* ==============================================================================
 * BLOCK 2: OPENING CONTEXT & FIXTURE SETUP
 * ============================================================================== */

static int test_number_bijective_roundtrip(void) {
    const int64_t test_values[] = {0, 1, -1, 2, -2, 4096, -4096, 65536, -65536, 1000000, -1000000};
    const size_t num_values = sizeof(test_values) / sizeof(test_values[0]);

    for (size_t i = 0; i < num_values; ++i) {
        int64_t original = test_values[i];
        omni_number_t n = omni_number_from_scalar(original);

        OMNI_TEST_ASSERT(n.is_enclosed == true, "Number must be enclosed");
        OMNI_TEST_ASSERT(n.scalar == original, "Number scalar must match original");

        /* Verify all 32 trits are in {-1, 0, +1} */
        for (size_t t = 0; t < OMNI_NUM_TRITS_PER_WORD; ++t) {
            OMNI_TEST_ASSERT(n.trits[t] >= -1 && n.trits[t] <= 1, "Trit must be in {-1, 0, +1}");
        }

        /* Verify reconstruction from trits */
        int64_t reconstructed = omni_number_to_scalar(n.trits);
        OMNI_TEST_ASSERT(reconstructed == original, "Reconstructed scalar must equal original value");
    }
    return 0;
}

static int test_number_equilibrium_law(void) {
    /* ALU Equilibrium Law: (+1) plus (-1) <=> 0 */
    bool eq_holds = omni_number_verify_equilibrium();
    OMNI_TEST_ASSERT(eq_holds == true, "Core ALU Zero-Sum Equilibrium Law must hold");
    return 0;
}

static int test_number_addition(void) {
    omni_number_t a = omni_number_from_scalar(4000);
    omni_number_t b = omni_number_from_scalar(96);
    omni_number_t sum = omni_number_add(a, b);

    OMNI_TEST_ASSERT(sum.scalar == 4096, "4000 + 96 must equal 4096");
    OMNI_TEST_ASSERT(sum.is_enclosed == true, "Sum must be enclosed");

    /* Test inverse cancellation */
    omni_number_t neg_sum = omni_number_from_scalar(-4096);
    omni_number_t zero = omni_number_add(sum, neg_sum);
    OMNI_TEST_ASSERT(zero.scalar == 0, "4096 + (-4096) must equal 0");

    return 0;
}

static int test_number_bounds_and_enclosure(void) {
    omni_number_t valid_num = omni_number_from_scalar(100);
    OMNI_TEST_ASSERT(omni_number_verify_bounds(&valid_num) == true, "Valid number must pass bounds check");

    omni_number_t corrupted = valid_num;
    corrupted.is_enclosed = false;
    OMNI_TEST_ASSERT(omni_number_verify_bounds(&corrupted) == false, "Unenclosed number must fail bounds check");

    return 0;
}

static int test_number_lifecycle_rolls(void) {
    omni_number_context_t ctx;
    omni_number_open(&ctx, 4096);

    OMNI_TEST_ASSERT(ctx.tripwire == OMNI_TRIPWIRE_ARMED, "Tripwire must be ARMED upon open");
    OMNI_TEST_ASSERT(ctx.input_quantity == 4096, "Input quantity must match 4096");

    omni_quad_state_t exec_status = omni_number_execute(&ctx);
    OMNI_TEST_ASSERT(exec_status == OMNI_STATE_PROCEED, "Number execute must PROCEED (+1)");
    OMNI_TEST_ASSERT(ctx.number.is_enclosed == true, "Number must be enclosed after execute");

    omni_quad_state_t close_status = omni_number_close(&ctx);
    OMNI_TEST_ASSERT(close_status == OMNI_STATE_PROCEED, "Number close must PROCEED (+1)");
    OMNI_TEST_ASSERT(ctx.tripwire == OMNI_TRIPWIRE_DISARMED, "Tripwire must be DISARMED upon clean close");

    return 0;
}

/* ==============================================================================
 * BLOCK 3: BODY PRIMITIVES & TEST SUITE RUNNER
 * ============================================================================== */

int test_number_suite(void) {
    printf("[SUITE: Number Invariant Type]\n");
    OMNI_TEST_RUN(test_number_bijective_roundtrip, "Bijective Radix-3 Transduction");
    OMNI_TEST_RUN(test_number_equilibrium_law, "ALU Zero-Sum Equilibrium Law");
    OMNI_TEST_RUN(test_number_addition, "Balanced Addition & Cancellation");
    OMNI_TEST_RUN(test_number_bounds_and_enclosure, "Single-Word Enclosure Bounds");
    OMNI_TEST_RUN(test_number_lifecycle_rolls, "4-Block Lifecycle Rolls");
    return 0;
}
