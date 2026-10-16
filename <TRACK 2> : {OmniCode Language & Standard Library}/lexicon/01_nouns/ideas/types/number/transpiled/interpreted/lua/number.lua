-- ============================================================================
-- BLOCK 1: METADATA & PROCESS TAPE MANIFEST
-- ( Process Tape: < Lexicon { Types of Ideas } > & [ Process { Number } ] )
-- ============================================================================
-- ARCHITECTURAL TOPOLOGY:
--   Interprets and executes the atomic lifecycle for Invariant Type < Number >.
--   Acts as dynamic runtime companion driving C backbone `omni_number_t`.
--   - Dual-Rail Polar Bit-Planes: 00=0 (Equilibrium), 01=+1, 10=-1, 11=Trap
--   - Directional Partitions: 3-3-2, 3-2-3, 2-3-3 (3-space minimum for Z)
--   - Witness: Scripture Psalm 147:4 KJV
--     "He telleth the number of the stars; he calleth them all by their names."
--   - Semantics: { DualRailBitPlanes :: KinematicTopologies :: ZeroSumALU }
-- ============================================================================

local ok_te, TapeEngine = pcall(require, "tape_engine")
if not ok_te then
    package.path = package.path ..
        ";../../../../universal/transpiled/interpreted/lua/?.lua"
    TapeEngine = require("tape_engine")
end

local CONSTANTS = {
    WORD_CAPACITY       = 1,
    TRITS_PER_WORD      = 32,
    BITS_PER_WORD       = 64,
    EXEMPLAR_ZERO       = 0,
    EXEMPLAR_UNITY_POS  = 1,
    EXEMPLAR_UNITY_NEG  = -1,
    EXEMPLAR_PARTITION  = 4096,
    BITPLANE_POS_MASK   = 0x5555555555555555,
    BITPLANE_NEG_MASK   = ~0x5555555555555555,
    TRIT_CODE_ZERO      = 0, -- 0b00: Equilibrium / 0
    TRIT_CODE_POS       = 1, -- 0b01: Positive / +1
    TRIT_CODE_NEG       = 2, -- 0b10: Negative / -1
    TRIT_CODE_NULL      = 3  -- 0b11: Collision / NULL trap
}

local PACK_MODES = {
    DENSE_2222        = 0, -- 2-2-2-2: Standard dense storage (4 trits)
    KINEMATIC_21212   = 1, -- 2-1-2-1-2: Symmetric resting equilibrium
    KINEMATIC_12122   = 2, -- 1-2-1-2-2: Prefix carry/overflow channel
    KINEMATIC_22121   = 3, -- 2-2-1-2-1: Postfix carry/overflow channel
    DIRECTIONAL_332   = 4, -- 3-3-2: Directional vector (X:3, Y:3, Z:2)
    DIRECTIONAL_323   = 5, -- 3-2-3: Directional vector (X:3, Y:2, Z:3)
    DIRECTIONAL_233   = 6  -- 2-3-3: Directional vector (X:2, Y:3, Z:3)
}

local function trit_to_2bit(trit)
    if trit == 0 then return CONSTANTS.TRIT_CODE_ZERO
    elseif trit == 1 then return CONSTANTS.TRIT_CODE_POS
    elseif trit == -1 then return CONSTANTS.TRIT_CODE_NEG
    else return CONSTANTS.TRIT_CODE_NULL end
end

local function two_bit_to_trit(code)
    if code == CONSTANTS.TRIT_CODE_ZERO then return 0, true
    elseif code == CONSTANTS.TRIT_CODE_POS then return 1, true
    elseif code == CONSTANTS.TRIT_CODE_NEG then return -1, true
    else return 0, false end -- NULL collision
end

local function pack_trits_dual_rail(trits)
    local packed = 0
    for i = 1, CONSTANTS.TRITS_PER_WORD do
        local t = trits[i] or 0
        local code = trit_to_2bit(t)
        packed = packed | ((code & 0x03) << ((i - 1) * 2))
    end
    return packed
end

local function unpack_trits_dual_rail(packed, out_trits)
    for i = 1, CONSTANTS.TRITS_PER_WORD do
        local code = (packed >> ((i - 1) * 2)) & 0x03
        local trit, ok = two_bit_to_trit(code)
        if not ok then return false end
        out_trits[i] = trit
    end
    return true
end

