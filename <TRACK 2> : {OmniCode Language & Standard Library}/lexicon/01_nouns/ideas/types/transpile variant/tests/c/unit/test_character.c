/* ==============================================================================
 * BLOCK 1: METADATA & TEST TOPOLOGY
 * ( Unit Test: < Lexicon { Types of Ideas } > & [ Test { Character } ] )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Validates the atomic invariant type < Character > in isolation:
 *   - Bijective round-trip transduction (8 bits <=> 5 balanced trits)
 *   - Strict single-cell enclosure boundary laws
 *   - Full 4-block lifecycle execution (open, execute, close)
 *   - Relational equality and comparison
 *   - Witness: Scripture Exodus 31:18 KJV
 *     "And he gave unto Moses, when he had made an end of communing with him upon
 *      mount Sinai, two tables of testimony, tables of stone, written with the
 *      finger of God."
 *   - Semantics: { GlyphFidelity :: BalancedTritLaw :: SingleCellEnclosure }
 * ============================================================================== */

#include "test_common.h"

/* ==============================================================================
 * BLOCK 2: OPENING CONTEXT & FIXTURE SETUP
 * ============================================================================== */

static int test_character_bijective_roundtrip(void) {
    /* Test exemplar characters across 5-trit cell capacity (0..121) */
    const uint8_t test_glyphs[] = {'A', 'Z', '0', '9', ' ', 0, 70, 100, 121};
    const size_t num_glyphs = sizeof(test_glyphs) / sizeof(test_glyphs[0]);

    for (size_t i = 0; i < num_glyphs; ++i) {
        uint8_t original = test_glyphs[i];
        omni_character_t c = omni_character_from_glyph(original);

        /* Verify enclosure */
        OMNI_TEST_ASSERT(c.is_enclosed == true, "Character must be enclosed");
        OMNI_TEST_ASSERT(c.glyph == original, "Character glyph must match original");

        /* Verify every trit is in valid balanced range {-1, 0, +1} */
        for (size_t t = 0; t < OMNI_CHAR_TRITS_PER_CELL; ++t) {
            OMNI_TEST_ASSERT(c.trits[t] >= -1 && c.trits[t] <= 1, "Trit must be in {-1, 0, +1}");
        }

        /* Verify inverse transduction reconstructs the exact byte */
        uint8_t reconstructed = omni_character_to_byte(c.trits);
        OMNI_TEST_ASSERT(reconstructed == original, "Inverse transduction must exactly match original byte");
    }
    return 0;
}

static int test_character_bounds_and_enclosure(void) {
    omni_character_t valid_char = omni_character_from_glyph('X');
    OMNI_TEST_ASSERT(omni_character_verify_bounds(&valid_char) == true, "Valid character must pass boundary check");

    /* Corrupt enclosure flag to test boundary tripwire */
    omni_character_t corrupted = valid_char;
    corrupted.is_enclosed = false;
    OMNI_TEST_ASSERT(omni_character_verify_bounds(&corrupted) == false, "Unenclosed character must fail boundary check");

    return 0;
}

static int test_character_relational_equality(void) {
    omni_character_t a1 = omni_character_from_glyph('M');
    omni_character_t a2 = omni_character_from_glyph('M');
    omni_character_t b  = omni_character_from_glyph('N');

    OMNI_TEST_ASSERT(omni_character_eq(&a1, &a2) == true, "Identical glyphs must evaluate equal");
    OMNI_TEST_ASSERT(omni_character_eq(&a1, &b) == false, "Distinct glyphs must not evaluate equal");

    return 0;
}

static int test_character_lifecycle_rolls(void) {
    omni_character_context_t ctx;
    omni_character_open(&ctx, 'K', OMNI_TARGET_STDOUT);

    OMNI_TEST_ASSERT(ctx.tripwire == OMNI_TRIPWIRE_ARMED, "Context tripwire must be ARMED upon open");
    OMNI_TEST_ASSERT(ctx.input_glyph == 'K', "Input glyph must be recorded in context");

    /* Execute the 3 declared rolls */
    omni_quad_state_t exec_status = omni_character_execute(&ctx);
    OMNI_TEST_ASSERT(exec_status == OMNI_STATE_PROCEED, "Character execute must PROCEED (+1)");
    OMNI_TEST_ASSERT(ctx.character.is_enclosed == true, "Character must be enclosed after execution");
    OMNI_TEST_ASSERT(ctx.character.glyph == 'K', "Character glyph must match input");

    /* Close and commit */
    omni_quad_state_t close_status = omni_character_close(&ctx);
    OMNI_TEST_ASSERT(close_status == OMNI_STATE_PROCEED, "Character close must PROCEED (+1)");
    OMNI_TEST_ASSERT(ctx.tripwire == OMNI_TRIPWIRE_DISARMED, "Tripwire must be DISARMED upon clean close");

    return 0;
}

static int test_character_bit_level_and_ascii_invariant(void) {
    omni_character_t a = omni_character_from_glyph('A'); /* 65 = 0b01000001 */
    OMNI_TEST_ASSERT(omni_character_is_ascii(&a) == true, "'A' must be ASCII");
    OMNI_TEST_ASSERT(omni_character_get_bit(&a, 0) == 1, "Bit 0 of 'A' must be 1");
    OMNI_TEST_ASSERT(omni_character_get_bit(&a, 1) == 0, "Bit 1 of 'A' must be 0");
    OMNI_TEST_ASSERT(omni_character_get_bit(&a, 6) == 1, "Bit 6 of 'A' must be 1");
    OMNI_TEST_ASSERT(omni_character_get_bit(&a, 7) == 0, "Bit 7 of 'A' must be 0 (parity rail)");

    omni_character_t high = omni_character_from_glyph(200);
    OMNI_TEST_ASSERT(omni_character_is_ascii(&high) == false, "200 is not standard ASCII");
    OMNI_TEST_ASSERT(omni_character_get_bit(&high, 7) == 1, "Bit 7 of 200 must be 1");

    return 0;
}

/* ==============================================================================
 * BLOCK 3: BODY PRIMITIVES & TEST SUITE RUNNER
 * ============================================================================== */

int test_character_suite(void) {
    printf("[SUITE: Character Invariant Type]\n");
    OMNI_TEST_RUN(test_character_bijective_roundtrip, "Bijective Round-Trip Transduction");
    OMNI_TEST_RUN(test_character_bounds_and_enclosure, "Single-Cell Enclosure Bounds");
    OMNI_TEST_RUN(test_character_bit_level_and_ascii_invariant, "Bit-Level Slicing & ASCII Invariant");
    OMNI_TEST_RUN(test_character_relational_equality, "Relational Equality");
    OMNI_TEST_RUN(test_character_lifecycle_rolls, "4-Block Lifecycle Rolls");
    return 0;
}
