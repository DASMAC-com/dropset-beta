entrypoint:
    # insn_len = insn.length
    ldxdw r3, [r2 - 8]
    # insn_d = insn.discriminant
    ldxb r4, [r2 + 0]
    # if insn_d == Discriminant::RegisterMarket \
    # return register_market
    jeq r4, 0, register_market
    # return Error::InvalidDiscriminant
    mov32 r0, 1
    exit
