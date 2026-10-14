-- ==============================================================================
-- BLOCK 1: METADATA & PROCESS TAPE MANIFEST
-- ( Process Tape: < Lexicon { Types of Ideas } > & [ Process { Character } ] )
-- ==============================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Interprets and executes the atomic lifecycle for Invariant Type < Character >.
--   Acts as the dynamic runtime companion driving C backbone struct `omni_character_t`.
--   - Defined Space: [ 1 Cell ] { 5 Trits :: 1 Byte }
--   - Execution Domain: Lexicon { Types of Ideas }
--   - Witness: Scripture Exodus 31:18 KJV
--     "And he gave unto Moses, when he had made an end of communing with him upon
--      mount Sinai, two tables of testimony, tables of stone, written with the
--      finger of God."
--   - Semantics: { GlyphInscription :: DualSubstrateTransduction :: SingleCellBound }
-- ==============================================================================

local TapeEngine = require("lua_tapes.tape_engine")

local CONSTANTS = {
    CELL_CAPACITY       = 1,
    TRITS_PER_CELL      = 5,
    EXEMPLAR_GLYPH_A    = string.byte('A')
}

local CharacterTape = TapeEngine.create_tape({
    metadata = {
        who     = "OmniCode Lexicon",
        what    = "Character --> INSCRIBE { Glyph }",
        when    = "Input Event",
        where   = { character_cell = 1 },
        why     = "Inscribe atomic character type invariant into defined space",
        how     = "+1 : PROCEED",
        witness = "Exodus 31:18 KJV"
    },
    constants = CONSTANTS,
    rolls = {
        -- ----------------------------------------------------------------------
        -- ROLL 1: GLYPH INSCRIPTION (Outside-in imposition onto C cell)
        -- ----------------------------------------------------------------------
        {
            name = "Inscribe",
            action = function(ctx)
                ctx.c_cell.glyph = ctx.glyph
                ctx.c_cell.is_enclosed = true
                return TapeEngine.STATE_PROCEED
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 2: DUAL-SUBSTRATE TRANSDUCTION (8 Bits <=> 5 Balanced Trits)
        -- ----------------------------------------------------------------------
        {
            name = "Transduce",
            action = function(ctx)
                local val = ctx.c_cell.glyph
                for i = 1, CONSTANTS.TRITS_PER_CELL do
                    local rem = val % 3
                    val = math.floor(val / 3)
                    if rem == 2 then
                        rem = -1
                        val = val + 1
                    elseif rem == -2 then
                        rem = 1
                        val = val - 1
                    end
                    ctx.c_cell.trits[i] = rem
                end
                return TapeEngine.STATE_PROCEED
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 3: IDENTITY & BOUND COMPARISON (Guard Single-Cell Boundary)
        -- ----------------------------------------------------------------------
        {
            name = "Compare",
            action = function(ctx)
                local identity_ok = (ctx.c_cell.glyph == ctx.glyph)
                local bounds_ok   = (ctx.c_cell.is_enclosed == true)
                return TapeEngine.assert_guard(identity_ok and bounds_ok, ctx,
                                               "Character glyph or single-cell bound violation")
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 4: TARGET PROJECTION (Inside-out emergence to output bus/terminal)
        -- ----------------------------------------------------------------------
        {
            name = "Project",
            action = function(ctx)
                if ctx.target then
                    table.insert(ctx.target, ctx.c_cell.glyph)
                end
                return TapeEngine.STATE_PROCEED
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 5: SEQUENCE TRANSITION (Character chains toward continuous Text ribbon)
        -- ----------------------------------------------------------------------
        {
            name = "Sequence",
            action = function(ctx)
                ctx.sequence_ready = true
                return TapeEngine.STATE_PROCEED
            end
        }
    }
})

-- ==============================================================================
-- BLOCK 2: OPENING BLOCK
-- ( Context Factory & C Cell Substrate Mount )
-- ==============================================================================

---
-- Mounts the tape head, initialises the C cell handle, and arms the tripwire.
-- @param glyph Outside-in byte payload (defaults to ASCII 'A').
-- @param target_buffer Optional destination array (terminal or VGA buffer).
-- @return Initialized context table.
function CharacterTape.open(glyph, target_buffer)
    return {
        glyph    = glyph or CharacterTape.constants.EXEMPLAR_GLYPH_A,
        target   = target_buffer or {},
        c_cell   = {
            glyph       = 0,
            trits       = {0, 0, 0, 0, 0},
            is_enclosed = false
        },
        status   = TapeEngine.STATE_PROCEED,
        tripwire = TapeEngine.TRIPWIRE_ARMED
    }
end

-- ==============================================================================
-- BLOCK 4: CLOSING BLOCK
-- ( Fourfold Detent Commit & Interlock Verification )
-- ==============================================================================

---
-- Closes and seals the Character tape execution context.
-- @param ctx Active context handle.
-- @return Quad-State commit status and diagnostic seal message.
function CharacterTape.close(ctx)
    return TapeEngine.verify_and_commit(ctx)
end

return CharacterTape
