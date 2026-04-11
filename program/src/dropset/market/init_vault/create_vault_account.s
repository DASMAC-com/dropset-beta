create_vault_account:
    # acct_size = frame.token_account_data_size
    ldxdw r8, [r6 + RM_FM_TOKEN_ACCOUNT_DATA_SIZE_OFF]
    # frame.create_account_data.space = acct_size
    stxdw [r6 + RM_FM_CREATE_ACCT_SPACE_UOFF], r8
    # acct_size = acct_size + common::account::STORAGE_OVERHEAD
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
    # frame.cpi[0].info.data_len = common::memory::LEN_ZERO
    mov64 r9, DATA_LEN_ZERO
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_DATA_LEN_UOFF], r9
    # frame.cpi[1].info.data_len = common::memory::LEN_ZERO
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_DATA_LEN_UOFF], r9
    # frame.cpi[0].meta.pubkey = &input.user.address
    # frame.cpi[0].info.key = &input.user.address
    ldxdw r8, [r6 + RM_FM_INPUT_OFF]
    add64 r8, IB_USER_ADDRESS_OFF
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_META_PUBKEY_UOFF], r8
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_KEY_UOFF], r8
    # frame.cpi[0].info.owner = &input.user.owner
    add64 r8, ACCT_ADDRESS_TO_OWNER_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_OWNER_UOFF], r8
    # frame.cpi[0].info.lamports = &input.user.lamports
    add64 r8, ACCT_OWNER_TO_LAMPORTS_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_LAMPORTS_UOFF], r8
    # frame.cpi[0].info.data = &input.user.data
    add64 r8, ACCT_LAMPORTS_TO_DATA_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_DATA_UOFF], r8
    # frame.cpi[1].meta.pubkey = &acct.address
    # frame.cpi[1].info.key = &acct.address
    mov64 r8, r7
    add64 r8, ACCT_ADDRESS_OFF
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_META_PUBKEY_UOFF], r8
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_KEY_UOFF], r8
    # frame.cpi[1].info.owner = &acct.owner
    add64 r8, ACCT_ADDRESS_TO_OWNER_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_OWNER_UOFF], r8
    # frame.cpi[1].info.lamports = &acct.lamports
    add64 r8, ACCT_OWNER_TO_LAMPORTS_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_1_ACCT_INFO_LAMPORTS_UOFF], r8
    # frame.cpi[1].info.data = &acct.data
    add64 r8, ACCT_LAMPORTS_TO_DATA_REL_OFF_IMM
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
    # frame.sol_instruction.account_len = market::register::CREATE_ACCOUNT_N_ACCOUNTS
    mov64 r8, RM_CREATE_ACCOUNT_N_ACCOUNTS
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
    # syscall.account_infos_len = market::register::CREATE_ACCOUNT_N_ACCOUNTS
    mov64 r3, RM_CREATE_ACCOUNT_N_ACCOUNTS
    # syscall.seeds = &frame.signers_seeds
    mov64 r4, r6
    add64 r4, RM_FM_SIGNERS_SEEDS_ADDR_UOFF
    # syscall.seeds_len = market::register::N_PDA_SIGNERS
    mov64 r5, RM_N_PDA_SIGNERS
    call sol_invoke_signed_c
    ja create_vault_account_return
