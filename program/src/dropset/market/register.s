# RegisterMarketData instruction data length.
.equ REGISTER_MARKET_DATA_LEN, 1

# RegisterMarketAccounts number of accounts.
.equ REGISTER_MARKET_ACCOUNTS_LEN, 10
.equ REGISTER_MARKET_ACCOUNTS_USER_POS, 0 # User account position.
.equ REGISTER_MARKET_ACCOUNTS_MARKET_POS, 1 # Market account position.
# Base Mint account position.
.equ REGISTER_MARKET_ACCOUNTS_BASE_MINT_POS, 2
# Quote Mint account position.
.equ REGISTER_MARKET_ACCOUNTS_QUOTE_MINT_POS, 3
# System Program account position.
.equ REGISTER_MARKET_ACCOUNTS_SYSTEM_PROGRAM_POS, 4
# Rent Sysvar account position.
.equ REGISTER_MARKET_ACCOUNTS_RENT_SYSVAR_POS, 5
# Base Token Program account position.
.equ REGISTER_MARKET_ACCOUNTS_BASE_TOKEN_PROGRAM_POS, 6
# Base Vault account position.
.equ REGISTER_MARKET_ACCOUNTS_BASE_VAULT_POS, 7
# Quote Token Program account position.
.equ REGISTER_MARKET_ACCOUNTS_QUOTE_TOKEN_PROGRAM_POS, 8
# Quote Vault account position.
.equ REGISTER_MARKET_ACCOUNTS_QUOTE_VAULT_POS, 9

