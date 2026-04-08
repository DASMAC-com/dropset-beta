market_pda_prelude:
    # if input.n_accounts < Accounts.count
    #     return ErrorCode::InvalidNumberOfAccounts
    jlt r3, RM_INSN_ACCTS_COUNT, e_invalid_number_of_accounts
    # if insn_len != Data.size
    #     return ErrorCode::InvalidInstructionLength
    jne r4, RM_INSN_DATA_SIZE, e_invalid_instruction_length
    # frame.program_id = &insn.program_id
    mov64 r4, r2
    add64 r4, RM_INSN_DATA_SIZE
    stxdw [r10 + RM_FM_PROGRAM_ID_OFF], r4
    # if input.user.data_len != common::memory::LEN_ZERO
    #     return ErrorCode::UserHasData
    ldxdw r9, [r1 + IB_USER_DATA_LEN_OFF]
    jne r9, DATA_LEN_ZERO, e_user_has_data
    # if input.market.duplicate != common::account::NON_DUP_MARKER
    #     return ErrorCode::MarketAccountIsDuplicate
    ldxb r9, [r1 + IB_MARKET_DUPLICATE_OFF]
    jne r9, ACCT_NON_DUP_MARKER, e_market_account_is_duplicate
    # if input.market.data_len != DATA_LEN_ZERO
    #     return ErrorCode::MarketHasData
    ldxdw r9, [r1 + IB_MARKET_DATA_LEN_OFF]
    jne r9, DATA_LEN_ZERO, e_market_has_data
    # if input.base_mint.duplicate != common::account::NON_DUP_MARKER
    #     return ErrorCode::BaseMintIsDuplicate
    ldxb r9, [r1 + RM_BASE_DUPLICATE_OFF]
    jne r9, ACCT_NON_DUP_MARKER, e_base_mint_is_duplicate
    # input_shifted = input + input.base_mint.padded_data_len
    ldxdw r9, [r1 + RM_BASE_DATA_LEN_OFF]
    add64 r9, DATA_LEN_MAX_PAD
    and64 r9, DATA_LEN_AND_MASK
    add64 r9, r1
    # frame.input_shifted = input_shifted
    stxdw [r10 + RM_FM_INPUT_SHIFTED_OFF], r9
    # if input_shifted.quote_mint.duplicate != common::account::NON_DUP_MARKER
    #     return ErrorCode::QuoteMintIsDuplicate
    ldxb r8, [r9 + RM_QUOTE_DUPLICATE_OFF]
    jne r8, ACCT_NON_DUP_MARKER, e_quote_mint_is_duplicate
    # quote_mint_padded_data_len = input_shifted.quote_mint.padded_data_len
    ldxdw r8, [r9 + RM_QUOTE_DATA_LEN_OFF]
    add64 r8, DATA_LEN_MAX_PAD
    and64 r8, DATA_LEN_AND_MASK
    # acct = &input_shifted.quote_mint
    add64 r9, RM_QUOTE_OFF
    # acct += quote_mint_padded_data_len + EmptyAccount.size
    add64 r9, r8
    add64 r9, SIZE_OF_EMPTY_ACCOUNT
    # if acct.duplicate != common::account::NON_DUP_MARKER
    #     return ErrorCode::SystemProgramIsDuplicate
    ldxb r7, [r9 + ACCT_DUPLICATE_OFF]
    jne r7, ACCT_NON_DUP_MARKER, e_system_program_is_duplicate
    # if acct.pubkey != frame.system_program_pubkey
    #     return ErrorCode::InvalidSystemProgramPubkey
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_0_OFF]
    ldxdw r8, [r10 + RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_0_OFF]
    jne r7, r8, e_invalid_system_program_pubkey
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_1_OFF]
    ldxdw r8, [r10 + RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_1_OFF]
    jne r7, r8, e_invalid_system_program_pubkey
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_2_OFF]
    ldxdw r8, [r10 + RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_2_OFF]
    jne r7, r8, e_invalid_system_program_pubkey
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_3_OFF]
    ldxdw r8, [r10 + RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_3_OFF]
    jne r7, r8, e_invalid_system_program_pubkey
    # frame.system_program_id = &acct.address
    mov64 r7, r9
    add64 r7, ACCT_ADDRESS_OFF
    stxdw [r10 + RM_FM_SYSTEM_PROGRAM_ID_OFF], r7
    # system_program_padded_data_len = acct.padded_data_len
    ldxdw r7, [r9 + ACCT_DATA_LEN_OFF]
    add64 r7, DATA_LEN_MAX_PAD
    and64 r7, DATA_LEN_AND_MASK
    # acct += system_program_padded_data_len + EmptyAccount.size
    add64 r9, r7
    add64 r9, SIZE_OF_EMPTY_ACCOUNT
    # if acct.duplicate != common::account::NON_DUP_MARKER
    #     return ErrorCode::RentSysvarIsDuplicate
    ldxb r7, [r9 + ACCT_DUPLICATE_OFF]
    jne r7, ACCT_NON_DUP_MARKER, e_rent_sysvar_is_duplicate
    # if acct.pubkey != common::pubkey::RENT
    #     return ErrorCode::InvalidRentSysvarPubkey
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_0_OFF]
    lddw r8, PUBKEY_RENT_CHUNK_0
    jne r7, r8, e_invalid_rent_sysvar_pubkey
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_1_OFF]
    lddw r8, PUBKEY_RENT_CHUNK_1
    jne r7, r8, e_invalid_rent_sysvar_pubkey
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_2_OFF]
    lddw r8, PUBKEY_RENT_CHUNK_2
    jne r7, r8, e_invalid_rent_sysvar_pubkey
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_3_OFF]
    # region: optimize_example
    # Optimize: common::pubkey::RENT chunk 3 hi bits are zero, so mov32
    # (1 CU) replaces lddw (2 CUs).
    mov32 r8, PUBKEY_RENT_CHUNK_3_LO
    # endregion: optimize_example
    jne r7, r8, e_invalid_rent_sysvar_pubkey
    # frame.rent = acct
    stxdw [r10 + RM_FM_RENT_OFF], r9
    ja market_pda_prelude_return
