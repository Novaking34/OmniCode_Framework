/* ==============================================================================
 * BLOCK 1: METADATA & INVARIANT TYPE DEFINITION
 * ( Invariant Type: < Idea { Type } : Number > )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Defines scalar quantities, integers, and balanced ternary measurements in
 *   the OmniCode Standard Library Track 2 C Transpile Variant.
 *   - Base Block: Primitive Invariant Type
 *   - Category: Idea { Type }
 *   - Footprint: [ 1 Memory Word ] { 32 Balanced Trits / 64 Bits }
 *   - System: OmniCode { Lexicon }
 *   - Witness: Scripture Psalm 147:4 KJV
 *     "He telleth the number of the stars; he calleth them all by their names."
 *   - Semantics: { ScalarQuantity :: DualRailBitPlane :: ZeroSumEquilibrium }
 *
 * DUAL-RAIL BIT-PLANE SUBSTRATE:
 *   Every trit is physically encoded as a 2-bit pair (N, P) on binary silicon:
 *     - 00: Equilibrium / True Zero (Neither rail active)
 *     - 01: Positive Unity / +1 (Positive rail P active, bit 0)
 *     - 10: Negative Unity / -1 (Negative rail N active, bit 1)
 *     - 11: Collision / NULL Trap (Both rails active: Tripwire fault)
 *   A 32-trit Word maps bijectively onto a native 64-bit integer (uint64_t).
 *
 * DYNAMIC PACKING TOPOLOGIES:
 *   Byte bit-packing is dynamic and kinematic:
 *     - 2-2-2-2: Pure dense packing (4 trits per byte, 32 trits per 64-bit word)
 *     - 2-1-2-1-2: Symmetric resting equilibrium with carry/guard channels
 *     - 1-2-1-2-2: Prefix carry (Leftward tape advance carry propagation)
 *     - 2-2-1-2-1: Postfix carry (Rightward tape rewind borrow propagation)
 *     - 3-3-2, 3-2-3, 2-3-3: 3-space directional partitions (Z-axis magnitude)
 * ============================================================================== */

#ifndef OMNI_NUMBER_H
#define OMNI_NUMBER_H

#include "omni_base.h"
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ------------------------------------------------------------------------------
 * 1. INVARIANT CONSTANTS, EXEMPLARS & BIT-PLANE MASKS
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

/** Primary Hardware Register Destination: Arithmetic Accumulator. */
#define OMNI_TARGET_ACCUMULATOR     ((uintptr_t)0x00000000)

/** Dual-Rail Positive Bit-Plane Mask (All even bits: 0, 2, 4, ... 62). */
#define OMNI_BITPLANE_POS_MASK      (0x5555555555555555ULL)

/** Dual-Rail Negative Bit-Plane Mask (All odd bits: 1, 3, 5, ... 63). */
#define OMNI_BITPLANE_NEG_MASK      (0xAAAAAAAAAAAAAAAAULL)

/** Dual-Rail 2-Bit Trit Constants. */
#define OMNI_TRIT_PAIR_ZERO         ((uint8_t)0x00) /* 00: Equilibrium (0) */
#define OMNI_TRIT_PAIR_POS          ((uint8_t)0x01) /* 01: Positive (+1)   */
#define OMNI_TRIT_PAIR_NEG          ((uint8_t)0x02) /* 10: Negative (-1)   */
#define OMNI_TRIT_PAIR_NULL         ((uint8_t)0x03) /* 11: Collision (NULL)*/

/* ------------------------------------------------------------------------------
 * 2. DYNAMIC PACKING TOPOLOGY ENUM
 * ------------------------------------------------------------------------------ */

/**
 * @brief Dynamic bit-packing topologies for bytes and 64-bit words.
 */
typedef enum {
    OMNI_PACK_2_2_2_2   = 0, /**< Pure dense packing (4 trits / byte) */
    OMNI_PACK_2_1_2_1_2 = 1, /**< Palindromic symmetric resting equilibrium */
    OMNI_PACK_1_2_1_2_2 = 2, /**< Prefix carry: Leftward tape advance */
    OMNI_PACK_2_2_1_2_1 = 3, /**< Postfix carry: Rightward tape rewind */
    OMNI_PACK_3_3_2     = 4, /**< 3-space directional partition (3-3-2) */
    OMNI_PACK_3_2_3     = 5, /**< 3-space directional partition (3-2-3) */
    OMNI_PACK_2_3_3     = 6  /**< 3-space directional partition (2-3-3) */
} omni_pack_mode_t;

/* ------------------------------------------------------------------------------
 * 3. FREESTANDING INVARIANT TYPE STRUCTURE: omni_number_t
 * ------------------------------------------------------------------------------ */

/**
 * @brief Invariant Type < Number >: The scalar quantity and measurement metric.
 *
 * Encloses the dual substrate:
 *   1. Physical Silicon Substrate: 64-Bit Signed Integer Scalar (int64_t).
 *   2. Dual-Rail Packed Substrate: Native 64-bit packed bit-planes (N, P).
 *   3. Virtual Balanced Trit Substrate: 32-Trit Machine Word array.
 *
 * Core Invariant Law:
 *   ENCLOSE [ Number ] WITHIN [ 1 Memory Word ] (Strict 32-trit capacity).
 *   LAW { Equilibrium } : [ (+1) plus (-1) <=> 0 ] (Foundational balance).
 */
