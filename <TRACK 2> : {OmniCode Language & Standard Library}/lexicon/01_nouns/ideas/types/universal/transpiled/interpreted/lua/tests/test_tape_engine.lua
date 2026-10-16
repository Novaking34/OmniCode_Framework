-- ===========================================================================
-- BLOCK 1: METADATA & TEST TOPOLOGY
-- ( Unit Test: < Lexicon { Types of Ideas } > & [ Test { Tape Engine } ] )
-- ===========================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Validates the universal Tape Engine, coroutine fibers, rollback snapshots,
--   and Quad-State connectives in isolation BEFORE being consumed by tapes.
--   - Witness: Scripture Proverbs 24:27 KJV
--     "Prepare thy work without, and make it fit for thyself in the field;
--      and afterwards build thine house."
--   - Semantics: { TapeEngineAxioms :: CoroutineFiber :: Connectives }
-- ===========================================================================

local script_dir = debug.getinfo(1, "S").source:match("@?(.*[/\\])") or "./"
package.path = script_dir .. "?.lua;" ..
               script_dir .. "../?.lua;" ..
               package.path

local TestCommon = require("test_common")
local TapeEngine = require("tape_engine")

local Suite = {}

function Suite.test_tape_creation()
    local tape = TapeEngine.create_tape({
        metadata = { who = "Kernel", what = "Test" },
        constants = { LIMIT = 10 },
        rolls = {}
    })
    TestCommon.assert(tape ~= nil, "Tape must be instantiated")
    TestCommon.assert_eq(tape.metadata.who, "Kernel", "Metadata must match")
    TestCommon.assert_eq(tape.constants.LIMIT, 10, "Constants must match")
end

function Suite.test_roll_execution_and_hooks()
    local pre_count = 0
    local post_count = 0

    local tape = TapeEngine.create_tape({
        rolls = {
            {
                name = "Step1",
                action = function(ctx)
                    ctx.val = (ctx.val or 0) + 5
                    return TapeEngine.STATE_PROCEED
                end
            },
            {
                name = "Step2",
                action = function(ctx)
                    ctx.val = ctx.val * 2
                    return TapeEngine.STATE_PROCEED
                end
            }
        }
    })

    tape:before(function() pre_count = pre_count + 1 end)
    tape:after(function() post_count = post_count + 1 end)

    local ctx = { val = 10 }
    local status = tape:execute_all(ctx)
    TestCommon.assert_eq(status, TapeEngine.STATE_PROCEED, "Execution PROCEED")
    TestCommon.assert_eq(ctx.val, 30, "Calculated value: (10 + 5) * 2 = 30")
    TestCommon.assert_eq(pre_count, 2, "2 pre-roll hooks fired")
    TestCommon.assert_eq(post_count, 2, "2 post-roll hooks fired")
end

function Suite.test_rollback_and_snapshots()
    local rollback_fired = false

    local tape = TapeEngine.create_tape({
        rolls = {
            {
                name = "Step1",
                action = function(ctx)
                    ctx.count = 100
                    return TapeEngine.STATE_PROCEED
                end
            },
            {
                name = "FaultyStep",
                action = function(ctx)
                    ctx.count = 999
                    return TapeEngine.STATE_ROLLBACK
                end
            }
        }
    })

    tape:on_rollback(function() rollback_fired = true end)

    local ctx = { count = 0 }
    local status = tape:execute_all(ctx)
    TestCommon.assert_eq(status, TapeEngine.STATE_ROLLBACK, "Yields ROLLBACK")
    TestCommon.assert(rollback_fired, "Rollback handler must be invoked")
    TestCommon.assert_eq(ctx.count, 100, "Context restored to last snapshot")
end

function Suite.test_dual_rail_substrate()
    TestCommon.assert_eq(TapeEngine.trit_to_pair(0), 0, "0 -> pair 0b00")
    TestCommon.assert_eq(TapeEngine.trit_to_pair(1), 1, "1 -> pair 0b01")
    TestCommon.assert_eq(TapeEngine.trit_to_pair(-1), 2, "-1 -> pair 0b10")

    local t0, ok0 = TapeEngine.pair_to_trit(0)
    TestCommon.assert(ok0 and t0 == 0, "0b00 decodes to 0")

    local t1, ok1 = TapeEngine.pair_to_trit(1)
    TestCommon.assert(ok1 and t1 == 1, "0b01 decodes to +1")

    local t2, ok2 = TapeEngine.pair_to_trit(2)
    TestCommon.assert(ok2 and t2 == -1, "0b10 decodes to -1")

    local _, ok3 = TapeEngine.pair_to_trit(3)
    TestCommon.assert(not ok3, "0b11 detects collision fault")

    -- SIMD rail swap negation
    local val = 1 -- bit 0 (positive rail)
    local neg = TapeEngine.negate_dual_rail(val)
    TestCommon.assert_eq(neg, 2, "Positive rail swapped to negative rail")
    local neg_neg = TapeEngine.negate_dual_rail(neg)
    TestCommon.assert_eq(neg_neg, val, "Double rail negation is identity")

    -- Collision detection
    TestCommon.assert(not TapeEngine.has_null_collision(val), "No collision")
    TestCommon.assert(TapeEngine.has_null_collision(3), "3 has collision")
end

