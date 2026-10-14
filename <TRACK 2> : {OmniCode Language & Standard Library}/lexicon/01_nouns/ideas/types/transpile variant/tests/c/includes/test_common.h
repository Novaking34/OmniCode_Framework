/* ==============================================================================
 * BLOCK 1: METADATA & TEST HARNESS TOPOLOGY
 * ( Test Framework: < OmniCode Standard Library Testbed > )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Shared test harness for the OmniCode Standard Library C Transpile Variant.
 *   Provides structured assertions, test suite runner macros, and tripwire checks.
 *   - Witness: Scripture Proverbs 24:27 KJV
 *     "Prepare thy work without, and make it fit for thyself in the field;
 *      and afterwards build thine house."
 *   - Semantics: { AssertionInterlock :: GranularFaultIsolation :: MultiFileTestbed }
 * ============================================================================== */

#ifndef OMNI_TEST_COMMON_H
#define OMNI_TEST_COMMON_H

#include "omni.h"
#include <stdio.h>
#include <stdbool.h>

/* ==============================================================================
 * BLOCK 2: OPENING CONTEXT & MACRO DEFINITIONS
 * ============================================================================== */

/** Global test assertion failure counter */
extern int g_omni_test_failures;

/**
 * @brief Evaluates an assertion expression; logs failure and returns 1 on error.
 */
#define OMNI_TEST_ASSERT(expr, msg) \
    do { \
        if (!(expr)) { \
            printf("    [FAIL] %s:%d: %s (Expression: %s)\n", __FILE__, __LINE__, (msg), #expr); \
            g_omni_test_failures++; \
            return 1; \
        } \
    } while (0)

/**
 * @brief Runs an individual test function and reports PASS / FAIL.
 */
#define OMNI_TEST_RUN(fn, test_name) \
    do { \
        printf("    -> %s ... ", (test_name)); \
        if ((fn)() != 0) { \
            printf("[FAILED]\n"); \
            return 1; \
        } \
        printf("[PASS]\n"); \
    } while (0)

/* ==============================================================================
 * BLOCK 3: BODY PRIMITIVES & SUITE EXPORTS
 * ============================================================================== */

/** Unit test suite prototypes */
int test_character_suite(void);
int test_number_suite(void);
int test_container_suite(void);
int test_site_suite(void);
int test_state_suite(void);

#endif /* OMNI_TEST_COMMON_H */