# Stack frame for REGISTER-MARKET.
# -------------------------------------------------------------------------
.equ RM_FM_ACCT_OFF, -480 # Saved acct pointer across INIT-VAULT syscall.
# Pointer to owning token program address.
.equ RM_FM_TOKEN_PROGRAM_ID_OFF, -472
.equ RM_FM_INPUT_OFF, -464 # Saved input buffer pointer.
.equ RM_FM_INPUT_SHIFTED_OFF, -456 # Saved input_shifted pointer.
.equ RM_FM_PDA_SEEDS_OFF, -448 # Signer seeds offset.
.equ RM_FM_PDA_SEEDS_N_SEEDS, 3 # Number of signer seeds.
.equ RM_FM_PDA_SEEDS_IDX_0_ADDR_OFF, -448 # Idx 0 signer seed address.
.equ RM_FM_PDA_SEEDS_IDX_0_LEN_OFF, -440 # Idx 0 signer seed length.
.equ RM_FM_PDA_SEEDS_IDX_1_ADDR_OFF, -432 # Idx 1 signer seed address.
.equ RM_FM_PDA_SEEDS_IDX_1_LEN_OFF, -424 # Idx 1 signer seed length.
.equ RM_FM_PDA_SEEDS_IDX_2_ADDR_OFF, -416 # Idx 2 signer seed address.
.equ RM_FM_PDA_SEEDS_IDX_2_LEN_OFF, -408 # Idx 2 signer seed length.
.equ RM_FM_PDA_OFF, -400 # PDA address.
.equ RM_FM_PDA_CHUNK_0_OFF, -400 # PDA address (chunk 0).
.equ RM_FM_PDA_CHUNK_1_OFF, -392 # PDA address (chunk 1).
.equ RM_FM_PDA_CHUNK_2_OFF, -384 # PDA address (chunk 2).
.equ RM_FM_PDA_CHUNK_3_OFF, -376 # PDA address (chunk 3).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_OFF, -368 # System Program pubkey.
# System Program pubkey (chunk 0).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_0_OFF, -368
# System Program pubkey (chunk 1).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_1_OFF, -360
# System Program pubkey (chunk 2).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_2_OFF, -352
# System Program pubkey (chunk 3).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_3_OFF, -344
.equ RM_FM_CREATE_ACCT_DATA_OFF, -336 # CreateAccount instruction data.
# Lamports field within CreateAccount instruction data.
.equ RM_FM_CREATE_ACCT_LAMPORTS_UOFF, -332
# Space field within CreateAccount instruction data.
.equ RM_FM_CREATE_ACCT_SPACE_UOFF, -324
# Owner field within CreateAccount instruction data.
.equ RM_FM_CREATE_ACCT_OWNER_UOFF, -316
# Owner field within CreateAccount instruction data (chunk 0).
.equ RM_FM_CREATE_ACCT_OWNER_CHUNK_0_UOFF, -316
# Owner field within CreateAccount instruction data (chunk 1).
.equ RM_FM_CREATE_ACCT_OWNER_CHUNK_1_UOFF, -308
# Owner field within CreateAccount instruction data (chunk 2).
.equ RM_FM_CREATE_ACCT_OWNER_CHUNK_2_UOFF, -300
# Owner field within CreateAccount instruction data (chunk 3).
.equ RM_FM_CREATE_ACCT_OWNER_CHUNK_3_UOFF, -292
.equ RM_FM_CPI_N_ACCOUNTS, 3 # Number of CPI accounts.
.equ RM_FM_CPI_SOL_ACCT_INFO_OFF, -280 # Start of SolAccountInfo vector.
.equ RM_FM_CPI_SOL_ACCT_META_OFF, -112 # Start of SolAccountMeta vector.
.equ RM_FM_CPI_IDX_0_ACCT_INFO_KEY_UOFF, -280 # Idx 0 account info key.
# Idx 0 account info lamports.
.equ RM_FM_CPI_IDX_0_ACCT_INFO_LAMPORTS_UOFF, -272
# Idx 0 account info data length.
.equ RM_FM_CPI_IDX_0_ACCT_INFO_DATA_LEN_UOFF, -264
.equ RM_FM_CPI_IDX_0_ACCT_INFO_DATA_UOFF, -256 # Idx 0 account info data.
.equ RM_FM_CPI_IDX_0_ACCT_INFO_OWNER_UOFF, -248 # Idx 0 account info owner.
# Idx 0 account info rent epoch.
.equ RM_FM_CPI_IDX_0_ACCT_INFO_RENT_EPOCH_UOFF, -240
# Idx 0 account info is signer.
.equ RM_FM_CPI_IDX_0_ACCT_INFO_IS_SIGNER_UOFF, -232
# Idx 0 account info is writable.
.equ RM_FM_CPI_IDX_0_ACCT_INFO_IS_WRITABLE_UOFF, -231
# Idx 0 account info executable.
.equ RM_FM_CPI_IDX_0_ACCT_INFO_EXECUTABLE_UOFF, -230
# Idx 0 account meta pubkey.
.equ RM_FM_CPI_IDX_0_ACCT_META_PUBKEY_UOFF, -112
# Idx 0 account meta is writable.
.equ RM_FM_CPI_IDX_0_ACCT_META_IS_WRITABLE_UOFF, -104
# Idx 0 account meta is signer.
.equ RM_FM_CPI_IDX_0_ACCT_META_IS_SIGNER_UOFF, -103
.equ RM_FM_CPI_IDX_1_ACCT_INFO_KEY_UOFF, -224 # Idx 1 account info key.
# Idx 1 account info lamports.
.equ RM_FM_CPI_IDX_1_ACCT_INFO_LAMPORTS_UOFF, -216
# Idx 1 account info data length.
.equ RM_FM_CPI_IDX_1_ACCT_INFO_DATA_LEN_UOFF, -208
.equ RM_FM_CPI_IDX_1_ACCT_INFO_DATA_UOFF, -200 # Idx 1 account info data.
.equ RM_FM_CPI_IDX_1_ACCT_INFO_OWNER_UOFF, -192 # Idx 1 account info owner.
# Idx 1 account info rent epoch.
.equ RM_FM_CPI_IDX_1_ACCT_INFO_RENT_EPOCH_UOFF, -184
# Idx 1 account info is signer.
.equ RM_FM_CPI_IDX_1_ACCT_INFO_IS_SIGNER_UOFF, -176
# Idx 1 account info is writable.
.equ RM_FM_CPI_IDX_1_ACCT_INFO_IS_WRITABLE_UOFF, -175
# Idx 1 account info executable.
.equ RM_FM_CPI_IDX_1_ACCT_INFO_EXECUTABLE_UOFF, -174
# Idx 1 account meta pubkey.
.equ RM_FM_CPI_IDX_1_ACCT_META_PUBKEY_UOFF, -96
# Idx 1 account meta is writable.
.equ RM_FM_CPI_IDX_1_ACCT_META_IS_WRITABLE_UOFF, -88
# Idx 1 account meta is signer.
.equ RM_FM_CPI_IDX_1_ACCT_META_IS_SIGNER_UOFF, -87
.equ RM_FM_CPI_IDX_2_ACCT_INFO_KEY_UOFF, -168 # Idx 2 account info key.
# Idx 2 account info lamports.
.equ RM_FM_CPI_IDX_2_ACCT_INFO_LAMPORTS_UOFF, -160
# Idx 2 account info data length.
.equ RM_FM_CPI_IDX_2_ACCT_INFO_DATA_LEN_UOFF, -152
.equ RM_FM_CPI_IDX_2_ACCT_INFO_DATA_UOFF, -144 # Idx 2 account info data.
.equ RM_FM_CPI_IDX_2_ACCT_INFO_OWNER_UOFF, -136 # Idx 2 account info owner.
# Idx 2 account info rent epoch.
.equ RM_FM_CPI_IDX_2_ACCT_INFO_RENT_EPOCH_UOFF, -128
# Idx 2 account info is signer.
.equ RM_FM_CPI_IDX_2_ACCT_INFO_IS_SIGNER_UOFF, -120
# Idx 2 account info is writable.
.equ RM_FM_CPI_IDX_2_ACCT_INFO_IS_WRITABLE_UOFF, -119
# Idx 2 account info executable.
.equ RM_FM_CPI_IDX_2_ACCT_INFO_EXECUTABLE_UOFF, -118
# Idx 2 account meta pubkey.
.equ RM_FM_CPI_IDX_2_ACCT_META_PUBKEY_UOFF, -80
# Idx 2 account meta is writable.
.equ RM_FM_CPI_IDX_2_ACCT_META_IS_WRITABLE_UOFF, -72
# Idx 2 account meta is signer.
.equ RM_FM_CPI_IDX_2_ACCT_META_IS_SIGNER_UOFF, -71
.equ RM_FM_SIGNERS_SEEDS_ADDR_UOFF, -64 # Signers seeds address.
.equ RM_FM_SIGNERS_SEEDS_LEN_UOFF, -56 # Signers seeds length.
.equ RM_FM_SOL_INSN_OFF, -48 # SolInstruction offset.
.equ RM_FM_SOL_INSN_PROGRAM_ID_UOFF, -48 # SolInstruction program ID.
.equ RM_FM_SOL_INSN_ACCOUNTS_UOFF, -40 # SolInstruction accounts pointer.
.equ RM_FM_SOL_INSN_ACCOUNT_LEN_UOFF, -32 # SolInstruction account length.
.equ RM_FM_SOL_INSN_DATA_UOFF, -24 # SolInstruction data pointer.
.equ RM_FM_SOL_INSN_DATA_LEN_UOFF, -16 # SolInstruction data length.
.equ RM_FM_BUMP_OFF, -8 # Bump seed.
.equ RM_FM_VAULT_INDEX_UOFF, -7 # Vault index for PDA derivation.
# Whether the current token program is Token 2022.
.equ RM_FM_TOKEN_PROGRAM_IS_2022_UOFF, -6
# From pda_seeds to sol_instruction.
.equ RM_FM_PDA_SEEDS_TO_SOL_INSN_REL_OFF_IMM, 400
# From pda to signers_seeds.
.equ RM_FM_PDA_TO_SIGNERS_SEEDS_REL_OFF_IMM, 336
# From create_account_data to CPI account metas.
.equ RM_FM_CREATE_ACCT_DATA_TO_CPI_ACCT_METAS_REL_OFF_IMM, 224
# -------------------------------------------------------------------------

