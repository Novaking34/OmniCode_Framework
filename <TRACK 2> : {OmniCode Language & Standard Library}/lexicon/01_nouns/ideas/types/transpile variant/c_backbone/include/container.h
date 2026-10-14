/* ==============================================================================
 * BLOCK 1: METADATA & INVARIANT TYPE DEFINITION
 * ( Invariant Type: < Idea { Type } : Container > )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Defines bounded memory envelopes, arrays, structures, and storage vessels
 *   in the OmniCode Standard Library Track 2 C Transpile Variant.
 *   - Base Block: Primitive Invariant Type
 *   - Category: Idea { Type }
 *   - Footprint: [ Dynamically Bounded Memory Envelope / Generic Slice ]
 *   - Ladder Ascension: Builds up with Numbers (/-^)
 *   - System: OmniCode { Lexicon }
 *   - Witness: Scripture 2 Kings 4:4 KJV
 *     "And when thou art come in, thou shalt shut the door upon thee and upon
 *      thy sons, and shalt pour out into all those vessels, and thou shalt set
 *      aside that which is full."
 *   - Semantics: { BoundedEnvelope :: StorageVessel :: MemoryCapsule :: ContainmentLaw }
 * ============================================================================== */

#ifndef OMNI_CONTAINER_H
#define OMNI_CONTAINER_H

#include "omni_base.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ------------------------------------------------------------------------------
 * 1. INVARIANT CONSTANTS & CANONICAL EXEMPLARS
 * ------------------------------------------------------------------------------ */

/** Canonical Unit Exemplar: Single element partition (1 unit). */
#define OMNI_CONTAINER_EXEMPLAR_UNIT        ((size_t)1)

/** Canonical Vessel Exemplar: Standard 64-element storage vessel. */
#define OMNI_CONTAINER_EXEMPLAR_VESSEL      ((size_t)64)

/** Canonical Storehouse Exemplar: Large 4096-element memory storehouse. */
#define OMNI_CONTAINER_EXEMPLAR_STOREHOUSE  ((size_t)4096)

/* ------------------------------------------------------------------------------
 * 2. FREESTANDING INVARIANT TYPE STRUCTURE: omni_container_t
 * ------------------------------------------------------------------------------ */

/**
 * @brief Invariant Type < Container >: The generic bounded memory envelope / slice.
 * 
 * Provides memory safety by construction without requiring a heap or garbage collector:
 *   - Wraps a memory buffer of arbitrary element types (Character, Number, Site, State).
 *   - Strictly enforces the Core Containment Law: Occupancy <= Capacity.
 * 
 * Core Invariant Law:
 *   ENCLOSE [ Storage ] WITHIN [ Capacity ]
 *   LAW { Containment } : [ { Occupancy } <= { Capacity } ]
 */
typedef struct {
    void   *buffer;                                     /**< Raw memory vessel holding discrete elements */
    size_t  element_size;                               /**< Footprint of a single element (sizeof(T) in bytes) */
    size_t  capacity;                                   /**< Allocated maximum element capacity boundary */
    size_t  occupancy;                                  /**< Current count of stored elements */
    bool    is_enclosed;                                /**< Invariant enclosure guarantee */
} omni_container_t;

/**
 * @brief Convenience static vessel initialized over a 64-byte storage partition.
 */
typedef struct {
    uint8_t          cells[OMNI_CONTAINER_EXEMPLAR_VESSEL]; /**< Concrete 64-byte static buffer */
    omni_container_t container;                             /**< Bounded slice wrapper */
} omni_byte_vessel_t;

/* ------------------------------------------------------------------------------
 * 3. 4-BLOCK PIPELINE CONTEXT: omni_container_context_t
 * ------------------------------------------------------------------------------ */

/**
 * @brief Execution context for Container encapsulation lifecycle rolls.
 */
typedef struct {
    const void            *payload_ptr;                 /**< Imposed outside-in payload buffer */
    size_t                 payload_count;               /**< Count of incoming items to encapsulate */
    omni_container_t       vessel;                      /**< Concrete bounded vessel envelope */
    uintptr_t              target_destination;          /**< Destination memory address or anchor */
    omni_quad_state_t      status;                      /**< Active Quad-State truth */
    omni_tripwire_state_t  tripwire;                    /**< Subsystem safety interlock */
} omni_container_context_t;

/* ==============================================================================
 * BLOCK 2: OPENING BLOCK
 * ( Outside-in Imposition & Context Initialization )
 * ============================================================================== */

