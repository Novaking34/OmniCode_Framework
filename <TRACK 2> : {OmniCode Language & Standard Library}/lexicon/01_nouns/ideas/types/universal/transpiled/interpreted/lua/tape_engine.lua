-- ===========================================================================
-- BLOCK 1: METADATA & ENGINE TOPOLOGY
-- ( Engine Core: < Lexicon { Types of Ideas } > & [ Process { Tape Engine } ] )
-- ===========================================================================
-- ARCHITECTURAL TOPOLOGY:
--   The interpreted execution ribbon companion to the OmniCode C Backbone.
--   Drives dynamic temporal causation, coroutines, and roll kinematics:
--   - Coroutine-based fiber stepping (yielding on AWAIT, resuming on tick)
--   - Pre-roll and post-roll dynamic event hooks
--   - State snapshotting and active rollback unwinding
--   - Structured handles over C memory layouts and invariants
--   - Witness: Scripture Proverbs 16:9 KJV
--     "A man's heart deviseth his way: but the LORD directeth his steps."
--   - Semantics: { TapeKinematics :: CoroutineFiber :: QuadStateDetents }
-- ===========================================================================

local TapeEngine = {
    -- Quad-State Truth Constants
    STATE_ROLLBACK = -1,     -- -1 : Retroactive Reversal / Error Unwind
    STATE_AWAIT    =  0,     --  0 : Equilibrium Standby / Hold Pending
    STATE_PROCEED  =  1,     -- +1 : Forward Causation / Affirmative Advance
    STATE_HALT     = "NULL", -- NULL : Terminal Detent / Stop Processor Clock

    -- Tripwire Interlock States
    TRIPWIRE_DISARMED = 0,
    TRIPWIRE_ARMED    = 1,
    TRIPWIRE_TRIPPED  = 2,

    -- Dual-Rail 2-Bit Trit Constants
    TRIT_PAIR_ZERO    = 0, -- 0b00: Equilibrium (0)
    TRIT_PAIR_POS     = 1, -- 0b01: Positive (+1)
    TRIT_PAIR_NEG     = 2, -- 0b10: Negative (-1)
    TRIT_PAIR_NULL    = 3, -- 0b11: Collision / NULL trap

    -- Dual-Rail Bitplane 64-Bit Masks
    BITPLANE_POS_MASK = 0x5555555555555555,
    BITPLANE_NEG_MASK = ~0x5555555555555555
}

-- Detect optional LuaJIT FFI for direct zero-copy C backbone linkage
local has_ffi, ffi = pcall(require, "ffi")
TapeEngine.has_ffi = has_ffi
TapeEngine.ffi = has_ffi and ffi or nil

---
-- Encodes a discrete balanced trit (-1, 0, 1) to a 2-bit pair.
function TapeEngine.trit_to_pair(trit)
    if trit == 0 then return TapeEngine.TRIT_PAIR_ZERO
    elseif trit == 1 then return TapeEngine.TRIT_PAIR_POS
    elseif trit == -1 then return TapeEngine.TRIT_PAIR_NEG
    else return TapeEngine.TRIT_PAIR_NULL end
end

---
-- Decodes a 2-bit pair into a discrete balanced trit.
-- Returns trit, ok_boolean (false on 0b11 collision).
function TapeEngine.pair_to_trit(code)
    local c = (code or 0) & 0x03
    if c == TapeEngine.TRIT_PAIR_ZERO then return 0, true
    elseif c == TapeEngine.TRIT_PAIR_POS then return 1, true
    elseif c == TapeEngine.TRIT_PAIR_NEG then return -1, true
    else return 0, false end
end

---
-- SIMD rail-swap negation of 64-bit packed dual-rail word.
function TapeEngine.negate_dual_rail(packed)
    local pos = packed & TapeEngine.BITPLANE_POS_MASK
    local neg = packed & TapeEngine.BITPLANE_NEG_MASK
    return ((pos << 1) & TapeEngine.BITPLANE_NEG_MASK) |
           ((neg >> 1) & TapeEngine.BITPLANE_POS_MASK)
end

