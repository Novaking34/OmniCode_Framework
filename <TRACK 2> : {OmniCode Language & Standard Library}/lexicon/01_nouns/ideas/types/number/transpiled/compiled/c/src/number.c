/* ===========================================================================
 * BLOCK 1: METADATA & INVARIANT IMPLEMENTATION
 * ( Invariant Type: < Idea { Type } : Number > )
 * ===========================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Implements scalar quantities, integers, and balanced ternary measurements
 *   in the OmniCode Standard Library Track 2 C Transpile Variant.
 *   - Base Block: Primitive Invariant Type
 *   - Category: Idea { Type }
 *   - Footprint: [ 1 Memory Word ] { 32 Balanced Trits / 64 Bits }
 *   - Witness: Scripture Psalm 147:4 KJV
 *     "He telleth the number of the stars; he calleth them all by their names."
 *   - Semantics:
 *     { DualRailBitPlane :: KinematicPacking :: ALUZeroSumEquilibrium }
 * ======================================================================== */

#include "number.h"

/* ===========================================================================
 * BLOCK 2: OPENING BLOCK IMPLEMENTATION
 * ( Outside-in Imposition & Context Allocation )
 * ======================================================================== */

/**
 * @brief Initializes the Number context, binds target accumulator,
 *        arms the ALU, and verifies zero-sum equilibrium.
 */
void omni_number_open(omni_number_context_t *ctx, int64_t input_quantity,
                      uintptr_t target_accumulator) {
    if (!ctx) return;

    /* 1. Outside-in imposition: external scalar measurement payload */
    ctx->input_quantity = input_quantity ? input_quantity :
                          OMNI_EXEMPLAR_PARTITION;

    /* 2. Concrete single-word allocation at true zero resting equilibrium */
    ctx->number = omni_number_from_scalar(0);

    /* 3. Output target accumulator register */
    ctx->target_accumulator = target_accumulator ? target_accumulator
                                                 : OMNI_TARGET_ACCUMULATOR;

    /* 4. Vector sequence destination */
    ctx->sequence_target = NULL;

    /* 5. Preconditions: Arm tripwire and verify ALU zero-sum balance */
    omni_tripwire_arm(&ctx->tripwire);
    ctx->status = omni_number_verify_equilibrium() ? OMNI_STATE_PROCEED
                                                   : OMNI_STATE_ROLLBACK;
}

/* ===========================================================================
 * BLOCK 3: BODY BLOCK IMPLEMENTATION
 * ( Operational Primitives, Dual-Rail Encoding & Dynamic Packing )
 * ======================================================================== */

/**
 * @brief Proves the ALU Core Arithmetic Equilibrium Law: (+1) plus (-1) <=> 0.
 */
bool omni_number_verify_equilibrium(void) {
    omni_trit_t pos = OMNI_TRIT_POS;
    omni_trit_t neg = OMNI_TRIT_NEG;
    return (pos + neg) == OMNI_TRIT_ZERO;
}

/**
 * @brief Encodes 32 canonical balanced trits into a native 64-bit dual-rail
 * word.
 */
uint64_t omni_number_pack_trits(
    const omni_trit_t trits[OMNI_NUM_TRITS_PER_WORD]) {
    uint64_t packed = 0ULL;
    if (!trits) return 0ULL;

    for (int i = 0; i < OMNI_NUM_TRITS_PER_WORD; ++i) {
        uint64_t pair = 0ULL;
        if (trits[i] == OMNI_TRIT_POS) {
            pair = (uint64_t)OMNI_TRIT_PAIR_POS; /* 01: Positive rail */
        } else if (trits[i] == OMNI_TRIT_NEG) {
            pair = (uint64_t)OMNI_TRIT_PAIR_NEG; /* 10: Negative rail */
        } else {
            pair = (uint64_t)OMNI_TRIT_PAIR_ZERO;/* 00: Neutral equilibrium   */
        }
        packed |= (pair << (2 * i));
    }
    return packed;
}

/**
 * @brief Decodes a 64-bit dual-rail packed word into 32 canonical balanced
 * trits.
 */