# Miscellaneous market registration constants.
# -------------------------------------------------------------------------
# From input buffer to base mint duplicate flag.
.equ RM_MISC_BASE_DUPLICATE_OFF, 20680
# From input buffer to base mint data length.
.equ RM_MISC_BASE_DATA_LEN_OFF, 20760
.equ RM_MISC_BASE_ADDR_OFF, 20688 # From input buffer to base mint address.
.equ RM_MISC_BASE_OWNER_OFF, 20720 # From input buffer to base mint owner.
# From input buffer to base mint owner (chunk 0).
.equ RM_MISC_BASE_OWNER_CHUNK_0_OFF, 20720
# From input buffer to base mint owner (chunk 1).
.equ RM_MISC_BASE_OWNER_CHUNK_1_OFF, 20728
# From input buffer to base mint owner (chunk 2).
.equ RM_MISC_BASE_OWNER_CHUNK_2_OFF, 20736
# From input buffer to base mint owner (chunk 3).
.equ RM_MISC_BASE_OWNER_CHUNK_3_OFF, 20744
.equ RM_MISC_QUOTE_OFF, 31016 # From input buffer to quote mint.
# From input buffer to quote mint duplicate flag.
.equ RM_MISC_QUOTE_DUPLICATE_OFF, 31016
# From input buffer to quote mint address.
.equ RM_MISC_QUOTE_ADDR_OFF, 31024
# From input buffer to quote mint owner.
.equ RM_MISC_QUOTE_OWNER_OFF, 31056
# From input buffer to quote mint owner (chunk 0).
.equ RM_MISC_QUOTE_OWNER_CHUNK_0_OFF, 31056
# From input buffer to quote mint owner (chunk 1).
.equ RM_MISC_QUOTE_OWNER_CHUNK_1_OFF, 31064
# From input buffer to quote mint owner (chunk 2).
.equ RM_MISC_QUOTE_OWNER_CHUNK_2_OFF, 31072
# From input buffer to quote mint owner (chunk 3).
.equ RM_MISC_QUOTE_OWNER_CHUNK_3_OFF, 31080
# From input buffer to quote mint data length.
.equ RM_MISC_QUOTE_DATA_LEN_OFF, 31096
# Number of seeds for market PDA derivation (base mint, quote mint).
.equ RM_MISC_TRY_FIND_MARKET_PDA_SEEDS_LEN, 2
# Number of seeds for vault PDA derivation (market address, vault index).
.equ RM_MISC_TRY_FIND_VAULT_PDA_SEEDS_LEN, 2
# Number of accounts for CreateAccount CPI (user, new account).
.equ RM_MISC_CREATE_ACCOUNT_N_ACCOUNTS, 2
.equ RM_MISC_N_PDA_SIGNERS, 1 # Number of PDA signers for CPI.
# Vault index for base mint in PDA derivation and vault creation.
.equ RM_MISC_VAULT_INDEX_BASE, 0
# Vault index for quote mint in PDA derivation and vault creation.
.equ RM_MISC_VAULT_INDEX_QUOTE, 1
# -------------------------------------------------------------------------

