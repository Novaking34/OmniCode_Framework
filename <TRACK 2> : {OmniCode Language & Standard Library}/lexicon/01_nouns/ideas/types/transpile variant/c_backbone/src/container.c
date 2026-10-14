/* ==============================================================================
 * BLOCK 1: METADATA & INVARIANT IMPLEMENTATION
 * ( Invariant Type: < Idea { Type } : Container > )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Implements bounded memory envelopes, arrays, structures, and storage vessels
 *   in the OmniCode Standard Library Track 2 C Transpile Variant.
 *   - Base Block: Primitive Invariant Type
 *   - Category: Idea { Type }
 *   - Footprint: [ Dynamically Bounded Memory Envelope / Generic Slice ]
 *   - Ladder Ascension: Builds up with Numbers (/-^)
 *   - Witness: Scripture 2 Kings 4:4 KJV
 *     "And when thou art come in, thou shalt shut the door upon thee and upon
 *      thy sons, and shalt pour out into all those vessels, and thou shalt set
 *      aside that which is full."
 *   - Semantics: { BoundedEnvelope :: StorageVessel :: MemoryCapsule :: ContainmentLaw }
 * ============================================================================== */

#include "container.h"

/**
 * @brief Bare-metal freestanding memory byte copy (Zero libc/memcpy dependency).
 */
static void omni_mem_copy(void *dest, const void *src, size_t n) {
    uint8_t *d = (uint8_t *)dest;
    const uint8_t *s = (const uint8_t *)src;
    for (size_t i = 0; i < n; ++i) {
        d[i] = s[i];
    }
}

/* ==============================================================================
 * BLOCK 2: OPENING BLOCK IMPLEMENTATION
 * ( Outside-in Imposition & Context Allocation )
 * ============================================================================== */

/**
 * @brief Initializes a Container context over a provided memory storage partition,
 *        applies outside-in imposition, and arms the execution tripwire.
 */
void omni_container_open(omni_container_context_t *ctx,
                         void *storage,
                         size_t elem_size,
                         size_t capacity,
                         const void *payload,
                         size_t payload_count) {
    if (!ctx) return;

    ctx->payload_ptr = payload;
    ctx->payload_count = payload_count;

    /* Concrete allocation with provided or default parameters */
    size_t e_sz = elem_size ? elem_size : sizeof(uint8_t);
    size_t cap  = capacity ? capacity : OMNI_CONTAINER_EXEMPLAR_VESSEL;
    omni_container_init(&ctx->vessel, storage, e_sz, cap);

    ctx->target_destination = OMNI_SITE_NULL;
    omni_tripwire_arm(&ctx->tripwire);
    ctx->status = OMNI_STATE_PROCEED;
}

/* ==============================================================================
 * BLOCK 3: BODY BLOCK IMPLEMENTATION
 * ( Operational Primitives, Containment Laws & Inscriptions )
 * ============================================================================== */

/**
 * @brief Initializes a generic bounded container slice over an existing memory buffer.
 */
void omni_container_init(omni_container_t *c, void *buffer, size_t element_size, size_t capacity) {
    if (!c) return;
    c->buffer = buffer;
    c->element_size = element_size;
    c->capacity = capacity;
    c->occupancy = 0;
    c->is_enclosed = (buffer != NULL && element_size > 0 && capacity > 0);
}

/**
 * @brief Initializes a canonical 64-byte storage vessel.
 */
void omni_byte_vessel_init(omni_byte_vessel_t *vessel) {
    if (!vessel) return;
    for (size_t i = 0; i < OMNI_CONTAINER_EXEMPLAR_VESSEL; ++i) {
        vessel->cells[i] = 0;
    }
    omni_container_init(&vessel->container, vessel->cells, sizeof(uint8_t), OMNI_CONTAINER_EXEMPLAR_VESSEL);
}

/**
 * @brief Core Containment Law Verification: Proves that Occupancy <= Capacity.
 */
bool omni_container_verify_containment(const omni_container_t *c) {
    if (!c) return false;
    return (c->is_enclosed && c->occupancy <= c->capacity);
}

/**
 * @brief Queries whether the container has reached its maximum capacity limit.
 */
bool omni_container_is_full(const omni_container_t *c) {
    if (!c) return true;
    return (c->occupancy >= c->capacity);
}

/**
 * @brief Queries whether the container has zero stored elements.
 */
bool omni_container_is_empty(const omni_container_t *c) {
    if (!c) return true;
    return (c->occupancy == 0);
}

/**
 * @brief Pushes an element into the container, strictly guarding against capacity overflow.
 */
