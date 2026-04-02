init_market_pda:
    # frame.create_account_data.space = MarketHeader.size
    mov64 r7, SIZE_OF_MARKET_HEADER
    stxdw [r10 + RM_FM_CREATE_ACCT_SPACE_UOFF], r7
    # acct_size = MarketHeader.size + account.STORAGE_OVERHEAD
    add64 r7, ACCT_STORAGE_OVERHEAD
    # lamports_per_byte = acct.data.lamports_per_byte
    ldxdw r8, [r9 + ACCT_DATA_OFF]
    # frame.lamports_per_byte = lamports_per_byte
    stxdw [r10 + RM_FM_LAMPORTS_PER_BYTE_OFF], r8
    # frame.create_account_data.lamports = acct_size * lamports_per_byte
    mul64 r7, r8
    stxdw [r10 + RM_FM_CREATE_ACCT_LAMPORTS_UOFF], r7
    # frame.pda_seeds[0].addr = input.base_mint.address
    mov64 r8, r1
    add64 r8, RM_MISC_BASE_ADDR_OFF
    stxdw [r10 + RM_FM_PDA_SEEDS_IDX_0_ADDR_OFF], r8
    # frame.pda_seeds[0].len = Address.size
    mov64 r8, SIZE_OF_ADDRESS
    stxdw [r10 + RM_FM_PDA_SEEDS_IDX_0_LEN_OFF], r8
    # frame.pda_seeds[1].addr = input_shifted.quote_mint.address
    ldxdw r8, [r10 + RM_FM_INPUT_SHIFTED_OFF]
    add64 r8, RM_MISC_QUOTE_ADDR_OFF
    stxdw [r10 + RM_FM_PDA_SEEDS_IDX_1_ADDR_OFF], r8
    # frame.pda_seeds[1].len = Address.size
    mov64 r8, SIZE_OF_ADDRESS
    stxdw [r10 + RM_FM_PDA_SEEDS_IDX_1_LEN_OFF], r8
    # frame.input = input
    stxdw [r10 + RM_FM_INPUT_OFF], r1
    # syscall.seeds = &frame.pda_seeds
    mov64 r1, r10
    add64 r1, RM_FM_PDA_SEEDS_OFF
    # syscall.program_id = &insn.program_id
    mov64 r3, r2
    add64 r3, REGISTER_MARKET_DATA_LEN
    # syscall.seeds_len = register_misc.TRY_FIND_MARKET_PDA_SEEDS_LEN
    mov64 r2, RM_MISC_TRY_FIND_MARKET_PDA_SEEDS_LEN
    # syscall.program_address = &frame.pda
    mov64 r4, r10
    add64 r4, RM_FM_PDA_OFF
    # syscall.bump_seed = &frame.bump
    mov64 r5, r10
    add64 r5, RM_FM_BUMP_OFF
    call sol_try_find_program_address
    # input = frame.input
    ldxdw r6, [r10 + RM_FM_INPUT_OFF]
    # if input.market.pubkey != frame.market_pda
    #     return ErrorCode::InvalidMarketPubkey
    ldxdw r7, [r6 + IB_MARKET_PUBKEY_CHUNK_0_OFF]
    ldxdw r8, [r10 + RM_FM_PDA_CHUNK_0_OFF]
    jne r7, r8, e_invalid_market_pubkey
    ldxdw r7, [r6 + IB_MARKET_PUBKEY_CHUNK_1_OFF]
    ldxdw r8, [r10 + RM_FM_PDA_CHUNK_1_OFF]
    jne r7, r8, e_invalid_market_pubkey
    ldxdw r7, [r6 + IB_MARKET_PUBKEY_CHUNK_2_OFF]
    ldxdw r8, [r10 + RM_FM_PDA_CHUNK_2_OFF]
    jne r7, r8, e_invalid_market_pubkey
    ldxdw r7, [r6 + IB_MARKET_PUBKEY_CHUNK_3_OFF]
    ldxdw r8, [r10 + RM_FM_PDA_CHUNK_3_OFF]
    jne r7, r8, e_invalid_market_pubkey
    # frame.pda_seeds[2].addr = &frame.bump
    stxdw [r10 + RM_FM_PDA_SEEDS_IDX_2_ADDR_OFF], r5
    # frame.pda_seeds.[2].len = u8.size
    mov64 r7, SIZE_OF_U8
    stxdw [r10 + RM_FM_PDA_SEEDS_IDX_2_LEN_OFF], r7
    # frame.create_account_data.owner = syscall.program_id
    ldxdw r7, [r3 + PUBKEY_CHUNK_0_OFF]
    stxdw [r10 + RM_FM_CREATE_ACCT_OWNER_CHUNK_0_UOFF], r7
    ldxdw r7, [r3 + PUBKEY_CHUNK_1_OFF]
    stxdw [r10 + RM_FM_CREATE_ACCT_OWNER_CHUNK_1_UOFF], r7
    ldxdw r7, [r3 + PUBKEY_CHUNK_2_OFF]
    stxdw [r10 + RM_FM_CREATE_ACCT_OWNER_CHUNK_2_UOFF], r7
    ldxdw r7, [r3 + PUBKEY_CHUNK_3_OFF]
    stxdw [r10 + RM_FM_CREATE_ACCT_OWNER_CHUNK_3_UOFF], r7
    # frame.cpi[0].info.is_signer = true
    # frame.cpi[0].info.is_writable = true
    sth [r10 + RM_FM_CPI_IDX_0_ACCT_INFO_IS_SIGNER_UOFF], CPI_WRITABLE_SIGNER
    # frame.cpi[0].meta.is_writable = true
    # frame.cpi[0].meta.is_signer = true
    sth [r10 + RM_FM_CPI_IDX_0_ACCT_META_IS_WRITABLE_UOFF], CPI_WRITABLE_SIGNER
    # frame.cpi[1].info.is_signer = true
    # frame.cpi[1].info.is_writable = true
    sth [r10 + RM_FM_CPI_IDX_1_ACCT_INFO_IS_SIGNER_UOFF], CPI_WRITABLE_SIGNER
    # frame.cpi[1].meta.is_writable = true
    # frame.cpi[1].meta.is_signer = true
    sth [r10 + RM_FM_CPI_IDX_1_ACCT_META_IS_WRITABLE_UOFF], CPI_WRITABLE_SIGNER
    # frame.cpi[0].meta.pubkey = &input.user.address
    # frame.cpi[0].info.key = &input.user.address
    add64 r6, IB_USER_PUBKEY_OFF
    stxdw [r10 + RM_FM_CPI_IDX_0_ACCT_META_PUBKEY_UOFF], r6
    stxdw [r10 + RM_FM_CPI_IDX_0_ACCT_INFO_KEY_UOFF], r6
    # frame.cpi[0].info.owner = &input.user.owner
    add64 r6, IB_ADDRESS_TO_OWNER_REL_OFF_IMM
    stxdw [r10 + RM_FM_CPI_IDX_0_ACCT_INFO_OWNER_UOFF], r6
    # frame.cpi[0].info.lamports = &input.user.lamports
    add64 r6, IB_OWNER_TO_LAMPORTS_REL_OFF_IMM
    stxdw [r10 + RM_FM_CPI_IDX_0_ACCT_INFO_LAMPORTS_UOFF], r6
    # frame.cpi[0].info.data = &input.user.data
    add64 r6, IB_LAMPORTS_TO_DATA_REL_OFF_IMM
    stxdw [r10 + RM_FM_CPI_IDX_0_ACCT_INFO_DATA_UOFF], r6
    # frame.cpi[1].meta.pubkey = &input.market.address
    # frame.cpi[1].info.key = &input.market.address
    add64 r6, IB_USER_DATA_TO_MARKET_ADDRESS_REL_OFF_IMM
    stxdw [r10 + RM_FM_CPI_IDX_1_ACCT_META_PUBKEY_UOFF], r6
    stxdw [r10 + RM_FM_CPI_IDX_1_ACCT_INFO_KEY_UOFF], r6
    # frame.cpi[1].info.owner = &input.market.owner
    add64 r6, IB_ADDRESS_TO_OWNER_REL_OFF_IMM
    stxdw [r10 + RM_FM_CPI_IDX_1_ACCT_INFO_OWNER_UOFF], r6
    # frame.cpi[1].info.lamports = &input.market.lamports
    add64 r6, IB_OWNER_TO_LAMPORTS_REL_OFF_IMM
    stxdw [r10 + RM_FM_CPI_IDX_1_ACCT_INFO_LAMPORTS_UOFF], r6
    # frame.cpi[1].info.data = &input.market.data
    add64 r6, IB_LAMPORTS_TO_DATA_REL_OFF_IMM
    stxdw [r10 + RM_FM_CPI_IDX_1_ACCT_INFO_DATA_UOFF], r6
    # frame.signers_seeds.addr = &frame.pda_seeds
    stxdw [r10 + RM_FM_SIGNERS_SEEDS_ADDR_UOFF], r1
    # frame.signers_seeds.len = frame.PDA_SEEDS_N_SEEDS
    mov64 r7, RM_FM_PDA_SEEDS_N_SEEDS
    stxdw [r10 + RM_FM_SIGNERS_SEEDS_LEN_UOFF], r7
    # frame.sol_instruction.program_id = frame.system_program_id
    ldxdw r7, [r10 + RM_FM_SYSTEM_PROGRAM_ID_OFF]
    stxdw [r10 + RM_FM_SOL_INSN_PROGRAM_ID_UOFF], r7
    # frame.sol_instruction.data = &frame.create_account_data
    mov64 r7, r10
    add64 r7, RM_FM_CREATE_ACCT_DATA_OFF
    stxdw [r10 + RM_FM_SOL_INSN_DATA_UOFF], r7
    # frame.sol_instruction.accounts = &frame.cpi.account_metas
    add64 r7, RM_FM_CREATE_ACCT_DATA_TO_CPI_ACCT_METAS_REL_OFF_IMM
    stxdw [r10 + RM_FM_SOL_INSN_ACCOUNTS_UOFF], r7
    # frame.sol_instruction.account_len = register_misc.CREATE_ACCOUNT_N_ACCOUNTS
    mov64 r7, RM_MISC_CREATE_ACCOUNT_N_ACCOUNTS
    stxdw [r10 + RM_FM_SOL_INSN_ACCOUNT_LEN_UOFF], r7
    # frame.sol_instruction.data_len = CreateAccountData.size
    mov64 r7, SIZE_OF_CREATE_ACCOUNT_DATA
    stxdw [r10 + RM_FM_SOL_INSN_DATA_LEN_UOFF], r7
    # syscall.instruction = &frame.sol_instruction
    add64 r1, RM_FM_PDA_SEEDS_TO_SOL_INSN_REL_OFF_IMM
    # syscall.account_infos = &frame.cpi.account_infos
    mov64 r2, r10
    add64 r2, RM_FM_CPI_SOL_ACCT_INFO_OFF
    # syscall.account_infos_len = register_misc.CREATE_ACCOUNT_N_ACCOUNTS
    mov64 r3, RM_MISC_CREATE_ACCOUNT_N_ACCOUNTS
    # syscall.seeds = &frame.signers_seeds
    add64 r4, RM_FM_PDA_TO_SIGNERS_SEEDS_REL_OFF_IMM
    # syscall.seeds_len = register_misc.N_PDA_SIGNERS
    mov64 r5, RM_MISC_N_PDA_SIGNERS
    call sol_invoke_signed_c
    ja init_market_pda_return