register_market:
    # if input.n_accounts < RegisterMarketAccounts.LEN
    #     return ErrorCode::InvalidNumberOfAccounts
    jlt r3, REGISTER_MARKET_ACCOUNTS_LEN, e_invalid_number_of_accounts
    # if insn_len != RegisterMarketData.LEN
    #     return ErrorCode::InvalidInstructionLength
    jne r4, REGISTER_MARKET_DATA_LEN, e_invalid_instruction_length
    # if input.user.data_len != data.DATA_LEN_ZERO
    #     return ErrorCode::UserHasData
    ldxdw r9, [r1 + IB_USER_DATA_LEN_OFF]
    jne r9, DATA_LEN_ZERO, e_user_has_data
    # if input.market.duplicate != account.NON_DUP_MARKER
    #     return ErrorCode::MarketAccountIsDuplicate
    ldxb r9, [r1 + IB_MARKET_DUPLICATE_OFF]
    jne r9, ACCT_NON_DUP_MARKER, e_market_account_is_duplicate
    # if input.market.data_len != DATA_LEN_ZERO
    #     return ErrorCode::MarketHasData
    ldxdw r9, [r1 + IB_MARKET_DATA_LEN_OFF]
    jne r9, DATA_LEN_ZERO, e_market_has_data
    # if input.base_mint.duplicate != account.NON_DUP_MARKER
    #     return ErrorCode::BaseMintIsDuplicate
    ldxb r9, [r1 + RM_MISC_BASE_DUPLICATE_OFF]
    jne r9, ACCT_NON_DUP_MARKER, e_base_mint_is_duplicate
    # frame.pda_seeds[0].addr = input.base_mint.pubkey
    mov64 r9, r1
    add64 r9, RM_MISC_BASE_ADDR_OFF
    stxdw [r10 + RM_FM_PDA_SEEDS_IDX_0_ADDR_OFF], r9
    # frame.pda_seeds[0].len = Address.size
    mov64 r9, SIZE_OF_ADDRESS
    stxdw [r10 + RM_FM_PDA_SEEDS_IDX_0_LEN_OFF], r9
    # input_shifted = input + input.base_mint.padded_data_len
    ldxdw r9, [r1 + RM_MISC_BASE_DATA_LEN_OFF]
    add64 r9, DATA_LEN_MAX_PAD
    and64 r9, DATA_LEN_AND_MASK
    add64 r9, r1
    # frame.input_shifted = input_shifted
    stxdw [r10 + RM_FM_INPUT_SHIFTED_OFF], r9
    # if input_shifted.quote_mint.duplicate != account.NON_DUP_MARKER
    #     return ErrorCode::QuoteMintIsDuplicate
    ldxb r8, [r9 + RM_MISC_QUOTE_DUPLICATE_OFF]
    jne r8, ACCT_NON_DUP_MARKER, e_quote_mint_is_duplicate
    # frame.pda_seeds[1].addr = input_shifted.quote_mint.pubkey
    mov64 r8, r9
    add64 r8, RM_MISC_QUOTE_ADDR_OFF
    stxdw [r10 + RM_FM_PDA_SEEDS_IDX_1_ADDR_OFF], r8
    # quote_mint_padded_data_len = input_shifted.quote_mint.padded_data_len
    ldxdw r8, [r9 + RM_MISC_QUOTE_DATA_LEN_OFF]
    add64 r8, DATA_LEN_MAX_PAD
    and64 r8, DATA_LEN_AND_MASK
    # acct = &input_shifted.quote_mint
    add64 r9, RM_MISC_QUOTE_OFF
    # acct += quote_mint_padded_data_len + EmptyAccount.size
    add64 r9, r8
    add64 r9, SIZE_OF_EMPTY_ACCOUNT
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
    # frame.pda_seeds.idx_2.addr = &frame.bump
    stxdw [r10 + RM_FM_PDA_SEEDS_IDX_2_ADDR_OFF], r5
    # frame.pda_seeds.idx_2.len = u8.size
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
    # if acct.duplicate != account.NON_DUP_MARKER
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
    # frame.sol_instruction.program_id = &acct.address
    mov64 r7, r9
    add64 r7, ACCT_ADDRESS_OFF
    stxdw [r10 + RM_FM_SOL_INSN_PROGRAM_ID_UOFF], r7
    # system_program_padded_data_len = acct.padded_data_len
    ldxdw r7, [r9 + ACCT_DATA_LEN_OFF]
    add64 r7, DATA_LEN_MAX_PAD
    and64 r7, DATA_LEN_AND_MASK
    # acct += system_program_padded_data_len + EmptyAccount.size
    add64 r9, r7
    add64 r9, SIZE_OF_EMPTY_ACCOUNT
    # if acct.duplicate != account.NON_DUP_MARKER
    #     return ErrorCode::RentSysvarIsDuplicate
    ldxb r7, [r9 + ACCT_DUPLICATE_OFF]
    jne r7, ACCT_NON_DUP_MARKER, e_rent_sysvar_is_duplicate
    # if acct.pubkey != pubkey.RENT
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
    # Optimize: pubkey.RENT chunk 3 hi bits are zero, so mov32
    # (1 CU) replaces lddw (2 CUs).
    mov32 r8, PUBKEY_RENT_CHUNK_3_LO
    jne r7, r8, e_invalid_rent_sysvar_pubkey
    # frame.create_account_data.space = MarketHeader.size
    mov64 r7, SIZE_OF_MARKET_HEADER
    stxdw [r10 + RM_FM_CREATE_ACCT_SPACE_UOFF], r7
    # acct_size = MarketHeader.size + account.STORAGE_OVERHEAD
    add64 r7, ACCT_STORAGE_OVERHEAD
    # lamports_per_byte = acct.data.lamports_per_byte
    ldxdw r8, [r9 + ACCT_DATA_OFF]
    # frame.create_account_data.lamports = acct_size * lamports_per_byte
    mul64 r7, r8
    stxdw [r10 + RM_FM_CREATE_ACCT_LAMPORTS_UOFF], r7
    # rent_sysvar_padded_data_len = acct.padded_data_len
    ldxdw r7, [r9 + ACCT_DATA_LEN_OFF]
    add64 r7, DATA_LEN_MAX_PAD
    and64 r7, DATA_LEN_AND_MASK
    # acct += rent_sysvar_padded_data_len + EmptyAccount.size
    add64 r9, r7
    add64 r9, SIZE_OF_EMPTY_ACCOUNT
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
    # syscall.instruction = &frame.sol_instruction (r1 from pda_seeds)
    add64 r1, RM_FM_PDA_SEEDS_TO_SOL_INSN_REL_OFF_IMM
    # syscall.account_infos = &frame.cpi.account_infos
    mov64 r2, r10
    add64 r2, RM_FM_CPI_SOL_ACCT_INFO_OFF
    # syscall.account_infos_len = register_misc.CREATE_ACCOUNT_N_ACCOUNTS
    mov64 r3, RM_MISC_CREATE_ACCOUNT_N_ACCOUNTS
    # syscall.seeds = &frame.signers_seeds (r4 from pda)
    add64 r4, RM_FM_PDA_TO_SIGNERS_SEEDS_REL_OFF_IMM
    # syscall.seeds_len = register_misc.N_PDA_SIGNERS
    mov64 r5, RM_MISC_N_PDA_SIGNERS
    call sol_invoke_signed_c
    # input = frame.input
    ldxdw r8, [r10 + RM_FM_INPUT_OFF]
    # input_shifted = frame.input_shifted
    ldxdw r6, [r10 + RM_FM_INPUT_SHIFTED_OFF]
    # if acct.duplicate != account.NON_DUP_MARKER
    #     return ErrorCode::BaseTokenProgramIsDuplicate
    ldxb r7, [r9 + ACCT_DUPLICATE_OFF]
    jne r7, ACCT_NON_DUP_MARKER, e_base_token_program_is_duplicate
    # if acct.pubkey != input.base_mint.owner
    #     return ErrorCode::BaseTokenProgramNotBaseMintOwner
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_0_OFF]
    ldxdw r1, [r8 + RM_MISC_BASE_OWNER_CHUNK_0_OFF]
    jne r7, r1, e_base_token_program_not_base_mint_owner
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_1_OFF]
    ldxdw r1, [r8 + RM_MISC_BASE_OWNER_CHUNK_1_OFF]
    jne r7, r1, e_base_token_program_not_base_mint_owner
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_2_OFF]
    ldxdw r1, [r8 + RM_MISC_BASE_OWNER_CHUNK_2_OFF]
    jne r7, r1, e_base_token_program_not_base_mint_owner
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_3_OFF]
    ldxdw r1, [r8 + RM_MISC_BASE_OWNER_CHUNK_3_OFF]
    jne r7, r1, e_base_token_program_not_base_mint_owner
    # if acct.pubkey != pubkey.TOKEN_PROGRAM
    #     if acct.pubkey != pubkey.TOKEN_2022_PROGRAM
    #         return ErrorCode::BaseTokenProgramNotTokenProgram
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_0_OFF]
    lddw r1, PUBKEY_TOKEN_PROGRAM_CHUNK_0
    jne r7, r1, register_market_check_base_token_2022
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_1_OFF]
    lddw r1, PUBKEY_TOKEN_PROGRAM_CHUNK_1
    jne r7, r1, register_market_check_base_token_2022
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_2_OFF]
    lddw r1, PUBKEY_TOKEN_PROGRAM_CHUNK_2
    jne r7, r1, register_market_check_base_token_2022
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_3_OFF]
    lddw r1, PUBKEY_TOKEN_PROGRAM_CHUNK_3
    jeq r7, r1, register_market_base_vault
