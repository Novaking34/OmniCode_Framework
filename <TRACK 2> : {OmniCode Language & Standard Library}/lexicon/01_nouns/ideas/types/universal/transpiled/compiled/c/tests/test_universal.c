/* ===========================================================================
 * BLOCK 1: METADATA & TEST TOPOLOGY
 * ( Unit Test: < Lexicon { Types of Ideas } > & [ Test { Universal Base } ] )
 * ===========================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Validates the universal base axioms, balanced ternary ALU, tripwires, and
 *   Quad-State connectives in isolation BEFORE being consumed downstream.
 *   - Witness: Scripture Proverbs 24:27 KJV
 *     "Prepare thy work without, and make it fit for thyself in the field;
 *      and afterwards build thine house."
 *   - Semantics: { BaseAxioms :: BalancedTernaryALU :: ConnectiveGuards }
 * ======================================================================== */

#include "test_common.h"

int g_omni_test_failures = 0;

static int test_base_preconditions(void) {
    omni_quad_state_t s = omni_base_verify_preconditions();
    OMNI_TEST_ASSERT(s == OMNI_STATE_PROCEED,
                     "Preconditions must PROCEED on 64-bit platform");
    return 0;
}

static int test_balanced_ternary_arithmetic(void) {
    /* Negation */
    OMNI_TEST_ASSERT(omni_trit_negate(OMNI_TRIT_POS) == OMNI_TRIT_NEG,
                     "Negate(+1) == -1");
    OMNI_TEST_ASSERT(omni_trit_negate(OMNI_TRIT_NEG) == OMNI_TRIT_POS,
                     "Negate(-1) == +1");
    OMNI_TEST_ASSERT(omni_trit_negate(OMNI_TRIT_ZERO) == OMNI_TRIT_ZERO,
                     "Negate(0) == 0");

    /* Multiplication */
    OMNI_TEST_ASSERT(omni_trit_multiply(OMNI_TRIT_POS, OMNI_TRIT_NEG) ==
                     OMNI_TRIT_NEG, "(+1) * (-1) == -1");
    OMNI_TEST_ASSERT(omni_trit_multiply(OMNI_TRIT_NEG, OMNI_TRIT_NEG) ==
                     OMNI_TRIT_POS, "(-1) * (-1) == +1");

    /* Addition with Carry */
    omni_trit_t cout = 0;
    omni_trit_t sum = omni_trit_add(OMNI_TRIT_POS, OMNI_TRIT_POS, 0, &cout);
    OMNI_TEST_ASSERT(sum == OMNI_TRIT_NEG && cout == OMNI_TRIT_POS,
                     "(+1) + (+1) == -1 with carry +1 (Radix-3: 1*3 - 1 = 2)");

    sum = omni_trit_add(OMNI_TRIT_POS, OMNI_TRIT_NEG, 0, &cout);
    OMNI_TEST_ASSERT(sum == OMNI_TRIT_ZERO && cout == OMNI_TRIT_ZERO,
                     "(+1) + (-1) == 0 with carry 0 (Equilibrium)");

    return 0;
}

