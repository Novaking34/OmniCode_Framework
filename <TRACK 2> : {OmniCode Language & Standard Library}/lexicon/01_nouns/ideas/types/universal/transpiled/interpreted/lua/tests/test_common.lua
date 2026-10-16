-- ===========================================================================
-- BLOCK 1: METADATA & TEST HARNESS TOPOLOGY
-- ( Test Framework: < OmniCode Standard Library Universal Lua Testbed > )
-- ===========================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Universal test harness for OmniCode Standard Library Lua Process Ribbons.
--   Provides structured assertions, test suite runner helpers, and tripwires.
--   - Witness: Scripture Proverbs 24:27 KJV
--     "Prepare thy work without, and make it fit for thyself in the field;
--      and afterwards build thine house."
--   - Semantics: { AssertionInterlock :: GranularFaultIsolation }
-- ===========================================================================

local TestCommon = {
    failures = 0,
    tests_run = 0
}

function TestCommon.assert(condition, msg)
    TestCommon.tests_run = TestCommon.tests_run + 1
    if not condition then
        TestCommon.failures = TestCommon.failures + 1
        error("    [FAIL] Assertion failed: " .. tostring(msg), 2)
    end
end

function TestCommon.assert_eq(actual, expected, msg)
    TestCommon.tests_run = TestCommon.tests_run + 1
    if actual ~= expected then
        TestCommon.failures = TestCommon.failures + 1
        error(string.format("    [FAIL] %s: Expected [%s], got [%s]",
                            tostring(msg), tostring(expected),
                            tostring(actual)), 2)
    end
end

function TestCommon.run_test(name, fn)
    io.write(string.format("    -> %s ... ", name))
    local ok, err = pcall(fn)
    if ok then
        print("[PASS]")
        return true
    else
        print("[FAILED]")
        print(tostring(err))
        return false
    end
end

return TestCommon
