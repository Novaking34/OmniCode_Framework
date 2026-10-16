/* ===========================================================================
 * BLOCK 1: METADATA & INVARIANT TYPE DEFINITION
 * ( Invariant Type: < Idea { Type } : Character > )
 * ===========================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Defines the atomic letter, glyph, and inscription rune in the OmniCode
 *   Standard Library Track 2 C Transpile Variant.
 *   - Base Block: Primitive Invariant Type
 *   - Category: Idea { Type }
 *   - Footprint: [ 1 Cell ] { 5 Balanced Trits / 8 Bits }
 *   - System: OmniCode { Lexicon }
 *   - Witness: Scripture Exodus 31:18 KJV
 *     "And he gave unto Moses, when he had made an end of communing with
 *      him upon mount Sinai..."
 *      mount Sinai, two tables of testimony, tables of stone, written with the
 *      finger of God."
 *   - Semantics: { AtomicGlyph :: LetterRune :: BalancedCellTransduction }
 *   - Invariant Law:
 *     { Character } = [ Enclosed ]: TRUE (Strict single-cell capacity).
 *
 * ONTOLOGICAL CORRESPONDENCE TO OMNICODE:
 *   OmniCode Block 1 declares the immutable laws of the universe for this type.
 *   In C, Block 1 is realized exclusively as:
 *     1. Invariant hardware target addresses and substrate constants.
 *     2. Freestanding invariant struct (omni_character_t) with dual
 *        substrate.
 *     3. Pure mathematical conversion and verification functions that operate
 *        without runtime context dependencies.
 * ======================================================================== */

#ifndef OMNI_CHARACTER_H
#define OMNI_CHARACTER_H

#include "omni_base.h"
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ---------------------------------------------------------------------------
 * 1. INVARIANT CONSTANTS & HARDWARE LOCATORS (OMNICODE BLOCK 1 & 2)
 * ------------------------------------------------------------------------ */

/** Number of balanced trits per character cell (3^5 = 243 states). */
#define OMNI_CHAR_TRITS_PER_CELL OMNI_TRITS_PER_CELL

/** Canonical Exemplar: ASCII 'A' (Byte value 65). */
#define OMNI_EXEMPLAR_GLYPH_A    ((uint8_t)'A')

/**
 * @brief Hardware Target Locators (Composed operator ::@).
 * 
 * OmniCode Specification:
 *   [ Target ]: { [ Terminal Stdout ] ::@ 0x01 |
 *                 [ Screen VGA ] ::@ 0x000B8000 }
 */
/** Hosted POSIX terminal stdout stream descriptor */
#define OMNI_TARGET_STDOUT       ((uintptr_t)0x01)
/** Bare-metal x86 text mode video memory MMIO buffer */
#define OMNI_TARGET_VGA          ((uintptr_t)0x000B8000)

/* ---------------------------------------------------------------------------
 * 2. FREESTANDING INVARIANT TYPE STRUCTURE: omni_character_t
 * ------------------------------------------------------------------------ */

/**
 * @brief Invariant Type < Character >: The atomic rune of symbolic computing.
 * 
 * Encloses the dual substrate:
 *   1. Physical Silicon Substrate: 1 Byte (8 bits, 0..255).
 *   2. Virtual Balanced Trit Substrate: 1 Cell (5 balanced trits, {-1, 0, +1}).
 * 
 * Invariant Law:
 *   { Character } = [ Enclosed ]: TRUE (Strict single-cell boundary).
 */
typedef struct {
    /** Physical byte substrate (8-bit ASCII/binary glyph) */
    uint8_t     glyph;
    /** Virtual cell substrate (5 balanced ternary trits) */
    omni_trit_t trits[OMNI_CHAR_TRITS_PER_CELL];
    /** Invariant boundary enclosure guarantee */
    bool        is_enclosed;
} omni_character_t;

/* ---------------------------------------------------------------------------
 * 3. PURE INVARIANT OPERATIONS & TRANSDUCTION LAWS
 * ------------------------------------------------------------------------ */

/**
 * @brief Inscription Constructor: Constructs a Character directly from a raw
 * 8-bit glyph.
 * 
 * Evaluates OmniCode: [ Inscribe ]: [ Glyph ] --> [ Character ]
 * Automatically performs bijective transduction into 5 balanced trits and
 * establishes the invariant enclosure seal (is_enclosed = true).
 * 
 * @param glyph The 8-bit byte codepoint to inscribe.
 * @return Fully initialized omni_character_t in equilibrium.
 */
omni_character_t omni_character_from_glyph(uint8_t glyph);