---
-- Detects whether any trit position has both rails active (NULL trap).
function TapeEngine.has_null_collision(packed)
    local pos = packed & TapeEngine.BITPLANE_POS_MASK
    local neg = (packed >> 1) & TapeEngine.BITPLANE_POS_MASK
    return (pos & neg) ~= 0
end

-- ===========================================================================
-- BLOCK 2: OPENING BLOCK
-- ( Subsystem Initialization, Preconditions & Tape Mounting )
-- ===========================================================================

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
    -- Registers a rollback handler invoked when a guard or roll fails.
    function tape:on_rollback(fn)
        table.insert(self.hooks.on_rollback, fn)
        return self
    end

    ---
    -- Spawns a coroutine fiber driving step-by-step execution.
    function tape:spawn_coroutine(ctx)
        return coroutine.create(function()
            for idx, roll in ipairs(self.rolls) do
                table.insert(self.snapshots, make_snapshot(ctx))

                for _, h in ipairs(self.hooks.before_roll) do
                    h(idx, roll.name, ctx)
                end

                local status = roll.action(ctx)
                ctx.status = status

                for _, h in ipairs(self.hooks.after_roll) do
                    h(idx, roll.name, ctx, status)
                end

                table.insert(self.history, {
                    roll_index = idx,
                    name       = roll.name,
                    status     = status
                })

                if status == TapeEngine.STATE_ROLLBACK then
                    for _, h in ipairs(self.hooks.on_rollback) do
                        h(idx, roll.name, ctx)
                    end
                    if #self.snapshots > 0 then
                        local last_good = table.remove(self.snapshots)
                        for k, v in pairs(last_good) do
                            ctx[k] = v
                        end
                    end
                    coroutine.yield(TapeEngine.STATE_ROLLBACK, idx, roll.name)
                    return TapeEngine.STATE_ROLLBACK
                elseif status == TapeEngine.STATE_AWAIT then
                    coroutine.yield("AWAIT", idx, roll.name, ctx)
                elseif status == TapeEngine.STATE_HALT then
                    coroutine.yield(TapeEngine.STATE_HALT, idx, roll.name)
                    return TapeEngine.STATE_HALT
                end
            end

            return TapeEngine.STATE_PROCEED, "Tape sealed in PROCEED (+1)"
        end)
    end

    ---
    -- Executes all rolls in a synchronous loop, automatically resuming AWAITs.
    function tape:execute_all(ctx)
        local co = self:spawn_coroutine(ctx)
        while coroutine.status(co) ~= "dead" do
            local ok, event, roll_idx = coroutine.resume(co)
            if not ok then
                error("Tape execution fatal error: " .. tostring(event))
            end
            if event == "AWAIT" then
                ctx.status = TapeEngine.STATE_PROCEED
            elseif event == TapeEngine.STATE_PROCEED
                or event == TapeEngine.STATE_HALT
                or event == TapeEngine.STATE_ROLLBACK then
                return event, roll_idx
            end
        end
        return ctx.status
    end

    return tape
end

-- ===========================================================================
-- BLOCK 3: BODY BLOCK
-- ( Connectives & Evaluators )
-- ===========================================================================

---
-- Evaluates adversative caveat gate: A ! B (A executes, BUT B must hold).
function TapeEngine.op_but(action, caveat)
    if caveat == TapeEngine.STATE_ROLLBACK then
        return TapeEngine.STATE_ROLLBACK
    end
    if caveat == TapeEngine.STATE_HALT then
        return TapeEngine.STATE_HALT
    end
    return action
end

---
-- Evaluates cumulative conjunction: A :: B (A AND B simultaneous).
function TapeEngine.op_and(a, b)
    if a == TapeEngine.STATE_HALT or b == TapeEngine.STATE_HALT then
        return TapeEngine.STATE_HALT
    end
    if a == TapeEngine.STATE_ROLLBACK or b == TapeEngine.STATE_ROLLBACK then
        return TapeEngine.STATE_ROLLBACK
    end
    if a == TapeEngine.STATE_AWAIT or b == TapeEngine.STATE_AWAIT then
        return TapeEngine.STATE_AWAIT
    end
    return TapeEngine.STATE_PROCEED
end