function Suite.test_connective_operators()
    -- BUT (!)
    TestCommon.assert_eq(
        TapeEngine.op_but(TapeEngine.STATE_PROCEED, TapeEngine.STATE_PROCEED),
        TapeEngine.STATE_PROCEED, "PROCEED ! PROCEED == PROCEED"
    )
    TestCommon.assert_eq(
        TapeEngine.op_but(TapeEngine.STATE_PROCEED, TapeEngine.STATE_ROLLBACK),
        TapeEngine.STATE_ROLLBACK, "PROCEED ! ROLLBACK == ROLLBACK"
    )

    -- AND (::)
    TestCommon.assert_eq(
        TapeEngine.op_and(TapeEngine.STATE_PROCEED, TapeEngine.STATE_PROCEED),
        TapeEngine.STATE_PROCEED, "PROCEED :: PROCEED == PROCEED"
    )
    TestCommon.assert_eq(
        TapeEngine.op_and(TapeEngine.STATE_PROCEED, TapeEngine.STATE_AWAIT),
        TapeEngine.STATE_AWAIT, "PROCEED :: AWAIT == AWAIT"
    )

    -- OR (|)
    TestCommon.assert_eq(
        TapeEngine.op_or(TapeEngine.STATE_PROCEED, TapeEngine.STATE_ROLLBACK),
        TapeEngine.STATE_PROCEED, "PROCEED | ROLLBACK == PROCEED"
    )
    TestCommon.assert_eq(
        TapeEngine.op_or(TapeEngine.STATE_ROLLBACK, TapeEngine.STATE_PROCEED),
        TapeEngine.STATE_PROCEED, "ROLLBACK | PROCEED == PROCEED"
    )

    -- NOT (--)
    TestCommon.assert_eq(
        TapeEngine.op_not(TapeEngine.STATE_PROCEED),
        TapeEngine.STATE_ROLLBACK, "-- PROCEED == ROLLBACK"
    )
    TestCommon.assert_eq(
        TapeEngine.op_not(TapeEngine.STATE_ROLLBACK),
        TapeEngine.STATE_PROCEED, "-- ROLLBACK == PROCEED"
    )

    -- NEQ (!=)
    TestCommon.assert_eq(
        TapeEngine.op_neq(TapeEngine.STATE_PROCEED, TapeEngine.STATE_ROLLBACK),
        TapeEngine.STATE_PROCEED, "PROCEED != ROLLBACK == PROCEED"
    )
    TestCommon.assert_eq(
        TapeEngine.op_neq(TapeEngine.STATE_PROCEED, TapeEngine.STATE_PROCEED),
        TapeEngine.STATE_ROLLBACK, "PROCEED != PROCEED == ROLLBACK"
    )

    -- IS_NOT (=!)
    TestCommon.assert_eq(
        TapeEngine.op_is_not(TapeEngine.STATE_AWAIT,
                             TapeEngine.STATE_ROLLBACK),
        TapeEngine.STATE_PROCEED, "AWAIT =! ROLLBACK == PROCEED"
    )

    -- LIKEN (=~)
    TestCommon.assert_eq(
        TapeEngine.op_liken(TapeEngine.STATE_PROCEED, TapeEngine.STATE_AWAIT),
        TapeEngine.STATE_PROCEED, "PROCEED =~ AWAIT == PROCEED"
    )

    -- RELATE (:)
    TestCommon.assert_eq(
        TapeEngine.op_relate(TapeEngine.STATE_PROCEED, TapeEngine.STATE_AWAIT),
        TapeEngine.STATE_AWAIT, "PROCEED : AWAIT == AWAIT"
    )
    TestCommon.assert_eq(
        TapeEngine.op_relate(TapeEngine.STATE_ROLLBACK,
                             TapeEngine.STATE_PROCEED),
        TapeEngine.STATE_ROLLBACK, "ROLLBACK : PROCEED == ROLLBACK"
    )
end

function Suite.test_literal_disambiguation()
    TestCommon.assert(TapeEngine.is_state(TapeEngine.STATE_PROCEED), "State +1")
    TestCommon.assert(TapeEngine.is_state(TapeEngine.STATE_AWAIT), "State 0")
    TestCommon.assert(TapeEngine.is_state(TapeEngine.STATE_ROLLBACK),
                      "State -1")
    TestCommon.assert(TapeEngine.is_state(TapeEngine.STATE_HALT), "State NULL")
    TestCommon.assert(not TapeEngine.is_state(42), "42 is not a state")

    -- Air quotes "0" literal resolution
    local v0, is_lit0 = TapeEngine.resolve_literal("0")
    TestCommon.assert(is_lit0 and v0 == 0, "Air quote 0 resolves to literal 0")

    -- Cast { 0 } table literal resolution
    local vt, is_litt = TapeEngine.resolve_literal({ 0 })
    TestCommon.assert(is_litt and vt == 0, "{ 0 } resolves to literal 0")

    -- Plain state 0 is not a literal
    local vs, is_lits = TapeEngine.resolve_literal(TapeEngine.STATE_AWAIT)
    TestCommon.assert((not is_lits) and vs == 0, "STATE_AWAIT is not literal")
end

print("===============================================================")
print("   OMNICODE LUA UNIVERSAL TAPE ENGINE UNIT TESTBED")
print("===============================================================")
TestCommon.run_test("Tape Instantiation & Topology", Suite.test_tape_creation)
TestCommon.run_test("Roll Execution & Event Hooks",
                    Suite.test_roll_execution_and_hooks)
TestCommon.run_test("Transactional Rollback & Snapshots",
                    Suite.test_rollback_and_snapshots)
TestCommon.run_test("Dual-Rail Polar Substrate & SIMD",
                    Suite.test_dual_rail_substrate)
TestCommon.run_test("Quad-State Connective Operators",
                    Suite.test_connective_operators)
TestCommon.run_test("State vs Literal Disambiguation",
                    Suite.test_literal_disambiguation)

if TestCommon.failures > 0 then
    io.stderr:write(string.format("[!] %d test(s) failed\n",
                                  TestCommon.failures))
    os.exit(1)
end
print("---------------------------------------------------------------")
print("[√] All Universal Tape Engine tests verified and passed!")