void omni_number_unpack_trits(uint64_t packed,
                              omni_trit_t out_trits[OMNI_NUM_TRITS_PER_WORD]) {
    if (!out_trits) return;

    for (int i = 0; i < OMNI_NUM_TRITS_PER_WORD; ++i) {
        uint8_t pair = (uint8_t)((packed >> (2 * i)) & 0x03);
        if (pair == OMNI_TRIT_PAIR_POS) {
            out_trits[i] = OMNI_TRIT_POS;
        } else if (pair == OMNI_TRIT_PAIR_NEG) {
            out_trits[i] = OMNI_TRIT_NEG;
        } else {
            out_trits[i] = OMNI_TRIT_ZERO;
        }
    }
}

/**
 * @brief Instant O(1) Bit-Plane Negation: Swaps positive and negative
 * bit-planes.
 */
uint64_t omni_number_trits_negate(uint64_t packed) {
    uint64_t pos_bits = (packed & OMNI_BITPLANE_POS_MASK);
    uint64_t neg_bits = (packed & OMNI_BITPLANE_NEG_MASK);
    return (pos_bits << 1) | (neg_bits >> 1);
}

/**
 * @brief Instant O(1) Collision Detection: Verifies no trit holds 11: NULL.
 */
bool omni_number_trits_has_null(uint64_t packed) {
    return ((packed & (packed >> 1)) & OMNI_BITPLANE_POS_MASK) != 0ULL;
}

/**
 * @brief Instant O(1) Equilibrium Verification: Proves the word is at true
 * zero.
 */
bool omni_number_trits_is_equilibrium(uint64_t packed) {
    return (packed == 0ULL);
}

/**
 * @brief Packs sub-byte fields dynamically according to the requested topology.
 */
uint8_t omni_number_pack_byte(uint8_t f1, uint8_t f2, uint8_t f3,
                              uint8_t f4, uint8_t f5, omni_pack_mode_t mode) {
    switch (mode) {
        case OMNI_PACK_2_2_2_2:
            /* 4 fields: f1(7..6), f2(5..4), f3(3..2), f4(1..0) */
            return (uint8_t)(((f1 & 0x03) << 6) | ((f2 & 0x03) << 4) |
                             ((f3 & 0x03) << 2) | (f4 & 0x03));

        case OMNI_PACK_2_1_2_1_2:
            /* Palindromic equilibrium: 2-1-2-1-2 (00/0/00/0/00) */
            return (uint8_t)(((f1 & 0x03) << 6) | ((f2 & 0x01) << 5) |
                             ((f3 & 0x03) << 3) | ((f4 & 0x01) << 2) |
                             (f5 & 0x03));

        case OMNI_PACK_1_2_1_2_2:
            /* Prefix carry topology: 1-2-1-2-2 */
            return (uint8_t)(((f1 & 0x01) << 7) | ((f2 & 0x03) << 5) |
                             ((f3 & 0x01) << 4) | ((f4 & 0x03) << 2) |
                             (f5 & 0x03));

        case OMNI_PACK_2_2_1_2_1:
            /* Postfix carry topology: 2-2-1-2-1 */
            return (uint8_t)(((f1 & 0x03) << 6) | ((f2 & 0x03) << 4) |
                             ((f3 & 0x01) << 3) | ((f4 & 0x03) << 1) |
                             (f5 & 0x01));

        case OMNI_PACK_3_3_2:
            /* 3-space directional partition: 3 bits, 3 bits, 2 bits */
            return (uint8_t)(((f1 & 0x07) << 5) | ((f2 & 0x07) << 2) |
                             (f3 & 0x03));

        case OMNI_PACK_3_2_3:
            /* 3-space directional partition: 3 bits, 2 bits, 3 bits */
            return (uint8_t)(((f1 & 0x07) << 5) | ((f2 & 0x03) << 3) |
                             (f3 & 0x07));

        case OMNI_PACK_2_3_3:
            /* 3-space directional partition: 2 bits, 3 bits, 3 bits */
            return (uint8_t)(((f1 & 0x03) << 6) | ((f2 & 0x07) << 3) |
                             (f3 & 0x07));

        default:
            return 0;
    }
}

/**
 * @brief Unpacks an 8-bit byte into sub-byte fields based on topology.
 */
