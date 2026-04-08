init_vault_token_account:
    # frame.cpi[0].meta.pubkey = &acct.address
    # frame.cpi[0].info.key = &acct.address
    mov64 r8, r7
    add64 r8, ACCT_ADDRESS_OFF
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_META_PUBKEY_UOFF], r8
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_KEY_UOFF], r8
    # frame.cpi[0].info.owner = &acct.owner
    add64 r8, IB_ADDRESS_TO_OWNER_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_OWNER_UOFF], r8
    # frame.cpi[0].info.lamports = &acct.lamports
    add64 r8, IB_OWNER_TO_LAMPORTS_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_LAMPORTS_UOFF], r8
    # frame.cpi[0].info.data = &acct.data
    add64 r8, IB_LAMPORTS_TO_DATA_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_DATA_UOFF], r8
    # frame.cpi[0].info.data_len = acct.data_len
    ldxdw r8, [r7 + ACCT_DATA_LEN_OFF]
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_DATA_LEN_UOFF], r8
    # mint = frame.mint
    ldxdw r8, [r6 + RM_FM_MINT_OFF]
    # frame.cpi[1].info.is_signer = false
    # frame.cpi[1].info.is_writable = false
    sth [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_IS_SIGNER_UOFF], CPI_READONLY_NON_SIGNER
    # frame.cpi[1].meta.is_writable = false
    # frame.cpi[1].meta.is_signer = false
    sth [r6 + RM_FM_CPI_IDX_1_ACCT_META_IS_WRITABLE_UOFF], CPI_READONLY_NON_SIGNER
    # frame.cpi[1].meta.pubkey = &mint.address
    # frame.cpi[1].info.key = &mint.address
    mov64 r9, r8
    add64 r9, ACCT_ADDRESS_OFF
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_META_PUBKEY_UOFF], r9
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_KEY_UOFF], r9
    # frame.cpi[1].info.owner = &mint.owner
    add64 r9, IB_ADDRESS_TO_OWNER_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_OWNER_UOFF], r9
    # frame.cpi[1].info.lamports = &mint.lamports
    add64 r9, IB_OWNER_TO_LAMPORTS_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_LAMPORTS_UOFF], r9
    # frame.cpi[1].info.data = &mint.data
    add64 r9, IB_LAMPORTS_TO_DATA_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_DATA_UOFF], r9
    # frame.cpi[1].info.data_len = mint.data_len
    ldxdw r9, [r8 + ACCT_DATA_LEN_OFF]
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_DATA_LEN_UOFF], r9
    # rent = frame.rent
    ldxdw r8, [r6 + RM_FM_RENT_OFF]
    # frame.cpi[2].meta.pubkey = &rent.address
    # frame.cpi[2].info.key = &rent.address
    mov64 r9, r8
    add64 r9, ACCT_ADDRESS_OFF
    stxdw [r6 + RM_FM_CPI_IDX_2_ACCT_META_PUBKEY_UOFF], r9
    stxdw [r6 + RM_FM_CPI_IDX_2_ACCT_INFO_KEY_UOFF], r9
    # frame.cpi[2].info.owner = &rent.owner
    add64 r9, IB_ADDRESS_TO_OWNER_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_2_ACCT_INFO_OWNER_UOFF], r9
    # frame.cpi[2].info.lamports = &rent.lamports
    add64 r9, IB_OWNER_TO_LAMPORTS_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_2_ACCT_INFO_LAMPORTS_UOFF], r9
    # frame.cpi[2].info.data = &rent.data
    add64 r9, IB_LAMPORTS_TO_DATA_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_2_ACCT_INFO_DATA_UOFF], r9
    # frame.cpi[2].info.data_len = rent.data_len
    ldxdw r9, [r8 + ACCT_DATA_LEN_OFF]
    stxdw [r6 + RM_FM_CPI_IDX_2_ACCT_INFO_DATA_LEN_UOFF], r9
    # frame.sol_instruction.program_id = frame.token_program_id
    ldxdw r8, [r6 + RM_FM_TOKEN_PROGRAM_ID_OFF]
    stxdw [r6 + RM_FM_SOL_INSN_PROGRAM_ID_UOFF], r8
    # frame.sol_instruction.accounts = &frame.cpi.account_metas
    mov64 r8, r6
    add64 r8, RM_FM_CPI_IDX_0_ACCT_META_PUBKEY_UOFF
    stxdw [r6 + RM_FM_SOL_INSN_ACCOUNTS_UOFF], r8
    # frame.sol_instruction.account_len = common::token::INITIALIZE_ACCOUNT_2_N_ACCOUNTS
    mov64 r8, TOKEN_INITIALIZE_ACCOUNT_2_N_ACCOUNTS
    stxdw [r6 + RM_FM_SOL_INSN_ACCOUNT_LEN_UOFF], r8
    # frame.initialize_account_2_data.discriminant = common::token::INITIALIZE_ACCOUNT_2_DISC
    stb [r6 + RM_FM_INIT_ACCT_2_DISC_UOFF], TOKEN_INITIALIZE_ACCOUNT_2_DISC
    # frame.initialize_account_2_data.proprietor = input.market.address
    ldxdw r8, [r6 + RM_FM_INPUT_OFF]
    add64 r8, IB_MARKET_ADDRESS_OFF
    ldxdw r9, [r8 + PUBKEY_CHUNK_0_OFF]
    stxdw [r6 + RM_FM_INIT_ACCT_2_PROPRIETOR_CHUNK_0_UOFF], r9
    ldxdw r9, [r8 + PUBKEY_CHUNK_1_OFF]
    stxdw [r6 + RM_FM_INIT_ACCT_2_PROPRIETOR_CHUNK_1_UOFF], r9
    ldxdw r9, [r8 + PUBKEY_CHUNK_2_OFF]
    stxdw [r6 + RM_FM_INIT_ACCT_2_PROPRIETOR_CHUNK_2_UOFF], r9
    ldxdw r9, [r8 + PUBKEY_CHUNK_3_OFF]
    stxdw [r6 + RM_FM_INIT_ACCT_2_PROPRIETOR_CHUNK_3_UOFF], r9
    # frame.sol_instruction.data = &frame.initialize_account_2_data
    mov64 r8, r6
    add64 r8, RM_FM_INIT_ACCT_2_DATA_OFF
    stxdw [r6 + RM_FM_SOL_INSN_DATA_UOFF], r8
    # frame.sol_instruction.data_len = InitializeAccount2.size
    mov64 r8, SIZE_OF_INITIALIZE_ACCOUNT2
    stxdw [r6 + RM_FM_SOL_INSN_DATA_LEN_UOFF], r8
    # syscall.instruction = &frame.sol_instruction
    mov64 r1, r6
    add64 r1, RM_FM_SOL_INSN_OFF
    # syscall.account_infos = &frame.cpi.account_infos
    mov64 r2, r6
    add64 r2, RM_FM_CPI_SOL_ACCT_INFO_OFF
    # syscall.account_infos_len = common::token::INITIALIZE_ACCOUNT_2_N_ACCOUNTS
    mov64 r3, TOKEN_INITIALIZE_ACCOUNT_2_N_ACCOUNTS
    # syscall.seeds_len = common::token::INITIALIZE_ACCOUNT_2_N_SEEDS
    mov64 r5, TOKEN_INITIALIZE_ACCOUNT_2_N_SEEDS
    call sol_invoke_signed_c
    ja init_vault_token_account_return
