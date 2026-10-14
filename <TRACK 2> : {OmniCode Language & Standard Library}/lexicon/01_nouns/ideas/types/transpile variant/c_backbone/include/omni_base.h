/* ==============================================================================
 * BLOCK 1: METADATA & INVARIANT AXIOMS BLOCK
 * ( Invariant Root: < Lexicon { Types of Ideas } > & [ Noun { Base Axioms } ] )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Foundational bare-metal substrates, balanced ternary types, and Quad-State
 *   truth logic governing the OmniCode Standard Library Track 2 C Transpile Variant.
 *   - Environment: Freestanding C99/C11 (Zero libc/OS dependency)
 *   - Word Size: 64-Bit Silicon Scalar / 32 Balanced Trits
 *   - Cell Size: 8-Bit Byte / 5 Balanced Trits
 *   - System: OmniCode { Lexicon }
 *   - Witness: Scripture Proverbs 11:1 KJV
 *     "A false balance is abomination to the LORD: but a just weight is his delight."
 *   - Semantics: { BalancedTernary :: QuadStateTruth :: SiliconAnchors :: TripwireGuards }
 * ============================================================================== */

#ifndef OMNI_BASE_H
#define OMNI_BASE_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ------------------------------------------------------------------------------
 * ARCHITECTURAL CONSTANTS & SUBSTRATE CAPACITIES
 * ------------------------------------------------------------------------------ */

/** Number of balanced trits packed into a single 8-bit memory Cell (3^5 = 243 states). */
#define OMNI_TRITS_PER_CELL         5

/** Number of balanced trits packed into a single 64-bit machine Word. */
#define OMNI_TRITS_PER_WORD         32

/** Footprint of the standard bounded Container vessel exemplar (in bytes). */
#define OMNI_VESSEL_DEFAULT_CELLS   64

/* ------------------------------------------------------------------------------
 * 1. BALANCED TERNARY SUBSTRATE: omni_trit_t
 * ------------------------------------------------------------------------------ */

/**
 * @brief Discrete balanced ternary digit (-1, 0, +1).
 * 
 * Unlike binary (0, 1), balanced ternary is inherently symmetrical around zero:
 *   - Negative (-1) : Backward causation, negation, deficit, or restoration.
 *   - Zero     ( 0) : Equilibrium, standby, neutral, or unperturbed balance.
 *   - Positive (+1) : Forward causation, affirmative assertion, or surplus.
 */
typedef int8_t omni_trit_t;

#define OMNI_TRIT_NEG   ((omni_trit_t)-1)
#define OMNI_TRIT_ZERO  ((omni_trit_t) 0)
#define OMNI_TRIT_POS   ((omni_trit_t) 1)

/* ------------------------------------------------------------------------------
 * 2. QUAD-STATE CAUSAL TRUTH: omni_quad_state_t
 * ------------------------------------------------------------------------------ */

/**
 * @brief The fourfold causal truth states of computation in OmniCode.
 * 
 * Unifies boolean logic, error handling, scheduling, and processor clock detents:
 *   - OMNI_STATE_PROCEED  (+1) : Forward Causation (+t). Operation completed in equilibrium.
 *   - OMNI_STATE_AWAIT    ( 0) : Equilibrium Standby (t=t). Pending asynchronous resolution.
 *   - OMNI_STATE_ROLLBACK (-1) : Retroactive Reversal (-t). Guard tripped; unwind causal drift.
 *   - OMNI_STATE_HALT   (-128) : Terminal Detent (t=empty). Clock detent; permanent halt (HLT).
 */
typedef enum {
    OMNI_STATE_ROLLBACK = -1,   /**< -1 : Retroactive Reversal / Error / Unwind */
    OMNI_STATE_AWAIT    =  0,   /**<  0 : Equilibrium Standby / Hold Pending */
    OMNI_STATE_PROCEED  =  1,   /**< +1 : Forward Causation / Affirmative Advancement */
    OMNI_STATE_HALT     = -128  /**< NULL : Terminal Detent / Stop Processor Clock */
} omni_quad_state_t;

/**
 * @brief Tertiary execution status subset for non-terminal roll checkpoints.
 */
typedef enum {
    OMNI_STATUS_ROLLBACK = -1,
    OMNI_STATUS_AWAIT    =  0,
    OMNI_STATUS_PROCEED  =  1
} omni_status_t;

/* ------------------------------------------------------------------------------
 * 3. HARDWARE COGNITIVE SITES (EXEMPLARS)
 * ------------------------------------------------------------------------------ */

/** Unmapped null pointer origin. */
#define OMNI_SITE_NULL          ((uintptr_t)0x00000000)

/** Real-mode Master Boot Record physical origin. */
#define OMNI_SITE_MBR_BOOT      ((uintptr_t)0x00007C00)

/** Hardware UART serial port base (COM1) for kernel telemetry. */
#define OMNI_SITE_COM1          ((uintptr_t)0x000003F8)

/** Physical memory-mapped VGA text-mode display buffer (80x25 characters). */
#define OMNI_SITE_VGA_BUFFER    ((uintptr_t)0x000B8000)

/* ------------------------------------------------------------------------------
 * 4. TRIPWIRE & INTEGRITY DETENTS: omni_tripwire_state_t
 * ------------------------------------------------------------------------------ */

/**
 * @brief Safety interlock state preventing silent memory corruption or causal leakage.
 */
