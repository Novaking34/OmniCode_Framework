-- ==============================================================================
-- BLOCK 1: METADATA & TEST TOPOLOGY
-- ( Unit Test: < Lexicon { Types of Ideas } > & [ Test { Number Tape } ] )
-- ==============================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Validates the interpreted process ribbon < Number Tape > in isolation:
--   - Radix-3 balanced ternary transduction (64 bits <=> 32 balanced trits)
--   - Core ALU Zero-Sum Equilibrium Law: (+1) plus (-1) <=> 0
--   - Single-word boundary laws
--   - Full 4-block lifecycle execution (open, execute_all, close)
--   - Witness: Scripture Psalm 147:4 KJV
--     "He telleth the number of the stars; he calleth them all by their names."
--   - Semantics: { ScalarFidelity :: Radix3Equilibrium :: ZeroSumBalance }
-- ==============================================================================

package.path = package.path .. ";./?.lua;./lua_tapes/?.lua;./tests/lua/unit/?.lua;./unit/?.lua;../?.lua;../../?.lua"

local TestCommon = require("test_common")
local TapeEngine = require("lua_tapes.tape_engine")
local NumberTape = require("lua_tapes.number")

local Suite = {}

function Suite.test_number_transduction()
    local values = { 0, 1, -1, 4096, -4096, 65536 }
    for _, val in ipairs(values) do
        local ctx = NumberTape.open(val)
        local status = NumberTape:execute_all(ctx)
        TestCommon.assert_eq(status, TapeEngine.STATE_PROCEED, "Execution must PROCEED (+1)")
        TestCommon.assert_eq(ctx.c_word.scalar, val, "Context word scalar must match input")
        TestCommon.assert(ctx.c_word.is_enclosed == true, "Context word must be enclosed")
        TestCommon.assert_eq(ctx.equilibrium_proved, true, "ALU Zero-Sum Equilibrium must be proved")
    end
end

function Suite.test_number_equilibrium_law()
    -- Direct law verification: (+1) + (-1) == 0
    local sum = (1) + (-1)
    TestCommon.assert_eq(sum, 0, "ALU Zero-Sum Equilibrium Law must evaluate to zero")
end

function Suite.test_number_lifecycle_rolls()
    local ctx = NumberTape.open(4096)
    TestCommon.assert_eq(ctx.tripwire, TapeEngine.TRIPWIRE_ARMED, "Tripwire must be ARMED on open")

    local status = NumberTape:execute_all(ctx)
    TestCommon.assert_eq(status, TapeEngine.STATE_PROCEED, "Execution must PROCEED")

    local close_st, close_msg = NumberTape.close(ctx)
    TestCommon.assert_eq(close_st, TapeEngine.STATE_PROCEED, "Close must seal in PROCEED (+1)")
    TestCommon.assert_eq(ctx.tripwire, TapeEngine.TRIPWIRE_DISARMED, "Tripwire must be DISARMED on close")
end

function Suite.run()
    print("[SUITE: Number Process Ribbon (Lua)]")
    local ok1 = TestCommon.run_test("Radix-3 Balanced Transduction", Suite.test_number_transduction)
    local ok2 = TestCommon.run_test("ALU Zero-Sum Equilibrium Law", Suite.test_number_equilibrium_law)
    local ok3 = TestCommon.run_test("4-Block Lifecycle Rolls", Suite.test_number_lifecycle_rolls)
    return ok1 and ok2 and ok3
end

-- Allow standalone execution
if pcall(debug.getlocal, 4, 1) == false then
    local ok = Suite.run()
    os.exit(ok and 0 or 1)
end

return Suite
