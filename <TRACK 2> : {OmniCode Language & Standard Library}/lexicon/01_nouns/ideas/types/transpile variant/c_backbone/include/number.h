/* ==============================================================================
 * BLOCK 1: METADATA & INVARIANT TYPE DEFINITION
 * ( Invariant Type: < Idea { Type } : Number > )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Defines scalar quantities, integers, and balanced ternary measurements in the
 *   OmniCode Standard Library Track 2 C Transpile Variant.
 *   - Base Block: Primitive Invariant Type
 *   - Category: Idea { Type }
 *   - Footprint: [ 1 Word ] { 32 Balanced Trits / 64 Bits }
 *   - System: OmniCode { Lexicon }
 *   - Witness: Scripture Psalm 147:4 KJV
 *     "He telleth the number of the stars; he calleth them all by their names."
 *   - Semantics: { ScalarQuantity :: NumericWord :: BalancedTernaryALU }
 * ============================================================================== */

#ifndef OMNI_NUMBER_H
#define OMNI_NUMBER_H

#include "omni_base.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ------------------------------------------------------------------------------
 * 1. INVARIANT CONSTANTS & CANONICAL EXEMPLARS
 * ------------------------------------------------------------------------------ */

/** Number of balanced trits per machine word. */
#define OMNI_NUM_TRITS_PER_WORD     OMNI_TRITS_PER_WORD
#define OMNI_NUMBER_TRITS_PER_WORD  OMNI_TRITS_PER_WORD

/** Canonical Zero Exemplar: Perfect balance and null displacement. */
#define OMNI_EXEMPLAR_ZERO          ((int64_t)0)

/** Canonical Positive Unity Exemplar: Forward causal affirmation (+1). */
#define OMNI_EXEMPLAR_UNITY_POS     ((int64_t)1)

/** Canonical Negative Unity Exemplar: Backward causal restoration (-1). */
#define OMNI_EXEMPLAR_UNITY_NEG     ((int64_t)-1)

/** Canonical Memory Partition Exemplar: Standard 4096-unit allocation boundary. */
#define OMNI_EXEMPLAR_PARTITION     ((int64_t)4096)

/* ------------------------------------------------------------------------------
 * 2. FREESTANDING INVARIANT TYPE STRUCTURE: omni_number_t
 * ------------------------------------------------------------------------------ */

/**
 * @brief Invariant Type < Number >: The scalar quantity and measurement metric.
 * 
 * Encloses the dual substrate:
 *   1. Physical Silicon Substrate: 64-Bit Signed Integer Scalar (int64_t).
 *   2. Virtual Balanced Trit Substrate: 32-Trit Machine Word.
 * 
 * Core Invariant Law:
 *   ENCLOSE [ Number ] WITHIN [ 1 Word ] (Strict 32-trit capacity).
 *   LAW { Equilibrium } : [ (+1) plus (-1) <=> 0 ] (Foundational zero-sum balance).
 */
typedef struct {
    int64_t     scalar;                                 /**< Physical silicon scalar integer (64 bits) */
    omni_trit_t trits[OMNI_NUM_TRITS_PER_WORD];         /**< Virtual machine word substrate (32 balanced trits) */
    bool        is_enclosed;                            /**< Invariant boundary enclosure guarantee */
} omni_number_t;

/* ------------------------------------------------------------------------------
 * 3. 4-BLOCK PIPELINE CONTEXT: omni_number_context_t
 * ------------------------------------------------------------------------------ */

/**
 * @brief Execution context for Number measurement and ALU lifecycle rolls.
 */
typedef struct {
    int64_t               input_quantity;               /**< Outside-in scalar measurement payload */
    omni_number_t         number;                       /**< Concrete single-word allocation */
    uintptr_t             target_accumulator;           /**< Target register accumulator destination */
    omni_quad_state_t     status;                       /**< Active Quad-State truth */
    omni_tripwire_state_t tripwire;                     /**< Subsystem safety interlock */
} omni_number_context_t;

