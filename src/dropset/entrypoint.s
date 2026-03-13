entrypoint:
    ldxdw r3, [r2 - 8]
    ldxb r4, [r2 + 0]
    jeq r4, 0, register_market
    mov32 r0, 1
    exit
