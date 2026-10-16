/* ===========================================================================
 * BLOCK 1: METADATA & INVARIANT AXIOMS BLOCK
 * ( Invariant Root: < Lexicon { Types of Ideas } > & [ Noun { Base Axioms } ] )
 * ===========================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Foundational bare-metal substrates, balanced ternary types, and Quad-State
 *   truth logic governing the OmniCode Standard Library Track 2 C Transpile.
 *   - Environment: Freestanding C99/C11 (Zero libc/OS dependency)
 *   - Word Size: 64-Bit Silicon Scalar / 32 Balanced Trits
 *   - Cell Size: 8-Bit Byte / 5 Balanced Trits
 *   - System: OmniCode { Lexicon }
 *   - Witness: Scripture Proverbs 11:1 KJV
 *     "A false balance is abomination to the LORD: but a just weight is his
 *      delight."
 *   - Semantics: { BalancedTernary :: QuadStateTruth :: TripwireGuards }
 * ======================================================================== */

#ifndef OMNI_BASE_H
#define OMNI_BASE_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ---------------------------------------------------------------------------
 * ARCHITECTURAL CONSTANTS & SUBSTRATE CAPACITIES
 * ------------------------------------------------------------------------ */

/** Number of balanced trits packed into an 8-bit memory Cell (3^5 = 243). */
#define OMNI_TRITS_PER_CELL         5

/** Number of balanced trits packed into a 64-bit machine Word. */
#define OMNI_TRITS_PER_WORD         32

/** Footprint of the standard bounded Container vessel exemplar (in bytes). */
#define OMNI_VESSEL_DEFAULT_CELLS   64

/* ---------------------------------------------------------------------------
 * 1. BALANCED TERNARY SUBSTRATE: omni_trit_t
 * ------------------------------------------------------------------------ */

/**
 * @brief Discrete balanced ternary digit (-1, 0, +1).
 *
 * Balanced ternary is inherently symmetrical around zero:
 *   - Negative (-1) : Backward causation, negation, deficit, or restoration.
 *   - Zero     ( 0) : Equilibrium, standby, neutral, or unperturbed balance.
 *   - Positive (+1) : Forward causation, affirmative assertion, or surplus.
 */
typedef int8_t omni_trit_t;

#define OMNI_TRIT_NEG   ((omni_trit_t)-1)
#define OMNI_TRIT_ZERO  ((omni_trit_t) 0)
#define OMNI_TRIT_POS   ((omni_trit_t) 1)

/* ---------------------------------------------------------------------------
 * BALANCED TERNARY DUAL-RAIL SUBSTRATE (POLAR BIT-PLANES)
 * ------------------------------------------------------------------------ */

/** Dual-Rail Positive Bit-Plane Mask (all even bits: 0, 2, 4, ... 62). */
#define OMNI_BITPLANE_POS_MASK      (0x5555555555555555ULL)

/** Dual-Rail Negative Bit-Plane Mask (all odd bits: 1, 3, 5, ... 63). */
#define OMNI_BITPLANE_NEG_MASK      (0xAAAAAAAAAAAAAAAAULL)

/** Dual-Rail 2-Bit Trit Constants on Binary Silicon. */
#define OMNI_TRIT_PAIR_ZERO         ((uint8_t)0x00) /* 00: Equilibrium (0) */
#define OMNI_TRIT_PAIR_POS          ((uint8_t)0x01) /* 01: Positive (+1)   */
#define OMNI_TRIT_PAIR_NEG          ((uint8_t)0x02) /* 10: Negative (-1)   */
#define OMNI_TRIT_PAIR_NULL         ((uint8_t)0x03) /* 11: Collision (NULL)*/

/* ---------------------------------------------------------------------------
 * 2. QUAD-STATE CAUSAL TRUTH: omni_quad_state_t
 * ------------------------------------------------------------------------ */

/**
 * @brief The fourfold causal truth states of computation in OmniCode.
 *
 * Unifies boolean logic, error handling, scheduling, and clock detents:
 *   - OMNI_STATE_PROCEED  (+1) : Forward Causation (+t). Equilibrium.
 *   - OMNI_STATE_AWAIT    ( 0) : Equilibrium Standby (t=t). Pending async.
 *   - OMNI_STATE_ROLLBACK (-1) : Retroactive Reversal (-t). Guard tripped.
 *   - OMNI_STATE_HALT     (-2) : Terminal Detent (NULL). Clock stopped.
 */
