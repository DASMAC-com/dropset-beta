# Instruction accounts.
# -------------------------------------------------------------------------
.equ RM_INSN_ACCTS_COUNT, 10 # Number of accounts.
.equ RM_INSN_ACCTS_USER_POS, 0 # User account position.
.equ RM_INSN_ACCTS_MARKET_POS, 1 # Market account position.
.equ RM_INSN_ACCTS_BASE_MINT_POS, 2 # Base Mint account position.
.equ RM_INSN_ACCTS_QUOTE_MINT_POS, 3 # Quote Mint account position.
.equ RM_INSN_ACCTS_SYSTEM_PROGRAM_POS, 4 # System Program account position.
.equ RM_INSN_ACCTS_RENT_SYSVAR_POS, 5 # Rent Sysvar account position.
# Base Token Program account position.
.equ RM_INSN_ACCTS_BASE_TOKEN_PROGRAM_POS, 6
.equ RM_INSN_ACCTS_BASE_VAULT_POS, 7 # Base Vault account position.
# Quote Token Program account position.
.equ RM_INSN_ACCTS_QUOTE_TOKEN_PROGRAM_POS, 8
.equ RM_INSN_ACCTS_QUOTE_VAULT_POS, 9 # Quote Vault account position.
# -------------------------------------------------------------------------

# Instruction data.
# -------------------------------------------------------------------------
.equ RM_INSN_DATA_SIZE, 1 # Instruction data size.
# -------------------------------------------------------------------------

# Market registration-related constants.
# -------------------------------------------------------------------------
# From input buffer to base mint duplicate flag.
.equ RM_BASE_DUPLICATE_OFF, 20680
# From input buffer to base mint data length.
.equ RM_BASE_DATA_LEN_OFF, 20760
# From input buffer to base mint address field.
.equ RM_BASE_ADDR_OFF, 20688
.equ RM_BASE_OWNER_OFF, 20720 # From input buffer to base mint owner.
# From input buffer to base mint owner (chunk 0).
.equ RM_BASE_OWNER_CHUNK_0_OFF, 20720
# From input buffer to base mint owner (chunk 1).
.equ RM_BASE_OWNER_CHUNK_1_OFF, 20728
# From input buffer to base mint owner (chunk 2).
.equ RM_BASE_OWNER_CHUNK_2_OFF, 20736
# From input buffer to base mint owner (chunk 3).
.equ RM_BASE_OWNER_CHUNK_3_OFF, 20744
.equ RM_QUOTE_OFF, 31016 # From input buffer to quote mint.
# From input buffer to quote mint duplicate flag.
.equ RM_QUOTE_DUPLICATE_OFF, 31016
# From input buffer to quote mint address field.
.equ RM_QUOTE_ADDR_OFF, 31024
.equ RM_QUOTE_OWNER_OFF, 31056 # From input buffer to quote mint owner.
# From input buffer to quote mint owner (chunk 0).
.equ RM_QUOTE_OWNER_CHUNK_0_OFF, 31056
# From input buffer to quote mint owner (chunk 1).
.equ RM_QUOTE_OWNER_CHUNK_1_OFF, 31064
# From input buffer to quote mint owner (chunk 2).
.equ RM_QUOTE_OWNER_CHUNK_2_OFF, 31072
# From input buffer to quote mint owner (chunk 3).
.equ RM_QUOTE_OWNER_CHUNK_3_OFF, 31080
# From input buffer to quote mint data length.
.equ RM_QUOTE_DATA_LEN_OFF, 31096
# Number of seeds for market PDA derivation (base mint, quote mint).
.equ RM_TRY_FIND_MARKET_PDA_SEEDS_LEN, 2
# Number of seeds for vault PDA derivation (market address, vault index).
.equ RM_TRY_FIND_VAULT_PDA_SEEDS_LEN, 2
# Number of accounts for system_program::CreateAccount CPI.
.equ RM_CREATE_ACCOUNT_N_ACCOUNTS, 2
.equ RM_N_PDA_SIGNERS, 1 # Number of PDA signers for CPI.
# -------------------------------------------------------------------------