typedef enum {
    OMNI_TRIPWIRE_DISARMED = 0, /**< Interlock inactive; safe quiescent state */
    OMNI_TRIPWIRE_ARMED    = 1, /**< Active guard monitoring execution bounds */
    OMNI_TRIPWIRE_TRIPPED  = 2  /**< Violation intercepted; execution blocked */
} omni_tripwire_state_t;

/* ==============================================================================
 * BLOCK 2: OPENING BLOCK
 * ( Subsystem Preconditions, Verification & Initializers )
 * ============================================================================== */

/**
 * @brief Verifies that the host hardware meets OmniCode's bare-metal preconditions:
 *   1. Standard 64-bit pointer architecture (sizeof(uintptr_t) == 8).
 *   2. Standard 8-bit octet cell substrate (sizeof(uint8_t) == 1).
 *   3. Zero-sum arithmetic equilibrium ((+1) + (-1) == 0).
 * 
 * @return OMNI_STATE_PROCEED if all preconditions hold; OMNI_STATE_ROLLBACK otherwise.
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

/**
 * @brief Arms a tripwire interlock before entering an untrusted or dynamic execution block.
 */
static inline void omni_tripwire_arm(omni_tripwire_state_t *tw) {
    if (tw) *tw = OMNI_TRIPWIRE_ARMED;
}

/**
 * @brief Disarms a tripwire upon successful verification and closing commit.
 */
static inline void omni_tripwire_disarm(omni_tripwire_state_t *tw) {
    if (tw) *tw = OMNI_TRIPWIRE_DISARMED;
}

/**
 * @brief Immediately trips the interlock, halting forward causation.
 */
static inline void omni_tripwire_trip(omni_tripwire_state_t *tw) {
    if (tw) *tw = OMNI_TRIPWIRE_TRIPPED;
}

/* ==============================================================================
 * BLOCK 3: BODY BLOCK
 * ( Balanced Ternary Algebra, Quad-State Logic & Guard Inlines )
 * ============================================================================== */

/**
 * @brief Negates a balanced trit along the causal vector (-t <=> +t).
 */
static inline omni_trit_t omni_trit_negate(omni_trit_t t) {
    return (omni_trit_t)(-t);
}

/**
 * @brief Multiplies two balanced trits (commutative zero-sum product).
 */
static inline omni_trit_t omni_trit_multiply(omni_trit_t a, omni_trit_t b) {
    return (omni_trit_t)(a * b);
}

/**
 * @brief Full 1-trit balanced ternary adder with carry output.
 * 
 * Truth Table:
 *   a   b  cin | sum carry
 *  -1  -1   -1 |   0    -1
 *  -1  -1    0 |  +1    -1
 *  -1  -1   +1 |  -1     0
 *  -1   0    0 |  -1     0
 *  -1  +1    0 |   0     0  (Equilibrium)
 *  +1  +1    0 |  -1    +1
 *  +1  +1   +1 |   0    +1
 */
static inline omni_trit_t omni_trit_add(omni_trit_t a, omni_trit_t b, omni_trit_t cin, omni_trit_t *cout) {
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

/**
 * @brief Evaluates an invariant guard condition.
 * 
 * If condition evaluates to true:
 *   Status is confirmed in PROCEED (+1).
 * If condition evaluates to false:
 *   Tripwire is tripped and status transitions to ROLLBACK (-1).
 */
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

/** Predicate queries for Quad-State truth */
static inline bool omni_is_proceed(omni_quad_state_t s) { return (s == OMNI_STATE_PROCEED); }
static inline bool omni_is_await(omni_quad_state_t s)   { return (s == OMNI_STATE_AWAIT); }
static inline bool omni_is_rollback(omni_quad_state_t s){ return (s == OMNI_STATE_ROLLBACK); }
static inline bool omni_is_halt(omni_quad_state_t s)    { return (s == OMNI_STATE_HALT); }

/* ==============================================================================
 * BLOCK 4: CLOSING BLOCK
 * ( Integrity Seals, Detents & Commit Status Functions )
 * ============================================================================== */

/**
 * @brief Validates the integrity of an execution context before sealing the commit status.
 * 
 * Verifies that:
 *   1. No tripwire was tripped during execution.
 *   2. Machine status is not trapped in an unhandled rollback.
 * 
 * @return OMNI_STATE_PROCEED if context is clean; OMNI_STATE_ROLLBACK on failure.
 */
static inline omni_quad_state_t omni_base_verify_integrity(omni_tripwire_state_t tw,
                                                           omni_quad_state_t status) {
    if (tw == OMNI_TRIPWIRE_TRIPPED || status == OMNI_STATE_ROLLBACK) {
        return OMNI_STATE_ROLLBACK;
    }
    return OMNI_STATE_PROCEED;
}

/**
 * @brief Closes and seals an execution block in PROCEED (+1).
 */
static inline omni_quad_state_t omni_commit_proceed(omni_tripwire_state_t *tw) {
    omni_tripwire_disarm(tw);
    return OMNI_STATE_PROCEED;
}

/**
 * @brief Closes and seals an execution block in ROLLBACK (-1).
 */
static inline omni_quad_state_t omni_commit_rollback(omni_tripwire_state_t *tw) {
    omni_tripwire_trip(tw);
    return OMNI_STATE_ROLLBACK;
}

/**
 * @brief Closes and seals an execution block in terminal detent HALT:NULL.
 */
static inline omni_quad_state_t omni_commit_halt(omni_tripwire_state_t *tw) {
    omni_tripwire_disarm(tw);
    return OMNI_STATE_HALT;
}

#ifdef __cplusplus
}
#endif

#endif /* OMNI_BASE_H */