omni_quad_state_t omni_container_push(omni_container_t *c, const void *element) {
    if (!c || !element || !c->buffer) return OMNI_STATE_ROLLBACK;

    /* Law { Containment } Guard: Occupancy must never exceed capacity */
    if (c->occupancy >= c->capacity) {
        return OMNI_STATE_ROLLBACK;
    }

    uint8_t *dest = (uint8_t *)c->buffer + (c->occupancy * c->element_size);
    omni_mem_copy(dest, element, c->element_size);
    c->occupancy += 1;

    return OMNI_STATE_PROCEED;
}

/**
 * @brief Retrieves a direct pointer to the element at the specified index.
 */
void *omni_container_get(const omni_container_t *c, size_t index) {
    if (!c || !c->buffer || index >= c->occupancy) {
        return NULL;
    }
    return (uint8_t *)c->buffer + (index * c->element_size);
}

/**
 * @brief Replaces or writes an element at the specified index.
 */
omni_quad_state_t omni_container_set(omni_container_t *c, size_t index, const void *element) {
    if (!c || !element || !c->buffer || index >= c->capacity) {
        return OMNI_STATE_ROLLBACK;
    }

    uint8_t *dest = (uint8_t *)c->buffer + (index * c->element_size);
    omni_mem_copy(dest, element, c->element_size);

    if (index >= c->occupancy) {
        c->occupancy = index + 1;
    }

    return OMNI_STATE_PROCEED;
}

/**
 * @brief Resets container occupancy to zero without modifying buffer contents.
 */
void omni_container_clear(omni_container_t *c) {
    if (!c) return;
    c->occupancy = 0;
}

/**
 * @brief Executes the Container lifecycle across declared operational rolls:
 *   - Roll 1: Build capsule envelope with numbers.
 *   - Roll 2: Encapsulate incoming payload.
 *   - Roll 3: Guard bounds under Law { Containment } (Occupancy <= Capacity).
 *   - Roll 4: Pour out and set aside vessel.
 */
omni_quad_state_t omni_container_execute(omni_container_context_t *ctx) {
    if (!ctx) return OMNI_STATE_ROLLBACK;

    /* ROLL 1: BUILD CAPSULE (Ladder build-up with Numbers) */
    bool capsule_valid = (ctx->vessel.capacity > 0 && ctx->vessel.is_enclosed);
    if (omni_guard_assert(capsule_valid, &ctx->tripwire, &ctx->status) != OMNI_STATE_PROCEED) {
        return OMNI_STATE_ROLLBACK;
    }

    /* ROLL 2: ENCAPSULATION (Inscribe payload into storage elements) */
    if (ctx->payload_ptr && ctx->payload_count > 0) {
        /* Law violation guard: Attempting to overflow bounded capacity */
        bool within_bounds = (ctx->payload_count <= ctx->vessel.capacity);
        if (omni_guard_assert(within_bounds, &ctx->tripwire, &ctx->status) != OMNI_STATE_PROCEED) {
            return OMNI_STATE_ROLLBACK;
        }

        const uint8_t *src_bytes = (const uint8_t *)ctx->payload_ptr;
        for (size_t i = 0; i < ctx->payload_count; ++i) {
            const void *elem = src_bytes + (i * ctx->vessel.element_size);
            omni_container_push(&ctx->vessel, elem);
        }
    }

    /* ROLL 3: IDENTITY & BOUND COMPARISON (Guard bounds under Law { Containment }) */
    bool containment_ok = omni_container_verify_containment(&ctx->vessel);
    if (omni_guard_assert(containment_ok, &ctx->tripwire, &ctx->status) != OMNI_STATE_PROCEED) {
        return OMNI_STATE_ROLLBACK;
    }

    /* ROLL 4: POUR OUT & SET ASIDE (Inside-out emergence) */
    /* Vessel sealed and set aside at destination coordinate */

    /* ROLL 5: SEQUENCE TRANSITION (Container chains to composite structure) */
    /* Ready to sequence into memory partitions */

    return ctx->status;
}

/* ==============================================================================
 * BLOCK 4: CLOSING BLOCK IMPLEMENTATION
 * ( Integrity Seals & Commit Status )
 * ============================================================================== */

/**
 * @brief Verifies all invariants and seals the Container commit status.
 */
omni_quad_state_t omni_container_close(omni_container_context_t *ctx) {
    if (!ctx) return OMNI_STATE_ROLLBACK;

    /* Integrity Check: Proves tripwire is untripped and containment law holds */
    bool integrity_ok = (omni_base_verify_integrity(ctx->tripwire, ctx->status) == OMNI_STATE_PROCEED) &&
                        omni_container_verify_containment(&ctx->vessel);

    if (!integrity_ok) {
        return omni_commit_rollback(&ctx->tripwire);
    }

    /* Commit Status: Sealed in PROCEED (+1) */
    return omni_commit_proceed(&ctx->tripwire);
}