typedef struct {
    int64_t     scalar;                                 /**< 64-bit scalar integer */
    uint64_t    packed_trits;                           /**< Dual-rail bit-planes (N, P) */
    omni_trit_t trits[OMNI_NUM_TRITS_PER_WORD];         /**< Unpacked 32 balanced trits */
    bool        is_enclosed;                            /**< Boundary enclosure seal */
} omni_number_t;

/* ------------------------------------------------------------------------------
 * 4. PURE INVARIANT OPERATIONS, BIT-PLANE & DUAL-RAIL LAWS
 * ------------------------------------------------------------------------------ */

/**
 * @brief Constructs a Number directly from a 64-bit scalar integer,
 *        automatically transducing it into dual-rail packed bit-planes
 *        and 32 balanced trits.
 *
 * @param scalar The 64-bit integer quantity to measure.
 * @return Fully initialized omni_number_t with is_enclosed = true.
 */
omni_number_t omni_number_from_scalar(int64_t scalar);

/**
 * @brief Bijective Radix-3 Transduction: Maps a 64-bit scalar to 32 balanced trits.
 *
 * @param val Input 64-bit scalar quantity.
 * @param out_trits Array of 32 balanced trits receiving the transduction.
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
 * @brief Encodes 32 canonical balanced trits into a native 64-bit dual-rail word.
 *
 * @param trits Array of 32 balanced trits {-1, 0, +1}.
 * @return Packed 64-bit dual-rail word (N, P bit-planes).
 */
uint64_t omni_number_pack_trits(const omni_trit_t trits[OMNI_NUM_TRITS_PER_WORD]);

/**
 * @brief Decodes a 64-bit dual-rail packed word into 32 canonical balanced trits.
 *
 * @param packed Packed 64-bit dual-rail word.
 * @param out_trits Destination array of 32 balanced trits.
 */
void omni_number_unpack_trits(uint64_t packed, omni_trit_t out_trits[OMNI_NUM_TRITS_PER_WORD]);

/**
 * @brief Instant O(1) Bit-Plane Negation: Swaps the positive and negative
 *        bit-planes across all 32 trits simultaneously with zero carry chains.
 *
 * @param packed Packed 64-bit dual-rail word.
 * @return Negated 64-bit dual-rail word (+N <-> -N).
 */
uint64_t omni_number_trits_negate(uint64_t packed);

/**
 * @brief Instant O(1) Collision Detection: Verifies that no trit in the word
 *        holds the illegal collision state (11: NULL).
 *
 * @param packed Packed 64-bit dual-rail word.
 * @return true if any trit is in collision (11: NULL); false if clean.
 */
bool omni_number_trits_has_null(uint64_t packed);

/**
 * @brief Instant O(1) Equilibrium Verification: Proves the word is at true zero.
 *
 * @param packed Packed 64-bit dual-rail word.
 * @return true if all trits are in resting zero equilibrium (00); false otherwise.
 */
bool omni_number_trits_is_equilibrium(uint64_t packed);

/**
 * @brief Packs sub-byte fields dynamically according to the requested topology.
 *
 * @param f1 First field value.
 * @param f2 Second field value.
 * @param f3 Third field value.
 * @param f4 Fourth field value (used in 2-2-2-2 and 5-segment modes).
 * @param f5 Fifth field value (used in 2-1-2-1-2, 1-2-1-2-2, 2-2-1-2-1).
 * @param mode Dynamic packing topology enum.
 * @return Packed 8-bit byte.
 */
uint8_t omni_number_pack_byte(uint8_t f1, uint8_t f2, uint8_t f3,
                              uint8_t f4, uint8_t f5, omni_pack_mode_t mode);

/**
 * @brief Unpacks an 8-bit byte into sub-byte fields based on topology.
 *
 * @param byte_val Input 8-bit byte.
 * @param out_fields Destination array of 5 field values.
 * @param mode Dynamic packing topology enum.
 */
void omni_number_unpack_byte(uint8_t byte_val, uint8_t out_fields[5],
                             omni_pack_mode_t mode);

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
 * @brief Subtracts two Numbers via exact balanced ternary negation: a + (-b).
 *
 * @param a Minuend number.
 * @param b Subtrahend number to negate and add.
 * @return Resulting difference as an invariant omni_number_t.
 */
omni_number_t omni_number_sub(omni_number_t a, omni_number_t b);

/**
 * @brief Negates a Number producing its exact symmetric polar opposite.
 *
 * @param a Number to negate.
 * @return Negated number with swapped bit-planes.
 */
omni_number_t omni_number_neg(omni_number_t a);

/**
 * @brief Evaluates relational equality between two numbers.
 *
 * @param a First number operand.
 * @param b Second number operand.
 * @return true if scalar values match and both are enclosed; false otherwise.
 */
