get_vault_size:
    # if !frame.token_program_is_2022
    #     acct_size = common::token::ACCOUNT_SIZE
    ldxb r1, [r6 + RM_FM_TOKEN_PROGRAM_IS_2022_UOFF]
    jne r1, DATA_BOOL_FALSE, get_vault_size_token_2022
    mov64 r1, TOKEN_ACCOUNT_SIZE
    stxdw [r6 + RM_FM_TOKEN_ACCOUNT_DATA_SIZE_OFF], r1
    ja get_vault_size_return
get_vault_size_token_2022:
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
    add64 r9, ACCT_ADDRESS_TO_OWNER_REL_OFF_IMM
    stxdw [r6 + RM_FM_CPI_IDX_0_ACCT_INFO_OWNER_UOFF], r9
    # frame.cpi[0].info.lamports = &mint.lamports
    add64 r9, ACCT_OWNER_TO_LAMPORTS_REL_OFF_IMM
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
    # frame.sol_instruction.account_len = common::token::GET_ACCOUNT_DATA_SIZE_N_ACCOUNTS
    mov64 r9, TOKEN_GET_ACCOUNT_DATA_SIZE_N_ACCOUNTS
    stxdw [r6 + RM_FM_SOL_INSN_ACCOUNT_LEN_UOFF], r9
    # frame.get_account_data_size_data = common::token::GET_ACCOUNT_DATA_SIZE_DISC
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
    # syscall.account_infos_len = common::token::GET_ACCOUNT_DATA_SIZE_N_ACCOUNTS
    mov64 r3, TOKEN_GET_ACCOUNT_DATA_SIZE_N_ACCOUNTS
    # syscall.seeds_len = common::token::GET_ACCOUNT_DATA_SIZE_N_SEEDS
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
    ja get_vault_size_return
