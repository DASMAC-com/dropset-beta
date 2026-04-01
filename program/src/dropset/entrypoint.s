# Offset from input buffer to number of accounts, in input buffer.
.equ IB_N_ACCTS_OFF, 0
# Offset from instruction data to instruction data length, in input buffer.
.equ INSN_LEN_OFF, -8
# Offset from instruction data to discriminant, in input buffer.
.equ INSN_DISC_OFF, 0
.equ RETURN_SUCCESS, 0 # Successful return code.

entrypoint:
    # n_accounts = input.n_accounts
    ldxdw r3, [r1 + IB_N_ACCTS_OFF]
    # insn_len = insn.length
    ldxdw r4, [r2 + INSN_LEN_OFF]
    # insn_disc = insn.discriminant
    ldxb r5, [r2 + INSN_DISC_OFF]
    # if insn_disc == Discriminant::RegisterMarket return REGISTER-MARKET
    jeq r5, DISC_REGISTER_MARKET, register_market
    # return ErrorCode::InvalidDiscriminant
    mov32 r0, E_INVALID_DISCRIMINANT
    exit