static int test_quad_state_connectives(void) {
    /* Connective: BUT (!) - Adversative Caveat */
    omni_quad_state_t act = OMNI_STATE_PROCEED;
    omni_quad_state_t cav_ok = OMNI_STATE_PROCEED;
    omni_quad_state_t cav_fail = OMNI_STATE_ROLLBACK;

    OMNI_TEST_ASSERT(omni_quad_state_but(act, cav_ok) == OMNI_STATE_PROCEED,
                     "PROCEED ! PROCEED must yield PROCEED");
    OMNI_TEST_ASSERT(omni_quad_state_but(act, cav_fail) == OMNI_STATE_ROLLBACK,
                     "PROCEED ! ROLLBACK must yield ROLLBACK");

    /* Connective: AND (::) - Conjunction */
    OMNI_TEST_ASSERT(omni_quad_state_and(OMNI_STATE_PROCEED, OMNI_STATE_PROCEED)
                     == OMNI_STATE_PROCEED, "PROCEED :: PROCEED == PROCEED");
    OMNI_TEST_ASSERT(omni_quad_state_and(OMNI_STATE_PROCEED, OMNI_STATE_AWAIT)
                     == OMNI_STATE_AWAIT, "PROCEED :: AWAIT == AWAIT");
    OMNI_TEST_ASSERT(
        omni_quad_state_and(OMNI_STATE_PROCEED, OMNI_STATE_ROLLBACK)
                     == OMNI_STATE_ROLLBACK, "PROCEED :: ROLLBACK == ROLLBACK");

    /* Connective: OR (|) - Disjunctive Alternative Valve */
    OMNI_TEST_ASSERT(omni_quad_state_or(OMNI_STATE_PROCEED, OMNI_STATE_ROLLBACK)
                     == OMNI_STATE_PROCEED, "PROCEED | ROLLBACK == PROCEED");
    OMNI_TEST_ASSERT(omni_quad_state_or(OMNI_STATE_ROLLBACK, OMNI_STATE_PROCEED)
                     == OMNI_STATE_PROCEED, "ROLLBACK | PROCEED == PROCEED");
    OMNI_TEST_ASSERT(
        omni_quad_state_or(OMNI_STATE_ROLLBACK, OMNI_STATE_ROLLBACK)
                     == OMNI_STATE_ROLLBACK, "ROLLBACK | ROLLBACK == ROLLBACK");

    /* Connective: NOT (--) - Truth Inversion */
    OMNI_TEST_ASSERT(omni_quad_state_not(OMNI_STATE_PROCEED) ==
                     OMNI_STATE_ROLLBACK, "-- PROCEED == ROLLBACK");
    OMNI_TEST_ASSERT(omni_quad_state_not(OMNI_STATE_ROLLBACK) ==
                     OMNI_STATE_PROCEED, "-- ROLLBACK == PROCEED");
    OMNI_TEST_ASSERT(omni_quad_state_not(OMNI_STATE_AWAIT) ==
                     OMNI_STATE_AWAIT, "-- AWAIT == AWAIT");

    /* Connective: NEQ (!=) - Inequality Gate */
    OMNI_TEST_ASSERT(omni_quad_state_neq(OMNI_STATE_PROCEED,
                                         OMNI_STATE_ROLLBACK) ==
                     OMNI_STATE_PROCEED, "PROCEED != ROLLBACK == PROCEED");
    OMNI_TEST_ASSERT(omni_quad_state_neq(OMNI_STATE_PROCEED,
                                         OMNI_STATE_PROCEED) ==
                     OMNI_STATE_ROLLBACK, "PROCEED != PROCEED == ROLLBACK");

    /* Connective: IS_NOT (=!) - Predicate Filter */
    OMNI_TEST_ASSERT(omni_quad_state_is_not(OMNI_STATE_AWAIT,
                                            OMNI_STATE_ROLLBACK) ==
                     OMNI_STATE_PROCEED, "AWAIT =! ROLLBACK == PROCEED");
    OMNI_TEST_ASSERT(omni_quad_state_is_not(OMNI_STATE_ROLLBACK,
                                            OMNI_STATE_ROLLBACK) ==
                     OMNI_STATE_ROLLBACK, "ROLLBACK =! ROLLBACK == ROLLBACK");

    /* Connective: LIKEN (=~) - Compatibility Match */
    OMNI_TEST_ASSERT(omni_quad_state_liken(OMNI_STATE_PROCEED,
                                           OMNI_STATE_AWAIT) ==
                     OMNI_STATE_PROCEED, "PROCEED =~ AWAIT == PROCEED");
    OMNI_TEST_ASSERT(omni_quad_state_liken(OMNI_STATE_PROCEED,
                                           OMNI_STATE_ROLLBACK) ==
                     OMNI_STATE_ROLLBACK, "PROCEED =~ ROLLBACK == ROLLBACK");

    /* Connective: RELATE (:) - Perspective Projection */
    OMNI_TEST_ASSERT(omni_quad_state_relate(OMNI_STATE_PROCEED,
                                            OMNI_STATE_AWAIT) ==
                     OMNI_STATE_AWAIT, "PROCEED : AWAIT == AWAIT");
    OMNI_TEST_ASSERT(omni_quad_state_relate(OMNI_STATE_ROLLBACK,
                                            OMNI_STATE_PROCEED) ==
                     OMNI_STATE_ROLLBACK, "ROLLBACK : PROCEED == ROLLBACK");

    /* State Validity */
    OMNI_TEST_ASSERT(omni_state_is_valid(OMNI_STATE_PROCEED), "PROCEED valid");
    OMNI_TEST_ASSERT(omni_state_is_valid(OMNI_STATE_AWAIT), "AWAIT valid");
    OMNI_TEST_ASSERT(omni_state_is_valid(OMNI_STATE_ROLLBACK),
                     "ROLLBACK valid");
    OMNI_TEST_ASSERT(omni_state_is_valid(OMNI_STATE_HALT), "HALT valid");
    OMNI_TEST_ASSERT(!omni_state_is_valid((omni_quad_state_t)42), "42 invalid");

    return 0;
}

