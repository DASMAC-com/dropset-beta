create_market_account:
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
    add64 r6, IB_USER_ADDRESS_OFF
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
    # frame.signers_seeds.len = frame.pda_seeds.n_seeds
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
    # frame.sol_instruction.account_len = market::register::CREATE_ACCOUNT_N_ACCOUNTS
    mov64 r7, RM_CREATE_ACCOUNT_N_ACCOUNTS
    stxdw [r10 + RM_FM_SOL_INSN_ACCOUNT_LEN_UOFF], r7
    # frame.sol_instruction.data_len = CreateAccountData.size
    mov64 r7, SIZE_OF_CREATE_ACCOUNT_DATA
    stxdw [r10 + RM_FM_SOL_INSN_DATA_LEN_UOFF], r7
    # syscall.instruction = &frame.sol_instruction
    add64 r1, RM_FM_PDA_SEEDS_TO_SOL_INSN_REL_OFF_IMM
    # syscall.account_infos = &frame.cpi.account_infos
    mov64 r2, r10
    add64 r2, RM_FM_CPI_SOL_ACCT_INFO_OFF
    # syscall.account_infos_len = market::register::CREATE_ACCOUNT_N_ACCOUNTS
    mov64 r3, RM_CREATE_ACCOUNT_N_ACCOUNTS
    # syscall.seeds = &frame.signers_seeds
    add64 r4, RM_FM_PDA_TO_SIGNERS_SEEDS_REL_OFF_IMM
    # syscall.seeds_len = market::register::N_PDA_SIGNERS
    mov64 r5, RM_N_PDA_SIGNERS
    call sol_invoke_signed_c
    # input.market.data.next = input + entrypoint::input_buffer::MARKET_DATA_BYTES
    ldxdw r6, [r10 + RM_FM_INPUT_OFF]
    mov64 r7, r6
    add64 r7, IB_MARKET_DATA_BYTES_OFF
    stxdw [r6 + IB_MARKET_DATA_NEXT_OFF], r7
    # input.market.data.bump = frame.bump
    ldxb r7, [r10 + RM_FM_BUMP_OFF]
    stxb [r6 + IB_MARKET_DATA_BUMP_OFF], r7
    ja create_market_account_return