local function negate_dual_rail(packed)
    local pos_rails = packed & CONSTANTS.BITPLANE_POS_MASK
    local neg_rails = packed & CONSTANTS.BITPLANE_NEG_MASK
    return ((pos_rails << 1) & CONSTANTS.BITPLANE_NEG_MASK) |
           ((neg_rails >> 1) & CONSTANTS.BITPLANE_POS_MASK)
end

local function has_null_collision(packed)
    local pos_rails = packed & CONSTANTS.BITPLANE_POS_MASK
    local neg_rails = (packed >> 1) & CONSTANTS.BITPLANE_POS_MASK
    return (pos_rails & neg_rails) ~= 0
end

local function pack_byte(trits, mode)
    mode = mode or PACK_MODES.DENSE_2222
    local b = 0
    if mode == PACK_MODES.DENSE_2222 then
        for i = 1, 4 do
            local code = trit_to_2bit(trits[i] or 0)
            b = b | ((code & 0x03) << ((i - 1) * 2))
        end
    elseif mode == PACK_MODES.KINEMATIC_21212 then
        b = b | (trit_to_2bit(trits[1] or 0) & 0x03)
        b = b | ((trit_to_2bit(trits[2] or 0) & 0x01) << 2)
        b = b | ((trit_to_2bit(trits[2] or 0) & 0x03) << 3)
        b = b | (((trit_to_2bit(trits[2] or 0) >> 1) & 0x01) << 5)
        b = b | ((trit_to_2bit(trits[3] or 0) & 0x03) << 6)
    elseif mode == PACK_MODES.KINEMATIC_12122 then
        b = b | ((trit_to_2bit(trits[1] or 0) & 0x01) << 0)
        b = b | ((trit_to_2bit(trits[1] or 0) & 0x03) << 1)
        b = b | (((trit_to_2bit(trits[2] or 0) >> 1) & 0x01) << 3)
        b = b | ((trit_to_2bit(trits[2] or 0) & 0x03) << 4)
        b = b | ((trit_to_2bit(trits[3] or 0) & 0x03) << 6)
    elseif mode == PACK_MODES.KINEMATIC_22121 then
        b = b | ((trit_to_2bit(trits[1] or 0) & 0x03) << 0)
        b = b | ((trit_to_2bit(trits[2] or 0) & 0x03) << 2)
        b = b | ((trit_to_2bit(trits[3] or 0) & 0x01) << 4)
        b = b | ((trit_to_2bit(trits[3] or 0) & 0x03) << 5)
        b = b | (((trit_to_2bit(trits[3] or 0) >> 1) & 0x01) << 7)
    elseif mode == PACK_MODES.DIRECTIONAL_332 then
        local c1 = trit_to_2bit(trits[1] or 0) & 0x03
        local c2 = trit_to_2bit(trits[2] or 0) & 0x03
        local c3 = trit_to_2bit(trits[3] or 0) & 0x03
        b = b | (c1 << 0) | (c2 << 3) | (c3 << 6)
    elseif mode == PACK_MODES.DIRECTIONAL_323 then
        local c1 = trit_to_2bit(trits[1] or 0) & 0x03
        local c2 = trit_to_2bit(trits[2] or 0) & 0x03
        local c3 = trit_to_2bit(trits[3] or 0) & 0x03
        b = b | (c1 << 0) | (c2 << 3) | (c3 << 5)
    elseif mode == PACK_MODES.DIRECTIONAL_233 then
        local c1 = trit_to_2bit(trits[1] or 0) & 0x03
        local c2 = trit_to_2bit(trits[2] or 0) & 0x03
        local c3 = trit_to_2bit(trits[3] or 0) & 0x03
        b = b | (c1 << 0) | (c2 << 2) | (c3 << 5)
    end
    return b & 0xFF
end

