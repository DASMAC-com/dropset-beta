# Instruction discriminants.
# -------------------------------------------------------------------------
.equ DISC_REGISTER_MARKET, 0 # Register a new market.
# -------------------------------------------------------------------------

# General entrypoint-related constants.
# -------------------------------------------------------------------------
.equ N_ACCTS_OFF, 0 # Offset from input buffer to number of accounts.
# Offset from instruction data to instruction data length.
.equ INSN_LEN_OFF, -8
.equ INSN_DISC_OFF, 0 # Offset from instruction data to discriminant.
.equ RETURN_SUCCESS, 0 # Successful return code.
# -------------------------------------------------------------------------

# Constants for static input buffer header with empty user, then market.
# -------------------------------------------------------------------------
.equ IB_USER_DATA_LEN_OFF, 88 # From input buffer to user data length.
.equ IB_USER_ADDRESS_OFF, 16 # From input buffer to user address field.
# From input buffer to user address field (chunk 0).
.equ IB_USER_ADDRESS_CHUNK_0_OFF, 16
# From input buffer to user address field (chunk 1).
.equ IB_USER_ADDRESS_CHUNK_1_OFF, 24
# From input buffer to user address field (chunk 2).
.equ IB_USER_ADDRESS_CHUNK_2_OFF, 32
# From input buffer to user address field (chunk 3).
.equ IB_USER_ADDRESS_CHUNK_3_OFF, 40
# From input buffer to market duplicate flag.
.equ IB_MARKET_DUPLICATE_OFF, 10344
# From input buffer to market data length.
.equ IB_MARKET_DATA_LEN_OFF, 10424
# From input buffer to market address field.
.equ IB_MARKET_ADDRESS_OFF, 10352
# From input buffer to market address field (chunk 0).
.equ IB_MARKET_ADDRESS_CHUNK_0_OFF, 10352
# From input buffer to market address field (chunk 1).
.equ IB_MARKET_ADDRESS_CHUNK_1_OFF, 10360
# From input buffer to market address field (chunk 2).
.equ IB_MARKET_ADDRESS_CHUNK_2_OFF, 10368
# From input buffer to market address field (chunk 3).
.equ IB_MARKET_ADDRESS_CHUNK_3_OFF, 10376
# From user data to market address in the input buffer.
.equ IB_USER_DATA_TO_MARKET_ADDRESS_REL_OFF_IMM, 10256
# -------------------------------------------------------------------------

entrypoint:
    # n_accounts = input.n_accounts
    ldxdw r3, [r1 + N_ACCTS_OFF]
    # len = insn.length
    ldxdw r4, [r2 + INSN_LEN_OFF]
    # disc = insn.discriminant
    ldxb r5, [r2 + INSN_DISC_OFF]
    # if insn_disc == Discriminant::RegisterMarket return REGISTER-MARKET
    jeq r5, DISC_REGISTER_MARKET, register_market
    # return ErrorCode::InvalidDiscriminant
    mov32 r0, E_INVALID_DISCRIMINANT
    exit
