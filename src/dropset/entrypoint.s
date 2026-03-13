entrypoint:
    ldxdw r3, [r2 - 8] # r3 = data_l
    ldxb r4, [r2 + 0] # r4 = d_d
    jeq r4, 0, register_market
    mov32 r0, 1
    exit