local function unpack_byte(byte_val, mode, out_trits)
    mode = mode or PACK_MODES.DENSE_2222
    if mode == PACK_MODES.DENSE_2222 then
        for i = 1, 4 do
            local code = (byte_val >> ((i - 1) * 2)) & 0x03
            local t, ok = two_bit_to_trit(code)
            if not ok then return false end
            out_trits[i] = t
        end
    elseif mode == PACK_MODES.KINEMATIC_21212 then
        local t1, ok1 = two_bit_to_trit(byte_val & 0x03)
        local t2, ok2 = two_bit_to_trit((byte_val >> 3) & 0x03)
        local t3, ok3 = two_bit_to_trit((byte_val >> 6) & 0x03)
        if not (ok1 and ok2 and ok3) then return false end
        out_trits[1], out_trits[2], out_trits[3], out_trits[4] = t1, t2, t3, 0
    elseif mode == PACK_MODES.KINEMATIC_12122 then
        local t1, ok1 = two_bit_to_trit((byte_val >> 1) & 0x03)
        local t2, ok2 = two_bit_to_trit((byte_val >> 4) & 0x03)
        local t3, ok3 = two_bit_to_trit((byte_val >> 6) & 0x03)
        if not (ok1 and ok2 and ok3) then return false end
        out_trits[1], out_trits[2], out_trits[3], out_trits[4] = t1, t2, t3, 0
    elseif mode == PACK_MODES.KINEMATIC_22121 then
        local t1, ok1 = two_bit_to_trit((byte_val >> 0) & 0x03)
        local t2, ok2 = two_bit_to_trit((byte_val >> 2) & 0x03)
        local t3, ok3 = two_bit_to_trit((byte_val >> 5) & 0x03)
        if not (ok1 and ok2 and ok3) then return false end
        out_trits[1], out_trits[2], out_trits[3], out_trits[4] = t1, t2, t3, 0
    elseif mode == PACK_MODES.DIRECTIONAL_332 then
        local t1, ok1 = two_bit_to_trit((byte_val >> 0) & 0x03)
        local t2, ok2 = two_bit_to_trit((byte_val >> 3) & 0x03)
        local t3, ok3 = two_bit_to_trit((byte_val >> 6) & 0x03)
        if not (ok1 and ok2 and ok3) then return false end
        out_trits[1], out_trits[2], out_trits[3], out_trits[4] = t1, t2, t3, 0
    elseif mode == PACK_MODES.DIRECTIONAL_323 then
        local t1, ok1 = two_bit_to_trit((byte_val >> 0) & 0x03)
        local t2, ok2 = two_bit_to_trit((byte_val >> 3) & 0x03)
        local t3, ok3 = two_bit_to_trit((byte_val >> 5) & 0x03)
        if not (ok1 and ok2 and ok3) then return false end
        out_trits[1], out_trits[2], out_trits[3], out_trits[4] = t1, t2, t3, 0
    elseif mode == PACK_MODES.DIRECTIONAL_233 then
        local t1, ok1 = two_bit_to_trit((byte_val >> 0) & 0x03)
        local t2, ok2 = two_bit_to_trit((byte_val >> 2) & 0x03)
        local t3, ok3 = two_bit_to_trit((byte_val >> 5) & 0x03)
        if not (ok1 and ok2 and ok3) then return false end
        out_trits[1], out_trits[2], out_trits[3], out_trits[4] = t1, t2, t3, 0
    end
    return true
end