static int test_dual_rail_substrate(void) {
    /* 1. 2-Bit Pair Encoding & Decoding */
    OMNI_TEST_ASSERT(omni_trit_to_pair(OMNI_TRIT_ZERO) == OMNI_TRIT_PAIR_ZERO,
                     "0 trit -> pair 0b00");
    OMNI_TEST_ASSERT(omni_trit_to_pair(OMNI_TRIT_POS) == OMNI_TRIT_PAIR_POS,
                     "+1 trit -> pair 0b01");
    OMNI_TEST_ASSERT(omni_trit_to_pair(OMNI_TRIT_NEG) == OMNI_TRIT_PAIR_NEG,
                     "-1 trit -> pair 0b10");

    omni_trit_t t = 0;
    bool ok = omni_pair_to_trit(OMNI_TRIT_PAIR_ZERO, &t);
    OMNI_TEST_ASSERT(ok && t == OMNI_TRIT_ZERO, "pair 0b00 -> 0 trit");

    ok = omni_pair_to_trit(OMNI_TRIT_PAIR_POS, &t);
    OMNI_TEST_ASSERT(ok && t == OMNI_TRIT_POS, "pair 0b01 -> +1 trit");

    ok = omni_pair_to_trit(OMNI_TRIT_PAIR_NEG, &t);
    OMNI_TEST_ASSERT(ok && t == OMNI_TRIT_NEG, "pair 0b10 -> -1 trit");

    ok = omni_pair_to_trit(OMNI_TRIT_PAIR_NULL, &t);
    OMNI_TEST_ASSERT(!ok, "pair 0b11 -> NULL collision fault");

    /* 2. Dual-Rail SIMD Negation */
    uint64_t val = 0x0000000000000001ULL; /* 1 trit POS in bit 0 */
    uint64_t neg = omni_dual_rail_negate(val);
    OMNI_TEST_ASSERT(neg == 0x0000000000000002ULL, "Negation swaps P->N rail");
    uint64_t neg_neg = omni_dual_rail_negate(neg);
    OMNI_TEST_ASSERT(neg_neg == val, "Double negation restores identity");

    /* 3. Collision Trap */
    OMNI_TEST_ASSERT(!omni_dual_rail_has_collision(val),
                     "val has no collision");
    uint64_t trap = 0x0000000000000003ULL; /* bits 0 & 1 set -> collision */
    OMNI_TEST_ASSERT(omni_dual_rail_has_collision(trap),
                     "trap reports collision");

    return 0;
}

static int test_tripwire_lifecycle(void) {
    omni_tripwire_state_t tw = OMNI_TRIPWIRE_DISARMED;
    omni_tripwire_arm(&tw);
    OMNI_TEST_ASSERT(tw == OMNI_TRIPWIRE_ARMED, "Tripwire must be ARMED");

    omni_tripwire_disarm(&tw);
    OMNI_TEST_ASSERT(tw == OMNI_TRIPWIRE_DISARMED, "Tripwire must be DISARMED");

    omni_tripwire_arm(&tw);
    omni_tripwire_trip(&tw);
    OMNI_TEST_ASSERT(tw == OMNI_TRIPWIRE_TRIPPED, "Tripwire must be TRIPPED");

    omni_quad_state_t integrity = omni_base_verify_integrity(tw,
        OMNI_STATE_PROCEED);
    OMNI_TEST_ASSERT(integrity == OMNI_STATE_ROLLBACK,
                     "Integrity must fail if tripwire was TRIPPED");

    return 0;
}

int main(void) {
    printf("===============================================================\n");
    printf("   OMNICODE C UNIVERSAL BASE UNIT TESTBED\n");
    printf("===============================================================\n");
    OMNI_TEST_RUN(test_base_preconditions,
                  "Hardware Preconditions Verification");
    OMNI_TEST_RUN(test_balanced_ternary_arithmetic,
                  "Balanced Ternary ALU Math");
    OMNI_TEST_RUN(test_dual_rail_substrate, "Dual-Rail Polar Substrate & SIMD");
    OMNI_TEST_RUN(test_quad_state_connectives, "Quad-State Connectives");
    OMNI_TEST_RUN(test_tripwire_lifecycle, "Tripwire Lifecycle & Integrity");

    if (g_omni_test_failures > 0) {
        fprintf(stderr, "[!] Universal base test suite failed with %d errors\n",
                g_omni_test_failures);
        return 1;
    }
    printf("---------------------------------------------------------------\n");
    printf("[√] All Universal Base Axioms verified and sealed!\n");
    return 0;
}