/**
 * @brief Initializes a Container context over a provided memory storage partition.
 * 
 * @param ctx Pointer to caller-allocated container context.
 * @param storage Memory buffer allocated by the caller (stack or static).
 * @param elem_size Footprint of a single element in bytes.
 * @param capacity Maximum number of elements storage can hold.
 * @param payload Outside-in memory payload to encapsulate.
 * @param payload_count Number of elements in the incoming payload.
 */
void omni_container_open(omni_container_context_t *ctx,
                         void *storage,
                         size_t elem_size,
                         size_t capacity,
                         const void *payload,
                         size_t payload_count);

/* ==============================================================================
 * BLOCK 3: BODY BLOCK
 * ( Operational Primitives, Containment Laws & Inscriptions )
 * ============================================================================== */

/**
 * @brief Initializes a generic bounded container slice over an existing memory buffer.
 * 
 * @param c Pointer to container struct to initialize.
 * @param buffer Pointer to underlying memory array.
 * @param element_size Byte size of each element.
 * @param capacity Maximum number of elements.
 */
void omni_container_init(omni_container_t *c, void *buffer, size_t element_size, size_t capacity);

/**
 * @brief Initializes a canonical 64-byte storage vessel.
 * 
 * @param vessel Pointer to byte vessel struct.
 */
void omni_byte_vessel_init(omni_byte_vessel_t *vessel);

/**
 * @brief Core Containment Law Verification: Proves that Occupancy <= Capacity.
 * 
 * @param c Pointer to container to evaluate.
 * @return true if Occupancy <= Capacity and properly enclosed; false otherwise.
 */
bool omni_container_verify_containment(const omni_container_t *c);

/**
 * @brief Queries whether the container has reached its maximum capacity limit.
 * 
 * @param c Pointer to container.
 * @return true if Occupancy >= Capacity; false otherwise.
 */
bool omni_container_is_full(const omni_container_t *c);

/**
 * @brief Queries whether the container has zero stored elements.
 * 
 * @param c Pointer to container.
 * @return true if Occupancy == 0; false otherwise.
 */
bool omni_container_is_empty(const omni_container_t *c);

/**
 * @brief Pushes an element into the container, advancing occupancy by 1.
 * 
 * @param c Pointer to container.
 * @param element Pointer to element data to copy in.
 * @return OMNI_STATE_PROCEED on successful insertion; OMNI_STATE_ROLLBACK if full.
 */
omni_quad_state_t omni_container_push(omni_container_t *c, const void *element);

/**
 * @brief Retrieves a direct pointer to the element at the specified index.
 * 
 * @param c Pointer to container.
 * @param index 0-based element index.
 * @return Pointer to element memory, or NULL if index >= occupancy.
 */
void *omni_container_get(const omni_container_t *c, size_t index);

/**
 * @brief Replaces or writes an element at the specified index.
 * 
 * @param c Pointer to container.
 * @param index 0-based element index.
 * @param element Pointer to element data to copy in.
 * @return OMNI_STATE_PROCEED on success; OMNI_STATE_ROLLBACK if index >= capacity.
 */
omni_quad_state_t omni_container_set(omni_container_t *c, size_t index, const void *element);

/**
 * @brief Resets container occupancy to zero without clearing underlying memory.
 * 
 * @param c Pointer to container.
 */
void omni_container_clear(omni_container_t *c);

/**
 * @brief Executes the full encapsulation lifecycle across declared rolls:
 *   - Roll 1: Build capsule envelope with numbers.
 *   - Roll 2: Encapsulate incoming payload.
 *   - Roll 3: Guard bounds under Law { Containment } (Occupancy <= Capacity).
 *   - Roll 4: Pour out and set aside vessel.
 * 
 * @param ctx Pointer to active container context.
 * @return OMNI_STATE_PROCEED on equilibrium; OMNI_STATE_ROLLBACK on overflow.
 */
omni_quad_state_t omni_container_execute(omni_container_context_t *ctx);

/* ==============================================================================
 * BLOCK 4: CLOSING BLOCK
 * ( Integrity Verification & Commit Status )
 * ============================================================================== */

/**
 * @brief Validates all invariants and seals the Container commit status.
 * 
 * @param ctx Pointer to active container context.
 * @return OMNI_STATE_PROCEED on successful seal; OMNI_STATE_ROLLBACK if tripped.
 */
omni_quad_state_t omni_container_close(omni_container_context_t *ctx);

#ifdef __cplusplus
}
#endif

#endif /* OMNI_CONTAINER_H */
