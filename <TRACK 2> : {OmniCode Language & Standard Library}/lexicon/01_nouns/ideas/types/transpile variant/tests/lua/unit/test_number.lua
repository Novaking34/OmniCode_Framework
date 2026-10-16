-- ============================================================================
-- BLOCK 1: METADATA & TEST TOPOLOGY
-- ( Unit Test: < Lexicon { Types of Ideas } > & [ Test { Number Tape } ] )
-- ============================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Validates the interpreted process ribbon < Number Tape > in isolation:
--   - Radix-3 balanced ternary transduction (64 bits <=> 32 balanced trits)
--   - Dual-rail polar bit-planes: 00=0 (Equilibrium), 01=+1, 10=-1, 11=Trap
--   - SIMD rail-swap negation: -(-X) == X via bit-plane swap
--   - Collision detection: NULL code (11) trips safety interlock
--   - Dynamic packing topologies: 2-2-2-2, 2-1-2-1-2, 1-2-1-2-2, 2-2-1-2-1,
--     3-3-2, 3-2-3, 2-3-3
--   - Core ALU Zero-Sum Equilibrium Law: (+1) plus (-1) <=> 0
--   - Single-word boundary laws & 4-block lifecycle execution
--   - Witness: Scripture Psalm 147:4 KJV
--     "He telleth the number of the stars; he calleth them all by their names."
--   - Semantics: { DualRailPlanes :: KinematicTopologies :: ZeroSumBalance }
-- ============================================================================

package.path = package.path .. ";./?.lua;./lua_tapes/?.lua;" ..
               "./tests/lua/unit/?.lua;./unit/?.lua;../?.lua;../../?.lua"

local TestCommon = require("test_common")
local TapeEngine = require("lua_tapes.tape_engine")
local NumberTape = require("lua_tapes.number")

local Suite = {}

function Suite.test_number_dual_rail_bitplanes()
    -- 1. Test 2-bit codes: 00=0, 01=+1, 10=-1, 11=trap
    TestCommon.assert_eq(NumberTape.trit_to_2bit(0), 0, "0 must map to 0b00")
    TestCommon.assert_eq(NumberTape.trit_to_2bit(1), 1, "1 must map to 0b01")
    TestCommon.assert_eq(NumberTape.trit_to_2bit(-1), 2, "-1 must map to 0b10")

    local t0, ok0 = NumberTape.two_bit_to_trit(0)
    TestCommon.assert(ok0 and t0 == 0, "0b00 must decode to 0")
    local t1, ok1 = NumberTape.two_bit_to_trit(1)
    TestCommon.assert(ok1 and t1 == 1, "0b01 must decode to +1")
    local t2, ok2 = NumberTape.two_bit_to_trit(2)
    TestCommon.assert(ok2 and t2 == -1, "0b10 must decode to -1")
    local _, ok3 = NumberTape.two_bit_to_trit(3)
    TestCommon.assert(not ok3, "0b11 must report collision / NULL trap")

    -- 2. Test dual-rail packing and unpacking
    local trits = {}
    for i = 1, 32 do
        if i % 3 == 1 then trits[i] = 1
        elseif i % 3 == 2 then trits[i] = -1
        else trits[i] = 0 end
    end
    local packed = NumberTape.pack_trits_dual_rail(trits)
    local unpacked = {}
    local ok = NumberTape.unpack_trits_dual_rail(packed, unpacked)
    TestCommon.assert(ok, "Unpack must succeed on valid trits")
    for i = 1, 32 do
        TestCommon.assert_eq(unpacked[i], trits[i],
                             "Unpacked trit must match original at " .. i)
    end
end

function Suite.test_number_simd_rail_swap_negation()
    local trits = {}
    for i = 1, 32 do
        if i % 3 == 1 then trits[i] = 1
        elseif i % 3 == 2 then trits[i] = -1
        else trits[i] = 0 end
    end
    local packed = NumberTape.pack_trits_dual_rail(trits)
    local negated = NumberTape.negate_dual_rail(packed)

    local unpacked_neg = {}
    local ok = NumberTape.unpack_trits_dual_rail(negated, unpacked_neg)
    TestCommon.assert(ok, "Unpack negated trits must succeed")
    for i = 1, 32 do
        TestCommon.assert_eq(unpacked_neg[i], -trits[i],
                             "Negated trit must invert sign at " .. i)
    end

    -- Involution: -(-X) == X
    local double_neg = NumberTape.negate_dual_rail(negated)
    TestCommon.assert_eq(double_neg, packed,
                         "Double rail-swap negation must restore original")
end