/* ==============================================================================
 * BLOCK 2: OPENING BLOCK
 * ( Outside-in Imposition & Context Initialization )
 * ============================================================================== */

/**
 * @brief Initializes the Number context, arms the ALU, and verifies equilibrium.
 * 
 * @param ctx Pointer to caller-allocated number context.
 * @param input_quantity Outside-in scalar quantity (defaults to 4096 if 0).
 */
void omni_number_open(omni_number_context_t *ctx, int64_t input_quantity);

/* ==============================================================================
 * BLOCK 3: BODY BLOCK
 * ( Operational Primitives, Arithmetic Laws & Inscriptions )
 * ============================================================================== */

/**
 * @brief Constructs a Number directly from a 64-bit scalar integer,
 *        automatically transducing it into 32 balanced trits.
 * 
 * @param scalar The 64-bit integer quantity to measure.
 * @return Fully initialized omni_number_t with is_enclosed = true.
 */
omni_number_t omni_number_from_scalar(int64_t scalar);

/**
 * @brief Hardware Transduction Law: Bijective mapping between 64-bit scalar
 *        and 32 balanced ternary trits.
 * 
 * @param val Input 64-bit scalar quantity.
 * @param out_trits Array of 32 balanced trits receiving the radix transduction.
 */
void omni_number_transduce(int64_t val, omni_trit_t out_trits[OMNI_NUM_TRITS_PER_WORD]);

/**
 * @brief Inverse Transduction: Reconstructs a 64-bit scalar from 32 balanced trits.
 * 
 * @param trits Array of 32 balanced trits.
 * @return Reconstructed 64-bit signed integer value.
 */
int64_t omni_number_to_scalar(const omni_trit_t trits[OMNI_NUM_TRITS_PER_WORD]);

/**
 * @brief Verifies the ALU Core Arithmetic Equilibrium Law: (+1) plus (-1) <=> 0.
 * 
 * @return true if zero-sum balance holds; false otherwise.
 */
bool omni_number_verify_equilibrium(void);

/**
 * @brief Adds two Numbers producing a new balanced Number word.
 * 
 * @param a First number summand.
 * @param b Second number summand.
 * @return Resulting sum as an invariant omni_number_t.
 */
omni_number_t omni_number_add(omni_number_t a, omni_number_t b);

/**
 * @brief Evaluates relational equality between two numbers.
 * 
 * @param a First number operand.
 * @param b Second number operand.
 * @return true if scalar values match; false otherwise.
 */
bool omni_number_eq(const omni_number_t *a, const omni_number_t *b);

/**
 * @brief Verifies that a number is properly enclosed within its single-word boundary.
 * 
 * @param n Pointer to number to verify.
 * @return true if valid and enclosed; false otherwise.
 */
bool omni_number_verify_bounds(const omni_number_t *n);

/**
 * @brief Executes the full measurement lifecycle across declared rolls:
 *   - Roll 1: Inscribe quantity into word.
 *   - Roll 2: Transduce 64 bits <=> 32 balanced trits.
 *   - Roll 3: Guard identity and boundary invariants.
 *   - Roll 4: Prove ALU zero-sum equilibrium.
 * 
 * @param ctx Pointer to active number context.
 * @return OMNI_STATE_PROCEED on equilibrium; OMNI_STATE_ROLLBACK on violation.
 */
omni_quad_state_t omni_number_execute(omni_number_context_t *ctx);

/* ==============================================================================
 * BLOCK 4: CLOSING BLOCK
 * ( Integrity Verification & Commit Status )
 * ============================================================================== */

/**
 * @brief Validates all invariants and seals the Number commit status.
 * 
 * @param ctx Pointer to active number context.
 * @return OMNI_STATE_PROCEED on successful seal; OMNI_STATE_ROLLBACK if tripped.
 */
omni_quad_state_t omni_number_close(omni_number_context_t *ctx);

#ifdef __cplusplus
}
#endif

#endif /* OMNI_NUMBER_H */