/**
 * @brief Hardware Transduction Law (<=>): Bijective mapping from 8-bit byte
 * to 5 balanced trits.
 * 
 * Evaluates OmniCode: [ Transduce ]: [ Character { Byte } ] <=> [ Cell {
 * Trits } ]
 * 
 * Mathematical Invariant:
 *   Each trit is in {-1, 0, +1}, satisfying:
 *     byte = sum_{i=0..4} trits[i] * 3^i
 *   Canonical Exemplar for 'A' (65):
 *     65 <=> [ -1, +1, +1, -1, +1 ]  (-1 + 3 + 9 - 27 + 81 = 65).
 * 
 * @param byte_val Input 8-bit byte value (0..255).
 * @param out_trits Destination array of 5 balanced trits.
 */
void omni_character_transduce(
    uint8_t byte_val,
    omni_trit_t out_trits[OMNI_CHAR_TRITS_PER_CELL]);

/**
 * @brief Inverse Transduction Law (<=>): Reconstructs an 8-bit byte from 5
 * balanced trits.
 * 
 * Evaluates the bidirectional symmetry of the invariant equivalence operator
 * (<=>).
 * Rebuilds the silicon byte from the balanced ternary representation.
 * 
 * @param trits Array of 5 balanced trits.
 * @return Reconstructed 8-bit unsigned byte value.
 */
uint8_t omni_character_to_byte(
    const omni_trit_t trits[OMNI_CHAR_TRITS_PER_CELL]);

/**
 * @brief Relational Equality Evaluation: Compares two character entities.
 * 
 * Evaluates OmniCode: [ Compare ]: [ { Character } = { Glyph } ]
 * 
 * @param a First character operand.
 * @param b Second character operand.
 * @return true if glyph values match and both are enclosed; false otherwise.
 */
bool omni_character_eq(const omni_character_t *a, const omni_character_t *b);

/**
 * @brief Boundary Invariant Verification: Proves single-cell enclosure.
 * 
 * Evaluates OmniCode: [ Bound ]: { Character } = [ Enclosed ]: TRUE
 * 
 * @param c Pointer to character instance to verify.
 * @return true if valid pointer and enclosure flag is sealed; false otherwise.
 */
bool omni_character_verify_bounds(const omni_character_t *c);

/**
 * @brief Bit-Level Extraction: Retrieves bit 0..7 from the glyph byte.
 * 
 * Slices into the shared 8-bit base level that Character shares with Number.
 * 
 * @param c Pointer to character entity.
 * @param bit_idx Bit index (0..7, where 0 is LSB and 7 is ASCII parity rail).
 * @return 1 if set, 0 if clear or on null pointer.
 */
uint8_t omni_character_get_bit(const omni_character_t *c, uint8_t bit_idx);

/**
 * @brief Standard ASCII Invariant Verification: Proves glyph in 0..127 range.
 * 
 * Validates bit 7 is 0, leaving bit 7 free to serve as an immutable witness.
 * 
 * @param c Pointer to character entity.
 * @return true if glyph <= 127 (bit 7 == 0); false otherwise.
 */
bool omni_character_is_ascii(const omni_character_t *c);

/* ===========================================================================
 * BLOCK 2: OPENING BLOCK
 * ( Contextual Genesis, Input Imposition & Hardware Target Binding )
 * ===========================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   OmniCode Block 2 brings the timeless laws of Block 1 down into active space
 *   and time for a concrete execution run:
 *     1. [ Glyph ]: Captures outside-in payload (CLI argument or stream input).
 *     2. [ Character ] & [ Cell ]: Allocates physical single-cell storage.
 *     3. [ Target ]: Binds to a concrete hardware address (Stdout or VGA MMIO).
 *     4. [ Preconditions ]: Checks ALU balance and arms the execution tripwire.
 * ======================================================================== */

/**
 * @brief Execution Context for Character Lifecycle Rolls.
 * 
 * Encapsulates the complete runtime environment for the 4-block lifecycle:
 * outside-in input, allocated cell, target hardware address, optional sequence
 * destination, and safety interlocks.
 */
typedef struct {
    /** [ Glyph ]: Imposed outside-in payload */
    uint8_t               input_glyph;
    /** [ Character ] & [ Cell ]: Concrete cell container */
    omni_character_t      character;
    /** [ Target ]: Hardware locator (0x01 Stdout or 0x000B8000 VGA) */
    uintptr_t             target_address;
    /** [ Sequence ]: Destination buffer for chaining into Text */
    void                 *sequence_target;
    /** [ Preconditions ]: Quad-State truth (+1:PROCEED) */
    omni_quad_state_t     status;
    /** [ Preconditions ]: Subsystem safety interlock */
    omni_tripwire_state_t tripwire;
} omni_character_context_t;

/**
 * @brief Initializes the Character context, applies outside-in imposition,
 *        binds the hardware target, and arms the safety tripwire.
 * 
 * Evaluates OmniCode Block 2: Opening { ... }
 * 
 * @param ctx Pointer to caller-allocated character execution context.
 * @param input_glyph Outside-in byte payload (defaults to exemplar 'A' if 0).
 * @param target_address Hardware locator (e.g. OMNI_TARGET_STDOUT or
 * OMNI_TARGET_VGA; defaults to STDOUT if 0).
 */
