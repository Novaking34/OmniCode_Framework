-- ==============================================================================
-- BLOCK 1: METADATA & TEST TOPOLOGY
-- ( Unit Test: < Lexicon { Types of Ideas } > & [ Test { Site Tape } ] )
-- ==============================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Validates the interpreted process ribbon < Site Tape > in isolation:
--   - Silicon coordinate binding
--   - Hardware Routing Law (MMIO vs RAM coordinates)
--   - Null pointer address rejection and safety interlock rollback
--   - Memory anchor emergence
--   - Full 4-block lifecycle execution (open, execute_all, close)
--   - Witness: Scripture Genesis 28:16 KJV
--     "And Jacob awaked out of his sleep, and he said, Surely the Lord is in
--      this place; and I knew it not."
--   - Semantics: { SiliconCoordinates :: HardwareRoutingLaw :: NonNullInterlock }
-- ==============================================================================

package.path     = package.path .. ";./?.lua;./lua_tapes/?.lua;./tests/lua/unit/?.lua;./unit/?.lua;../?.lua;../../?.lua"

local TestCommon = require("test_common")
local TapeEngine = require("lua_tapes.tape_engine")
local SiteTape   = require("lua_tapes.site")

local Suite      = {}

function Suite.test_site_routing()
    -- VGA buffer -> MMIO
    local vga_ctx = SiteTape.open(SiteTape.constants.SITE_VGA)
    local vga_st = SiteTape:execute_all(vga_ctx)
    TestCommon.assert_eq(vga_st, TapeEngine.STATE_PROCEED, "VGA site must PROCEED")
    TestCommon.assert_eq(vga_ctx.c_site.routing, "HARDWARE_MMIO (::@)", "VGA must route to MMIO")
    TestCommon.assert(vga_ctx.memory_anchor.anchored == true, "Memory anchor must be set")

    -- COM1 serial port -> MMIO
    local com1_ctx = SiteTape.open(SiteTape.constants.SITE_COM1)
    local com1_st = SiteTape:execute_all(com1_ctx)
    TestCommon.assert_eq(com1_st, TapeEngine.STATE_PROCEED, "COM1 site must PROCEED")
    TestCommon.assert_eq(com1_ctx.c_site.routing, "HARDWARE_MMIO (::@)", "COM1 must route to MMIO")

    -- MBR origin -> RAM
    local mbr_ctx = SiteTape.open(SiteTape.constants.SITE_MBR)
    local mbr_st = SiteTape:execute_all(mbr_ctx)
    TestCommon.assert_eq(mbr_st, TapeEngine.STATE_PROCEED, "MBR site must PROCEED")
    TestCommon.assert_eq(mbr_ctx.c_site.routing, "PHYSICAL_RAM (@)", "MBR must route to RAM")
end

function Suite.test_site_null_tripwire()
    local null_ctx = SiteTape.open(0x00000000)
    local status = SiteTape:execute_all(null_ctx)

    TestCommon.assert_eq(status, TapeEngine.STATE_ROLLBACK, "Null coordinate must trigger STATE_ROLLBACK (-1)")
    TestCommon.assert_eq(null_ctx.tripwire, TapeEngine.TRIPWIRE_TRIPPED, "Tripwire must be TRIPPED on null address")
end

function Suite.test_site_lifecycle_rolls()
    local ctx = SiteTape.open(SiteTape.constants.SITE_VGA)
    TestCommon.assert_eq(ctx.tripwire, TapeEngine.TRIPWIRE_ARMED, "Tripwire must be ARMED on open")

    local status = SiteTape:execute_all(ctx)
    TestCommon.assert_eq(status, TapeEngine.STATE_PROCEED, "Execution must PROCEED")

    local close_st, close_msg = SiteTape.close(ctx)
    TestCommon.assert_eq(close_st, TapeEngine.STATE_PROCEED, "Close must seal in PROCEED (+1)")
    TestCommon.assert_eq(ctx.tripwire, TapeEngine.TRIPWIRE_DISARMED, "Tripwire must be DISARMED on close")
end

function Suite.run()
    print("[SUITE: Site Process Ribbon (Lua)]")
    local ok1 = TestCommon.run_test("Hardware Routing Law (MMIO vs RAM)", Suite.test_site_routing)
    local ok2 = TestCommon.run_test("Null Address Rejection & Interlock Rollback", Suite.test_site_null_tripwire)
    local ok3 = TestCommon.run_test("4-Block Lifecycle Rolls", Suite.test_site_lifecycle_rolls)
    return ok1 and ok2 and ok3
end

-- Allow standalone execution
if pcall(debug.getlocal, 4, 1) == false then
    local ok = Suite.run()
    os.exit(ok and 0 or 1)
end

return Suite