bool omni_number_eq(const omni_number_t *a, const omni_number_t *b);

/**
 * @brief Verifies that a number is properly enclosed within single-word bounds
 *        and free of NULL collision traps.
 *
 * @param n Pointer to number to verify.
 * @return true if valid, enclosed, and trap-free; false otherwise.
 */
bool omni_number_verify_bounds(const omni_number_t *n);

/* ==============================================================================
 * BLOCK 2: OPENING BLOCK
 * ( Outside-In Imposition, Accumulator Binding & Preconditions )
 * ============================================================================== */

/**
 * @brief Execution context for Number measurement and ALU lifecycle rolls.
 */
typedef struct {
    int64_t               input_quantity;     /**< Outside-in scalar payload */
    omni_number_t         number;             /**< Concrete single-word allocation */
    uintptr_t             target_accumulator; /**< Target accumulator destination */
    void                 *sequence_target;    /**< Destination for Vector sequence */
    omni_quad_state_t     status;             /**< Active Quad-State truth */
    omni_tripwire_state_t tripwire;           /**< Subsystem safety interlock */
} omni_number_context_t;

/**
 * @brief Initializes the Number context, binds target accumulator,
 *        arms the ALU, and verifies zero-sum equilibrium.
 *
 * Evaluates OmniCode Block 2: Opening { ... }
 *
 * @param ctx Pointer to caller-allocated number context.
 * @param input_quantity Outside-in scalar quantity (defaults to 4096 if 0).
 * @param target_accumulator Target accumulator register (defaults to 0x00).
 */
void omni_number_open(omni_number_context_t *ctx, int64_t input_quantity,
                      uintptr_t target_accumulator);

/* ==============================================================================
 * BLOCK 3: BODY BLOCK
 * ( Operational Rolls, Guard Interlocks & Vector Sequencing )
 * ============================================================================== */

/**
 * @brief Roll 1: Inscribes outside-in quantity into the word container.
 *
 * Evaluates OmniCode: [ Inscribe ] : { Quantity ] --> INSCRIBE [ Word ] = [ Number` ]
 *
 * @param ctx Active execution context.
 * @param quantity The scalar quantity being inscribed.
 */
void omni_number_inscribe(omni_number_context_t *ctx, int64_t quantity);

/**
 * @brief Roll 2: Transduces Number` through the balanced ternary ALU.
 *
 * Evaluates OmniCode: [ Transduce ] : [ Number` { 64 Bits } ] <=> TRANSDUCE [ Word` { 32 Trits } ]
 *
 * @param ctx Active execution context.
 */
void omni_number_transduce_context(omni_number_context_t *ctx);

/**
 * @brief Roll 3: Evaluates adversative guard assertion (!).
 *
 * Evaluates OmniCode: [ Compare ] : { Number` = Quantity } & { Number` <= [ 1 Word ] }
 *                     ! GUARD --> [ Status` = +1:PROCEED ]
 *
 * @param ctx Active execution context.
 * @return OMNI_STATE_PROCEED on equilibrium; OMNI_STATE_ROLLBACK on violation.
 */
omni_quad_state_t omni_number_guard(omni_number_context_t *ctx);

/**
 * @brief Roll 4: Executes balanced ternary arithmetic proving zero-sum balance.
 *
 * Evaluates OmniCode: [ Calculate ] : [ ALU ] :
 *                     [ (+1) plus (-1) ] <=> CALCULATE [ 0 ] = [ Equilibrium` ]
 *
 * @param ctx Active execution context.
 * @return OMNI_STATE_PROCEED on confirmed balance; OMNI_STATE_ROLLBACK on disequilibrium.
 */
omni_quad_state_t omni_number_calculate(omni_number_context_t *ctx);

/**
 * @brief Roll 5: Yields Number` to continuous Vector coordinate sequence.
 *
 * Evaluates OmniCode: [ Sequence ] : [ Number` } --> SEQUENCE [ Vector ] = [ Vector` ]
 *
 * @param n Pointer to the Number being sequenced.
 * @param sequence_target Pointer to destination vector container.
 * @return true if sequence yield succeeded; false on overflow.
 */
bool omni_number_sequence(const omni_number_t *n, void *sequence_target);

/**
 * @brief Executes the complete 5-roll operational pipeline across declared turns:
 *   Roll 1 (Inscribe) -> Roll 2 (Transduce) -> Roll 3 (Guard !) ->
 *   Roll 4 (Calculate) -> Roll 5 (Sequence).
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
 * @brief Validates all invariants, verifies consensus, disarms tripwire,
 *        and seals the final Number commit status (+1:PROCEED).
 *
 * Evaluates OmniCode Block 4: Closing { ... }
 *
 * @param ctx Pointer to active number context.
 * @return OMNI_STATE_PROCEED on successful seal; OMNI_STATE_ROLLBACK if tripped.
 */
omni_quad_state_t omni_number_close(omni_number_context_t *ctx);

#ifdef __cplusplus
}
#endif

#endif /* OMNI_NUMBER_H */