void omni_character_open(
    omni_character_context_t *ctx,
    uint8_t input_glyph,
    uintptr_t target_address);

/* ===========================================================================
 * BLOCK 3: BODY BLOCK
 * ( Operational Rolls, Guard Interlocks & Hardware Emission )
 * ===========================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   OmniCode Block 3 processes the discrete causal turns (rolls) that advance
 *   state along the causal timeline using the prime operator (`):
 *     - Roll 1: Inscribe input glyph into the Cell container.
 *     - Roll 2: Transduce 8 bits <=> 5 balanced trits.
 *     - Roll 3: Guard identity & boundary invariants via adversative catch (!).
 *     - Roll 4: Project Character` to the hardware target (terminal or VGA).
 *     - Roll 5: Sequence Character` into continuous Text ribbon if bound.
 * ======================================================================== */

/**
 * @brief Roll 1: Inscribes outside-in glyph into the cell, advancing state to
 * Character`.
 * 
 * Evaluates OmniCode: [ Inscribe ] : [ Glyph ] --> INSCRIBE [ Cell ] = [
 * Character` ]
 * 
 * @param ctx Active execution context.
 * @param glyph The byte codepoint being inscribed.
 */
void omni_character_inscribe(omni_character_context_t *ctx, uint8_t glyph);

/**
 * @brief Roll 4: Projects Character` outward to the bound hardware target
 * address.
 * 
 * Evaluates OmniCode: [ Project ] : [ Character` ] --> PROJECT [ Target ] = [
 * Target` ]
 * 
 * Machine Effect:
 * - Target == 0x01 (Stdout): Emits glyph byte to file descriptor 1
 * (terminal).
 * - Target == 0x000B8000 (VGA): Writes glyph + attribute to MMIO video
 * memory.
 * 
 * @param c Pointer to the verified Character to project.
 * @param target_address The physical destination address (0x01 or 0x000B8000).
 * @return true if projection succeeded; false on hardware fault.
 */
bool omni_character_project(
    const omni_character_t *c,
    uintptr_t target_address);

/**
 * @brief Roll 5: Yields Character` to continuous Text ribbon sequence.
 * 
 * Evaluates OmniCode: [ Sequence ] : [ Character` ] --> SEQUENCE [ Text ] = [
 * Text` ]
 * 
 * Machine Effect:
 * If sequence_target is non-NULL, appends the character glyph into the
 * sequence
 *   destination buffer. If NULL, passes as a single atomic character cycle.
 * 
 * @param c Pointer to the Character being sequenced.
 * @param sequence_target Pointer to destination buffer or container.
 * @return true if sequence yield succeeded; false on buffer overflow.
 */
bool omni_character_sequence(const omni_character_t *c, void *sequence_target);

/**
 * @brief Executes the complete 5-roll operational pipeline across declared
 * turns:
 *   Roll 1 (Inscribe) -> Roll 2 (Transduce) -> Roll 3 (Guard !) ->
 *   Roll 4 (Project)   -> Roll 5 (Sequence).
 * 
 * The Guard turn (Roll 3) enforces the adversative catch (!):
 *   If identity and boundary invariants hold, advances status to +1:PROCEED.
 * If any discrepancy is detected, trips the wire and rolls back to
 * -1:ROLLBACK.
 * 
 * @param ctx Pointer to armed character execution context.
 * @return OMNI_STATE_PROCEED (+1) on equilibrium; OMNI_STATE_ROLLBACK (-1) on
 * violation.
 */
omni_quad_state_t omni_character_execute(omni_character_context_t *ctx);

/* ===========================================================================
 * BLOCK 4: CLOSING BLOCK
 * ( Integrity Verification, Covenant Sealing & Quad-State Exit )
 * ===========================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   OmniCode Block 4 reconciles all rolls, validates tripwire status, and seals
 *   the permanent commit status (+1:PROCEED, 0:AWAIT, NULL:HALT, -1:ROLLBACK).
 *   In hosted environments, +1:PROCEED maps to process exit code 0.
 * ======================================================================== */

/**
 * @brief Validates all invariants, verifies operational consensus, disarms the
 *        tripwire, and seals the final Character commit status.
 * 
 * Evaluates OmniCode Block 4: Closing { ... }
 * 
 * @param ctx Pointer to active character execution context.
 * @return OMNI_STATE_PROCEED (+1) on successful seal; OMNI_STATE_ROLLBACK
 * (-1) if tripped.
 */
omni_quad_state_t omni_character_close(omni_character_context_t *ctx);

#ifdef __cplusplus
}
#endif

#endif /* OMNI_CHARACTER_H */