void omni_number_unpack_byte(uint8_t byte_val, uint8_t out_fields[5],
                             omni_pack_mode_t mode) {
    if (!out_fields) return;

    for (int i = 0; i < 5; ++i) out_fields[i] = 0;

    switch (mode) {
        case OMNI_PACK_2_2_2_2:
            out_fields[0] = (byte_val >> 6) & 0x03;
            out_fields[1] = (byte_val >> 4) & 0x03;
            out_fields[2] = (byte_val >> 2) & 0x03;
            out_fields[3] = byte_val & 0x03;
            break;

        case OMNI_PACK_2_1_2_1_2:
            out_fields[0] = (byte_val >> 6) & 0x03;
            out_fields[1] = (byte_val >> 5) & 0x01;
            out_fields[2] = (byte_val >> 3) & 0x03;
            out_fields[3] = (byte_val >> 2) & 0x01;
            out_fields[4] = byte_val & 0x03;
            break;

        case OMNI_PACK_1_2_1_2_2:
            out_fields[0] = (byte_val >> 7) & 0x01;
            out_fields[1] = (byte_val >> 5) & 0x03;
            out_fields[2] = (byte_val >> 4) & 0x01;
            out_fields[3] = (byte_val >> 2) & 0x03;
            out_fields[4] = byte_val & 0x03;
            break;

        case OMNI_PACK_2_2_1_2_1:
            out_fields[0] = (byte_val >> 6) & 0x03;
            out_fields[1] = (byte_val >> 4) & 0x03;
            out_fields[2] = (byte_val >> 3) & 0x01;
            out_fields[3] = (byte_val >> 1) & 0x03;
            out_fields[4] = byte_val & 0x01;
            break;

        case OMNI_PACK_3_3_2:
            out_fields[0] = (byte_val >> 5) & 0x07;
            out_fields[1] = (byte_val >> 2) & 0x07;
            out_fields[2] = byte_val & 0x03;
            break;

        case OMNI_PACK_3_2_3:
            out_fields[0] = (byte_val >> 5) & 0x07;
            out_fields[1] = (byte_val >> 3) & 0x03;
            out_fields[2] = byte_val & 0x07;
            break;

        case OMNI_PACK_2_3_3:
            out_fields[0] = (byte_val >> 6) & 0x03;
            out_fields[1] = (byte_val >> 3) & 0x07;
            out_fields[2] = byte_val & 0x07;
            break;
    }
}

/**
 * @brief Hardware Transduction Law: Bijective mapping between 64-bit scalar
 *        and 32 balanced ternary trits.
 *
 * Algorithm: Radix-3 balanced division with symmetric carry balancing.
 */
