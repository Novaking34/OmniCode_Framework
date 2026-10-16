/* ===========================================================================
 * OMNICODE FREESTANDING STANDARD LIBRARY (C MASTER UMBRELLA HEADER)
 * ===========================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Master header exposing all primitive invariant types and operational
 *   primitives for OmniCode bare-metal and hosted runtimes.
 *   - Category: Standard Library Track 2 C Transpile Variant
 *   - Invariant Types Included:
 *       < Base >      : Substrates, Balanced Trits, Quad-State & Connectives
 *       < Character > : Atomic Glyph / Letter Rune (1 Cell / 5 Trits)
 *       < Number >    : Scalar Quantity / Measure (1 Word / 32 Trits)
 *       < Container > : Generic Bounded Space Envelope / Slice
 *       < Site >      : Silicon Coordinate / Address (RAM @ vs MMIO ::@)
 *       < State >     : Quad-State Causal Truth & Loop Stepper
 * ========================================================================= */

#ifndef OMNI_H
#define OMNI_H

#include "omni_base.h"

/* Modular Type Substrates (Character & Number) */
#if __has_include("character.h")
#include "character.h"
#endif

#if __has_include("number.h")
#include "number.h"
#endif

/* Transitional Primitive Types */
#if __has_include("container.h")
#include "container.h"
#endif

#if __has_include("site.h")
#include "site.h"
#endif

#if __has_include("state.h")
#include "state.h"
#endif

#endif /* OMNI_H */
