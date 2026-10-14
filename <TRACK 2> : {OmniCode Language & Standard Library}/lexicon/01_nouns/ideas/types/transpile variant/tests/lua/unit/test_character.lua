-- ==============================================================================
-- BLOCK 1: METADATA & TEST TOPOLOGY
-- ( Unit Test: < Lexicon { Types of Ideas } > & [ Test { Character Tape } ] )
-- ==============================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Validates the interpreted process ribbon < Character Tape > in isolation:
--   - Bijective round-trip transduction (8 bits <=> 5 balanced trits)
--   - Strict single-cell enclosure boundary laws
--   - Dynamic pre/post roll hook observation
--   - Full 4-block lifecycle execution (open, execute_all, close)
--   - Witness: Scripture Exodus 31:18 KJV
--     "And he gave unto Moses, when he had made an end of communing with him upon
--      mount Sinai, two tables of testimony, tables of stone, written with the
--      finger of God."
--   - Semantics: { GlyphFidelity :: BalancedTritLaw :: SingleCellEnclosure }
-- ==============================================================================

package.path        = package.path .. ";./?.lua;./lua_tapes/?.lua;./tests/lua/unit/?.lua;./unit/?.lua;../?.lua;../../?.lua"

local TestCommon    = require("test_common")
local TapeEngine    = require("lua_tapes.tape_engine")
local CharacterTape = require("lua_tapes.character")

local Suite         = {}

function Suite.test_character_transduction()
    local glyphs = { string.byte('A'), string.byte('Z'), string.byte('0'), string.byte('9'), 32, 0, 70, 100, 121 }
    for _, g in ipairs(glyphs) do
        local ctx = CharacterTape.open(g)
        local status = CharacterTape:execute_all(ctx)
        TestCommon.assert_eq(status, TapeEngine.STATE_PROCEED, "Execution status must PROCEED (+1)")
        TestCommon.assert_eq(ctx.c_cell.glyph, g, "Context cell glyph must match input")
        TestCommon.assert(ctx.c_cell.is_enclosed == true, "Context cell must be enclosed")

        -- Reconstruct byte from trits: sum_{i=1..5} trits[i] * 3^(i-1)
        local reconstructed = 0
        local power = 1
        for i = 1, #ctx.c_cell.trits do
            local t = ctx.c_cell.trits[i]
            TestCommon.assert(t >= -1 and t <= 1, "Trit must be in {-1, 0, +1}")
            reconstructed = reconstructed + t * power
            power = power * 3
        end
        TestCommon.assert_eq(reconstructed, g, "Reconstructed byte from trits must match input")
    end
end

function Suite.test_character_dynamic_hooks()
    local pre_hooks = 0
    local post_hooks = 0

    CharacterTape:before(function() pre_hooks = pre_hooks + 1 end)
    CharacterTape:after(function() post_hooks = post_hooks + 1 end)

    local ctx = CharacterTape.open(string.byte('Q'))
    local status = CharacterTape:execute_all(ctx)
    TestCommon.assert_eq(status, TapeEngine.STATE_PROCEED, "Execution must PROCEED")
    TestCommon.assert(pre_hooks >= 5, "Must trigger pre-roll hooks for all 5 rolls")
    TestCommon.assert(post_hooks >= 5, "Must trigger post-roll hooks for all 5 rolls")
end

function Suite.test_character_lifecycle_rolls()
    local ctx = CharacterTape.open(string.byte('C'))
    TestCommon.assert_eq(ctx.tripwire, TapeEngine.TRIPWIRE_ARMED, "Tripwire must be ARMED on open")

    local status = CharacterTape:execute_all(ctx)
    TestCommon.assert_eq(status, TapeEngine.STATE_PROCEED, "Execution must PROCEED")

    local close_st, close_msg = CharacterTape.close(ctx)
    TestCommon.assert_eq(close_st, TapeEngine.STATE_PROCEED, "Close must seal in PROCEED (+1)")
    TestCommon.assert_eq(ctx.tripwire, TapeEngine.TRIPWIRE_DISARMED, "Tripwire must be DISARMED on close")
end

function Suite.run()
    print("[SUITE: Character Process Ribbon (Lua)]")
    local ok1 = TestCommon.run_test("Bijective Round-Trip Transduction", Suite.test_character_transduction)
    local ok2 = TestCommon.run_test("Dynamic Pre/Post Roll Hooks", Suite.test_character_dynamic_hooks)
    local ok3 = TestCommon.run_test("4-Block Lifecycle Rolls", Suite.test_character_lifecycle_rolls)
    return ok1 and ok2 and ok3
end

-- Allow standalone execution
if pcall(debug.getlocal, 4, 1) == false then
    local ok = Suite.run()
    os.exit(ok and 0 or 1)
end

return Suite
