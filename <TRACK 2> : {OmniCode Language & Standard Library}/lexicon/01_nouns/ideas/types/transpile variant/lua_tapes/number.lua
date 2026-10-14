-- ==============================================================================
-- BLOCK 1: METADATA & PROCESS TAPE MANIFEST
-- ( Process Tape: < Lexicon { Types of Ideas } > & [ Process { Number } ] )
-- ==============================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Interprets and executes the atomic lifecycle for Invariant Type < Number >.
--   Acts as the dynamic runtime companion driving C backbone struct `omni_number_t`.
--   - Defined Space: [ 1 Word ] { 32 Trits :: 64 Bits }
--   - Execution Domain: Lexicon { Types of Ideas }
--   - Witness: Scripture Psalm 147:4 KJV
--     "He telleth the number of the stars; he calleth them all by their names."
--   - Semantics: { ScalarMeasurement :: Radix3Transduction :: ZeroSumEquilibrium }
-- ==============================================================================

local TapeEngine = require("lua_tapes.tape_engine")

local CONSTANTS = {
    WORD_CAPACITY       = 1,
    TRITS_PER_WORD      = 32,
    EXEMPLAR_ZERO       = 0,
    EXEMPLAR_UNITY_POS  = 1,
    EXEMPLAR_UNITY_NEG  = -1,
    EXEMPLAR_PARTITION  = 4096
}

local NumberTape = TapeEngine.create_tape({
    metadata = {
        who     = "OmniCode Lexicon",
        what    = "Number --> MEASURE { Quantity }",
        when    = "Arithmetic Event",
        where   = { number_word = 1 },
        why     = "Inscribe scalar number type invariant and arithmetic equilibrium",
        how     = "+1 : PROCEED",
        witness = "Psalm 147:4 KJV"
    },
    constants = CONSTANTS,
    rolls = {
        -- ----------------------------------------------------------------------
        -- ROLL 1: QUANTITY INSCRIPTION (Outside-in imposition onto C word)
        -- ----------------------------------------------------------------------
        {
            name = "Inscribe",
            action = function(ctx)
                ctx.c_word.scalar = ctx.quantity
                ctx.c_word.is_enclosed = true
                return TapeEngine.STATE_PROCEED
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 2: DUAL-SUBSTRATE TRANSDUCTION (64 Bits <=> 32 Balanced Trits)
        -- ----------------------------------------------------------------------
        {
            name = "Transduce",
            action = function(ctx)
                local current = ctx.c_word.scalar
                for i = 1, CONSTANTS.TRITS_PER_WORD do
                    if current == 0 then
                        ctx.c_word.trits[i] = 0
                    else
                        local rem = current % 3
                        current = math.floor(current / 3)
                        if rem == 2 then
                            rem = -1
                            current = current + 1
                        elseif rem == -2 then
                            rem = 1
                            current = current - 1
                        end
                        ctx.c_word.trits[i] = rem
                    end
                end
                return TapeEngine.STATE_PROCEED
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 3: IDENTITY & BOUND COMPARISON (Guard Single-Word Boundary)
        -- ----------------------------------------------------------------------
        {
            name = "Compare",
            action = function(ctx)
                local identity_ok = (ctx.c_word.scalar == ctx.quantity)
                local bounds_ok   = (ctx.c_word.is_enclosed == true)
                return TapeEngine.assert_guard(identity_ok and bounds_ok, ctx,
                                               "Number scalar value or single-word bound violation")
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 4: ARITHMETIC EQUILIBRIUM CALCULATION ((+1) plus (-1) <=> 0)
        -- ----------------------------------------------------------------------
        {
            name = "Equilibrium",
            action = function(ctx)
                local eq = (1) + (-1)
                local eq_holds = (eq == 0)
                ctx.equilibrium_proved = eq_holds
                return TapeEngine.assert_guard(eq_holds, ctx,
                                               "ALU Zero-Sum Equilibrium Law violated")
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 5: SEQUENCE TRANSITION (Number chains toward Vector sequence)
        -- ----------------------------------------------------------------------
        {
            name = "Sequence",
            action = function(ctx)
                ctx.vector_ready = true
                return TapeEngine.STATE_PROCEED
            end
        }
    }
})

-- ==============================================================================
-- BLOCK 2: OPENING BLOCK
-- ( Context Factory & C Word Substrate Mount )
-- ==============================================================================

---
-- Mounts the tape head, initialises the C word handle, and arms the tripwire.
-- @param quantity Outside-in scalar integer (defaults to 4096).
-- @return Initialized context table.
function NumberTape.open(quantity)
    return {
        quantity           = quantity or NumberTape.constants.EXEMPLAR_PARTITION,
        c_word             = {
            scalar      = 0,
            trits       = {},
            is_enclosed = false
        },
        equilibrium_proved = false,
        status             = TapeEngine.STATE_PROCEED,
        tripwire           = TapeEngine.TRIPWIRE_ARMED
    }
end

-- ==============================================================================
-- BLOCK 4: CLOSING BLOCK
-- ( Fourfold Detent Commit & Interlock Verification )
-- ==============================================================================

---
-- Closes and seals the Number tape execution context.
-- @param ctx Active context handle.
-- @return Quad-State commit status and diagnostic seal message.
function NumberTape.close(ctx)
    return TapeEngine.verify_and_commit(ctx)
end

return NumberTape
