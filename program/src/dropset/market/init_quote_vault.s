init_quote_vault:
    # if acct.duplicate == common::account::NON_DUP_MARKER
    ldxb r7, [r9 + ACCT_DUPLICATE_OFF]
    jne r7, ACCT_NON_DUP_MARKER, init_quote_vault_dup
    # if acct.pubkey != input_shifted.quote_mint.owner
    #     return ErrorCode::NonDupQuoteTokenProgramNotQuoteMintOwner
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_0_OFF]
    ldxdw r2, [r6 + RM_QUOTE_OWNER_CHUNK_0_OFF]
    jne r7, r2, e_non_dup_quote_token_program_not_quote_mint_owner
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_1_OFF]
    ldxdw r2, [r6 + RM_QUOTE_OWNER_CHUNK_1_OFF]
    jne r7, r2, e_non_dup_quote_token_program_not_quote_mint_owner
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_2_OFF]
    ldxdw r2, [r6 + RM_QUOTE_OWNER_CHUNK_2_OFF]
    jne r7, r2, e_non_dup_quote_token_program_not_quote_mint_owner
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_3_OFF]
    ldxdw r2, [r6 + RM_QUOTE_OWNER_CHUNK_3_OFF]
    jne r7, r2, e_non_dup_quote_token_program_not_quote_mint_owner
    # if acct.pubkey != common::pubkey::TOKEN_PROGRAM
    #     if acct.pubkey != common::pubkey::TOKEN_2022_PROGRAM
    #         return ErrorCode::QuoteTokenProgramNotTokenProgram
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_0_OFF]
    lddw r2, PUBKEY_TOKEN_PROGRAM_CHUNK_0
    jne r7, r2, init_quote_vault_check_token_2022
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_1_OFF]
    lddw r2, PUBKEY_TOKEN_PROGRAM_CHUNK_1
    jne r7, r2, init_quote_vault_check_token_2022
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_2_OFF]
    lddw r2, PUBKEY_TOKEN_PROGRAM_CHUNK_2
    jne r7, r2, init_quote_vault_check_token_2022
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_3_OFF]
    lddw r2, PUBKEY_TOKEN_PROGRAM_CHUNK_3
    jeq r7, r2, init_quote_vault_is_token_program
init_quote_vault_check_token_2022:
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_0_OFF]
    lddw r2, PUBKEY_TOKEN_2022_PROGRAM_CHUNK_0
    jne r7, r2, e_quote_token_program_not_token_program
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_1_OFF]
    lddw r2, PUBKEY_TOKEN_2022_PROGRAM_CHUNK_1
    jne r7, r2, e_quote_token_program_not_token_program
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_2_OFF]
    lddw r2, PUBKEY_TOKEN_2022_PROGRAM_CHUNK_2
    jne r7, r2, e_quote_token_program_not_token_program
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_3_OFF]
    lddw r2, PUBKEY_TOKEN_2022_PROGRAM_CHUNK_3
    jne r7, r2, e_quote_token_program_not_token_program
    # frame.token_program_is_2022 = true
    stb [r10 + RM_FM_TOKEN_PROGRAM_IS_2022_UOFF], DATA_BOOL_TRUE
    ja init_quote_vault_advance_non_dup
init_quote_vault_is_token_program:
    # frame.token_program_is_2022 = false
    stb [r10 + RM_FM_TOKEN_PROGRAM_IS_2022_UOFF], DATA_BOOL_FALSE
init_quote_vault_advance_non_dup:
    # frame.token_program_id = &acct.address
    mov64 r7, r9
    add64 r7, ACCT_ADDRESS_OFF
    stxdw [r10 + RM_FM_TOKEN_PROGRAM_ID_OFF], r7
    # quote_token_program_padded_data_len = acct.padded_data_len
    ldxdw r7, [r9 + ACCT_DATA_LEN_OFF]
    add64 r7, DATA_LEN_MAX_PAD
    and64 r7, DATA_LEN_AND_MASK
    # acct += quote_token_program_padded_data_len + EmptyAccount.size
    add64 r9, r7
    add64 r9, SIZE_OF_EMPTY_ACCOUNT
    ja init_quote_vault_done_token_program
init_quote_vault_dup:
    # if acct.duplicate != Accounts::BaseTokenProgram
    #     return ErrorCode::InvalidQuoteTokenProgramDuplicate
    jne r7, RM_INSN_ACCTS_BASE_TOKEN_PROGRAM_POS, e_invalid_quote_token_program_duplicate
    # if input.base_mint.owner != input_shifted.quote_mint.owner
    #     return ErrorCode::DupQuoteTokenProgramNotQuoteMintOwner
    ldxdw r7, [r8 + RM_BASE_OWNER_CHUNK_0_OFF]
    ldxdw r2, [r6 + RM_QUOTE_OWNER_CHUNK_0_OFF]
    jne r7, r2, e_dup_quote_token_program_not_quote_mint_owner
    ldxdw r7, [r8 + RM_BASE_OWNER_CHUNK_1_OFF]
    ldxdw r2, [r6 + RM_QUOTE_OWNER_CHUNK_1_OFF]
    jne r7, r2, e_dup_quote_token_program_not_quote_mint_owner
    ldxdw r7, [r8 + RM_BASE_OWNER_CHUNK_2_OFF]
    ldxdw r2, [r6 + RM_QUOTE_OWNER_CHUNK_2_OFF]
    jne r7, r2, e_dup_quote_token_program_not_quote_mint_owner
    ldxdw r7, [r8 + RM_BASE_OWNER_CHUNK_3_OFF]
    ldxdw r2, [r6 + RM_QUOTE_OWNER_CHUNK_3_OFF]
    jne r7, r2, e_dup_quote_token_program_not_quote_mint_owner
    # acct += u64.size
    add64 r9, SIZE_OF_U64
init_quote_vault_done_token_program:
    # if acct.duplicate != common::account::NON_DUP_MARKER
    #     return ErrorCode::QuoteVaultIsDuplicate
    ldxb r7, [r9 + ACCT_DUPLICATE_OFF]
    jne r7, ACCT_NON_DUP_MARKER, e_quote_vault_is_duplicate
    # if acct.data_len != common::memory::LEN_ZERO
    #     return ErrorCode::QuoteVaultHasData
    ldxdw r7, [r9 + ACCT_DATA_LEN_OFF]
    jne r7, DATA_LEN_ZERO, e_quote_vault_has_data
    # frame.vault_index = market::VAULT_INDEX_QUOTE
    stb [r10 + RM_FM_VAULT_INDEX_UOFF], MKT_VAULT_INDEX_QUOTE
    # frame.mint = &input_shifted.quote_mint
    mov64 r7, r6
    add64 r7, RM_QUOTE_DUPLICATE_OFF
    stxdw [r10 + RM_FM_MINT_OFF], r7
    # INIT-VAULT(acct, frame)
    mov64 r1, r9
    mov64 r2, r10
    call init_vault
    # input.market_header.quote_vault_bump = frame.bump
    ldxb r7, [r10 + RM_FM_BUMP_OFF]
    stxb [r8 + IB_MARKET_HEADER_QUOTE_VAULT_BUMP_OFF], r7
    ja init_quote_vault_return
