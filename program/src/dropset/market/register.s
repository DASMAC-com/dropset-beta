# RegisterMarketData instruction data length.
.equ REGISTER_MARKET_DATA_LEN, 1

# RegisterMarketAccounts number of accounts.
.equ REGISTER_MARKET_ACCOUNTS_LEN, 10

# Stack frame for REGISTER-MARKET.
# -------------------------------------------------------------------------
.equ RM_FM_PDA_SEEDS_OFF, -608 # Signer seeds offset.
.equ RM_FM_PDA_SEEDS_N_SEEDS, 3 # Number of signer seeds.
.equ RM_FM_PDA_SEEDS_BASE_ADDR_OFF, -608 # Base signer seed address.
.equ RM_FM_PDA_SEEDS_BASE_LEN_OFF, -600 # Base signer seed length.
.equ RM_FM_PDA_SEEDS_QUOTE_ADDR_OFF, -592 # Quote signer seed address.
.equ RM_FM_PDA_SEEDS_QUOTE_LEN_OFF, -584 # Quote signer seed length.
.equ RM_FM_PDA_SEEDS_BUMP_ADDR_OFF, -576 # Bump signer seed address.
.equ RM_FM_PDA_SEEDS_BUMP_LEN_OFF, -568 # Bump signer seed length.
.equ RM_FM_PDA_OFF, -560 # PDA address.
.equ RM_FM_PDA_CHUNK_0_OFF, -560 # PDA address (chunk 0).
.equ RM_FM_PDA_CHUNK_1_OFF, -552 # PDA address (chunk 1).
.equ RM_FM_PDA_CHUNK_2_OFF, -544 # PDA address (chunk 2).
.equ RM_FM_PDA_CHUNK_3_OFF, -536 # PDA address (chunk 3).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_OFF, -528 # System Program pubkey.
# System Program pubkey (chunk 0).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_0_OFF, -528
# System Program pubkey (chunk 1).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_1_OFF, -520
# System Program pubkey (chunk 2).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_2_OFF, -512
# System Program pubkey (chunk 3).
.equ RM_FM_SYSTEM_PROGRAM_PUBKEY_CHUNK_3_OFF, -504
.equ RM_FM_BUMP_OFF, -8 # Bump seed.
.equ RM_FM_CREATE_ACCT_DATA_OFF, -496 # CreateAccount instruction data.
# Lamports field within CreateAccount instruction data.
.equ RM_FM_CREATE_ACCT_LAMPORTS_UOFF, -492
# Space field within CreateAccount instruction data.
.equ RM_FM_CREATE_ACCT_SPACE_UOFF, -484
# Owner field within CreateAccount instruction data.
.equ RM_FM_CREATE_ACCT_OWNER_UOFF, -476
# Owner field within CreateAccount instruction data (chunk 0).
.equ RM_FM_CREATE_ACCT_OWNER_CHUNK_0_UOFF, -476
# Owner field within CreateAccount instruction data (chunk 1).
.equ RM_FM_CREATE_ACCT_OWNER_CHUNK_1_UOFF, -468
# Owner field within CreateAccount instruction data (chunk 2).
.equ RM_FM_CREATE_ACCT_OWNER_CHUNK_2_UOFF, -460
# Owner field within CreateAccount instruction data (chunk 3).
.equ RM_FM_CREATE_ACCT_OWNER_CHUNK_3_UOFF, -452
.equ RM_FM_CPI_N_ACCOUNTS, 6 # Number of CPI accounts.
.equ RM_FM_CPI_SOL_ACCT_INFO_OFF, -440 # Start of SolAccountInfo vector.
.equ RM_FM_CPI_SOL_ACCT_META_OFF, -104 # Start of SolAccountMeta vector.
.equ RM_FM_CPI_USER_ACCT_INFO_KEY_UOFF, -440 # User account info key.
# User account info lamports.
.equ RM_FM_CPI_USER_ACCT_INFO_LAMPORTS_UOFF, -432
# User account info data length.
.equ RM_FM_CPI_USER_ACCT_INFO_DATA_LEN_UOFF, -424
.equ RM_FM_CPI_USER_ACCT_INFO_DATA_UOFF, -416 # User account info data.
.equ RM_FM_CPI_USER_ACCT_INFO_OWNER_UOFF, -408 # User account info owner.
# User account info rent epoch.
.equ RM_FM_CPI_USER_ACCT_INFO_RENT_EPOCH_UOFF, -400
# User account info is signer.
.equ RM_FM_CPI_USER_ACCT_INFO_IS_SIGNER_UOFF, -392
# User account info is writable.
.equ RM_FM_CPI_USER_ACCT_INFO_IS_WRITABLE_UOFF, -391
# User account info executable.
.equ RM_FM_CPI_USER_ACCT_INFO_EXECUTABLE_UOFF, -390
.equ RM_FM_CPI_USER_ACCT_META_PUBKEY_UOFF, -104 # User account meta pubkey.
# User account meta is writable.
.equ RM_FM_CPI_USER_ACCT_META_IS_WRITABLE_UOFF, -96
# User account meta is signer.
.equ RM_FM_CPI_USER_ACCT_META_IS_SIGNER_UOFF, -95
.equ RM_FM_CPI_TARGET_ACCT_INFO_KEY_UOFF, -384 # Target account info key.
# Target account info lamports.
.equ RM_FM_CPI_TARGET_ACCT_INFO_LAMPORTS_UOFF, -376
# Target account info data length.
.equ RM_FM_CPI_TARGET_ACCT_INFO_DATA_LEN_UOFF, -368
.equ RM_FM_CPI_TARGET_ACCT_INFO_DATA_UOFF, -360 # Target account info data.
# Target account info owner.
.equ RM_FM_CPI_TARGET_ACCT_INFO_OWNER_UOFF, -352
# Target account info rent epoch.
.equ RM_FM_CPI_TARGET_ACCT_INFO_RENT_EPOCH_UOFF, -344
# Target account info is signer.
.equ RM_FM_CPI_TARGET_ACCT_INFO_IS_SIGNER_UOFF, -336
# Target account info is writable.
.equ RM_FM_CPI_TARGET_ACCT_INFO_IS_WRITABLE_UOFF, -335
# Target account info executable.
.equ RM_FM_CPI_TARGET_ACCT_INFO_EXECUTABLE_UOFF, -334
# Target account meta pubkey.
.equ RM_FM_CPI_TARGET_ACCT_META_PUBKEY_UOFF, -88
# Target account meta is writable.
.equ RM_FM_CPI_TARGET_ACCT_META_IS_WRITABLE_UOFF, -80
# Target account meta is signer.
.equ RM_FM_CPI_TARGET_ACCT_META_IS_SIGNER_UOFF, -79
# Proprietor account info key.
.equ RM_FM_CPI_PROPRIETOR_ACCT_INFO_KEY_UOFF, -328
# Proprietor account info lamports.
.equ RM_FM_CPI_PROPRIETOR_ACCT_INFO_LAMPORTS_UOFF, -320
# Proprietor account info data length.
.equ RM_FM_CPI_PROPRIETOR_ACCT_INFO_DATA_LEN_UOFF, -312
# Proprietor account info data.
.equ RM_FM_CPI_PROPRIETOR_ACCT_INFO_DATA_UOFF, -304
# Proprietor account info owner.
.equ RM_FM_CPI_PROPRIETOR_ACCT_INFO_OWNER_UOFF, -296
# Proprietor account info rent epoch.
.equ RM_FM_CPI_PROPRIETOR_ACCT_INFO_RENT_EPOCH_UOFF, -288
# Proprietor account info is signer.
.equ RM_FM_CPI_PROPRIETOR_ACCT_INFO_IS_SIGNER_UOFF, -280
# Proprietor account info is writable.
.equ RM_FM_CPI_PROPRIETOR_ACCT_INFO_IS_WRITABLE_UOFF, -279
# Proprietor account info executable.
.equ RM_FM_CPI_PROPRIETOR_ACCT_INFO_EXECUTABLE_UOFF, -278
# Proprietor account meta pubkey.
.equ RM_FM_CPI_PROPRIETOR_ACCT_META_PUBKEY_UOFF, -72
# Proprietor account meta is writable.
.equ RM_FM_CPI_PROPRIETOR_ACCT_META_IS_WRITABLE_UOFF, -64
# Proprietor account meta is signer.
.equ RM_FM_CPI_PROPRIETOR_ACCT_META_IS_SIGNER_UOFF, -63
.equ RM_FM_CPI_MINT_ACCT_INFO_KEY_UOFF, -272 # Mint account info key.
# Mint account info lamports.
.equ RM_FM_CPI_MINT_ACCT_INFO_LAMPORTS_UOFF, -264
# Mint account info data length.
.equ RM_FM_CPI_MINT_ACCT_INFO_DATA_LEN_UOFF, -256
.equ RM_FM_CPI_MINT_ACCT_INFO_DATA_UOFF, -248 # Mint account info data.
.equ RM_FM_CPI_MINT_ACCT_INFO_OWNER_UOFF, -240 # Mint account info owner.
# Mint account info rent epoch.
.equ RM_FM_CPI_MINT_ACCT_INFO_RENT_EPOCH_UOFF, -232
# Mint account info is signer.
.equ RM_FM_CPI_MINT_ACCT_INFO_IS_SIGNER_UOFF, -224
# Mint account info is writable.
.equ RM_FM_CPI_MINT_ACCT_INFO_IS_WRITABLE_UOFF, -223
# Mint account info executable.
.equ RM_FM_CPI_MINT_ACCT_INFO_EXECUTABLE_UOFF, -222
.equ RM_FM_CPI_MINT_ACCT_META_PUBKEY_UOFF, -56 # Mint account meta pubkey.
# Mint account meta is writable.
.equ RM_FM_CPI_MINT_ACCT_META_IS_WRITABLE_UOFF, -48
# Mint account meta is signer.
.equ RM_FM_CPI_MINT_ACCT_META_IS_SIGNER_UOFF, -47
# System Program account info key.
.equ RM_FM_CPI_SYSTEM_PROGRAM_ACCT_INFO_KEY_UOFF, -216
# System Program account info lamports.
.equ RM_FM_CPI_SYSTEM_PROGRAM_ACCT_INFO_LAMPORTS_UOFF, -208
# System Program account info data length.
.equ RM_FM_CPI_SYSTEM_PROGRAM_ACCT_INFO_DATA_LEN_UOFF, -200
# System Program account info data.
.equ RM_FM_CPI_SYSTEM_PROGRAM_ACCT_INFO_DATA_UOFF, -192
# System Program account info owner.
.equ RM_FM_CPI_SYSTEM_PROGRAM_ACCT_INFO_OWNER_UOFF, -184
# System Program account info rent epoch.
.equ RM_FM_CPI_SYSTEM_PROGRAM_ACCT_INFO_RENT_EPOCH_UOFF, -176
# System Program account info is signer.
.equ RM_FM_CPI_SYSTEM_PROGRAM_ACCT_INFO_IS_SIGNER_UOFF, -168
# System Program account info is writable.
.equ RM_FM_CPI_SYSTEM_PROGRAM_ACCT_INFO_IS_WRITABLE_UOFF, -167
# System Program account info executable.
.equ RM_FM_CPI_SYSTEM_PROGRAM_ACCT_INFO_EXECUTABLE_UOFF, -166
# System Program account meta pubkey.
.equ RM_FM_CPI_SYSTEM_PROGRAM_ACCT_META_PUBKEY_UOFF, -40
# System Program account meta is writable.
.equ RM_FM_CPI_SYSTEM_PROGRAM_ACCT_META_IS_WRITABLE_UOFF, -32
# System Program account meta is signer.
.equ RM_FM_CPI_SYSTEM_PROGRAM_ACCT_META_IS_SIGNER_UOFF, -31
# Token Program account info key.
.equ RM_FM_CPI_TOKEN_PROGRAM_ACCT_INFO_KEY_UOFF, -160
# Token Program account info lamports.
.equ RM_FM_CPI_TOKEN_PROGRAM_ACCT_INFO_LAMPORTS_UOFF, -152
# Token Program account info data length.
.equ RM_FM_CPI_TOKEN_PROGRAM_ACCT_INFO_DATA_LEN_UOFF, -144
# Token Program account info data.
.equ RM_FM_CPI_TOKEN_PROGRAM_ACCT_INFO_DATA_UOFF, -136
# Token Program account info owner.
.equ RM_FM_CPI_TOKEN_PROGRAM_ACCT_INFO_OWNER_UOFF, -128
# Token Program account info rent epoch.
.equ RM_FM_CPI_TOKEN_PROGRAM_ACCT_INFO_RENT_EPOCH_UOFF, -120
# Token Program account info is signer.
.equ RM_FM_CPI_TOKEN_PROGRAM_ACCT_INFO_IS_SIGNER_UOFF, -112
# Token Program account info is writable.
.equ RM_FM_CPI_TOKEN_PROGRAM_ACCT_INFO_IS_WRITABLE_UOFF, -111
# Token Program account info executable.
.equ RM_FM_CPI_TOKEN_PROGRAM_ACCT_INFO_EXECUTABLE_UOFF, -110
# Token Program account meta pubkey.
.equ RM_FM_CPI_TOKEN_PROGRAM_ACCT_META_PUBKEY_UOFF, -24
# Token Program account meta is writable.
.equ RM_FM_CPI_TOKEN_PROGRAM_ACCT_META_IS_WRITABLE_UOFF, -16
# Token Program account meta is signer.
.equ RM_FM_CPI_TOKEN_PROGRAM_ACCT_META_IS_SIGNER_UOFF, -15
# -------------------------------------------------------------------------