# Stack frame for REGISTER-MARKET.
# -------------------------------------------------------------------------
.equ RM_FM_TOKEN_PROGRAM_ID_OFF, -592 # Pointer to token program ID.
.equ RM_FM_PROGRAM_ID_OFF, -584 # Pointer to program ID in input buffer.
.equ RM_FM_INPUT_OFF, -576 # Saved input buffer pointer.
.equ RM_FM_INPUT_SHIFTED_OFF, -568 # Saved input_shifted pointer.
.equ RM_FM_LAMPORTS_PER_BYTE_OFF, -560 # From Rent sysvar.
# Return value from spl_token_2022::GetAccountDataSize.
.equ RM_FM_TOKEN_ACCOUNT_DATA_SIZE_OFF, -552
# Pointer to mint account for vault initialization.
.equ RM_FM_MINT_OFF, -544
.equ RM_FM_RENT_OFF, -536 # Pointer to Rent sysvar account.
.equ RM_FM_PDA_SEEDS_OFF, -528 # Signer seeds offset.
.equ RM_FM_PDA_SEEDS_N_SEEDS, 3 # Number of signer seeds.
.equ RM_FM_PDA_SEEDS_IDX_0_ADDR_OFF, -528 # Idx 0 signer seed address.
.equ RM_FM_PDA_SEEDS_IDX_0_LEN_OFF, -520 # Idx 0 signer seed length.
.equ RM_FM_PDA_SEEDS_IDX_1_ADDR_OFF, -512 # Idx 1 signer seed address.
.equ RM_FM_PDA_SEEDS_IDX_1_LEN_OFF, -504 # Idx 1 signer seed length.
.equ RM_FM_PDA_SEEDS_IDX_2_ADDR_OFF, -496 # Idx 2 signer seed address.
.equ RM_FM_PDA_SEEDS_IDX_2_LEN_OFF, -488 # Idx 2 signer seed length.
.equ RM_FM_PDA_OFF, -480 # PDA pubkey.
.equ RM_FM_PDA_CHUNK_0_OFF, -480 # PDA pubkey (chunk 0).
.equ RM_FM_PDA_CHUNK_1_OFF, -472 # PDA pubkey (chunk 1).
.equ RM_FM_PDA_CHUNK_2_OFF, -464 # PDA pubkey (chunk 2).
.equ RM_FM_PDA_CHUNK_3_OFF, -456 # PDA pubkey (chunk 3).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_OFF, -448 # System Program pubkey.
# System Program pubkey (chunk 0).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_0_OFF, -448
# System Program pubkey (chunk 1).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_1_OFF, -440
# System Program pubkey (chunk 2).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_2_OFF, -432
# System Program pubkey (chunk 3).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_3_OFF, -424
.equ RM_FM_SYSTEM_PROGRAM_ID_OFF, -416 # System Program ID in input buffer.
# Get return data program ID for CPI calls.
.equ RM_FM_GET_RETURN_DATA_PROGRAM_ID_OFF, -408
# system_program::CreateAccount instruction data.
.equ RM_FM_CREATE_ACCT_DATA_OFF, -376
# system_program::CreateAccount lamports field.
.equ RM_FM_CREATE_ACCT_LAMPORTS_UOFF, -372
# system_program::CreateAccount space field.
.equ RM_FM_CREATE_ACCT_SPACE_UOFF, -364
# system_program::CreateAccount owner field.
.equ RM_FM_CREATE_ACCT_OWNER_UOFF, -356
# system_program::CreateAccount owner field (chunk 0).
.equ RM_FM_CREATE_ACCT_OWNER_CHUNK_0_UOFF, -356
# system_program::CreateAccount owner field (chunk 1).
.equ RM_FM_CREATE_ACCT_OWNER_CHUNK_1_UOFF, -348
# system_program::CreateAccount owner field (chunk 2).
.equ RM_FM_CREATE_ACCT_OWNER_CHUNK_2_UOFF, -340
# system_program::CreateAccount owner field (chunk 3).
.equ RM_FM_CREATE_ACCT_OWNER_CHUNK_3_UOFF, -332
# spl_token_2022::GetAccountDataSize CPI instruction data.
.equ RM_FM_GET_ACCOUNT_DATA_SIZE_DATA_UOFF, -324
.equ RM_FM_VAULT_INDEX_UOFF, -323 # Vault index for PDA derivation.
# Whether the current token program is Token 2022.
.equ RM_FM_TOKEN_PROGRAM_IS_2022_UOFF, -322
# spl_token::InitializeAccount2 CPI instruction data.
.equ RM_FM_INIT_ACCT_2_DATA_OFF, -320
# spl_token::InitializeAccount2 discriminant field.
.equ RM_FM_INIT_ACCT_2_DISC_UOFF, -320
# spl_token::InitializeAccount2 proprietor field.
.equ RM_FM_INIT_ACCT_2_PROPRIETOR_UOFF, -319
# spl_token::InitializeAccount2 proprietor field (chunk 0).
.equ RM_FM_INIT_ACCT_2_PROPRIETOR_CHUNK_0_UOFF, -319
# spl_token::InitializeAccount2 proprietor field (chunk 1).
.equ RM_FM_INIT_ACCT_2_PROPRIETOR_CHUNK_1_UOFF, -311
# spl_token::InitializeAccount2 proprietor field (chunk 2).
.equ RM_FM_INIT_ACCT_2_PROPRIETOR_CHUNK_2_UOFF, -303
# spl_token::InitializeAccount2 proprietor field (chunk 3).
.equ RM_FM_INIT_ACCT_2_PROPRIETOR_CHUNK_3_UOFF, -295
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
# From pda_seeds to sol_instruction.
.equ RM_FM_PDA_SEEDS_TO_SOL_INSN_REL_OFF_IMM, 480
# From pda to signers_seeds.
.equ RM_FM_PDA_TO_SIGNERS_SEEDS_REL_OFF_IMM, 416
# From create_account_data to CPI account metas.
.equ RM_FM_CREATE_ACCT_DATA_TO_CPI_ACCT_METAS_REL_OFF_IMM, 264
# -------------------------------------------------------------------------

register_market:
    # acct = MARKET-PDA-PRELUDE(input, insn, n_accounts, insn_len, frame)
    ja market_pda_prelude
market_pda_prelude_return:
    # INIT-MARKET-PDA(input, insn, acct, frame)
    ja init_market_pda
init_market_pda_return:
    # input_shifted = INIT-BASE-VAULT(acct, frame)
    ja init_base_vault
init_base_vault_return:
    # if acct.duplicate == common::account::NON_DUP_MARKER
    ldxb r7, [r9 + ACCT_DUPLICATE_OFF]
    jne r7, ACCT_NON_DUP_MARKER, register_market_base_vault_dup
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
register_market_done_token_programs:
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
    # input.market.data.quote_vault_bump = frame.bump
    ldxb r7, [r10 + RM_FM_BUMP_OFF]
    stxb [r8 + IB_MARKET_DATA_QUOTE_VAULT_BUMP_OFF], r7
    exit
