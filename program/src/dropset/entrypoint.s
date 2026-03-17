# Offset from instruction data to instruction data length, in input buffer.
.equ INSN_LEN_OFF, -8
# Offset from instruction data to discriminant, in input buffer.
.equ INSN_DISC_OFF, 0

entrypoint:
    # insn_len = insn.length
    ldxdw r3, [r2 + INSN_LEN_OFF]
    # insn_disc = insn.discriminant
    ldxb r4, [r2 + INSN_DISC_OFF]
    # if insn_disc == Discriminant::RegisterMarket return REGISTER-MARKET
    jeq r4, DISC_REGISTER_MARKET, register_market
    # return ErrorCode::InvalidDiscriminant
    mov32 r0, E_INVALID_DISCRIMINANT
    exit
