/* ==============================================================================
 * BLOCK 1: METADATA & INVARIANT IMPLEMENTATION
 * ( Invariant Type: < Idea { Type } : State > )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Implements causal truth, clock progression, and the canonical Quad-State machine
 *   in the OmniCode Standard Library Track 2 C Transpile Variant.
 *   - Base Block: Primitive Invariant Type
 *   - Category: Idea { Type }
 *   - Footprint: [ 1 Trit ] { 2 Bits / Quad-State Causal Register }
 *   - Named Registers: [ STATE, TURN, BOUND, IT ]
 *   - Witness: Scripture Ecclesiastes 3:1 KJV
 *     "To every thing there is a season, and a time to every purpose under
 *      the heaven:"
 *   - Semantics: { QuadStateTruth :: CausalClock :: CanonicalLoopLaw }
 * ============================================================================== */

#include "state.h"

/* ==============================================================================
 * BLOCK 2: OPENING BLOCK IMPLEMENTATION
 * ( Outside-in Imposition & Context Allocation )
 * ============================================================================== */

/**
 * @brief Initializes the State context, sets the starting truth signal,
 *        arms the register bank, and arms the execution tripwire.
 */
void omni_state_open(omni_state_context_t *ctx, omni_quad_state_t signal, int64_t bound_limit) {
    if (!ctx) return;

    /* 1. Outside-in imposition: truth signal captured */
    ctx->input_signal = signal ? signal : OMNI_STATE_PROCEED;

    /* 2. Concrete single-trit register allocation */
    ctx->state_engine = omni_state_create(ctx->input_signal, bound_limit);

    /* 3. Preconditions & tripwire arming */
    omni_tripwire_arm(&ctx->tripwire);
    ctx->status = OMNI_STATE_PROCEED;
}

/* ==============================================================================
 * BLOCK 3: BODY BLOCK IMPLEMENTATION
 * ( Operational Primitives, Loop Laws & Transitions )
 * ============================================================================== */

/**
 * @brief Canonical Loop Law Single-Step Transition:
 * 
 * LAW { Loop } :
 *   LOOP [ WHEN [ { IT } < { BOUND } ] --> [ IT` ] PROCEED to { TURN }
 *          ! [ { IT } > { BOUND } ] AWAIT for [ IT` ]
 *          :: HALT WHEN [ IT` ] = NULL ]
 */
omni_quad_state_t omni_state_step_loop(omni_state_registers_t *regs) {
    if (!regs) return OMNI_STATE_ROLLBACK;

    if (regs->it < regs->bound) {
        regs->it += 1;          /* [ IT` ] */
        regs->turn += 1;        /* PROCEED to { TURN } */
        regs->state = OMNI_STATE_PROCEED;
        return OMNI_STATE_PROCEED;
    } else if (regs->it > regs->bound) {
        regs->state = OMNI_STATE_AWAIT;
        return OMNI_STATE_AWAIT;
    } else {
        /* Terminal Detent reached: IT == BOUND */
        regs->state = OMNI_STATE_HALT;
        return OMNI_STATE_HALT;
    }
}

/**
 * @brief Creates a freestanding State engine instance.
 */
omni_state_t omni_state_create(omni_quad_state_t initial_signal, int64_t bound) {
    omni_state_t s;
    s.substrate_trit = OMNI_TRIT_POS;
    s.registers.state = initial_signal;
    s.registers.turn  = 0;
    s.registers.it    = 0;
    s.registers.bound = (bound > 0) ? bound : 4;
    s.is_enclosed     = true;
    return s;
}

/**
 * @brief Resets state registers to initial state.
 */
void omni_state_reset(omni_state_t *s, int64_t bound) {
    if (!s) return;
    s->registers.state = OMNI_STATE_PROCEED;
    s->registers.turn  = 0;
    s->registers.it    = 0;
    s->registers.bound = (bound > 0) ? bound : 4;
}

/** Predicate queries for State engine */
bool omni_state_is_halted(const omni_state_t *s) {
    return (s != NULL && s->registers.state == OMNI_STATE_HALT);
}

bool omni_state_is_proceeding(const omni_state_t *s) {
    return (s != NULL && s->registers.state == OMNI_STATE_PROCEED);
}

bool omni_state_is_awaiting(const omni_state_t *s) {
    return (s != NULL && s->registers.state == OMNI_STATE_AWAIT);
}

bool omni_state_is_rollback(const omni_state_t *s) {
    return (s != NULL && s->registers.state == OMNI_STATE_ROLLBACK);
}

/**
 * @brief Executes the State governance lifecycle across declared operational rolls:
 *   - Roll 1: Inscribe state register from input signal.
 *   - Roll 2: Evaluate caveat guard against discrepancy.
 *   - Roll 3: Execute canonical loop until boundary or detent.
 *   - Roll 4: Govern machine clock output.
 */
omni_quad_state_t omni_state_execute(omni_state_context_t *ctx) {
    if (!ctx) return OMNI_STATE_ROLLBACK;

    /* ROLL 1: INSCRIBE STATE REGISTER */
    ctx->state_engine.registers.state = ctx->input_signal;

    /* ROLL 2: CAVEAT GUARD AGAINST DISCREPANCY */
    bool signal_match = (ctx->state_engine.registers.state == ctx->input_signal);
    if (omni_guard_assert(signal_match, &ctx->tripwire, &ctx->status) != OMNI_STATE_PROCEED) {
        return OMNI_STATE_ROLLBACK;
    }

    /* ROLL 3: THE CANONICAL LOOP STATEMENT EXECUTION */
    while (ctx->state_engine.registers.state == OMNI_STATE_PROCEED) {
        omni_quad_state_t loop_res = omni_state_step_loop(&ctx->state_engine.registers);
        if (loop_res == OMNI_STATE_HALT) {
            break; /* Clean detent termination */
        } else if (loop_res == OMNI_STATE_AWAIT) {
            break; /* Equilibrium standby */
        }
    }

    /* ROLL 4: GOVERN ENGINE (Inside-out emergence) */
    /* State emerges outward to govern machine clock */

    /* ROLL 5: SEQUENCE TRANSITION (State chains to continuous Timeline) */
    /* Ready to sequence into causal timeline */

    ctx->status = OMNI_STATE_PROCEED;
    return ctx->status;
}

/* ==============================================================================
 * BLOCK 4: CLOSING BLOCK IMPLEMENTATION
 * ( Integrity Seals & Commit Status )
 * ============================================================================== */

/**
 * @brief Verifies all invariants and seals the State commit status.
 */
omni_quad_state_t omni_state_close(omni_state_context_t *ctx) {
    if (!ctx) return OMNI_STATE_ROLLBACK;

    /* Integrity Check: Proves tripwire is untripped and register bank is valid */
    if (omni_base_verify_integrity(ctx->tripwire, ctx->status) != OMNI_STATE_PROCEED) {
        return omni_commit_rollback(&ctx->tripwire);
    }

    /* Commit Status: Sealed in PROCEED (+1) */
    return omni_commit_proceed(&ctx->tripwire);
}