# Miscellaneous market registration constants.
# -------------------------------------------------------------------------
# From input buffer to base mint duplicate flag.
.equ RM_MISC_BASE_DUPLICATE_OFF, 20680
# From input buffer to base mint data length.
.equ RM_MISC_BASE_DATA_LEN_OFF, 20760
.equ RM_MISC_BASE_ADDR_OFF, 20688 # From input buffer to base mint address.
.equ RM_MISC_QUOTE_OFF, 31016 # From input buffer to quote mint.
# From input buffer to quote mint duplicate flag.
.equ RM_MISC_QUOTE_DUPLICATE_OFF, 31016
# From input buffer to quote mint address.
.equ RM_MISC_QUOTE_ADDR_OFF, 31024
# From input buffer to quote mint data length.
.equ RM_MISC_QUOTE_DATA_LEN_OFF, 31096
# Number of seeds for market PDA derivation (base, quote).
.equ RM_MISC_TRY_FIND_PDA_SEEDS_LEN, 2
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
    # frame.pda_seeds.base.addr = input.base_mint.pubkey
    mov64 r9, r1
    add64 r9, RM_MISC_BASE_ADDR_OFF
    stxdw [r10 + RM_FM_PDA_SEEDS_BASE_ADDR_OFF], r9
    # frame.pda_seeds.base.len = Address.size
    mov64 r9, SIZE_OF_ADDRESS
    stxdw [r10 + RM_FM_PDA_SEEDS_BASE_LEN_OFF], r9
    # input_shifted = input + input.base_mint.padded_data_len
    ldxdw r9, [r1 + RM_MISC_BASE_DATA_LEN_OFF]
    add64 r9, DATA_LEN_MAX_PAD
    and64 r9, DATA_LEN_AND_MASK
    add64 r9, r1
    # if input_shifted.quote_mint.duplicate != account.NON_DUP_MARKER
    #     return ErrorCode::QuoteMintIsDuplicate
    ldxb r8, [r9 + RM_MISC_QUOTE_DUPLICATE_OFF]
    jne r8, ACCT_NON_DUP_MARKER, e_quote_mint_is_duplicate
    # frame.pda_seeds.quote.addr = input_shifted.quote_mint.pubkey
    mov64 r8, r9
    add64 r8, RM_MISC_QUOTE_ADDR_OFF
    stxdw [r10 + RM_FM_PDA_SEEDS_QUOTE_ADDR_OFF], r8
    # quote_mint_padded_data_len = input_shifted.quote_mint.padded_data_len
    ldxdw r8, [r9 + RM_MISC_QUOTE_DATA_LEN_OFF]
    add64 r8, DATA_LEN_MAX_PAD
    and64 r8, DATA_LEN_AND_MASK
    # acct = &input_shifted.quote_mint
    add64 r9, RM_MISC_QUOTE_OFF
    # acct += quote_mint_padded_data_len + EmptyAccount.size
    add64 r9, r8
    add64 r9, SIZE_OF_EMPTY_ACCOUNT
    # frame.pda_seeds.quote.len = Address.size
    mov64 r8, SIZE_OF_ADDRESS
    stxdw [r10 + RM_FM_PDA_SEEDS_QUOTE_LEN_OFF], r8
    # Store(input)
    mov64 r6, r1
    # syscall.seeds = &frame.pda_seeds
    mov64 r1, r10
    add64 r1, RM_FM_PDA_SEEDS_OFF
    # syscall.program_id = &insn.program_id
    mov64 r3, r2
    add64 r3, REGISTER_MARKET_DATA_LEN
    # syscall.seeds_len = register_misc.TRY_FIND_PDA_SEEDS_LEN
    mov64 r2, RM_MISC_TRY_FIND_PDA_SEEDS_LEN
    # syscall.program_address = RegisterMarketFrame.pda
    mov64 r4, r10
    add64 r4, RM_FM_PDA_OFF
    # syscall.bump_seed = RegisterMarketFrame.bump
    mov64 r5, r10
    add64 r5, RM_FM_BUMP_OFF
    call sol_try_find_program_address
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
    exit
