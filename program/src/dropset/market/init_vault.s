init_vault:
    # Store(frame)
    mov64 r6, r2
    # Store(acct)
    mov64 r7, r1
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
    # if !frame.token_program_is_2022
    #     acct_size = token.ACCOUNT_SIZE
    ldxb r1, [r6 + RM_FM_TOKEN_PROGRAM_IS_2022_UOFF]
    jne r1, DATA_BOOL_FALSE, init_vault_get_account_data_size
    mov64 r1, TOKEN_ACCOUNT_SIZE
    stxdw [r6 + RM_FM_TOKEN_ACCOUNT_DATA_SIZE_OFF], r1
    ja init_vault_create_account
init_vault_get_account_data_size:
    # frame.cpi[0].info.is_signer = false
    # frame.cpi[0].info.is_writable = false
    sth [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_IS_SIGNER_UOFF], CPI_READONLY_NON_SIGNER
    # frame.cpi[0].meta.is_writable = false
    # frame.cpi[0].meta.is_signer = false
    sth [r6 + RM_FM_CPI_IDX_0_ACCT_META_IS_WRITABLE_UOFF], CPI_READONLY_NON_SIGNER
    # mint = frame.mint
    ldxdw r8, [r6 + RM_FM_MINT_OFF]
    # frame.cpi[0].meta.pubkey = &mint.address
    mov64 r9, r8
    add64 r9, ACCT_ADDRESS_OFF
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_META_PUBKEY_UOFF], r9
    # frame.cpi[0].info.key = &mint.address
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_KEY_UOFF], r9
    # frame.cpi[0].info.owner = &mint.owner
    add64 r9, IB_ADDRESS_TO_OWNER_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_OWNER_UOFF], r9
    # frame.cpi[0].info.lamports = &mint.lamports
    add64 r9, IB_OWNER_TO_LAMPORTS_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_LAMPORTS_UOFF], r9
    # frame.cpi[0].info.data_len = mint.data_len
    ldxdw r9, [r8 + ACCT_DATA_LEN_OFF]
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_DATA_LEN_UOFF], r9
    # frame.cpi[0].info.data = &mint.data
    mov64 r9, r8
    add64 r9, ACCT_DATA_OFF
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_DATA_UOFF], r9
    # frame.sol_instruction.program_id = frame.token_program_id
    ldxdw r9, [r6 + RM_FM_TOKEN_PROGRAM_ID_OFF]
    stxdw [r6 + RM_FM_SOL_INSN_PROGRAM_ID_UOFF], r9
    # frame.sol_instruction.accounts = &frame.cpi[0].meta
    mov64 r9, r6
    add64 r9, RM_FM_CPI_IDX_0_ACCT_META_PUBKEY_UOFF
    stxdw [r6 + RM_FM_SOL_INSN_ACCOUNTS_UOFF], r9
    # frame.sol_instruction.account_len = token.GET_ACCOUNT_DATA_SIZE_N_ACCOUNTS
    mov64 r9, TOKEN_GET_ACCOUNT_DATA_SIZE_N_ACCOUNTS
    stxdw [r6 + RM_FM_SOL_INSN_ACCOUNT_LEN_UOFF], r9
    # frame.get_account_data_size_data = token.GET_ACCOUNT_DATA_SIZE_DISC
    stb [r6 + RM_FM_GET_ACCOUNT_DATA_SIZE_DATA_UOFF], TOKEN_GET_ACCOUNT_DATA_SIZE_DISC
    # frame.sol_instruction.data = &frame.get_account_data_size_data
    mov64 r9, r6
    add64 r9, RM_FM_GET_ACCOUNT_DATA_SIZE_DATA_UOFF
    stxdw [r6 + RM_FM_SOL_INSN_DATA_UOFF], r9
    # frame.sol_instruction.data_len = u8.size
    mov64 r9, SIZE_OF_U8
    stxdw [r6 + RM_FM_SOL_INSN_DATA_LEN_UOFF], r9
    # syscall.instruction = &frame.sol_instruction
    mov64 r1, r6
    add64 r1, RM_FM_SOL_INSN_OFF
    # syscall.account_infos = &frame.cpi[0].info
    mov64 r2, r6
    add64 r2, RM_FM_CPI_SOL_ACCT_INFO_OFF
    # syscall.account_infos_len = token.GET_ACCOUNT_DATA_SIZE_N_ACCOUNTS
    mov64 r3, TOKEN_GET_ACCOUNT_DATA_SIZE_N_ACCOUNTS
    # syscall.seeds_len = token.GET_ACCOUNT_DATA_SIZE_N_SEEDS
    mov64 r5, TOKEN_GET_ACCOUNT_DATA_SIZE_N_SEEDS
    call sol_invoke_signed_c
    # syscall.bytes = &frame.token_account_data_size
    mov64 r1, r6
    add64 r1, RM_FM_TOKEN_ACCOUNT_DATA_SIZE_OFF
    # syscall.bytes_len = u64.size
    mov64 r2, SIZE_OF_U64
    # syscall.program_id = &frame.get_return_data_program_id
    mov64 r3, r6
    add64 r3, RM_FM_GET_RETURN_DATA_PROGRAM_ID_OFF
    call sol_get_return_data