typedef enum {
    OMNI_STATE_ROLLBACK = -1,   /**< -1 : Retroactive Reversal / Error */
    OMNI_STATE_AWAIT    =  0,   /**<  0 : Equilibrium Standby / Hold */
    OMNI_STATE_PROCEED  =  1,   /**< +1 : Forward Causation / Advance */
    OMNI_STATE_HALT     = -2    /**< NULL: Terminal Detent / Clock Stop */
} omni_quad_state_t;

/**
 * @brief Tertiary execution status subset for non-terminal checkpoints.
 */
typedef enum {
    OMNI_STATUS_ROLLBACK = -1,
    OMNI_STATUS_AWAIT    =  0,
    OMNI_STATUS_PROCEED  =  1
} omni_status_t;

/* ---------------------------------------------------------------------------
 * 3. HARDWARE COGNITIVE SITES (EXEMPLARS)
 * ------------------------------------------------------------------------ */

/** Unmapped null pointer origin. */
#define OMNI_SITE_NULL          ((uintptr_t)0x00000000)

/** Real-mode Master Boot Record physical origin. */
#define OMNI_SITE_MBR_BOOT      ((uintptr_t)0x00007C00)

/** Hardware UART serial port base (COM1) for kernel telemetry. */
#define OMNI_SITE_COM1          ((uintptr_t)0x000003F8)

/** Physical memory-mapped VGA text-mode display buffer (80x25 characters). */
#define OMNI_SITE_VGA_BUFFER    ((uintptr_t)0x000B8000)

/* ---------------------------------------------------------------------------
 * 4. TRIPWIRE & INTEGRITY DETENTS: omni_tripwire_state_t
 * ------------------------------------------------------------------------ */

/**
 * @brief Safety interlock state preventing silent memory corruption.
 */
typedef enum {
    OMNI_TRIPWIRE_DISARMED = 0, /**< Interlock inactive; safe quiescent */
    OMNI_TRIPWIRE_ARMED    = 1, /**< Active guard monitoring bounds */
    OMNI_TRIPWIRE_TRIPPED  = 2  /**< Violation intercepted; blocked */
} omni_tripwire_state_t;

/* ===========================================================================
 * BLOCK 2: OPENING BLOCK
 * ( Subsystem Preconditions, Verification & Initializers )
 * ======================================================================== */

/**
 * @brief Verifies that host hardware meets OmniCode's bare-metal preconditions.
 */
static inline omni_quad_state_t omni_base_verify_preconditions(void) {
    if (sizeof(uintptr_t) != 8) return OMNI_STATE_ROLLBACK;
    if (sizeof(uint8_t)   != 1) return OMNI_STATE_ROLLBACK;

    /* Prove Proverbs 11:1 Law { Equilibrium }: (+1) plus (-1) <=> 0 */
    omni_trit_t pos = OMNI_TRIT_POS;
    omni_trit_t neg = OMNI_TRIT_NEG;
    if ((pos + neg) != OMNI_TRIT_ZERO) return OMNI_STATE_ROLLBACK;

    return OMNI_STATE_PROCEED;
}

static inline void omni_tripwire_arm(omni_tripwire_state_t *tw) {
    if (tw) *tw = OMNI_TRIPWIRE_ARMED;
}

static inline void omni_tripwire_disarm(omni_tripwire_state_t *tw) {
    if (tw) *tw = OMNI_TRIPWIRE_DISARMED;
}

static inline void omni_tripwire_trip(omni_tripwire_state_t *tw) {
    if (tw) *tw = OMNI_TRIPWIRE_TRIPPED;
}

/* ===========================================================================
 * BLOCK 3: BODY BLOCK
 * ( Balanced Ternary Algebra, Quad-State Logic & Connective Inlines )
 * ======================================================================== */

static inline omni_trit_t omni_trit_negate(omni_trit_t t) {
    return (omni_trit_t)(-t);
}

static inline omni_trit_t omni_trit_multiply(omni_trit_t a, omni_trit_t b) {
    return (omni_trit_t)(a * b);
}

static inline omni_trit_t omni_trit_add(omni_trit_t a, omni_trit_t b,
                                        omni_trit_t cin, omni_trit_t *cout) {
    int32_t sum = (int32_t)a + (int32_t)b + (int32_t)cin;
    omni_trit_t carry = 0;

    if (sum > 1) {
        sum -= 3;
        carry = 1;
    } else if (sum < -1) {
        sum += 3;
        carry = -1;
    }

    if (cout) *cout = carry;
    return (omni_trit_t)sum;
}