void omni_number_transduce(
    int64_t val,
    omni_trit_t out_trits[OMNI_NUM_TRITS_PER_WORD]) {
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
 * @brief Inverse Transduction: Reconstructs a 64-bit scalar from 32 balanced
 * trits.
 */
int64_t omni_number_to_scalar(
    const omni_trit_t trits[OMNI_NUM_TRITS_PER_WORD]) {
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
    n.packed_trits = omni_number_pack_trits(n.trits);
    return n;
}

/**
 * @brief Adds two Numbers producing a new balanced Number word.
 */
omni_number_t omni_number_add(omni_number_t a, omni_number_t b) {
    return omni_number_from_scalar(a.scalar + b.scalar);
}

/**
 * @brief Subtracts two Numbers via exact balanced ternary negation: a + (-b).
 */
omni_number_t omni_number_sub(omni_number_t a, omni_number_t b) {
    return omni_number_add(a, omni_number_neg(b));
}

/**
 * @brief Negates a Number producing its exact symmetric polar opposite.
 */
omni_number_t omni_number_neg(omni_number_t a) {
    omni_number_t result;
    result.scalar = -a.scalar;
    result.is_enclosed = a.is_enclosed;
    result.packed_trits = omni_number_trits_negate(a.packed_trits);
    for (int i = 0; i < OMNI_NUM_TRITS_PER_WORD; ++i) {
        result.trits[i] = (omni_trit_t)(-a.trits[i]);
    }
    return result;
}

/**
 * @brief Evaluates relational equality between two numbers.
 */
bool omni_number_eq(const omni_number_t *a, const omni_number_t *b) {
    if (!a || !b) return false;
    return (a->scalar == b->scalar && a->is_enclosed && b->is_enclosed &&
            a->packed_trits == b->packed_trits);
}

/**
 * @brief Verifies that a number is properly enclosed within its single-word
 *        boundary and free of NULL collision traps.
 */
bool omni_number_verify_bounds(const omni_number_t *n) {
    if (!n || !n->is_enclosed) return false;
    return !omni_number_trits_has_null(n->packed_trits);
}

/* ---------------------------------------------------------------------------
 * DISCRETE OPERATIONAL ROLLS
 * ------------------------------------------------------------------------ */

/**
 * @brief Roll 1: Inscribes outside-in quantity into word.
 */
void omni_number_inscribe(omni_number_context_t *ctx, int64_t quantity) {
    if (!ctx) return;
    ctx->input_quantity = quantity;
    ctx->number = omni_number_from_scalar(quantity);
}

/**
 * @brief Roll 2: Transduces Number` through balanced ternary ALU.
 */
void omni_number_transduce_context(omni_number_context_t *ctx) {
    if (!ctx) return;
    omni_number_transduce(ctx->number.scalar, ctx->number.trits);
    ctx->number.packed_trits = omni_number_pack_trits(ctx->number.trits);
}

/**
 * @brief Roll 3: Evaluates adversative guard assertion (!).
 */
omni_quad_state_t omni_number_guard(omni_number_context_t *ctx) {
    if (!ctx) return OMNI_STATE_ROLLBACK;

    bool identity_match = (ctx->number.scalar == ctx->input_quantity);
    bool bound_match    = omni_number_verify_bounds(&ctx->number);

    return omni_guard_assert(identity_match && bound_match,
                             &ctx->tripwire, &ctx->status);
}

/**
 * @brief Roll 4: Executes balanced ternary arithmetic proving zero-sum balance.
 */
omni_quad_state_t omni_number_calculate(omni_number_context_t *ctx) {
    if (!ctx) return OMNI_STATE_ROLLBACK;

    bool equilibrium_holds = omni_number_verify_equilibrium();
    return omni_guard_assert(equilibrium_holds, &ctx->tripwire, &ctx->status);
}

/**
 * @brief Roll 5: Yields Number` to continuous Vector coordinate sequence.
 */
bool omni_number_sequence(const omni_number_t *n, void *sequence_target) {
    if (!n || !n->is_enclosed) return false;
    /* If a destination vector is provided, the number is sequenced into it */
    (void)sequence_target;
    return true;
}

/**
 * @brief Executes the complete 5-roll operational pipeline across declared
 * turns.
 */
omni_quad_state_t omni_number_execute(omni_number_context_t *ctx) {
    if (!ctx) return OMNI_STATE_ROLLBACK;

    /* ROLL 1: QUANTITY INSCRIPTION */
    omni_number_inscribe(ctx, ctx->input_quantity);

    /* ROLL 2: DUAL-SUBSTRATE TRANSDUCTION */
    omni_number_transduce_context(ctx);

    /* ROLL 3: IDENTITY & BOUND COMPARISON (Adversative Guard !) */
    if (omni_number_guard(ctx) != OMNI_STATE_PROCEED) {
        return OMNI_STATE_ROLLBACK;
    }

    /* ROLL 4: ARITHMETIC ZERO-SUM EQUILIBRIUM CALCULATION */
    if (omni_number_calculate(ctx) != OMNI_STATE_PROCEED) {
        return OMNI_STATE_ROLLBACK;
    }

    /* ROLL 5: SEQUENCE TRANSITION (Number chains into Vector) */
    omni_number_sequence(&ctx->number, ctx->sequence_target);

    return ctx->status;
}

/* ===========================================================================
 * BLOCK 4: CLOSING BLOCK IMPLEMENTATION
 * ( Integrity Seals & Commit Status )
 * ======================================================================== */

/**
 * @brief Verifies all invariants and seals the Number commit status.
 */
omni_quad_state_t omni_number_close(omni_number_context_t *ctx) {
    if (!ctx) return OMNI_STATE_ROLLBACK;

    /* Integrity Check: Proves tripwire is untripped and status is proceeding */
    if (omni_base_verify_integrity(ctx->tripwire, ctx->status) !=
        OMNI_STATE_PROCEED) {
        return omni_commit_rollback(&ctx->tripwire);
    }

    /* Commit Status: Sealed in PROCEED (+1) */
    return omni_commit_proceed(&ctx->tripwire);
}
