init_vault:
    # Store(frame)
    mov64 r6, r2
    # Store(acct)
    mov64 r7, r1
    # frame.pda_seeds[0].addr = &input.market.address
    ldxdw r8, [r6 + RM_FM_INPUT_OFF]
    add64 r8, IB_MARKET_ADDRESS_OFF
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
    # syscall.seeds_len = market::register::TRY_FIND_VAULT_PDA_SEEDS_LEN
    mov64 r2, RM_TRY_FIND_VAULT_PDA_SEEDS_LEN
    # syscall.program_id = frame.program_id
    ldxdw r3, [r6 + RM_FM_PROGRAM_ID_OFF]
    # syscall.program_address = &frame.pda
    mov64 r4, r6
    add64 r4, RM_FM_PDA_OFF
    # syscall.bump_seed = &frame.bump
    mov64 r5, r6
    add64 r5, RM_FM_BUMP_OFF
    call sol_try_find_program_address
    # if acct.address != frame.pda
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
    # acct_size = GET-VAULT-SIZE(frame)
    ja get_vault_size
get_vault_size_return:
    # CREATE-VAULT-ACCOUNT(frame, acct)
    ja create_vault_account
create_vault_account_return:
    # INIT-VAULT-TOKEN-ACCOUNT(frame, acct)
    ja init_vault_token_account
init_vault_token_account_return:
    exit
init_vault_invalid_pda:
    # if frame.vault_index == market::VAULT_INDEX_BASE
    #     return ErrorCode::InvalidBaseVaultPubkey
    # else
    #     return ErrorCode::InvalidQuoteVaultPubkey
    ldxb r1, [r6 + RM_FM_VAULT_INDEX_UOFF]
    jeq r1, MKT_VAULT_INDEX_BASE, e_invalid_base_vault_pubkey
    mov32 r0, E_INVALID_QUOTE_VAULT_PUBKEY
    exit