/* ---------------------------------------------------------------------------
 * BALANCED TERNARY DUAL-RAIL HARDWARE CONVERSIONS
 * ------------------------------------------------------------------------ */

static inline uint8_t omni_trit_to_pair(omni_trit_t trit) {
    if (trit == OMNI_TRIT_ZERO) return OMNI_TRIT_PAIR_ZERO;
    if (trit == OMNI_TRIT_POS)  return OMNI_TRIT_PAIR_POS;
    if (trit == OMNI_TRIT_NEG)  return OMNI_TRIT_PAIR_NEG;
    return OMNI_TRIT_PAIR_NULL;
}

static inline bool omni_pair_to_trit(uint8_t pair, omni_trit_t *out_trit) {
    switch (pair & 0x03) {
        case OMNI_TRIT_PAIR_ZERO:
            if (out_trit) *out_trit = OMNI_TRIT_ZERO;
            return true;
        case OMNI_TRIT_PAIR_POS:
            if (out_trit) *out_trit = OMNI_TRIT_POS;
            return true;
        case OMNI_TRIT_PAIR_NEG:
            if (out_trit) *out_trit = OMNI_TRIT_NEG;
            return true;
        default: /* OMNI_TRIT_PAIR_NULL */
            if (out_trit) *out_trit = OMNI_TRIT_ZERO;
            return false;
    }
}

static inline uint64_t omni_dual_rail_negate(uint64_t packed) {
    uint64_t pos = packed & OMNI_BITPLANE_POS_MASK;
    uint64_t neg = packed & OMNI_BITPLANE_NEG_MASK;
    return ((pos << 1) & OMNI_BITPLANE_NEG_MASK) |
           ((neg >> 1) & OMNI_BITPLANE_POS_MASK);
}

static inline bool omni_dual_rail_has_collision(uint64_t packed) {
    uint64_t pos = packed & OMNI_BITPLANE_POS_MASK;
    uint64_t neg = (packed >> 1) & OMNI_BITPLANE_POS_MASK;
    return (pos & neg) != 0;
}

/* ---------------------------------------------------------------------------
 * 5. BALANCED TERNARY CONNECTIVE OPERATORS (!, ::, |, --)
 * ------------------------------------------------------------------------ */

/**
 * @brief Adversative caveat gate: A ! B (A executes, BUT caveat B must hold).
 */
static inline omni_quad_state_t omni_quad_state_but(omni_quad_state_t action,
                                                    omni_quad_state_t caveat) {
    if (caveat == OMNI_STATE_ROLLBACK) return OMNI_STATE_ROLLBACK;
    if (caveat == OMNI_STATE_HALT)     return OMNI_STATE_HALT;
    return action;
}

/**
 * @brief Cumulative conjunction: A :: B (A AND B simultaneous).
 */
static inline omni_quad_state_t omni_quad_state_and(omni_quad_state_t a,
                                                    omni_quad_state_t b) {
    if (a == OMNI_STATE_HALT || b == OMNI_STATE_HALT) return OMNI_STATE_HALT;
    if (a == OMNI_STATE_ROLLBACK || b == OMNI_STATE_ROLLBACK) {
        return OMNI_STATE_ROLLBACK;
    }
    if (a == OMNI_STATE_AWAIT || b == OMNI_STATE_AWAIT) return OMNI_STATE_AWAIT;
    return OMNI_STATE_PROCEED;
}

/**
 * @brief Disjunctive alternative valve: A | B (A OR B).
 */
static inline omni_quad_state_t omni_quad_state_or(omni_quad_state_t a,
                                                   omni_quad_state_t b) {
    if (a == OMNI_STATE_PROCEED || b == OMNI_STATE_PROCEED) {
        return OMNI_STATE_PROCEED;
    }
    if (a == OMNI_STATE_AWAIT || b == OMNI_STATE_AWAIT) return OMNI_STATE_AWAIT;
    if (a == OMNI_STATE_ROLLBACK && b == OMNI_STATE_ROLLBACK) {
        return OMNI_STATE_ROLLBACK;
    }
    return OMNI_STATE_HALT;
}

/**
 * @brief Balanced ternary truth inversion: -- A (NOT A).
 */
static inline omni_quad_state_t omni_quad_state_not(omni_quad_state_t a) {
    if (a == OMNI_STATE_PROCEED)  return OMNI_STATE_ROLLBACK;
    if (a == OMNI_STATE_ROLLBACK) return OMNI_STATE_PROCEED;
    return a;
}