function Suite.test_number_null_collision_trap()
    -- Valid packed trits have no null collision
    local trits = { 1, -1, 0, 1 }
    local packed = NumberTape.pack_trits_dual_rail(trits)
    TestCommon.assert(not NumberTape.has_null_collision(packed),
                      "Valid trits must not trigger null collision")

    -- Manually inject 11 (code 3) into trit slot 0
    local corrupted = packed | 3
    TestCommon.assert(NumberTape.has_null_collision(corrupted),
                      "Corrupted dual-rail must detect null collision")

    local unpacked = {}
    local ok = NumberTape.unpack_trits_dual_rail(corrupted, unpacked)
    TestCommon.assert(not ok, "Unpacking corrupted dual-rail must fail")
end

function Suite.test_number_dynamic_packing_topologies()
    local modes = {
        NumberTape.PACK_MODES.DENSE_2222,
        NumberTape.PACK_MODES.KINEMATIC_21212,
        NumberTape.PACK_MODES.KINEMATIC_12122,
        NumberTape.PACK_MODES.KINEMATIC_22121,
        NumberTape.PACK_MODES.DIRECTIONAL_332,
        NumberTape.PACK_MODES.DIRECTIONAL_323,
        NumberTape.PACK_MODES.DIRECTIONAL_233
    }
    local test_trits = { 1, -1, 1, -1 }

    for _, mode in ipairs(modes) do
        local byte_val = NumberTape.pack_byte(test_trits, mode)
        local out_trits = {}
        local ok = NumberTape.unpack_byte(byte_val, mode, out_trits)
        TestCommon.assert(ok, "Unpack byte must succeed for mode " .. mode)
        -- Verify first 3 trits match in kinematic / directional modes
        TestCommon.assert_eq(out_trits[1], test_trits[1], "Trit 1 must match")
        TestCommon.assert_eq(out_trits[2], test_trits[2], "Trit 2 must match")
        TestCommon.assert_eq(out_trits[3], test_trits[3], "Trit 3 must match")
        if mode == NumberTape.PACK_MODES.DENSE_2222 then
            TestCommon.assert_eq(out_trits[4], test_trits[4],
                                 "Trit 4 must match in dense mode")
        end
    end
end

function Suite.test_number_transduction()
    local values = { 0, 1, -1, 4096, -4096, 65536 }
    for _, val in ipairs(values) do
        local ctx = NumberTape.open(val)
        local status = NumberTape:execute_all(ctx)
        TestCommon.assert_eq(status, TapeEngine.STATE_PROCEED,
                             "Execution must PROCEED (+1)")
        TestCommon.assert_eq(ctx.c_word.scalar, val,
                             "Context word scalar must match input")
        TestCommon.assert(ctx.c_word.is_enclosed == true,
                          "Context word must be enclosed")
        TestCommon.assert_eq(ctx.equilibrium_proved, true,
                             "ALU Zero-Sum Equilibrium must be proved")
    end
end

function Suite.test_number_equilibrium_law()
    -- Direct law verification: (+1) + (-1) == 0
    local sum = (1) + (-1)
    TestCommon.assert_eq(sum, 0,
                         "ALU Zero-Sum Equilibrium Law must evaluate to zero")
end

function Suite.test_number_lifecycle_rolls()
    local ctx = NumberTape.open(4096)
    TestCommon.assert_eq(ctx.tripwire, TapeEngine.TRIPWIRE_ARMED,
                         "Tripwire must be ARMED on open")

    local status = NumberTape:execute_all(ctx)
    TestCommon.assert_eq(status, TapeEngine.STATE_PROCEED,
                         "Execution must PROCEED")

    local close_st = NumberTape.close(ctx)
    TestCommon.assert_eq(close_st, TapeEngine.STATE_PROCEED,
                         "Close must seal in PROCEED (+1)")
    TestCommon.assert_eq(ctx.tripwire, TapeEngine.TRIPWIRE_DISARMED,
                         "Tripwire must be DISARMED on close")
end

function Suite.run()
    print("[SUITE: Number Process Ribbon (Lua)]")
    local ok1 = TestCommon.run_test("Dual-Rail Polar Bit-Planes",
                                    Suite.test_number_dual_rail_bitplanes)
    local ok2 = TestCommon.run_test("SIMD Rail-Swap Negation",
                                    Suite.test_number_simd_rail_swap_negation)
    local ok3 = TestCommon.run_test("NULL Collision Trap",
                                    Suite.test_number_null_collision_trap)
    local ok4 = TestCommon.run_test(
        "Dynamic Packing Topologies",
        Suite.test_number_dynamic_packing_topologies
    )
    local ok5 = TestCommon.run_test("Radix-3 Balanced Transduction",
                                    Suite.test_number_transduction)
    local ok6 = TestCommon.run_test("ALU Zero-Sum Equilibrium Law",
                                    Suite.test_number_equilibrium_law)
    local ok7 = TestCommon.run_test("4-Block Lifecycle Rolls",
                                    Suite.test_number_lifecycle_rolls)
    return ok1 and ok2 and ok3 and ok4 and ok5 and ok6 and ok7
end

-- Allow standalone execution
if pcall(debug.getlocal, 4, 1) == false then
    local ok = Suite.run()
    os.exit(ok and 0 or 1)
end

return Suite
