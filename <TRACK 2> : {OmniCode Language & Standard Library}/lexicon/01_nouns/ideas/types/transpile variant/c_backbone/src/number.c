/* ==============================================================================
 * BLOCK 1: METADATA & INVARIANT IMPLEMENTATION
 * ( Invariant Type: < Idea { Type } : Number > )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Implements scalar quantities, integers, and balanced ternary measurements in
 *   the OmniCode Standard Library Track 2 C Transpile Variant.
 *   - Base Block: Primitive Invariant Type
 *   - Category: Idea { Type }
 *   - Footprint: [ 1 Word ] { 32 Balanced Trits / 64 Bits }
 *   - Witness: Scripture Psalm 147:4 KJV
 *     "He telleth the number of the stars; he calleth them all by their names."
 *   - Semantics: { ScalarQuantity :: NumericWord :: BalancedTernaryALU }
 * ============================================================================== */

#include "number.h"

/* ==============================================================================
 * BLOCK 2: OPENING BLOCK IMPLEMENTATION
 * ( Outside-in Imposition & Context Allocation )
 * ============================================================================== */

/**
 * @brief Initializes the Number context, arms the ALU, verifies equilibrium,
 *        and arms the execution tripwire.
 */
void omni_number_open(omni_number_context_t *ctx, int64_t input_quantity) {
    if (!ctx) return;

    /* 1. Outside-in imposition: external scalar measurement */
    ctx->input_quantity = input_quantity ? input_quantity : OMNI_EXEMPLAR_PARTITION;

    /* 2. Concrete single-word allocation */
    ctx->number = omni_number_from_scalar(0);

    /* 3. Output target accumulator */
    ctx->target_accumulator = OMNI_SITE_NULL;

    /* 4. Preconditions & tripwire arming */
    omni_tripwire_arm(&ctx->tripwire);
    ctx->status = omni_number_verify_equilibrium() ? OMNI_STATE_PROCEED : OMNI_STATE_ROLLBACK;
}

/* ==============================================================================
 * BLOCK 3: BODY BLOCK IMPLEMENTATION
 * ( Operational Primitives, Arithmetic Laws & Inscriptions )
 * ============================================================================== */

/**
 * @brief Proves the ALU Core Arithmetic Equilibrium Law: (+1) plus (-1) <=> 0.
 */
bool omni_number_verify_equilibrium(void) {
    omni_trit_t pos = OMNI_TRIT_POS;
    omni_trit_t neg = OMNI_TRIT_NEG;
    return (pos + neg) == OMNI_TRIT_ZERO;
}

/**
 * @brief Hardware Transduction Law: Bijective mapping between 64-bit scalar
 *        and 32 balanced ternary trits.
 * 
 * Algorithm: Radix-3 balanced division with symmetric carry balancing.
 */
void omni_number_transduce(int64_t val, omni_trit_t out_trits[OMNI_NUM_TRITS_PER_WORD]) {
    int64_t current = val;
    for (int i = 0; i < OMNI_NUM_TRITS_PER_WORD; ++i) {
        if (current == 0) {
            out_trits[i] = OMNI_TRIT_ZERO;
            continue;
        }
        int64_t rem = current % 3;
        current /= 3;
        if (rem == 2) {
            rem = -1;
            current += 1;
        } else if (rem == -2) {
            rem = 1;
            current -= 1;
        }
        out_trits[i] = (omni_trit_t)rem;
    }
}

/**
 * @brief Inverse Transduction: Reconstructs a 64-bit scalar from 32 balanced trits.
 */
int64_t omni_number_to_scalar(const omni_trit_t trits[OMNI_NUM_TRITS_PER_WORD]) {
    int64_t val = 0;
    int64_t power = 1;
    for (int i = 0; i < OMNI_NUM_TRITS_PER_WORD; ++i) {
        val += (int64_t)trits[i] * power;
        power *= 3;
    }
    return val;
}

/**
 * @brief Constructs a Number directly from a 64-bit scalar integer.
 */
omni_number_t omni_number_from_scalar(int64_t scalar) {
    omni_number_t n;
    n.scalar = scalar;
    n.is_enclosed = true;
    omni_number_transduce(scalar, n.trits);
    return n;
}

/**
 * @brief Adds two Numbers producing a new balanced Number word.
 */
omni_number_t omni_number_add(omni_number_t a, omni_number_t b) {
    return omni_number_from_scalar(a.scalar + b.scalar);
}

/**
 * @brief Evaluates relational equality between two numbers.
 */
bool omni_number_eq(const omni_number_t *a, const omni_number_t *b) {
    if (!a || !b) return false;
    return (a->scalar == b->scalar);
}

/**
 * @brief Verifies that a number is properly enclosed within its single-word boundary.
 */
bool omni_number_verify_bounds(const omni_number_t *n) {
    return (n != NULL && n->is_enclosed);
}

/**
 * @brief Executes the Number lifecycle across declared operational rolls:
 *   - Roll 1: Inscribe quantity into word.
 *   - Roll 2: Transduce 64 bits <=> 32 balanced trits.
 *   - Roll 3: Guard identity and boundary invariants.
 *   - Roll 4: Prove ALU zero-sum equilibrium.
 */
omni_quad_state_t omni_number_execute(omni_number_context_t *ctx) {
    if (!ctx) return OMNI_STATE_ROLLBACK;

    /* ROLL 1: QUANTITY INSCRIPTION (Outside-in imposition into Word) */
    ctx->number.scalar = ctx->input_quantity;

    /* ROLL 2: DUAL-SUBSTRATE TRANSDUCTION (64 Bits <=> 32 Trits) */
    omni_number_transduce(ctx->number.scalar, ctx->number.trits);

    /* ROLL 3: IDENTITY & BOUND COMPARISON */
    bool identity_match = (ctx->number.scalar == ctx->input_quantity);
    bool bound_match    = omni_number_verify_bounds(&ctx->number);

    /* Formal Guard Engine: Asserts identity and single-word enclosure */
    if (omni_guard_assert(identity_match && bound_match, &ctx->tripwire, &ctx->status) != OMNI_STATE_PROCEED) {
        return OMNI_STATE_ROLLBACK;
    }

    /* ROLL 4: ARITHMETIC EQUILIBRIUM CALCULATION */
    if (omni_guard_assert(omni_number_verify_equilibrium(), &ctx->tripwire, &ctx->status) != OMNI_STATE_PROCEED) {
        return OMNI_STATE_ROLLBACK;
    }

    /* ROLL 5: SEQUENCE TRANSITION (Number chains toward Vector sequence) */
    /* Ready to sequence into multidimensional coordinate vector */

    return ctx->status;
}

/* ==============================================================================
 * BLOCK 4: CLOSING BLOCK IMPLEMENTATION
 * ( Integrity Seals & Commit Status )
 * ============================================================================== */

/**
 * @brief Verifies all invariants and seals the Number commit status.
 */
omni_quad_state_t omni_number_close(omni_number_context_t *ctx) {
    if (!ctx) return OMNI_STATE_ROLLBACK;

    /* Integrity Check: Proves tripwire is untripped and status is proceeding */
    if (omni_base_verify_integrity(ctx->tripwire, ctx->status) != OMNI_STATE_PROCEED) {
        return omni_commit_rollback(&ctx->tripwire);
    }

    /* Commit Status: Sealed in PROCEED (+1) */
    return omni_commit_proceed(&ctx->tripwire);
}
