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
# From address to owner in a runtime account.
.equ IB_ADDRESS_TO_OWNER_REL_OFF_IMM, 32
# From owner to lamports in a runtime account.
.equ IB_OWNER_TO_LAMPORTS_REL_OFF_IMM, 32
# From lamports to data start in a runtime account.
.equ IB_LAMPORTS_TO_DATA_REL_OFF_IMM, 16
# From user data to market address in the input buffer.
.equ IB_USER_DATA_TO_MARKET_ADDRESS_REL_OFF_IMM, 10256
# From input buffer to market header next pointer.
.equ IB_MARKET_HEADER_NEXT_OFF, 10464
# From input buffer to market header bump.
.equ IB_MARKET_HEADER_BUMP_OFF, 10536
# From input buffer to market header base vault bump.
.equ IB_MARKET_HEADER_BASE_VAULT_BUMP_OFF, 10569
# From input buffer to market header quote vault bump.
.equ IB_MARKET_HEADER_QUOTE_VAULT_BUMP_OFF, 10602
# From input buffer to first sector in market memory map.
.equ IB_MARKET_SECTORS_START_OFF, 10627
# Absolute SBPF pointer to first sector in market memory map.
.equ IB_MARKET_SECTORS_START_PTR_WD, 17179879811
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
