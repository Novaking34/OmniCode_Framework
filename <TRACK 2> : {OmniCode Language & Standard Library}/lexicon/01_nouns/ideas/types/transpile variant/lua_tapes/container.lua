-- ==============================================================================
-- BLOCK 1: METADATA & PROCESS TAPE MANIFEST
-- ( Process Tape: < Lexicon { Types of Ideas } > & [ Process { Container } ] )
-- ==============================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Interprets and executes the atomic lifecycle for Invariant Type < Container >.
--   Acts as the dynamic runtime companion driving generic C backbone struct `omni_container_t`.
--   - Defined Space: Generic Bounded Memory Envelope / Slice
--   - Ladder Ascension: Build-up with Numbers (/-^)
--   - Witness: Scripture 2 Kings 4:4 KJV
--     "And when thou art come in, thou shalt shut the door upon thee and upon
--      thy sons, and shalt pour out into all those vessels, and thou shalt set
--      aside that which is full."
--   - Semantics: { MemoryCapsule :: BoundedEnvelope :: ContainmentLaw }
-- ==============================================================================

local TapeEngine = require("lua_tapes.tape_engine")

local CONSTANTS = {
    EXEMPLAR_UNIT       = 1,
    EXEMPLAR_VESSEL     = 64,
    EXEMPLAR_STOREHOUSE = 4096
}

local ContainerTape = TapeEngine.create_tape({
    metadata = {
        who     = "OmniCode Lexicon",
        what    = "Container --> ENCAPSULATE { Payload }",
        when    = "Allocation Event",
        where   = { container_envelope = 64 },
        why     = "Inscribe bounded container type invariant and storage envelopes",
        how     = "+1 : PROCEED",
        witness = "2 Kings 4:4 KJV"
    },
    constants = CONSTANTS,
    rolls = {
        -- ----------------------------------------------------------------------
        -- ROLL 1: BUILD CAPSULE (Ladder build-up with Numbers)
        -- ----------------------------------------------------------------------
        {
            name = "BuildCapsule",
            action = function(ctx)
                local valid_cap = (ctx.c_container.capacity > 0)
                if valid_cap then
                    ctx.c_container.is_enclosed = true
                end
                return TapeEngine.assert_guard(valid_cap, ctx, "Container capacity must be greater than zero")
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 2: ENCAPSULATION (Inscribe payload into bounded C envelope)
        -- ----------------------------------------------------------------------
        {
            name = "Encapsulate",
            action = function(ctx)
                local items = ctx.payload
                local count = #items

                -- Law { Containment } Guard: Occupancy must never exceed capacity limit
                local within_capacity = (count <= ctx.c_container.capacity)
                if not within_capacity then
                    return TapeEngine.assert_guard(false, ctx, "Payload count exceeds bounded container capacity")
                end

                for i = 1, count do
                    ctx.c_container.buffer[i] = items[i]
                end
                ctx.c_container.occupancy = count
                return TapeEngine.STATE_PROCEED
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 3: IDENTITY & BOUND COMPARISON (Guard bounds under Law { Containment })
        -- ----------------------------------------------------------------------
        {
            name = "GuardBounds",
            action = function(ctx)
                local containment_valid = (ctx.c_container.occupancy <= ctx.c_container.capacity)
                local enclosed_valid    = (ctx.c_container.is_enclosed == true)
                return TapeEngine.assert_guard(containment_valid and enclosed_valid, ctx,
                                               "Core Containment Law violated: Occupancy > Capacity")
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 4: POUR OUT & SET ASIDE (Inside-out emergence)
        -- ----------------------------------------------------------------------
        {
            name = "PourOut",
            action = function(ctx)
                ctx.sealed_vessel = {
                    origin  = ctx.c_container.buffer,
                    target  = ctx.target,
                    sealed  = true
                }
                return TapeEngine.STATE_PROCEED
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 5: SEQUENCE TRANSITION (Container chains to composite structure)
        -- ----------------------------------------------------------------------
        {
            name = "Sequence",
            action = function(ctx)
                ctx.structure_ready = true
                return TapeEngine.STATE_PROCEED
            end
        }
    }
})

-- ==============================================================================
-- BLOCK 2: OPENING BLOCK
-- ( Context Factory & C Container Slice Substrate Mount )
-- ==============================================================================

---
-- Mounts the tape head, initialises the generic C container handle, and arms the tripwire.
-- @param payload Outside-in array of items (e.g. Character bytes or Numbers).
-- @param capacity Maximum number of elements (defaults to 64).
-- @param element_size Byte footprint of a single element (defaults to 1).
-- @return Initialized context table.
function ContainerTape.open(payload, capacity, element_size)
    local cap  = capacity or CONSTANTS.EXEMPLAR_VESSEL
    local e_sz = element_size or 1
    return {
        payload     = payload or {},
        target      = 0x00000000,
        c_container = {
            buffer       = {},
            element_size = e_sz,
            capacity     = cap,
            occupancy    = 0,
            is_enclosed  = false
        },
        status      = TapeEngine.STATE_PROCEED,
        tripwire    = TapeEngine.TRIPWIRE_ARMED
    }
end

-- ==============================================================================
-- BLOCK 4: CLOSING BLOCK
-- ( Fourfold Detent Commit & Interlock Verification )
-- ==============================================================================

---
-- Closes and seals the Container tape execution context.
-- @param ctx Active context handle.
-- @return Quad-State commit status and diagnostic seal message.
function ContainerTape.close(ctx)
    return TapeEngine.verify_and_commit(ctx)
end

return ContainerTape