/**
 * @brief Inequality verification: A != B (A not to be B).
 */
static inline omni_quad_state_t omni_quad_state_neq(omni_quad_state_t a,
                                                    omni_quad_state_t b) {
    if (a == OMNI_STATE_HALT || b == OMNI_STATE_HALT) return OMNI_STATE_HALT;
    return (a != b) ? OMNI_STATE_PROCEED : OMNI_STATE_ROLLBACK;
}

/**
 * @brief Predicate is-not check: A =! B (State A is not target B).
 */
static inline omni_quad_state_t omni_quad_state_is_not(
    omni_quad_state_t a,
    omni_quad_state_t target)
{
    return (a != target) ? OMNI_STATE_PROCEED : OMNI_STATE_ROLLBACK;
}

/**
 * @brief Liken to / compatibility match: A =~ B.
 */
static inline omni_quad_state_t omni_quad_state_liken(omni_quad_state_t a,
                                                      omni_quad_state_t b) {
    if (a == OMNI_STATE_HALT || b == OMNI_STATE_HALT) return OMNI_STATE_HALT;
    if (a == b) return OMNI_STATE_PROCEED;
    if ((a == OMNI_STATE_PROCEED || a == OMNI_STATE_AWAIT) &&
        (b == OMNI_STATE_PROCEED || b == OMNI_STATE_AWAIT)) {
        return OMNI_STATE_PROCEED;
    }
    return OMNI_STATE_ROLLBACK;
}

/**
 * @brief Relation projection valve: Context : Aspect.
 */
static inline omni_quad_state_t omni_quad_state_relate(omni_quad_state_t ctx,
                                                       omni_quad_state_t asp) {
    if (ctx == OMNI_STATE_HALT)     return OMNI_STATE_HALT;
    if (ctx == OMNI_STATE_ROLLBACK) return OMNI_STATE_ROLLBACK;
    return asp;
}

/**
 * @brief Verifies whether a given state is one of the defined causal states.
 */
static inline bool omni_state_is_valid(omni_quad_state_t s) {
    return (s == OMNI_STATE_PROCEED ||
            s == OMNI_STATE_AWAIT   ||
            s == OMNI_STATE_ROLLBACK||
            s == OMNI_STATE_HALT);
}

static inline omni_quad_state_t omni_guard_assert(bool condition,
                                                  omni_tripwire_state_t *tw,
                                                  omni_quad_state_t *status) {
    if (condition) {
        if (status) *status = OMNI_STATE_PROCEED;
        return OMNI_STATE_PROCEED;
    } else {
        if (tw) *tw = OMNI_TRIPWIRE_TRIPPED;
        if (status) *status = OMNI_STATE_ROLLBACK;
        return OMNI_STATE_ROLLBACK;
    }
}

static inline bool omni_is_proceed(omni_quad_state_t s) {
    return (s == OMNI_STATE_PROCEED);
}
static inline bool omni_is_await(omni_quad_state_t s) {
    return (s == OMNI_STATE_AWAIT);
}
static inline bool omni_is_rollback(omni_quad_state_t s) {
    return (s == OMNI_STATE_ROLLBACK);
}
static inline bool omni_is_halt(omni_quad_state_t s) {
    return (s == OMNI_STATE_HALT);
}

/* ===========================================================================
 * BLOCK 4: CLOSING BLOCK
 * ( Integrity Seals, Detents & Commit Status Functions )
 * ======================================================================== */

static inline omni_quad_state_t omni_base_verify_integrity(
    omni_tripwire_state_t tw,
    omni_quad_state_t status)
{
    if (tw == OMNI_TRIPWIRE_TRIPPED || status == OMNI_STATE_ROLLBACK) {
        return OMNI_STATE_ROLLBACK;
    }
    return OMNI_STATE_PROCEED;
}

static inline omni_quad_state_t omni_commit_proceed(omni_tripwire_state_t *tw) {
    omni_tripwire_disarm(tw);
    return OMNI_STATE_PROCEED;
}

static inline omni_quad_state_t omni_commit_rollback(
    omni_tripwire_state_t *tw) {
    omni_tripwire_trip(tw);
    return OMNI_STATE_ROLLBACK;
}

static inline omni_quad_state_t omni_commit_halt(omni_tripwire_state_t *tw) {
    omni_tripwire_disarm(tw);
    return OMNI_STATE_HALT;
}

#ifdef __cplusplus
}
#endif

#endif /* OMNI_BASE_H */