register_market_check_base_token_2022:
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_0_OFF]
    lddw r1, PUBKEY_TOKEN_2022_PROGRAM_CHUNK_0
    jne r7, r1, e_base_token_program_not_token_program
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_1_OFF]
    lddw r1, PUBKEY_TOKEN_2022_PROGRAM_CHUNK_1
    jne r7, r1, e_base_token_program_not_token_program
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_2_OFF]
    lddw r1, PUBKEY_TOKEN_2022_PROGRAM_CHUNK_2
    jne r7, r1, e_base_token_program_not_token_program
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_3_OFF]
    lddw r1, PUBKEY_TOKEN_2022_PROGRAM_CHUNK_3
    jne r7, r1, e_base_token_program_not_token_program
    # frame.token_program_is_2022 = true
    stb [r10 + RM_FM_TOKEN_PROGRAM_IS_2022_UOFF], DATA_BOOL_TRUE
register_market_base_vault:
    # frame.token_program_id = &acct.address
    mov64 r7, r9
    add64 r7, ACCT_ADDRESS_OFF
    stxdw [r10 + RM_FM_TOKEN_PROGRAM_ID_OFF], r7
    # base_token_program_padded_data_len = acct.padded_data_len
    ldxdw r7, [r9 + ACCT_DATA_LEN_OFF]
    add64 r7, DATA_LEN_MAX_PAD
    and64 r7, DATA_LEN_AND_MASK
    # acct += base_token_program_padded_data_len + EmptyAccount.size
    add64 r9, r7
    add64 r9, SIZE_OF_EMPTY_ACCOUNT
    # frame.vault_index = register_misc.VAULT_INDEX_BASE
    stb [r10 + RM_FM_VAULT_INDEX_UOFF], RM_MISC_VAULT_INDEX_BASE
    # result = INIT-VAULT(acct, frame)
    mov64 r1, r9
    mov64 r2, r10
    call init_vault
    # if result != entrypoint.RETURN_SUCCESS
    #     return result
    jeq r0, RETURN_SUCCESS, register_market_quote_token_program
    exit
