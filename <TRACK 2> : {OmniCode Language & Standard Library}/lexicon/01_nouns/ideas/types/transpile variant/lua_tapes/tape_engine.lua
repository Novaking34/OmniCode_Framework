-- ==============================================================================
-- BLOCK 1: METADATA & ENGINE TOPOLOGY
-- ( Engine Core: < Lexicon { Types of Ideas } > & [ Process { Tape Engine } ] )
-- ==============================================================================
-- ARCHITECTURAL TOPOLOGY:
--   The interpreted execution ribbon companion to the OmniCode C Backbone.
--   Drives the dynamic temporal causation, coroutines, and roll kinematics:
--   - Coroutine-based fiber stepping (yielding on AWAIT, resuming on tick)
--   - Pre-roll and post-roll dynamic event hooks
--   - State snapshotting and active rollback unwinding
--   - Structured handles over C memory layouts and invariants
--   - Witness: Scripture Proverbs 16:9 KJV
--     "A man's heart deviseth his way: but the LORD directeth his steps."
--   - Semantics: { TapeKinematics :: CoroutineFiber :: RollHooks :: QuadStateDetents }
-- ==============================================================================

local TapeEngine = {
    -- Quad-State Truth Constants
    STATE_ROLLBACK = -1,   -- -1 : Retroactive Reversal / Error Unwind
    STATE_AWAIT    =  0,   --  0 : Equilibrium Standby / Hold Pending
    STATE_PROCEED  =  1,   -- +1 : Forward Causation / Affirmative Advancement
    STATE_HALT     = "NULL",-- NULL : Terminal Detent / Stop Processor Clock

    -- Tripwire Interlock States
    TRIPWIRE_DISARMED = 0,
    TRIPWIRE_ARMED    = 1,
    TRIPWIRE_TRIPPED  = 2
}

-- Detect optional LuaJIT FFI for direct zero-copy C backbone linkage
local has_ffi, ffi = pcall(require, "ffi")
TapeEngine.has_ffi = has_ffi
TapeEngine.ffi = has_ffi and ffi or nil

-- ==============================================================================
-- BLOCK 2: OPENING BLOCK
-- ( Subsystem Initialization, Preconditions & Tape Mounting )
-- ==============================================================================

---
-- Deep copy snapshot utility for transactional rollback safety.
-- @param ctx The context table to clone.
-- @return Independent deep copy of the context.
local function make_snapshot(ctx)
    local snap = {}
    for k, v in pairs(ctx) do
        if type(v) == "table" then
            snap[k] = make_snapshot(v)
        else
            snap[k] = v
        end
    end
    return snap
end

---
-- Creates and mounts an interpreted Tape Process Ribbon.
-- @param spec Table containing metadata, constants, and declared rolls.
-- @return Mounted tape object.
function TapeEngine.create_tape(spec)
    local tape = {
        metadata    = spec.metadata or {},
        constants   = spec.constants or {},
        rolls       = spec.rolls or {},
        hooks       = {
            before_roll = {},
            after_roll  = {},
            on_rollback = {}
        },
        snapshots   = {},
        history     = {}
    }

    ---
    -- Registers a dynamic hook invoked immediately before a roll executes.
    function tape:before(fn)
        table.insert(self.hooks.before_roll, fn)
        return self
    end

    ---
    -- Registers a dynamic hook invoked immediately after a roll executes.
    function tape:after(fn)
        table.insert(self.hooks.after_roll, fn)
        return self
    end

    ---
    -- Registers a rollback handler invoked when a roll trips the interlock.
    function tape:on_rollback(fn)
        table.insert(self.hooks.on_rollback, fn)
        return self
    end

    ---
    -- Restores a previously captured execution snapshot into the active context.
    function tape:restore_snapshot(ctx, roll_index)
        local snap = self.snapshots[roll_index]
        if snap then
            for k, v in pairs(snap) do
                ctx[k] = v
            end
        end
    end

