/* ==============================================================================
 * BLOCK 1: METADATA & INVARIANT TYPE DEFINITION
 * ( Invariant Type: < Idea { Type } : State > )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Defines causal truth, clock progression, and the canonical Quad-State machine
 *   in the OmniCode Standard Library Track 2 C Transpile Variant.
 *   - Base Block: Primitive Invariant Type
 *   - Category: Idea { Type }
 *   - Footprint: [ 1 Trit ] { 2 Bits / Quad-State Causal Register }
 *   - Named Registers: [ STATE, TURN, BOUND, IT ]
 *   - System: OmniCode { Lexicon }
 *   - Witness: Scripture Ecclesiastes 3:1 KJV
 *     "To every thing there is a season, and a time to every purpose under
 *      the heaven:"
 *   - Semantics: { QuadStateTruth :: CausalClock :: CanonicalLoopLaw }
 * ============================================================================== */

#ifndef OMNI_STATE_H
#define OMNI_STATE_H

#include "omni_base.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ------------------------------------------------------------------------------
 * 1. CANONICAL NAMED REGISTERS: omni_state_registers_t
 * ------------------------------------------------------------------------------ */

/**
 * @brief The four canonical named registers governing execution kinematics.
 * 
 * Named Registers:
 *   - STATE : Primary causal truth register (+1: PROCEED, 0: AWAIT, -1: ROLLBACK, HALT: NULL).
 *   - TURN  : Discrete cadence pulse counter (increments on affirmative causation).
 *   - BOUND : Dynamic loop termination threshold.
 *   - IT    : Active accumulator iterator stepping along the causal tape.
 */
typedef struct {
    omni_quad_state_t state;                            /**< STATE: Primary Quad-State truth register */
    int64_t           turn;                             /**< TURN: Cadence step counter */
    int64_t           bound;                            /**< BOUND: Loop termination boundary */
    int64_t           it;                               /**< IT: Active accumulator iterator */
} omni_state_registers_t;

/* ------------------------------------------------------------------------------
 * 2. FREESTANDING INVARIANT TYPE STRUCTURE: omni_state_t
 * ------------------------------------------------------------------------------ */

/**
 * @brief Invariant Type < State >: The causal truth and control flow engine.
 * 
 * Encloses the physical trit substrate and register bank:
 *   - Single-trit physical substrate (2 bits in silicon).
 *   - Implements the Canonical Looping Law connecting Quad-State to hardware registers.
 * 
 * Core Invariant Law:
 *   LAW { Quad-State } : { T4 = [ +1:PROCEED | 0:AWAIT | -1:ROLLBACK | HALT:NULL ] }
 *   LAW { Loop } : LOOP [ WHEN [ { IT } < { BOUND } ] --> [ IT` ] PROCEED to { TURN }
 *                         ! [ { IT } > { BOUND } ] AWAIT for [ IT` ]
 *                         :: HALT WHEN [ IT` ] = NULL ]
 */
typedef struct {
    omni_trit_t             substrate_trit;             /**< 1 Trit physical substrate */
    omni_state_registers_t  registers;                  /**< Canonical Quad-State register bank */
    bool                    is_enclosed;                /**< Invariant boundary enclosure guarantee */
} omni_state_t;

/* ------------------------------------------------------------------------------
 * 3. 4-BLOCK PIPELINE CONTEXT: omni_state_context_t
 * ------------------------------------------------------------------------------ */

/**
 * @brief Execution context for State governance and loop lifecycle rolls.
 */
typedef struct {
    omni_quad_state_t     input_signal;                 /**< Outside-in causal truth signal */
    omni_state_t          state_engine;                 /**< Concrete single-trit register bank */
    omni_quad_state_t     status;                       /**< Active Quad-State truth */
    omni_tripwire_state_t tripwire;                     /**< Subsystem safety interlock */
} omni_state_context_t;

/* ==============================================================================
 * BLOCK 2: OPENING BLOCK
 * ( Outside-in Imposition & Context Initialization )
 * ============================================================================== */

/**
 * @brief Initializes the State context with an input signal and loop boundary.
 * 
 * @param ctx Pointer to caller-allocated state context.
 * @param signal Initial truth signal (defaults to PROCEED if 0).
 * @param bound_limit Maximum loop iteration threshold.
 */
void omni_state_open(omni_state_context_t *ctx, omni_quad_state_t signal, int64_t bound_limit);

/* ==============================================================================
 * BLOCK 3: BODY BLOCK
 * ( Operational Primitives, Loop Laws & Transitions )
 * ============================================================================== */

/**
 * @brief Creates a freestanding State engine with specified signal and boundary.
 * 
 * @param initial_signal Starting Quad-State truth.
 * @param bound Maximum loop bound.
 * @return Fully initialized omni_state_t.
 */
omni_state_t omni_state_create(omni_quad_state_t initial_signal, int64_t bound);

/**
 * @brief Canonical Loop Law Single-Step Transition:
 * 
 * Executes one step of the formal loop grammar:
 *   - WHEN IT < BOUND: Increments IT and TURN; returns OMNI_STATE_PROCEED.
 *   - WHEN IT > BOUND: Transitions to standby; returns OMNI_STATE_AWAIT.
 *   - WHEN IT == BOUND: Reaches detent; returns OMNI_STATE_HALT.
 * 
 * @param regs Pointer to register bank.
 * @return Resulting Quad-State condition after the step.
 */
omni_quad_state_t omni_state_step_loop(omni_state_registers_t *regs);

/**
 * @brief Resets register bank counters to 0 while setting state to PROCEED.
 * 
 * @param s Pointer to state engine.
 * @param bound New loop bound.
 */
void omni_state_reset(omni_state_t *s, int64_t bound);

/** Predicate queries for State engine */
bool omni_state_is_halted(const omni_state_t *s);
bool omni_state_is_proceeding(const omni_state_t *s);
bool omni_state_is_awaiting(const omni_state_t *s);
bool omni_state_is_rollback(const omni_state_t *s);

/**
 * @brief Executes the full governance lifecycle across declared rolls:
 *   - Roll 1: Inscribe state register from input signal.
 *   - Roll 2: Evaluate caveat guard against discrepancy.
 *   - Roll 3: Execute canonical loop until boundary or detent.
 *   - Roll 4: Govern machine clock output.
 * 
 * @param ctx Pointer to active state context.
 * @return Final Quad-State condition upon detent.
 */
omni_quad_state_t omni_state_execute(omni_state_context_t *ctx);

/* ==============================================================================
 * BLOCK 4: CLOSING BLOCK
 * ( Integrity Verification & Commit Status )
 * ============================================================================== */

/**
 * @brief Validates all invariants and seals the State commit status.
 * 
 * @param ctx Pointer to active state context.
 * @return OMNI_STATE_PROCEED on successful seal; OMNI_STATE_ROLLBACK if tripped.
 */
omni_quad_state_t omni_state_close(omni_state_context_t *ctx);

#ifdef __cplusplus
}
#endif

#endif /* OMNI_STATE_H */
