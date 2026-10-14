/* ==============================================================================
 * BLOCK 1: METADATA & INVARIANT TYPE DEFINITION
 * ( Invariant Type: < Idea { Type } : Site > )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Defines physical memory addresses, hardware ports, and MMIO coordinates in the
 *   OmniCode Standard Library Track 2 C Transpile Variant.
 *   - Base Block: Primitive Invariant Type
 *   - Category: Idea { Type }
 *   - Footprint: [ 1 Address ] { 64-Bit Pointer / Physical Silicon Coordinate }
 *   - Ladder Ascension: Builds up with Numbers (/-^)
 *   - System: OmniCode { Lexicon }
 *   - Witness: Scripture Genesis 28:16 KJV
 *     "And Jacob awaked out of his sleep, and he said, Surely the Lord is in
 *      this place; and I knew it not."
 *   - Semantics: { PhysicalAddress :: MMIOPort :: SiliconCoordinate :: RoutingLaw }
 * ============================================================================== */

#ifndef OMNI_SITE_H
#define OMNI_SITE_H

#include "omni_base.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ------------------------------------------------------------------------------
 * 1. HARDWARE ROUTING DOMAIN ENUM: omni_site_routing_t
 * ------------------------------------------------------------------------------ */

/**
 * @brief Hardware Routing Law domain classification.
 * 
 * Law { Routing }:
 *   - RAM Physical (@)   : Standard cacheable RAM memory pointer.
 *   - MMIO Hardware (::@) : Memory-mapped I/O port requiring volatile access.
 */
typedef enum {
    OMNI_ROUTING_RAM_PHYSICAL  = 0, /**< '@' Physical system RAM address */
    OMNI_ROUTING_HARDWARE_MMIO = 1  /**< '::@' Hardware MMIO device register or port */
} omni_site_routing_t;

/* ------------------------------------------------------------------------------
 * 2. FREESTANDING INVARIANT TYPE STRUCTURE: omni_site_t
 * ------------------------------------------------------------------------------ */

/**
 * @brief Invariant Type < Site >: The physical silicon address coordinate.
 * 
 * Encloses the 64-bit silicon address pointer and its hardware routing domain:
 *   - Strictly non-null when valid (0x0 is mapped to unmapped detent).
 *   - Resolves routing bijectively between RAM and device hardware ports.
 * 
 * Core Invariant Law:
 *   ENCLOSE [ Site ] WITHIN [ 1 Address ]
 *   LAW { Routing } : [ Physical RAM @ ] | [ Hardware MMIO ::@ ]
 */
typedef struct {
    uintptr_t           coordinate;                     /**< Physical 64-bit silicon address pointer */
    omni_site_routing_t routing;                        /**< Hardware routing classification (RAM vs MMIO) */
    bool                is_enclosed;                    /**< Invariant boundary enclosure guarantee */
} omni_site_t;

/* ------------------------------------------------------------------------------
 * 3. 4-BLOCK PIPELINE CONTEXT: omni_site_context_t
 * ------------------------------------------------------------------------------ */

/**
 * @brief Execution context for Site address binding and routing lifecycle rolls.
 */
typedef struct {
    uintptr_t             input_coordinate;             /**< Outside-in target silicon coordinate */
    omni_site_t           site;                         /**< Concrete single-address allocation */
    omni_quad_state_t     status;                       /**< Active Quad-State truth */
    omni_tripwire_state_t tripwire;                     /**< Subsystem safety interlock */
} omni_site_context_t;

/* ==============================================================================
 * BLOCK 2: OPENING BLOCK
 * ( Outside-in Imposition & Context Initialization )
 * ============================================================================== */

/**
 * @brief Initializes the Site context and binds the target coordinate.
 * 
 * @param ctx Pointer to caller-allocated site context.
 * @param input_coord Target silicon address (defaults to VGA buffer if 0).
 */
void omni_site_open(omni_site_context_t *ctx, uintptr_t input_coord);

/* ==============================================================================
 * BLOCK 3: BODY BLOCK
 * ( Operational Primitives, Routing Laws & Memory Access )
 * ============================================================================== */

/**
 * @brief Constructs a Site directly from a 64-bit address pointer,
 *        automatically resolving its hardware routing classification.
 * 
 * @param address Physical silicon memory or MMIO address.
 * @return Fully initialized omni_site_t.
 */
omni_site_t omni_site_from_address(uintptr_t address);

/**
 * @brief Hardware Routing Law: Distinguishes physical RAM pointers from
 *        hardware MMIO registers and serial ports.
 * 
 * @param address Physical target address.
 * @return OMNI_ROUTING_HARDWARE_MMIO if within device port ranges; RAM otherwise.
 */
omni_site_routing_t omni_site_resolve_routing(uintptr_t address);

/**
 * @brief Calculates an offset Site from a base Site coordinate.
 * 
 * @param site Base reference site pointer.
 * @param offset Signed byte displacement (+offset or -offset).
 * @return New displaced omni_site_t coordinate.
 */
omni_site_t omni_site_offset(const omni_site_t *site, intptr_t offset);

/**
 * @brief Verifies whether a site coordinate is valid and non-null.
 * 
 * @param site Pointer to site to evaluate.
 * @return true if site is valid, enclosed, and non-null; false otherwise.
 */
bool omni_site_is_valid(const omni_site_t *site);

/**
 * @brief Evaluates relational equality between two sites.
 * 
 * @param a First site operand.
 * @param b Second site operand.
 * @return true if addresses and routing domains match; false otherwise.
 */
bool omni_site_eq(const omni_site_t *a, const omni_site_t *b);

/**
 * @brief Safely reads an 8-bit byte from the site coordinate plus offset.
 *        Employs volatile memory semantics to prevent compiler cache elision on MMIO.
 * 
 * @param site Base site coordinate.
 * @param offset Byte displacement.
 * @return Read byte value (returns 0 if site is invalid/null).
 */
uint8_t omni_site_read_u8(const omni_site_t *site, size_t offset);

/**
 * @brief Safely writes an 8-bit byte to the site coordinate plus offset.
 *        Employs volatile memory semantics for bare-metal hardware registers.
 * 
 * @param site Base site coordinate.
 * @param offset Byte displacement.
 * @param value Byte value to inscribe.
 */
void omni_site_write_u8(const omni_site_t *site, size_t offset, uint8_t value);

/**
 * @brief Executes the full location lifecycle across declared rolls:
 *   - Roll 1: Locate address with numbers.
 *   - Roll 2: Bind coordinate and classify routing.
 *   - Roll 3: Guard non-null identity invariants.
 *   - Roll 4: Route and anchor memory target.
 * 
 * @param ctx Pointer to active site context.
 * @return OMNI_STATE_PROCEED on valid address; OMNI_STATE_ROLLBACK on null or error.
 */
omni_quad_state_t omni_site_execute(omni_site_context_t *ctx);

/* ==============================================================================
 * BLOCK 4: CLOSING BLOCK
 * ( Integrity Verification & Commit Status )
 * ============================================================================== */

/**
 * @brief Validates all invariants and seals the Site commit status.
 * 
 * @param ctx Pointer to active site context.
 * @return OMNI_STATE_PROCEED on successful seal; OMNI_STATE_ROLLBACK if tripped.
 */
omni_quad_state_t omni_site_close(omni_site_context_t *ctx);

#ifdef __cplusplus
}
#endif

#endif /* OMNI_SITE_H */
