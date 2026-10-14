-- ==============================================================================
-- BLOCK 1: METADATA & TEST ORCHESTRATOR TOPOLOGY
-- ( Test Orchestrator: < Lexicon { Types of Ideas } > & [ Multi-File Testbed (Lua) ] )
-- ==============================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Primary test orchestrator for the OmniCode Standard Library Lua Process Ribbons.
--   Executes multi-file unit test suites followed by integrated cross-type pipelines:
--   1. Unit Suite: Character Tape (Exodus 31:18 KJV)
--   2. Unit Suite: Number Tape (Psalm 147:4 KJV)
--   3. Unit Suite: Container Tape (2 Kings 4:4 KJV)
--   4. Unit Suite: Site Tape (Genesis 28:16 KJV)
--   5. Unit Suite: State Tape (Ecclesiastes 3:1 KJV)
--   6. Safety Interlock & Transactional Rollback (Proverbs 16:9 KJV)
--   7. Coroutine Fiber Stepping & AWAIT Yield/Resume
--   8. Integrated Composition: Container<Character> Traversed via State Loop
--   - Witness: Scripture Proverbs 24:27 KJV
--     "Prepare thy work without, and make it fit for thyself in the field;
--      and afterwards build thine house."
--   - Semantics: { MultiFileTestbed :: CrossTypeIntegration :: DetentVerification }
-- ==============================================================================

package.path         = package.path .. ";./?.lua;./lua_tapes/?.lua;./tests/lua/unit/?.lua;./unit/?.lua;../unit/?.lua;../?.lua;../../?.lua"

local TestCommon     = require("test_common")
local TapeEngine     = require("lua_tapes.tape_engine")
local CharacterSuite = require("test_character")
local NumberSuite    = require("test_number")
local ContainerSuite = require("test_container")
local SiteSuite      = require("test_site")
local StateSuite     = require("test_state")

local CharacterTape  = require("lua_tapes.character")
local ContainerTape  = require("lua_tapes.container")
local SiteTape       = require("lua_tapes.site")
local StateTape      = require("lua_tapes.state")

print("==============================================================================")
print("   OMNICODE STANDARD LIBRARY LUA ENGINE: MULTI-FILE TEST HARNESS")
print("==============================================================================\n")

-- ------------------------------------------------------------------------------
-- 1. EXECUTE UNIT SUITES
-- ------------------------------------------------------------------------------
local suites = {
    CharacterSuite,
    NumberSuite,
    ContainerSuite,
    SiteSuite,
    StateSuite
}

for _, s in ipairs(suites) do
    local ok = s.run()
    if not ok then
        print(string.format("\n[ABORT] Unit suite failed! (%d failure(s))", TestCommon.failures))
        os.exit(1)
    end
    print("")
end

-- ------------------------------------------------------------------------------
-- 2. SAFETY INTERLOCK & TRANSACTIONAL ROLLBACK TEST
-- ------------------------------------------------------------------------------
print("[SUITE: Safety Interlock & Transactional Rollback]")
local rollback_caught = false
SiteTape:on_rollback(function(tape, roll_idx, roll_name, ctx, snapshot)
    rollback_caught = true
end)

local bad_site_ctx = SiteTape.open(0x00000000)
local bad_status, bad_msg = SiteTape:execute_all(bad_site_ctx)
TestCommon.assert_eq(bad_status, TapeEngine.STATE_ROLLBACK, "Expected STATE_ROLLBACK (-1)")
TestCommon.assert_eq(bad_site_ctx.tripwire, TapeEngine.TRIPWIRE_TRIPPED, "Expected TRIPWIRE_TRIPPED")
TestCommon.assert(rollback_caught == true, "Expected on_rollback hook to fire")
print(string.format("    -> Rollback Interlock: Status=%s, Tripwire=%d (%s) [PASS]\n",
    tostring(bad_status), bad_site_ctx.tripwire, bad_site_ctx.error_reason))

-- ------------------------------------------------------------------------------
-- 3. COROUTINE FIBER STEPPING & YIELD/RESUME TEST
-- ------------------------------------------------------------------------------
print("[SUITE: Coroutine Fiber Stepping & AWAIT Yield/Resume]")
local FiberTape = TapeEngine.create_tape({
    metadata = { name = "FiberSteppingTape" },
    rolls = {
        {
            name = "PhaseOne",
            action = function(ctx)
                ctx.counter = ctx.counter + 1
                return TapeEngine.STATE_AWAIT -- Request coroutine yield
            end
        },
        {
            name = "PhaseTwo",
            action = function(ctx)
                ctx.counter = ctx.counter * 10
                return TapeEngine.STATE_PROCEED
            end
        }
    }
})

local fiber_ctx = { counter = 5, status = TapeEngine.STATE_PROCEED, tripwire = TapeEngine.TRIPWIRE_ARMED }
local co = FiberTape:spawn_coroutine(fiber_ctx)

-- First resume: yields on PhaseOne AWAIT
local ok1, ev1, roll1, name1 = coroutine.resume(co)
TestCommon.assert(ok1 and ev1 == "AWAIT" and fiber_ctx.counter == 6, "Expected AWAIT yield at counter=6")
print(string.format("    -> Yielded on '%s' at roll %d [%s] (Counter=%d) [PASS]",
    ev1, roll1, name1, fiber_ctx.counter))

-- Reset status from AWAIT to PROCEED for phase two resume
fiber_ctx.status = TapeEngine.STATE_PROCEED
local ok2, ev2, msg2 = coroutine.resume(co)
TestCommon.assert(ok2 and ev2 == TapeEngine.STATE_PROCEED and fiber_ctx.counter == 60, "Expected PROCEED at counter=60")
print(string.format("    -> Resumed to Completion: Status=%s (Counter=%d, %s) [PASS]\n",
    tostring(ev2), fiber_ctx.counter, msg2))

-- ------------------------------------------------------------------------------
-- 4. CROSS-TYPE COMPOSITION PIPELINE
-- ------------------------------------------------------------------------------
print("[SUITE: Cross-Type Composition Pipeline]")
local word_bytes = {
    string.byte('O'), string.byte('M'), string.byte('N'), string.byte('I'),
    string.byte('C'), string.byte('O'), string.byte('D'), string.byte('E')
}
local container_ctx = ContainerTape.open(word_bytes, 32, 1)
local cont_status, cont_msg = ContainerTape:execute_all(container_ctx)
TestCommon.assert_eq(cont_status, TapeEngine.STATE_PROCEED, "Container execute must PROCEED")

io.write("    -> Driving Traversal via Canonical State Loop:\n       Transduced Output: \"")
local state_ctx = StateTape.open(TapeEngine.STATE_PROCEED, container_ctx.c_container.occupancy)

while state_ctx.c_state.registers.state == TapeEngine.STATE_PROCEED do
    local idx = state_ctx.c_state.registers.it + 1
    local ch = container_ctx.c_container.buffer[idx]
    if ch then
        io.write(string.char(ch))
    end
    StateTape.step_loop(state_ctx)
end
io.write("\"\n")

TestCommon.assert_eq(state_ctx.c_state.registers.state, TapeEngine.STATE_HALT, "State must reach HALT:NULL detent")
print(string.format("       Final Detent: %s (Detent at IT=%d, TURN=%d) [PASS]\n",
    tostring(state_ctx.c_state.registers.state),
    state_ctx.c_state.registers.it,
    state_ctx.c_state.registers.turn))

print("==============================================================================")
print("   [ALL LUA ENGINE TESTS PASS]: FULL 4-BLOCK PARITY SEALED IN PROCEED (+1)")
print("==============================================================================")
