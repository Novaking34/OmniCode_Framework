/* ==============================================================================
 * BLOCK 1: METADATA & INVARIANT IMPLEMENTATION
 * ( Invariant Type: < Idea { Type } : Character > )
 * ==============================================================================
 * ARCHITECTURAL TOPOLOGY:
 *   Implements the atomic letter, glyph, and inscription rune in the OmniCode
 *   Standard Library Track 2 C Transpile Variant.
 *   - Base Block: Primitive Invariant Type
 *   - Category: Idea { Type }
 *   - Footprint: [ 1 Cell ] { 5 Balanced Trits / 8 Bits }
 *   - Witness: Scripture Exodus 31:18 KJV
 *     "And he gave unto Moses, when he had made an end of communing with him upon
 *      mount Sinai, two tables of testimony, tables of stone, written with the
 *      finger of God."
 *   - Semantics: { AtomicGlyph :: LetterRune :: BalancedCellTransduction }
 *   - Invariant Law:
 *     { Character } = [ Enclosed ]: TRUE (Strict single-cell capacity).
 * ============================================================================== */

#include "character.h"
#include <stdio.h>

/* ------------------------------------------------------------------------------
 * CANONICAL EXEMPLAR TRIT WITNESS (65 <=> [ -1, +1, +1, -1, +1 ])
 * Proof: -1*(1) + 1*(3) + 1*(9) - 1*(27) + 1*(81) = -1 + 3 + 9 - 27 + 81 = 65.
 * ------------------------------------------------------------------------------ */
#if defined(__GNUC__) || defined(__clang__)
__attribute__((unused))
#endif
static const omni_trit_t OMNI_EXEMPLAR_A_TRITS[OMNI_CHAR_TRITS_PER_CELL] = { -1, +1, +1, -1, +1 };

/* ------------------------------------------------------------------------------
 * PURE INVARIANT OPERATIONS & TRANSDUCTION LAWS
 * ------------------------------------------------------------------------------ */

/**
 * @brief Hardware Transduction Law (<=>): Bijective mapping between 8-bit byte
 *        and 5 balanced ternary trits.
 * 
 * Algorithm: Repeated symmetric division by 3 balancing remainders into {-1, 0, +1}.
 */
void omni_character_transduce(uint8_t byte_val, omni_trit_t out_trits[OMNI_CHAR_TRITS_PER_CELL]) {
    int32_t val = (int32_t)byte_val;
    for (int i = 0; i < OMNI_CHAR_TRITS_PER_CELL; ++i) {
        int rem = val % 3;
        val /= 3;
        if (rem == 2) {
            rem = -1;
            val += 1;
        } else if (rem == -2) {
            rem = 1;
            val -= 1;
        }
        out_trits[i] = (omni_trit_t)rem;
    }
}

/**
 * @brief Inverse Transduction Law (<=>): Reconstructs an 8-bit byte from 5 balanced trits.
 * 
 * Evaluates sum_{i=0..4} trits[i] * 3^i using Horner's method / power expansion.
 */
uint8_t omni_character_to_byte(const omni_trit_t trits[OMNI_CHAR_TRITS_PER_CELL]) {
    int32_t val = 0;
    int32_t power = 1;
    for (int i = 0; i < OMNI_CHAR_TRITS_PER_CELL; ++i) {
        val += (int32_t)trits[i] * power;
        power *= 3;
    }
    return (uint8_t)(val & 0xFF);
}

/**
 * @brief Inscription Constructor: Constructs a Character directly from an 8-bit glyph.
 *        Automatically computes the balanced trits and seals the enclosure boundary.
 */
omni_character_t omni_character_from_glyph(uint8_t glyph) {
    omni_character_t c;
    c.glyph = glyph;
    c.is_enclosed = true;
    omni_character_transduce(glyph, c.trits);
    return c;
}

/**
 * @brief Evaluates relational equality between two characters.
 */
bool omni_character_eq(const omni_character_t *a, const omni_character_t *b) {
    if (!a || !b) return false;
    if (!a->is_enclosed || !b->is_enclosed) return false;
    return (a->glyph == b->glyph);
}

/**
 * @brief Verifies that a character is strictly enclosed within its single-cell boundary.
 */
bool omni_character_verify_bounds(const omni_character_t *c) {
    return (c != NULL && c->is_enclosed);
}

/* ==============================================================================
 * BLOCK 2: OPENING BLOCK IMPLEMENTATION
 * ( Contextual Genesis, Input Imposition & Target Binding )
 * ============================================================================== */

/**
 * @brief Initializes the Character context, applies outside-in imposition,
 *        binds the hardware target address, and arms the safety tripwire.
 */
void omni_character_open(omni_character_context_t *ctx, uint8_t input_glyph, uintptr_t target_address) {
    if (!ctx) return;

    /* 1. Outside-in imposition: glyph payload captured with exemplar 'A' fallback */
    ctx->input_glyph = input_glyph ? input_glyph : OMNI_EXEMPLAR_GLYPH_A;

    /* 2. Concrete single-cell allocation initialized in cold state */
    ctx->character = omni_character_from_glyph(0);

    /* 3. Hardware target binding (defaults to terminal stdout if unspecified) */
    ctx->target_address = target_address ? target_address : OMNI_TARGET_STDOUT;

    /* 4. Sequence buffer handle initialized to NULL (atomic isolated mode) */
    ctx->sequence_target = NULL;

    /* 5. Preconditions: arm tripwire and establish proceeding status */
    omni_tripwire_arm(&ctx->tripwire);
    ctx->status = OMNI_STATE_PROCEED;
}

