-- ==============================================================================
-- BLOCK 1: METADATA & TEST TOPOLOGY
-- ( Unit Test: < Lexicon { Types of Ideas } > & [ Test { Container Tape } ] )
-- ==============================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Validates the interpreted process ribbon < Container Tape > in isolation:
--   - Bounded memory capsule build-up
--   - Payload encapsulation into bounded envelope
--   - Law of Containment verification (Occupancy <= Capacity)
--   - Overflow detection and tripwire interlock rollback
--   - Full 4-block lifecycle execution (open, execute_all, close)
--   - Witness: Scripture 2 Kings 4:4 KJV
--     "And when thou art come in, thou shalt shut the door upon thee and upon
--      thy sons, and shalt pour out into all those vessels, and thou shalt set
--      aside that which is full."
--   - Semantics: { MemoryCapsule :: BoundedEnvelope :: ContainmentLaw }
-- ==============================================================================

package.path        = package.path .. ";./?.lua;./lua_tapes/?.lua;./tests/lua/unit/?.lua;./unit/?.lua;../?.lua;../../?.lua"

local TestCommon    = require("test_common")
local TapeEngine    = require("lua_tapes.tape_engine")
local ContainerTape = require("lua_tapes.container")

local Suite         = {}

function Suite.test_container_encapsulation()
    local payload = { 10, 20, 30, 40, 50 }
    local ctx = ContainerTape.open(payload, 16, 1)

    local status = ContainerTape:execute_all(ctx)
    TestCommon.assert_eq(status, TapeEngine.STATE_PROCEED, "Execution must PROCEED (+1)")
    TestCommon.assert_eq(ctx.c_container.occupancy, 5, "Container occupancy must be 5")
    TestCommon.assert_eq(ctx.c_container.capacity, 16, "Container capacity must be 16")
    TestCommon.assert(ctx.c_container.is_enclosed == true, "Container must be enclosed")
    TestCommon.assert(ctx.sealed_vessel ~= nil, "Sealed vessel must emerge from Roll 4")
    TestCommon.assert_eq(ctx.sealed_vessel.sealed, true, "Vessel must be sealed")
end

function Suite.test_container_overflow_tripwire()
    -- Create payload exceeding capacity limit
    local oversized = {}
    for i = 1, 10 do oversized[i] = i end

    local ctx = ContainerTape.open(oversized, 4, 1) -- Capacity is 4, payload is 10
    local status = ContainerTape:execute_all(ctx)

    TestCommon.assert_eq(status, TapeEngine.STATE_ROLLBACK, "Overflow must trigger STATE_ROLLBACK (-1)")
    TestCommon.assert_eq(ctx.tripwire, TapeEngine.TRIPWIRE_TRIPPED, "Tripwire must be TRIPPED on containment violation")
end

function Suite.test_container_lifecycle_rolls()
    local ctx = ContainerTape.open({ 1, 2, 3 }, 8, 1)
    TestCommon.assert_eq(ctx.tripwire, TapeEngine.TRIPWIRE_ARMED, "Tripwire must be ARMED on open")

    local status = ContainerTape:execute_all(ctx)
    TestCommon.assert_eq(status, TapeEngine.STATE_PROCEED, "Execution must PROCEED")

    local close_st, close_msg = ContainerTape.close(ctx)
    TestCommon.assert_eq(close_st, TapeEngine.STATE_PROCEED, "Close must seal in PROCEED (+1)")
    TestCommon.assert_eq(ctx.tripwire, TapeEngine.TRIPWIRE_DISARMED, "Tripwire must be DISARMED on close")
end

function Suite.run()
    print("[SUITE: Container Process Ribbon (Lua)]")
    local ok1 = TestCommon.run_test("Bounded Payload Encapsulation", Suite.test_container_encapsulation)
    local ok2 = TestCommon.run_test("Overflow Containment Tripwire & Rollback", Suite.test_container_overflow_tripwire)
    local ok3 = TestCommon.run_test("4-Block Lifecycle Rolls", Suite.test_container_lifecycle_rolls)
    return ok1 and ok2 and ok3
end

-- Allow standalone execution
if pcall(debug.getlocal, 4, 1) == false then
    local ok = Suite.run()
    os.exit(ok and 0 or 1)
end

return Suite