---
-- Evaluates disjunctive alternative valve: A | B (A OR B).
function TapeEngine.op_or(a, b)
    if a == TapeEngine.STATE_PROCEED or b == TapeEngine.STATE_PROCEED then
        return TapeEngine.STATE_PROCEED
    end
    if a == TapeEngine.STATE_AWAIT or b == TapeEngine.STATE_AWAIT then
        return TapeEngine.STATE_AWAIT
    end
    if a == TapeEngine.STATE_ROLLBACK and b == TapeEngine.STATE_ROLLBACK then
        return TapeEngine.STATE_ROLLBACK
    end
    return TapeEngine.STATE_HALT
end

---
-- Evaluates balanced ternary truth inversion: -- A (NOT A).
function TapeEngine.op_not(a)
    if a == TapeEngine.STATE_PROCEED then
        return TapeEngine.STATE_ROLLBACK
    end
    if a == TapeEngine.STATE_ROLLBACK then
        return TapeEngine.STATE_PROCEED
    end
    return a
end

---
-- Evaluates inequality verification: A != B (A not to be B).
function TapeEngine.op_neq(a, b)
    if a == TapeEngine.STATE_HALT or b == TapeEngine.STATE_HALT then
        return TapeEngine.STATE_HALT
    end
    return (a ~= b) and TapeEngine.STATE_PROCEED or TapeEngine.STATE_ROLLBACK
end

---
-- Evaluates predicate is-not check: A =! B (State A is not target B).
function TapeEngine.op_is_not(a, target)
    return (a ~= target) and TapeEngine.STATE_PROCEED
                         or TapeEngine.STATE_ROLLBACK
end

---
-- Evaluates liken to / compatibility match: A =~ B.
function TapeEngine.op_liken(a, b)
    if a == TapeEngine.STATE_HALT or b == TapeEngine.STATE_HALT then
        return TapeEngine.STATE_HALT
    end
    if a == b then return TapeEngine.STATE_PROCEED end
    if (a == TapeEngine.STATE_PROCEED or a == TapeEngine.STATE_AWAIT) and
       (b == TapeEngine.STATE_PROCEED or b == TapeEngine.STATE_AWAIT) then
        return TapeEngine.STATE_PROCEED
    end
    return TapeEngine.STATE_ROLLBACK
end

---
-- Evaluates relation projection valve: Context : Aspect.
function TapeEngine.op_relate(ctx, asp)
    if ctx == TapeEngine.STATE_HALT then return TapeEngine.STATE_HALT end
    if ctx == TapeEngine.STATE_ROLLBACK then
        return TapeEngine.STATE_ROLLBACK
    end
    return asp
end

---
-- Determines if a value represents an OmniCode causal state.
function TapeEngine.is_state(val)
    return val == TapeEngine.STATE_PROCEED or
           val == TapeEngine.STATE_AWAIT   or
           val == TapeEngine.STATE_ROLLBACK or
           val == TapeEngine.STATE_HALT
end

---
-- Disambiguates an intended literal from a quad-state detent.
-- Handles air-quoted "0" or cast { 0 } / [ 0 ] / < 0 > table envelopes.
-- Returns (numeric_value, is_literal_bool).
function TapeEngine.resolve_literal(val)
    if type(val) == "string" and (val == "0" or val == "1" or val == "-1") then
        return tonumber(val), true
    elseif type(val) == "table" and val.is_literal then
        return val.value, true
    elseif type(val) == "table" and val[1] ~= nil and #val == 1 then
        return val[1], true
    elseif type(val) == "number" then
        return val, false
    else
        return val, false
    end
end

-- ===========================================================================
-- BLOCK 4: CLOSING BLOCK
-- ( Integrity Seals & Commit Verification )
-- ===========================================================================

---
-- Evaluates an invariant guard condition inside a roll action.
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
function TapeEngine.verify_and_commit(ctx)
    if ctx.tripwire == TapeEngine.TRIPWIRE_TRIPPED
        or ctx.status == TapeEngine.STATE_ROLLBACK then
        return TapeEngine.STATE_ROLLBACK, "Commit rejected: tripwire tripped"
    end
    ctx.tripwire = TapeEngine.TRIPWIRE_DISARMED
    return TapeEngine.STATE_PROCEED, "Context sealed in PROCEED (+1)"
end

return TapeEngine
