/* ==============================================================================
 * BLOCK 1: METADATA & INVARIANT IMPLEMENTATION
 * ( Invariant Type: < Idea { Type } : Site > )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Implements physical memory addresses, hardware ports, and MMIO coordinates in
 *   the OmniCode Standard Library Track 2 C Transpile Variant.
 *   - Base Block: Primitive Invariant Type
 *   - Category: Idea { Type }
 *   - Footprint: [ 1 Address ] { 64-Bit Pointer / Physical Silicon Coordinate }
 *   - Ladder Ascension: Builds up with Numbers (/-^)
 *   - Witness: Scripture Genesis 28:16 KJV
 *     "And Jacob awaked out of his sleep, and he said, Surely the Lord is in
 *      this place; and I knew it not."
 *   - Semantics: { PhysicalAddress :: MMIOPort :: SiliconCoordinate :: RoutingLaw }
 * ============================================================================== */

#include "site.h"

/* ==============================================================================
 * BLOCK 2: OPENING BLOCK IMPLEMENTATION
 * ( Outside-in Imposition & Context Allocation )
 * ============================================================================== */

/**
 * @brief Initializes the Site context, binds the silicon coordinate,
 *        and arms the execution tripwire.
 */
void omni_site_open(omni_site_context_t *ctx, uintptr_t input_coord) {
    if (!ctx) return;

    /* 1. Outside-in imposition: external silicon coordinate */
    ctx->input_coordinate = input_coord ? input_coord : OMNI_SITE_VGA_BUFFER;

    /* 2. Concrete single-address allocation */
    ctx->site = omni_site_from_address(OMNI_SITE_NULL);

    /* 3. Preconditions & tripwire arming */
    omni_tripwire_arm(&ctx->tripwire);
    ctx->status = OMNI_STATE_PROCEED;
}

/* ==============================================================================
 * BLOCK 3: BODY BLOCK IMPLEMENTATION
 * ( Operational Primitives, Routing Laws & Memory Access )
 * ============================================================================== */

/**
 * @brief Hardware Routing Law: Distinguishes physical RAM pointers from
 *        hardware MMIO registers and serial ports.
 */
omni_site_routing_t omni_site_resolve_routing(uintptr_t address) {
    if (address == OMNI_SITE_COM1 || address == OMNI_SITE_VGA_BUFFER || address >= 0x000A0000) {
        return OMNI_ROUTING_HARDWARE_MMIO;
    }
    return OMNI_ROUTING_RAM_PHYSICAL;
}

/**
 * @brief Constructs a Site directly from a 64-bit address pointer.
 */
omni_site_t omni_site_from_address(uintptr_t address) {
    omni_site_t s;
    s.coordinate = address;
    s.routing = omni_site_resolve_routing(address);
    s.is_enclosed = (address != OMNI_SITE_NULL);
    return s;
}

/**
 * @brief Calculates an offset Site from a base Site coordinate.
 */
omni_site_t omni_site_offset(const omni_site_t *site, intptr_t offset) {
    if (!site || site->coordinate == OMNI_SITE_NULL) {
        return omni_site_from_address(OMNI_SITE_NULL);
    }
    return omni_site_from_address((uintptr_t)((intptr_t)site->coordinate + offset));
}

/**
 * @brief Verifies whether a site coordinate is valid and non-null.
 */
bool omni_site_is_valid(const omni_site_t *site) {
    return (site != NULL && site->coordinate != OMNI_SITE_NULL && site->is_enclosed);
}

/**
 * @brief Evaluates relational equality between two sites.
 */
bool omni_site_eq(const omni_site_t *a, const omni_site_t *b) {
    if (!a || !b) return false;
    return (a->coordinate == b->coordinate && a->routing == b->routing);
}

/**
 * @brief Safely reads an 8-bit byte from the site coordinate plus offset.
 *        Employs volatile memory semantics for bare-metal MMIO safety.
 */
uint8_t omni_site_read_u8(const omni_site_t *site, size_t offset) {
    if (!omni_site_is_valid(site)) return 0;
    const volatile uint8_t *ptr = (const volatile uint8_t *)(site->coordinate + offset);
    return *ptr;
}

/**
 * @brief Safely writes an 8-bit byte to the site coordinate plus offset.
 *        Employs volatile memory semantics for bare-metal MMIO safety.
 */
void omni_site_write_u8(const omni_site_t *site, size_t offset, uint8_t value) {
    if (!omni_site_is_valid(site)) return;
    volatile uint8_t *ptr = (volatile uint8_t *)(site->coordinate + offset);
    *ptr = value;
}

/**
 * @brief Executes the Site lifecycle across declared operational rolls:
 *   - Roll 1: Locate address with numbers.
 *   - Roll 2: Bind coordinate and classify routing.
 *   - Roll 3: Guard non-null identity invariants.
 *   - Roll 4: Route and anchor memory target.
 */
omni_quad_state_t omni_site_execute(omni_site_context_t *ctx) {
    if (!ctx) return OMNI_STATE_ROLLBACK;

    /* ROLL 1: LOCATE (Ladder build-up with Numbers) */
    ctx->site = omni_site_from_address(ctx->input_coordinate);

    /* ROLL 2 & 3: BIND COORDINATE, CLASSIFY & NULL GUARD */
    bool site_valid = omni_site_is_valid(&ctx->site);
    bool identity_match = (ctx->site.coordinate == ctx->input_coordinate);

    /* Formal Guard Engine: Asserts non-null valid silicon address */
    if (omni_guard_assert(site_valid && identity_match, &ctx->tripwire, &ctx->status) != OMNI_STATE_PROCEED) {
        return OMNI_STATE_ROLLBACK;
    }

    /* ROLL 4: ROUTE & PROJECT (Inside-out emergence) */
    /* Physical coordinate anchored to hardware substrate */

    /* ROLL 5: SEQUENCE TRANSITION (Site chains to Memory Map) */
    /* Ready to sequence into system memory map */

    return ctx->status;
}

/* ==============================================================================
 * BLOCK 4: CLOSING BLOCK IMPLEMENTATION
 * ( Integrity Seals & Commit Status )
 * ============================================================================== */

/**
 * @brief Verifies all invariants and seals the Site commit status.
 */
omni_quad_state_t omni_site_close(omni_site_context_t *ctx) {
    if (!ctx) return OMNI_STATE_ROLLBACK;

    /* Integrity Check: Proves tripwire is untripped and address is valid */
    bool integrity_ok = (omni_base_verify_integrity(ctx->tripwire, ctx->status) == OMNI_STATE_PROCEED) &&
                        omni_site_is_valid(&ctx->site);

    if (!integrity_ok) {
        return omni_commit_rollback(&ctx->tripwire);
    }

    /* Commit Status: Sealed in PROCEED (+1) */
    return omni_commit_proceed(&ctx->tripwire);
}