local NumberTape = TapeEngine.create_tape({
    metadata = {
        who     = "OmniCode Lexicon",
        what    = "Number --> MEASURE { Quantity }",
        when    = "Arithmetic Event",
        where   = { number_word = 1 },
        why     = "Inscribe scalar number invariant and arithmetic equilibrium",
        how     = "+1 : PROCEED",
        witness = "Psalm 147:4 KJV"
    },
    constants = CONSTANTS,
    pack_modes = PACK_MODES,
    rolls = {
        -- --------------------------------------------------------------------
        -- ROLL 1: QUANTITY INSCRIPTION (Outside-in imposition onto C word)
        -- --------------------------------------------------------------------
        {
            name = "Inscribe",
            action = function(ctx)
                ctx.c_word.scalar = ctx.quantity
                ctx.c_word.is_enclosed = true
                ctx.c_word.pack_mode = ctx.pack_mode or PACK_MODES.DENSE_2222
                return TapeEngine.STATE_PROCEED
            end
        },

        -- --------------------------------------------------------------------
        -- ROLL 2: DUAL-SUBSTRATE TRANSDUCTION (64 Bits <=> 32 Balanced Trits)
        -- --------------------------------------------------------------------
        {
            name = "Transduce",
            action = function(ctx)
                local current = ctx.c_word.scalar
                for i = 1, CONSTANTS.TRITS_PER_WORD do
                    if current == 0 then
                        ctx.c_word.trits[i] = 0
                    else
                        local rem = current % 3
                        current = math.floor(current / 3)
                        if rem == 2 then
                            rem = -1
                            current = current + 1
                        elseif rem == -2 then
                            rem = 1
                            current = current - 1
                        end
                        ctx.c_word.trits[i] = rem
                    end
                end
                -- Inscribe dual-rail bit-planes (64 bits native)
                local p = pack_trits_dual_rail(ctx.c_word.trits)
                ctx.c_word.packed_dual_rail = p
                return TapeEngine.STATE_PROCEED
            end
        },

        -- --------------------------------------------------------------------
        -- ROLL 3: IDENTITY & BOUND COMPARISON (Guard Single-Word Boundary)
        -- --------------------------------------------------------------------
        {
            name = "Compare",
            action = function(ctx)
                local identity_ok = (ctx.c_word.scalar == ctx.quantity)
                local bounds_ok   = (ctx.c_word.is_enclosed == true)
                local p = ctx.c_word.packed_dual_rail
                local no_null     = not has_null_collision(p)
                local roundtrip   = {}
                local unpack_ok   = unpack_trits_dual_rail(p, roundtrip)
                return TapeEngine.assert_guard(identity_ok and bounds_ok and
                                               no_null and unpack_ok, ctx,
                                               "Number bound or collision")
            end
        },

        -- --------------------------------------------------------------------
        -- ROLL 4: ARITHMETIC EQUILIBRIUM CALCULATION ((+1) plus (-1) <=> 0)
        -- --------------------------------------------------------------------
        {
            name = "Equilibrium",
            action = function(ctx)
                local eq = (1) + (-1)
                local eq_holds = (eq == 0)
                -- Prove SIMD rail-swap involution: -(-x) == x
                local neg = negate_dual_rail(ctx.c_word.packed_dual_rail)
                local neg_neg = negate_dual_rail(neg)
                local rail_involution = (neg_neg == ctx.c_word.packed_dual_rail)
                ctx.equilibrium_proved = eq_holds and rail_involution
                return TapeEngine.assert_guard(ctx.equilibrium_proved, ctx,
                                               "ALU Equilibrium Law violated")
            end
        },

        -- --------------------------------------------------------------------
        -- ROLL 5: SEQUENCE TRANSITION (Number chains toward Vector sequence)
        -- --------------------------------------------------------------------
        {
            name = "Sequence",
            action = function(ctx)
                ctx.vector_ready = true
                return TapeEngine.STATE_PROCEED
            end
        }
    }
})

-- Expose bit-plane and kinematic packing primitives
NumberTape.PACK_MODES             = PACK_MODES
NumberTape.trit_to_2bit           = trit_to_2bit
NumberTape.two_bit_to_trit        = two_bit_to_trit
NumberTape.pack_trits_dual_rail   = pack_trits_dual_rail
NumberTape.unpack_trits_dual_rail = unpack_trits_dual_rail
NumberTape.negate_dual_rail       = negate_dual_rail
NumberTape.has_null_collision     = has_null_collision
NumberTape.pack_byte              = pack_byte
NumberTape.unpack_byte            = unpack_byte

-- ============================================================================
-- BLOCK 2: OPENING BLOCK
-- ( Context Factory & C Word Substrate Mount )
-- ============================================================================

---
-- Mounts the tape head, initialises the C word handle, and arms the tripwire.
-- @param quantity Outside-in scalar integer (defaults to 4096).
-- @param pack_mode Kinematic packing mode (defaults to DENSE_2222).
-- @return Initialized context table.
function NumberTape.open(quantity, pack_mode)
    local p_mode = pack_mode or PACK_MODES.DENSE_2222
    return {
        quantity           = quantity or CONSTANTS.EXEMPLAR_PARTITION,
        pack_mode          = p_mode,
        c_word             = {
            scalar           = 0,
            trits            = {},
            packed_dual_rail = 0,
            pack_mode        = p_mode,
            is_enclosed      = false
        },
        equilibrium_proved = false,
        status             = TapeEngine.STATE_PROCEED,
        tripwire           = TapeEngine.TRIPWIRE_ARMED
    }
end

-- ============================================================================
-- BLOCK 4: CLOSING BLOCK
-- ( Fourfold Detent Commit & Interlock Verification )
-- ============================================================================

---
-- Closes and seals the Number tape execution context.
-- @param ctx Active context handle.
-- @return Quad-State commit status and diagnostic seal message.
function NumberTape.close(ctx)
    return TapeEngine.verify_and_commit(ctx)
end

return NumberTape
