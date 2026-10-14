-- ==============================================================================
-- BLOCK 1: METADATA & PROCESS TAPE MANIFEST
-- ( Process Tape: < Lexicon { Types of Ideas } > & [ Process { Site } ] )
-- ==============================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Interprets and executes the atomic lifecycle for Invariant Type < Site >.
--   Acts as the dynamic runtime companion driving C backbone struct `omni_site_t`.
--   - Defined Space: [ 1 Address ] { 64 Bits / Silicon Coordinate }
--   - Ladder Ascension: Build-up with Numbers (/-^)
--   - Witness: Scripture Genesis 28:16 KJV
--     "And Jacob awaked out of his sleep, and he said, Surely the Lord is in
--      this place; and I knew it not."
--   - Semantics: { SiliconCoordinate :: HardwareRoutingLaw :: NullPointerDetent }
-- ==============================================================================

local TapeEngine = require("lua_tapes.tape_engine")

local CONSTANTS = {
    SITE_NULL   = 0x00000000,
    SITE_MBR    = 0x00007C00,
    SITE_COM1   = 0x000003F8,
    SITE_VGA    = 0x000B8000
}

local SiteTape = TapeEngine.create_tape({
    metadata = {
        who     = "OmniCode Lexicon",
        what    = "Site --> LOCATE { PhysicalAddress }",
        when    = "Addressing Event",
        where   = { silicon_coordinate = 1 },
        why     = "Inscribe physical site address invariant and hardware MMIO coordinates",
        how     = "+1 : PROCEED",
        witness = "Genesis 28:16 KJV"
    },
    constants = CONSTANTS,
    rolls = {
        -- ----------------------------------------------------------------------
        -- ROLL 1: LOCATE (Ladder build-up with Numbers)
        -- ----------------------------------------------------------------------
        {
            name = "Locate",
            action = function(ctx)
                local not_null = (ctx.coordinate ~= CONSTANTS.SITE_NULL)
                if not_null then
                    ctx.c_site.is_enclosed = true
                end
                return TapeEngine.assert_guard(not_null, ctx,
                                               "Null pointer address cannot be located as a valid silicon site")
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 2: BIND COORDINATE (Inscribe address & resolve RAM vs MMIO routing)
        -- ----------------------------------------------------------------------
        {
            name = "BindCoordinate",
            action = function(ctx)
                ctx.c_site.coordinate = ctx.coordinate
                -- Hardware Routing Law: COM1, VGA, or >= 0xA0000 route to MMIO; otherwise RAM
                if ctx.coordinate == CONSTANTS.SITE_COM1 or 
                   ctx.coordinate == CONSTANTS.SITE_VGA  or 
                   ctx.coordinate >= 0x000A0000 then
                    ctx.c_site.routing = "HARDWARE_MMIO (::@)"
                else
                    ctx.c_site.routing = "PHYSICAL_RAM (@)"
                end
                return TapeEngine.STATE_PROCEED
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 3: IDENTITY & NULL GUARD (Guard Non-Null Validity)
        -- ----------------------------------------------------------------------
        {
            name = "GuardAddress",
            action = function(ctx)
                local identity_ok = (ctx.c_site.coordinate == ctx.coordinate)
                local valid_site  = (ctx.c_site.coordinate ~= CONSTANTS.SITE_NULL and ctx.c_site.is_enclosed)
                return TapeEngine.assert_guard(identity_ok and valid_site, ctx,
                                               "Site address failed non-null validation check")
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 4: ROUTE & PROJECT (Inside-out emergence)
        -- ----------------------------------------------------------------------
        {
            name = "Route",
            action = function(ctx)
                ctx.memory_anchor = {
                    coordinate = ctx.c_site.coordinate,
                    routing    = ctx.c_site.routing,
                    anchored   = true
                }
                return TapeEngine.STATE_PROCEED
            end
        },

        -- ----------------------------------------------------------------------
        -- ROLL 5: SEQUENCE TRANSITION (Site chains to Memory Map)
        -- ----------------------------------------------------------------------
        {
            name = "Sequence",
            action = function(ctx)
                ctx.memory_map_ready = true
                return TapeEngine.STATE_PROCEED
            end
        }
    }
})

-- ==============================================================================
-- BLOCK 2: OPENING BLOCK
-- ( Context Factory & C Site Substrate Mount )
-- ==============================================================================

---
-- Mounts the tape head, initialises the C site handle, and arms the tripwire.
-- @param coordinate Outside-in 64-bit silicon address (defaults to VGA buffer).
-- @return Initialized context table.
function SiteTape.open(coordinate)
    return {
        coordinate = coordinate or CONSTANTS.SITE_VGA,
        c_site     = {
            coordinate  = 0,
            routing     = "UNRESOLVED",
            is_enclosed = false
        },
        status     = TapeEngine.STATE_PROCEED,
        tripwire   = TapeEngine.TRIPWIRE_ARMED
    }
end

-- ==============================================================================
-- BLOCK 4: CLOSING BLOCK
-- ( Fourfold Detent Commit & Interlock Verification )
-- ==============================================================================

---
-- Closes and seals the Site tape execution context.
-- @param ctx Active context handle.
-- @return Quad-State commit status and diagnostic seal message.
function SiteTape.close(ctx)
    return TapeEngine.verify_and_commit(ctx)
end

return SiteTape