init_vault_create_account:
    # acct_size = frame.token_account_data_size
    ldxdw r8, [r6 + RM_FM_TOKEN_ACCOUNT_DATA_SIZE_OFF]
    # frame.create_account_data.space = acct_size
    stxdw [r6 + RM_FM_CREATE_ACCT_SPACE_UOFF], r8
    # acct_size = acct_size + account.STORAGE_OVERHEAD
    add64 r8, ACCT_STORAGE_OVERHEAD
    # frame.create_account_data.lamports = acct_size * frame.lamports_per_byte
    ldxdw r9, [r6 + RM_FM_LAMPORTS_PER_BYTE_OFF]
    mul64 r8, r9
    stxdw [r6 + RM_FM_CREATE_ACCT_LAMPORTS_UOFF], r8
    # frame.create_account_data.owner = frame.token_program_id
    ldxdw r8, [r6 + RM_FM_TOKEN_PROGRAM_ID_OFF]
    ldxdw r9, [r8 + PUBKEY_CHUNK_0_OFF]
    stxdw [r6 + RM_FM_CREATE_ACCT_OWNER_CHUNK_0_UOFF], r9
    ldxdw r9, [r8 + PUBKEY_CHUNK_1_OFF]
    stxdw [r6 + RM_FM_CREATE_ACCT_OWNER_CHUNK_1_UOFF], r9
    ldxdw r9, [r8 + PUBKEY_CHUNK_2_OFF]
    stxdw [r6 + RM_FM_CREATE_ACCT_OWNER_CHUNK_2_UOFF], r9
    ldxdw r9, [r8 + PUBKEY_CHUNK_3_OFF]
    stxdw [r6 + RM_FM_CREATE_ACCT_OWNER_CHUNK_3_UOFF], r9
    # frame.cpi[0].info.is_signer = true
    # frame.cpi[0].info.is_writable = true
    sth [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_IS_SIGNER_UOFF], CPI_WRITABLE_SIGNER
    # frame.cpi[0].meta.is_writable = true
    # frame.cpi[0].meta.is_signer = true
    sth [r6 + RM_FM_CPI_IDX_0_ACCT_META_IS_WRITABLE_UOFF], CPI_WRITABLE_SIGNER
    # frame.cpi[1].info.is_signer = true
    # frame.cpi[1].info.is_writable = true
    sth [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_IS_SIGNER_UOFF], CPI_WRITABLE_SIGNER
    # frame.cpi[1].meta.is_writable = true
    # frame.cpi[1].meta.is_signer = true
    sth [r6 + RM_FM_CPI_IDX_1_ACCT_META_IS_WRITABLE_UOFF], CPI_WRITABLE_SIGNER
    # frame.cpi[0].info.data_len = data.LEN_ZERO
    mov64 r9, DATA_LEN_ZERO
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_DATA_LEN_UOFF], r9
    # frame.cpi[1].info.data_len = data.LEN_ZERO
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_DATA_LEN_UOFF], r9
    # frame.cpi[0].meta.pubkey = &input.user.address
    # frame.cpi[0].info.key = &input.user.address
    ldxdw r8, [r6 + RM_FM_INPUT_OFF]
    add64 r8, IB_USER_PUBKEY_OFF
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_META_PUBKEY_UOFF], r8
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_KEY_UOFF], r8
    # frame.cpi[0].info.owner = &input.user.owner
    add64 r8, IB_ADDRESS_TO_OWNER_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_OWNER_UOFF], r8
    # frame.cpi[0].info.lamports = &input.user.lamports
    add64 r8, IB_OWNER_TO_LAMPORTS_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_LAMPORTS_UOFF], r8
    # frame.cpi[0].info.data = &input.user.data
    add64 r8, IB_LAMPORTS_TO_DATA_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_DATA_UOFF], r8
    # frame.cpi[1].meta.pubkey = &acct.address
    # frame.cpi[1].info.key = &acct.address
    mov64 r8, r7
    add64 r8, ACCT_ADDRESS_OFF
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_META_PUBKEY_UOFF], r8
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_KEY_UOFF], r8
    # frame.cpi[1].info.owner = &acct.owner
    add64 r8, IB_ADDRESS_TO_OWNER_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_OWNER_UOFF], r8
    # frame.cpi[1].info.lamports = &acct.lamports
    add64 r8, IB_OWNER_TO_LAMPORTS_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_LAMPORTS_UOFF], r8
    # frame.cpi[1].info.data = &acct.data
    add64 r8, IB_LAMPORTS_TO_DATA_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_DATA_UOFF], r8
    # frame.sol_instruction.program_id = frame.system_program_id
    ldxdw r8, [r6 + RM_FM_SYSTEM_PROGRAM_ID_OFF]
    stxdw [r6 + RM_FM_SOL_INSN_PROGRAM_ID_UOFF], r8
    # frame.sol_instruction.data = &frame.create_account_data
    mov64 r8, r6
    add64 r8, RM_FM_CREATE_ACCT_DATA_OFF
    stxdw [r6 + RM_FM_SOL_INSN_DATA_UOFF], r8
    # frame.sol_instruction.accounts = &frame.cpi.account_metas
    add64 r8, RM_FM_CREATE_ACCT_DATA_TO_CPI_ACCT_METAS_REL_OFF_IMM
    stxdw [r6 + RM_FM_SOL_INSN_ACCOUNTS_UOFF], r8
    # frame.sol_instruction.account_len = register_misc.CREATE_ACCOUNT_N_ACCOUNTS
    mov64 r8, RM_MISC_CREATE_ACCOUNT_N_ACCOUNTS
    stxdw [r6 + RM_FM_SOL_INSN_ACCOUNT_LEN_UOFF], r8
    # frame.sol_instruction.data_len = CreateAccountData.size
    mov64 r8, SIZE_OF_CREATE_ACCOUNT_DATA
    stxdw [r6 + RM_FM_SOL_INSN_DATA_LEN_UOFF], r8
    # syscall.instruction = &frame.sol_instruction
    mov64 r1, r6
    add64 r1, RM_FM_SOL_INSN_OFF
    # syscall.account_infos = &frame.cpi.account_infos
    mov64 r2, r6
    add64 r2, RM_FM_CPI_SOL_ACCT_INFO_OFF
    # syscall.account_infos_len = register_misc.CREATE_ACCOUNT_N_ACCOUNTS
    mov64 r3, RM_MISC_CREATE_ACCOUNT_N_ACCOUNTS
    # syscall.seeds = &frame.signers_seeds
    mov64 r4, r6
    add64 r4, RM_FM_SIGNERS_SEEDS_ADDR_UOFF
    # syscall.seeds_len = register_misc.N_PDA_SIGNERS
    mov64 r5, RM_MISC_N_PDA_SIGNERS
    call sol_invoke_signed_c
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
    # Populate SolInstruction for InitializeAccount2.
    # frame.sol_instruction.program_id = frame.token_program_id
    ldxdw r8, [r6 + RM_FM_TOKEN_PROGRAM_ID_OFF]
    stxdw [r6 + RM_FM_SOL_INSN_PROGRAM_ID_UOFF], r8
    # frame.sol_instruction.accounts = &frame.cpi.account_metas
    mov64 r8, r6
    add64 r8, RM_FM_CPI_IDX_0_ACCT_META_PUBKEY_UOFF
    stxdw [r6 + RM_FM_SOL_INSN_ACCOUNTS_UOFF], r8
    # frame.sol_instruction.account_len = token.INITIALIZE_ACCOUNT_2_N_ACCOUNTS
    mov64 r8, TOKEN_INITIALIZE_ACCOUNT_2_N_ACCOUNTS
    stxdw [r6 + RM_FM_SOL_INSN_ACCOUNT_LEN_UOFF], r8
    # frame.initialize_account_2_data.discriminant = token.INITIALIZE_ACCOUNT_2_DISC
    stb [r6 + RM_FM_INIT_ACCT_2_DISC_UOFF], TOKEN_INITIALIZE_ACCOUNT_2_DISC
    # frame.initialize_account_2_data.proprietor = input.market.address
    ldxdw r8, [r6 + RM_FM_INPUT_OFF]
    add64 r8, IB_MARKET_PUBKEY_OFF
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
    # syscall.account_infos_len = token.INITIALIZE_ACCOUNT_2_N_ACCOUNTS
    mov64 r3, TOKEN_INITIALIZE_ACCOUNT_2_N_ACCOUNTS
    # syscall.seeds_len = token.INITIALIZE_ACCOUNT_2_N_SEEDS
    mov64 r5, TOKEN_INITIALIZE_ACCOUNT_2_N_SEEDS
    call sol_invoke_signed_c
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
