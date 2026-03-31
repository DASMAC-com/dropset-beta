init_vault:
    mov64 r6, r2
    mov64 r7, r1
    # frame.acct = acct
    stxdw [r6 + RM_FM_ACCT_OFF], r1
    # frame.pda_seeds[0].addr = &input.market.address
    ldxdw r8, [r6 + RM_FM_INPUT_OFF]
    add64 r8, IB_MARKET_PUBKEY_OFF
    stxdw [r6 + RM_FM_PDA_SEEDS_IDX_0_ADDR_OFF], r8
    # frame.pda_seeds[1].addr = &frame.vault_index
    mov64 r8, r6
    add64 r8, RM_FM_VAULT_INDEX_UOFF
    stxdw [r6 + RM_FM_PDA_SEEDS_IDX_1_ADDR_OFF], r8
    # frame.pda_seeds[1].len = u8.size
    mov64 r8, SIZE_OF_U8
    stxdw [r6 + RM_FM_PDA_SEEDS_IDX_1_LEN_OFF], r8
    # syscall.seeds = &frame.pda_seeds
    mov64 r1, r6
    add64 r1, RM_FM_PDA_SEEDS_OFF
    # syscall.seeds_len = register_misc.TRY_FIND_VAULT_PDA_SEEDS_LEN
    mov64 r2, RM_MISC_TRY_FIND_VAULT_PDA_SEEDS_LEN
    # syscall.program_id = frame.token_program_id
    ldxdw r3, [r6 + RM_FM_TOKEN_PROGRAM_ID_OFF]
    # syscall.program_address = &frame.pda
    mov64 r4, r6
    add64 r4, RM_FM_PDA_OFF
    # syscall.bump_seed = &frame.bump
    mov64 r5, r6
    add64 r5, RM_FM_BUMP_OFF
    call sol_try_find_program_address
    # Verify vault address matches derived PDA.
    # r7 = acct, r6 = frame (callee-saved, preserved across syscall)
    ldxdw r1, [r7 + ACCT_ADDRESS_CHUNK_0_OFF]
    ldxdw r2, [r6 + RM_FM_PDA_CHUNK_0_OFF]
    jne r1, r2, init_vault_invalid_pda
    ldxdw r1, [r7 + ACCT_ADDRESS_CHUNK_1_OFF]
    ldxdw r2, [r6 + RM_FM_PDA_CHUNK_1_OFF]
    jne r1, r2, init_vault_invalid_pda
    ldxdw r1, [r7 + ACCT_ADDRESS_CHUNK_2_OFF]
    ldxdw r2, [r6 + RM_FM_PDA_CHUNK_2_OFF]
    jne r1, r2, init_vault_invalid_pda
    ldxdw r1, [r7 + ACCT_ADDRESS_CHUNK_3_OFF]
    ldxdw r2, [r6 + RM_FM_PDA_CHUNK_3_OFF]
    jne r1, r2, init_vault_invalid_pda
    # Restore acct pointer. r0 = FLOW_RESULT_OK from syscall.
    mov64 r1, r7
    exit
init_vault_invalid_pda:
    # if frame.vault_index == register_misc.VAULT_INDEX_BASE
    #     return ErrorCode::InvalidBaseVaultPubkey
    # else
    #     return ErrorCode::InvalidQuoteVaultPubkey
    ldxb r1, [r6 + RM_FM_VAULT_INDEX_UOFF]
    jeq r1, RM_MISC_VAULT_INDEX_BASE, e_invalid_base_vault_pubkey
    mov32 r0, E_INVALID_QUOTE_VAULT_PUBKEY
    exit
