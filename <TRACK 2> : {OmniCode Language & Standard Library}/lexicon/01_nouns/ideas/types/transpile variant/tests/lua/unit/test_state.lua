-- ==============================================================================
-- BLOCK 1: METADATA & TEST TOPOLOGY
-- ( Unit Test: < Lexicon { Types of Ideas } > & [ Test { State Tape } ] )
-- ==============================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Validates the interpreted process ribbon < State Tape > in isolation:
--   - Quad-State named registers: [ STATE TURN BOUND IT ]
--   - Canonical Loop Law cadence and single-step detents:
--       WHEN IT < BOUND  --> PROCEED (+1)
--       !    IT > BOUND  --> AWAIT (0)
--       ::   IT == BOUND --> HALT (NULL)
--   - Clock governance emergence
--   - Full 4-block lifecycle execution (open, execute_all, close)
--   - Witness: Scripture Ecclesiastes 3:1 KJV
--     "To every thing there is a season, and a time to every purpose under
--      the heaven:"
--   - Semantics: { QuadStateTruth :: CanonicalLoopCadence :: CausalDetents }
-- ==============================================================================

package.path = package.path .. ";./?.lua;./lua_tapes/?.lua;./tests/lua/unit/?.lua;./unit/?.lua;../?.lua;../../?.lua"

local TestCommon = require("test_common")
local TapeEngine = require("lua_tapes.tape_engine")
local StateTape  = require("lua_tapes.state")

local Suite = {}

function Suite.test_state_canonical_loop()
    local ctx = StateTape.open(TapeEngine.STATE_PROCEED, 3)

    TestCommon.assert_eq(ctx.c_state.registers.bound, 3, "Bound must be 3")
    TestCommon.assert_eq(ctx.c_state.registers.it, 0, "Initial IT must be 0")
    TestCommon.assert_eq(ctx.c_state.registers.turn, 0, "Initial TURN must be 0")

    -- Step 1
    local st1 = StateTape.step_loop(ctx)
    TestCommon.assert_eq(st1, TapeEngine.STATE_PROCEED, "Step 1 must PROCEED")
    TestCommon.assert_eq(ctx.c_state.registers.it, 1, "IT must be 1")

    -- Step 2
    local st2 = StateTape.step_loop(ctx)
    TestCommon.assert_eq(st2, TapeEngine.STATE_PROCEED, "Step 2 must PROCEED")
    TestCommon.assert_eq(ctx.c_state.registers.it, 2, "IT must be 2")

    -- Step 3
    local st3 = StateTape.step_loop(ctx)
    TestCommon.assert_eq(st3, TapeEngine.STATE_PROCEED, "Step 3 must PROCEED")
    TestCommon.assert_eq(ctx.c_state.registers.it, 3, "IT must be 3")

    -- Step 4: IT == BOUND -> HALT:NULL
    local st4 = StateTape.step_loop(ctx)
    TestCommon.assert_eq(st4, TapeEngine.STATE_HALT, "Step 4 must reach terminal detent HALT:NULL")
    TestCommon.assert_eq(ctx.c_state.registers.state, TapeEngine.STATE_HALT, "Register state must be HALT:NULL")
end

function Suite.test_state_await_transition()
    local ctx = StateTape.open(TapeEngine.STATE_PROCEED, 4)
    ctx.c_state.registers.it = 5 -- Manually set IT > BOUND

    local st = StateTape.step_loop(ctx)
    TestCommon.assert_eq(st, TapeEngine.STATE_AWAIT, "IT > BOUND must transition to AWAIT (0)")
end

function Suite.test_state_lifecycle_rolls()
    local ctx = StateTape.open(TapeEngine.STATE_PROCEED, 4)
    TestCommon.assert_eq(ctx.tripwire, TapeEngine.TRIPWIRE_ARMED, "Tripwire must be ARMED on open")

    local status = StateTape:execute_all(ctx)
    TestCommon.assert_eq(status, TapeEngine.STATE_HALT, "Execution must reach terminal detent HALT:NULL")
    TestCommon.assert_eq(ctx.c_state.registers.it, 4, "Final IT must equal bound 4")
    TestCommon.assert_eq(ctx.c_state.registers.state, TapeEngine.STATE_HALT, "Register state must reach HALT:NULL")

    local close_st, close_msg = StateTape.close(ctx)
    TestCommon.assert_eq(close_st, TapeEngine.STATE_PROCEED, "Close must seal in PROCEED (+1)")
    TestCommon.assert_eq(ctx.tripwire, TapeEngine.TRIPWIRE_DISARMED, "Tripwire must be DISARMED on close")
end

function Suite.run()
    print("[SUITE: State Process Ribbon (Lua)]")
    local ok1 = TestCommon.run_test("Canonical Loop Law Cadence (IT < BOUND -> HALT)", Suite.test_state_canonical_loop)
    local ok2 = TestCommon.run_test("Standby Transition (IT > BOUND -> AWAIT)", Suite.test_state_await_transition)
    local ok3 = TestCommon.run_test("4-Block Lifecycle Rolls", Suite.test_state_lifecycle_rolls)
    return ok1 and ok2 and ok3
end

-- Allow standalone execution
if pcall(debug.getlocal, 4, 1) == false then
    local ok = Suite.run()
    os.exit(ok and 0 or 1)
end

return Suite