register_market_quote_token_program:
    # base_vault_padded_data_len = acct.padded_data_len
    ldxdw r7, [r9 + ACCT_DATA_LEN_OFF]
    add64 r7, DATA_LEN_MAX_PAD
    and64 r7, DATA_LEN_AND_MASK
    # acct += base_vault_padded_data_len + EmptyAccount.size
    add64 r9, r7
    add64 r9, SIZE_OF_EMPTY_ACCOUNT
    # if acct.duplicate == account.NON_DUP_MARKER
    ldxb r7, [r9 + ACCT_DUPLICATE_OFF]
    jne r7, ACCT_NON_DUP_MARKER, register_market_base_vault_dup
    # if acct.pubkey != input_shifted.quote_mint.owner
    #     return ErrorCode::NonDupQuoteTokenProgramNotQuoteMintOwner
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_0_OFF]
    ldxdw r2, [r6 + RM_MISC_QUOTE_OWNER_CHUNK_0_OFF]
    jne r7, r2, e_non_dup_quote_token_program_not_quote_mint_owner
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_1_OFF]
    ldxdw r2, [r6 + RM_MISC_QUOTE_OWNER_CHUNK_1_OFF]
    jne r7, r2, e_non_dup_quote_token_program_not_quote_mint_owner
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_2_OFF]
    ldxdw r2, [r6 + RM_MISC_QUOTE_OWNER_CHUNK_2_OFF]
    jne r7, r2, e_non_dup_quote_token_program_not_quote_mint_owner
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_3_OFF]
    ldxdw r2, [r6 + RM_MISC_QUOTE_OWNER_CHUNK_3_OFF]
    jne r7, r2, e_non_dup_quote_token_program_not_quote_mint_owner
    # if acct.pubkey != pubkey.TOKEN_PROGRAM
    #     if acct.pubkey != pubkey.TOKEN_2022_PROGRAM
    #         return ErrorCode::QuoteTokenProgramNotTokenProgram
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_0_OFF]
    lddw r2, PUBKEY_TOKEN_PROGRAM_CHUNK_0
    jne r7, r2, register_market_check_quote_token_2022
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_1_OFF]
    lddw r2, PUBKEY_TOKEN_PROGRAM_CHUNK_1
    jne r7, r2, register_market_check_quote_token_2022
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_2_OFF]
    lddw r2, PUBKEY_TOKEN_PROGRAM_CHUNK_2
    jne r7, r2, register_market_check_quote_token_2022
    ldxdw r7, [r9 + ACCT_ADDRESS_CHUNK_3_OFF]
    lddw r2, PUBKEY_TOKEN_PROGRAM_CHUNK_3
    jeq r7, r2, register_market_quote_is_token_program