-- ==============================================================================
-- BLOCK 3: BODY BLOCK
-- ( Coroutine Kinematics, Roll Stepping & Hook Dispatch )
-- ==============================================================================

    ---
    -- Spawns a coroutine fiber that steps through the declared rolls.
    -- Yields on AWAIT (0), halts on HALT (NULL), and unwinds on ROLLBACK (-1).
    -- @param ctx Context handle holding the C struct proxy and tripwires.
    -- @return Lua coroutine object.
    function tape:spawn_coroutine(ctx)
        return coroutine.create(function()
            ctx.status = TapeEngine.STATE_PROCEED
            ctx.tripwire = TapeEngine.TRIPWIRE_ARMED

            for i, roll in ipairs(self.rolls) do
                -- 1. Snapshot state prior to roll execution for transaction rollback
                self.snapshots[i] = make_snapshot(ctx)

                -- 2. Dispatch pre-roll hooks
                for _, h in ipairs(self.hooks.before_roll) do
                    h(self, i, roll.name, ctx)
                end

                -- 3. Execute the declared roll action
                local status = roll.action(ctx)
                ctx.status = status or ctx.status

                -- 4. Dispatch post-roll hooks
                for _, h in ipairs(self.hooks.after_roll) do
                    h(self, i, roll.name, ctx, ctx.status)
                end

                -- Record execution trace
                table.insert(self.history, { roll = i, name = roll.name, status = ctx.status })

                -- 5. Quad-State transition evaluation
                if ctx.status == TapeEngine.STATE_AWAIT then
                    -- Yield execution back to host/game loop until external condition resolves
                    coroutine.yield("AWAIT", i, roll.name, ctx)
                elseif ctx.status == TapeEngine.STATE_ROLLBACK then
                    -- Invariant violation: trip safety interlock and trigger rollback unwinding
                    ctx.tripwire = TapeEngine.TRIPWIRE_TRIPPED
                    for _, h in ipairs(self.hooks.on_rollback) do
                        h(self, i, roll.name, ctx, self.snapshots[i])
                    end
                    return TapeEngine.STATE_ROLLBACK, "Tape rolled back at roll " .. i .. " [" .. roll.name .. "]"
                elseif ctx.status == TapeEngine.STATE_HALT then
                    -- Terminal Detent reached
                    return TapeEngine.STATE_HALT, "Tape reached terminal detent (HALT:NULL)"
                end
            end

            ctx.tripwire = TapeEngine.TRIPWIRE_DISARMED
            return TapeEngine.STATE_PROCEED, "Tape sealed in PROCEED (+1)"
        end)
    end

    ---
    -- Executes all rolls in a synchronous loop, automatically resuming AWAITs.
    -- @param ctx Context handle.
    -- @return Final Quad-State status code and descriptive seal message.
    function tape:execute_all(ctx)
        local co = self:spawn_coroutine(ctx)
        while coroutine.status(co) ~= "dead" do
            local ok, event, roll_idx, roll_name, curr_ctx = coroutine.resume(co)
            if not ok then
                error("Tape execution fatal error: " .. tostring(event))
            end
            if event == "AWAIT" then
                -- In synchronous batch mode, acknowledge standby and resume
                curr_ctx.status = TapeEngine.STATE_PROCEED
            elseif event == TapeEngine.STATE_PROCEED or event == TapeEngine.STATE_HALT or event == TapeEngine.STATE_ROLLBACK then
                return event, roll_idx
            end
        end
        return ctx.status
    end

    return tape
end

-- ==============================================================================
-- BLOCK 4: CLOSING BLOCK
-- ( Integrity Seals & Commit Verification )
-- ==============================================================================

---
-- Evaluates an invariant guard condition inside a roll action.
-- Trips the tripwire and triggers ROLLBACK on condition failure.
-- @param condition Boolean expression to verify.
-- @param ctx Context handle.
-- @param error_reason Message recorded upon tripwire trigger.
-- @return STATE_PROCEED if condition is true; STATE_ROLLBACK otherwise.
function TapeEngine.assert_guard(condition, ctx, error_reason)
    if condition then
        return TapeEngine.STATE_PROCEED
    else
        ctx.tripwire = TapeEngine.TRIPWIRE_TRIPPED
        ctx.error_reason = error_reason or "Invariant guard violation"
        return TapeEngine.STATE_ROLLBACK
    end
end

---
-- Verifies the final integrity of a context before closing commit.
-- @param ctx Context handle.
-- @return STATE_PROCEED if clean; STATE_ROLLBACK if tripped.
function TapeEngine.verify_and_commit(ctx)
    if ctx.tripwire == TapeEngine.TRIPWIRE_TRIPPED or ctx.status == TapeEngine.STATE_ROLLBACK then
        return TapeEngine.STATE_ROLLBACK, "Commit rejected: tripwire tripped"
    end
    ctx.tripwire = TapeEngine.TRIPWIRE_DISARMED
    return TapeEngine.STATE_PROCEED, "Context sealed in PROCEED (+1)"
end

return TapeEngine