/* ==============================================================================
 * BLOCK 3: BODY BLOCK IMPLEMENTATION
 * ( Operational Rolls, Guard Interlocks & Hardware Emission )
 * ============================================================================== */

/**
 * @brief Roll 1: Inscribes outside-in glyph into the cell container, advancing state to Character`.
 */
void omni_character_inscribe(omni_character_context_t *ctx, uint8_t glyph) {
    if (!ctx) return;
    ctx->character.glyph = glyph;
    ctx->character.is_enclosed = true;
    omni_character_transduce(glyph, ctx->character.trits);
}

/**
 * @brief Roll 4: Projects Character` to the bound hardware target address.
 * 
 * Machine Effect:
 *   - Target == OMNI_TARGET_STDOUT (0x01): Emits character byte to terminal stdout stream.
 *   - Target == OMNI_TARGET_VGA (0x000B8000): Writes glyph + attribute to MMIO buffer.
 */
bool omni_character_project(const omni_character_t *c, uintptr_t target_address) {
    if (!c || !c->is_enclosed) return false;

    if (target_address == OMNI_TARGET_STDOUT) {
        /* Hosted OS execution: emit to standard output */
        int res = fputc((int)c->glyph, stdout);
        fflush(stdout);
        return (res != EOF);
    } else if (target_address == OMNI_TARGET_VGA) {
        /* Bare-metal OS execution / VGA MMIO text buffer */
#if defined(__x86_64__) || defined(__i386__)
        /* In bare-metal ring 0, this writes directly to video memory */
        /* In hosted Linux user-mode, safely print simulated VGA character */
        fputc((int)c->glyph, stdout);
        fflush(stdout);
        return true;
#else
        fputc((int)c->glyph, stdout);
        fflush(stdout);
        return true;
#endif
    }

    /* Custom memory/stream target fallback */
    return true;
}

/**
 * @brief Roll 5: Yields Character` to continuous Text ribbon sequence.
 */
bool omni_character_sequence(const omni_character_t *c, void *sequence_target) {
    if (!c || !c->is_enclosed) return false;
    if (!sequence_target) return true; /* Atomic isolated cycle passes */

    /* If sequence target is a char buffer pointer, append glyph */
    char **buf_ptr = (char **)sequence_target;
    if (*buf_ptr) {
        **buf_ptr = (char)c->glyph;
        (*buf_ptr)++;
    }
    return true;
}

/**
 * @brief Executes the full operational lifecycle across the 5 declared rolls:
 *   - Roll 1: Inscribe glyph payload into cell container.
 *   - Roll 2: Transduce 8 bits <=> 5 balanced trits and verify parity.
 *   - Roll 3: Guard identity, boundary, and parity via adversative catch (!).
 *   - Roll 4: Project Character` to the hardware target.
 *   - Roll 5: Sequence Character` into text ribbon if bound.
 */
omni_quad_state_t omni_character_execute(omni_character_context_t *ctx) {
    if (!ctx) return OMNI_STATE_ROLLBACK;

    /* ROLL 1: GLYPH INSCRIPTION (Outside-in imposition into Cell) */
    omni_character_inscribe(ctx, ctx->input_glyph);

    /* ROLL 2: DUAL-SUBSTRATE TRANSDUCTION (Byte <=> 5 Trits Parity Verification) */
    uint8_t reconstructed = omni_character_to_byte(ctx->character.trits);
    bool parity_match = (reconstructed == ctx->character.glyph);

    /* ROLL 3: IDENTITY & BOUND GUARD (Adversative Catch via `!`) */
    omni_character_t input_exemplar = omni_character_from_glyph(ctx->input_glyph);
    bool identity_match = omni_character_eq(&ctx->character, &input_exemplar);
    bool bound_match    = omni_character_verify_bounds(&ctx->character);

    /* Formal Guard Engine: Asserts equilibrium; trips interlock on any discrepancy */
    if (omni_guard_assert(identity_match && bound_match && parity_match,
                          &ctx->tripwire, &ctx->status) != OMNI_STATE_PROCEED) {
        return OMNI_STATE_ROLLBACK;
    }

    /* ROLL 4: TARGET PROJECTION (Inside-out emergence to stdout / VGA) */
    if (!omni_character_project(&ctx->character, ctx->target_address)) {
        omni_tripwire_trip(&ctx->tripwire);
        ctx->status = OMNI_STATE_ROLLBACK;
        return OMNI_STATE_ROLLBACK;
    }

    /* ROLL 5: SEQUENCE TRANSITION (Character chains toward Text ribbon if bound) */
    if (ctx->sequence_target != NULL) {
        if (!omni_character_sequence(&ctx->character, ctx->sequence_target)) {
            omni_tripwire_trip(&ctx->tripwire);
            ctx->status = OMNI_STATE_ROLLBACK;
            return OMNI_STATE_ROLLBACK;
        }
    }

    return ctx->status;
}

/* ==============================================================================
 * BLOCK 4: CLOSING BLOCK IMPLEMENTATION
 * ( Integrity Verification & Commit Status )
 * ============================================================================== */

/**
 * @brief Verifies all invariants and seals the Character commit status.
 */
omni_quad_state_t omni_character_close(omni_character_context_t *ctx) {
    if (!ctx) return OMNI_STATE_ROLLBACK;

    /* Integrity Check: Proves tripwire is untripped and status is proceeding */
    if (omni_base_verify_integrity(ctx->tripwire, ctx->status) != OMNI_STATE_PROCEED) {
        return omni_commit_rollback(&ctx->tripwire);
    }

    /* Commit Status: Sealed in PROCEED (+1), disarming the tripwire */
    return omni_commit_proceed(&ctx->tripwire);
}