register_market_check_quote_token_2022:
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
    ja register_market_advance_quote_non_dup
register_market_quote_is_token_program:
    # frame.token_program_is_2022 = false
    stb [r10 + RM_FM_TOKEN_PROGRAM_IS_2022_UOFF], DATA_BOOL_FALSE
register_market_advance_quote_non_dup:
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
    ja register_market_done_token_programs
register_market_base_vault_dup:
    # if acct.duplicate != RegisterMarketAccounts::BaseTokenProgram
    #     return ErrorCode::InvalidQuoteTokenProgramDuplicate
    jne r7, REGISTER_MARKET_ACCOUNTS_BASE_TOKEN_PROGRAM_POS, e_invalid_quote_token_program_duplicate
    # if input.base_mint.owner != input_shifted.quote_mint.owner
    #     return ErrorCode::DupQuoteTokenProgramNotQuoteMintOwner
    ldxdw r7, [r8 + RM_MISC_BASE_OWNER_CHUNK_0_OFF]
    ldxdw r2, [r6 + RM_MISC_QUOTE_OWNER_CHUNK_0_OFF]
    jne r7, r2, e_dup_quote_token_program_not_quote_mint_owner
    ldxdw r7, [r8 + RM_MISC_BASE_OWNER_CHUNK_1_OFF]
    ldxdw r2, [r6 + RM_MISC_QUOTE_OWNER_CHUNK_1_OFF]
    jne r7, r2, e_dup_quote_token_program_not_quote_mint_owner
    ldxdw r7, [r8 + RM_MISC_BASE_OWNER_CHUNK_2_OFF]
    ldxdw r2, [r6 + RM_MISC_QUOTE_OWNER_CHUNK_2_OFF]
    jne r7, r2, e_dup_quote_token_program_not_quote_mint_owner
    ldxdw r7, [r8 + RM_MISC_BASE_OWNER_CHUNK_3_OFF]
    ldxdw r2, [r6 + RM_MISC_QUOTE_OWNER_CHUNK_3_OFF]
    jne r7, r2, e_dup_quote_token_program_not_quote_mint_owner
    # acct += u64.size
    add64 r9, SIZE_OF_U64
register_market_done_token_programs:
    # frame.vault_index = register_misc.VAULT_INDEX_QUOTE
    stb [r10 + RM_FM_VAULT_INDEX_UOFF], RM_MISC_VAULT_INDEX_QUOTE
    # INIT-VAULT(acct, frame)
    mov64 r1, r9
    mov64 r2, r10
    call init_vault
    exit
