/* ==============================================================================
 * BLOCK 1: METADATA & TEST TOPOLOGY
 * ( Unit Test: < Lexicon { Types of Ideas } > & [ Test { State } ] )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Validates the causal truth governor invariant type < State > in isolation:
 *   - Quad-State named registers: [ STATE TURN BOUND IT ]
 *   - Canonical Loop Law cadence and single-step detent transitions:
 *       WHEN IT < BOUND  --> PROCEED (+1)
 *       !    IT > BOUND  --> AWAIT (0)
 *       ::   IT == BOUND --> HALT (NULL)
 *   - State predicates and register reset mechanics
 *   - Full 4-block lifecycle execution (open, execute, close)
 *   - Witness: Scripture Ecclesiastes 3:1 KJV
 *     "To every thing there is a season, and a time to every purpose under
 *      the heaven:"
 *   - Semantics: { QuadStateTruth :: CanonicalLoopCadence :: CausalDetents }
 * ============================================================================== */

#include "test_common.h"

/* ==============================================================================
 * BLOCK 2: OPENING CONTEXT & FIXTURE SETUP
 * ============================================================================== */

static int test_state_canonical_loop(void) {
    omni_state_t s = omni_state_create(OMNI_STATE_PROCEED, 3);

    OMNI_TEST_ASSERT(s.registers.bound == 3, "Loop bound must be 3");
    OMNI_TEST_ASSERT(s.registers.it == 0, "Initial IT must be 0");
    OMNI_TEST_ASSERT(s.registers.turn == 0, "Initial TURN must be 0");
    OMNI_TEST_ASSERT(omni_state_is_proceeding(&s) == true, "Initial state must be PROCEED");

    /* Step 1: IT 0 < 3 -> increments to IT=1, TURN=1, returns PROCEED */
    omni_quad_state_t st1 = omni_state_step_loop(&s.registers);
    OMNI_TEST_ASSERT(st1 == OMNI_STATE_PROCEED, "Step 1 must PROCEED");
    OMNI_TEST_ASSERT(s.registers.it == 1 && s.registers.turn == 1, "IT and TURN must be 1");

    /* Step 2: IT 1 < 3 -> increments to IT=2, TURN=2, returns PROCEED */
    omni_quad_state_t st2 = omni_state_step_loop(&s.registers);
    OMNI_TEST_ASSERT(st2 == OMNI_STATE_PROCEED, "Step 2 must PROCEED");
    OMNI_TEST_ASSERT(s.registers.it == 2 && s.registers.turn == 2, "IT and TURN must be 2");

    /* Step 3: IT 2 < 3 -> increments to IT=3, TURN=3, returns PROCEED */
    omni_quad_state_t st3 = omni_state_step_loop(&s.registers);
    OMNI_TEST_ASSERT(st3 == OMNI_STATE_PROCEED, "Step 3 must PROCEED");
    OMNI_TEST_ASSERT(s.registers.it == 3 && s.registers.turn == 3, "IT and TURN must be 3");

    /* Step 4: IT == BOUND (3 == 3) -> Reaches terminal detent HALT:NULL */
    omni_quad_state_t st4 = omni_state_step_loop(&s.registers);
    OMNI_TEST_ASSERT(st4 == OMNI_STATE_HALT, "Step 4 (IT == BOUND) must reach HALT:NULL detent");
    OMNI_TEST_ASSERT(omni_state_is_halted(&s) == true, "State must report halted");

    return 0;
}

static int test_state_await_transition(void) {
    omni_state_t s = omni_state_create(OMNI_STATE_PROCEED, 4);

    /* Manually position IT > BOUND to simulate overrun awaiting input */
    s.registers.it = 5;
    omni_quad_state_t await_st = omni_state_step_loop(&s.registers);
    OMNI_TEST_ASSERT(await_st == OMNI_STATE_AWAIT, "IT > BOUND must transition to AWAIT (0)");
    OMNI_TEST_ASSERT(omni_state_is_awaiting(&s) == true, "State engine must report awaiting");

    return 0;
}

static int test_state_predicates_and_reset(void) {
    omni_state_t s = omni_state_create(OMNI_STATE_PROCEED, 2);

    /* Step until halt */
    omni_state_step_loop(&s.registers);
    omni_state_step_loop(&s.registers);
    omni_state_step_loop(&s.registers);
    OMNI_TEST_ASSERT(omni_state_is_halted(&s) == true, "Engine must be halted");

    /* Reset engine */
    omni_state_reset(&s, 5);
    OMNI_TEST_ASSERT(s.registers.bound == 5, "New bound must be 5");
    OMNI_TEST_ASSERT(s.registers.it == 0, "IT must reset to 0");
    OMNI_TEST_ASSERT(s.registers.turn == 0, "TURN must reset to 0");
    OMNI_TEST_ASSERT(omni_state_is_proceeding(&s) == true, "Reset must restore PROCEED status");

    return 0;
}

static int test_state_lifecycle_rolls(void) {
    omni_state_context_t ctx;
    omni_state_open(&ctx, OMNI_STATE_PROCEED, 4);

    OMNI_TEST_ASSERT(ctx.tripwire == OMNI_TRIPWIRE_ARMED, "Tripwire must be ARMED upon open");

    /* Execute full governance cadence */
    omni_quad_state_t exec_status = omni_state_execute(&ctx);
    OMNI_TEST_ASSERT(exec_status == OMNI_STATE_PROCEED, "State execute must PROCEED (+1)");
    OMNI_TEST_ASSERT(ctx.state_engine.registers.it == 4, "Final IT must equal bound 4");
    OMNI_TEST_ASSERT(ctx.state_engine.registers.state == OMNI_STATE_HALT, "Registers must reach HALT:NULL detent");

    omni_quad_state_t close_status = omni_state_close(&ctx);
    OMNI_TEST_ASSERT(close_status == OMNI_STATE_PROCEED, "State close must PROCEED (+1)");
    OMNI_TEST_ASSERT(ctx.tripwire == OMNI_TRIPWIRE_DISARMED, "Tripwire must be DISARMED upon clean close");

    return 0;
}

/* ==============================================================================
 * BLOCK 3: BODY PRIMITIVES & TEST SUITE RUNNER
 * ============================================================================== */

int test_state_suite(void) {
    printf("[SUITE: State Invariant Type]\n");
    OMNI_TEST_RUN(test_state_canonical_loop, "Canonical Loop Law Cadence (IT < BOUND -> HALT)");
    OMNI_TEST_RUN(test_state_await_transition, "Standby Transition (IT > BOUND -> AWAIT)");
    OMNI_TEST_RUN(test_state_predicates_and_reset, "Predicate Queries & Engine Reset");
    OMNI_TEST_RUN(test_state_lifecycle_rolls, "4-Block Lifecycle Rolls");
    return 0;
}
