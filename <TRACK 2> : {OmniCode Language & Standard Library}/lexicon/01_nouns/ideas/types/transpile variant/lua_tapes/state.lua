-- ==============================================================================
-- BLOCK 1: METADATA & PROCESS TAPE MANIFEST
-- ( Process Tape: < Lexicon { Types of Ideas } > & [ Process { State } ] )
-- ==============================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Interprets and executes the atomic lifecycle for Invariant Type < State >.
--   Acts as the dynamic runtime companion driving C backbone struct `omni_state_t`.
--   - Defined Space: [ 1 Trit ] { 2 Bits / Quad-State Causal Register }
--   - Named Registers: [ STATE TURN BOUND IT ]
--   - Witness: Scripture Ecclesiastes 3:1 KJV
--     "To every thing there is a season, and a time to every purpose under
--      the heaven:"
--   - Semantics: { QuadStateTruth :: CausalClock :: CanonicalLoopLaw }
-- ==============================================================================

local TapeEngine = require("lua_tapes.tape_engine")

local CONSTANTS = {
    STATE_PROCEED  = TapeEngine.STATE_PROCEED,
    STATE_AWAIT    = TapeEngine.STATE_AWAIT,
    STATE_ROLLBACK = TapeEngine.STATE_ROLLBACK,
    STATE_HALT     = TapeEngine.STATE_HALT,
    DEFAULT_BOUND  = 4
}

local StateTape = TapeEngine.create_tape({
    metadata = {
        who     = "OmniCode Lexicon",
        what    = "State --> GOVERN { CausalTruth }",
        when    = "Control Event",
        where   = { state_register = 1 },
        why     = "Inscribe quad-state truth invariant and execute canonical loops",
        how     = "+1 : PROCEED",
        witness = "Ecclesiastes 3:1 KJV"
    },
    constants = CONSTANTS,
    rolls = {
        -- ----------------------------------------------------------------------
        -- ROLL 1: INSCRIBE STATE REGISTER
        -- ----------------------------------------------------------------------
        {
            name = "Inscribe",
            action = function(ctx)
                ctx.c_state.registers.state = ctx.signal
                ctx.c_state.is_enclosed = true
                return TapeEngine.STATE_PROCEED
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 2: CAVEAT GUARD AGAINST DISCREPANCY
        -- ----------------------------------------------------------------------
        {
            name = "GuardSignal",
            action = function(ctx)
                local signal_match = (ctx.c_state.registers.state == ctx.signal)
                return TapeEngine.assert_guard(signal_match, ctx,
                                               "State register truth signal discrepancy")
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 3: CANONICAL LOOP STEPPER (Iterative Quad-State Cadence)
        -- ----------------------------------------------------------------------
        {
            name = "LoopCadence",
            action = function(ctx)
                local regs = ctx.c_state.registers
                while regs.state == TapeEngine.STATE_PROCEED do
                    if regs.it < regs.bound then
                        regs.it = regs.it + 1
                        regs.turn = regs.turn + 1
                        regs.state = TapeEngine.STATE_PROCEED
                    elseif regs.it > regs.bound then
                        regs.state = TapeEngine.STATE_AWAIT
                        break
                    else
                        -- Terminal Detent reached: IT == BOUND
                        regs.state = TapeEngine.STATE_HALT
                        break
                    end
                end
                return regs.state
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 4: GOVERN ENGINE (Inside-out emergence to machine clock)
        -- ----------------------------------------------------------------------
        {
            name = "GovernClock",
            action = function(ctx)
                ctx.clock_active = (ctx.c_state.registers.state ~= TapeEngine.STATE_HALT)
                return TapeEngine.STATE_PROCEED
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 5: SEQUENCE TRANSITION (State chains to continuous Timeline)
        -- ----------------------------------------------------------------------
        {
            name = "Sequence",
            action = function(ctx)
                ctx.timeline_ready = true
                return TapeEngine.STATE_PROCEED
            end
        }
    }
})

-- ==============================================================================
-- BLOCK 2: OPENING BLOCK
-- ( Context Factory & C State Substrate Mount )
-- ==============================================================================

---
-- Mounts the tape head, initialises the C state register bank handle, and arms the tripwire.
-- @param signal Outside-in truth signal (defaults to PROCEED).
-- @param bound Maximum iteration threshold (defaults to 4).
-- @return Initialized context table.
function StateTape.open(signal, bound)
    local sig = signal or CONSTANTS.STATE_PROCEED
    local bnd = bound or CONSTANTS.DEFAULT_BOUND
    return {
        signal   = sig,
        c_state  = {
            substrate_trit = 1,
            registers = {
                state = sig,
                turn  = 0,
                bound = bnd,
                it    = 0
            },
            is_enclosed = false
        },
        status   = TapeEngine.STATE_PROCEED,
        tripwire = TapeEngine.TRIPWIRE_ARMED
    }
end

-- ==============================================================================
-- BLOCK 3: OPERATIONAL PRIMITIVES
-- ( Single-Step Loop Law Execution )
-- ==============================================================================

---
-- Single-steps the Canonical Loop Law across named registers:
--   LOOP [ WHEN [ { IT } < { BOUND } ] --> [ IT` ] PROCEED to { TURN }
--          ! [ { IT } > { BOUND } ] AWAIT for [ IT` ]
--          :: HALT WHEN [ IT` ] = NULL ]
-- @param ctx Context handle holding register bank.
-- @return New Quad-State condition after the single step.
function StateTape.step_loop(ctx)
    local regs = ctx.c_state.registers
    if regs.it < regs.bound then
        regs.it = regs.it + 1
        regs.turn = regs.turn + 1
        regs.state = TapeEngine.STATE_PROCEED
        return TapeEngine.STATE_PROCEED
    elseif regs.it > regs.bound then
        regs.state = TapeEngine.STATE_AWAIT
        return TapeEngine.STATE_AWAIT
    else
        regs.state = TapeEngine.STATE_HALT
        return TapeEngine.STATE_HALT
    end
end

-- ==============================================================================
-- BLOCK 4: CLOSING BLOCK
-- ( Fourfold Detent Commit & Interlock Verification )
-- ==============================================================================

---
-- Closes and seals the State tape execution context.
-- @param ctx Active context handle.
-- @return Quad-State commit status and diagnostic seal message.
function StateTape.close(ctx)
    return TapeEngine.verify_and_commit(ctx)
end

return StateTape
