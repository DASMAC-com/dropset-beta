# RegisterMarketData instruction data length.
.equ REGISTER_MARKET_DATA_LEN, 1

# RegisterMarketAccounts number of accounts.
.equ REGISTER_MARKET_ACCOUNTS_LEN, 10

# Stack frame for REGISTER-MARKET.
# -------------------------------------------------------------------------
.equ RM_PDA_SEEDS_OFF, -88 # Signer seeds offset.
.equ RM_PDA_SEEDS_N_SEEDS, 3 # Number of signer seeds.
.equ RM_PDA_SEEDS_BASE_ADDR_OFF, -88 # Base signer seed address.
.equ RM_PDA_SEEDS_BASE_LEN_OFF, -80 # Base signer seed length.
.equ RM_PDA_SEEDS_QUOTE_ADDR_OFF, -72 # Quote signer seed address.
.equ RM_PDA_SEEDS_QUOTE_LEN_OFF, -64 # Quote signer seed length.
.equ RM_PDA_SEEDS_BUMP_ADDR_OFF, -56 # Bump signer seed address.
.equ RM_PDA_SEEDS_BUMP_LEN_OFF, -48 # Bump signer seed length.
.equ RM_PDA_OFF, -40 # PDA address.
.equ RM_PDA_CHUNK_0_OFF, -40 # PDA address (chunk 0).
.equ RM_PDA_CHUNK_1_OFF, -32 # PDA address (chunk 1).
.equ RM_PDA_CHUNK_2_OFF, -24 # PDA address (chunk 2).
.equ RM_PDA_CHUNK_3_OFF, -16 # PDA address (chunk 3).
.equ RM_BUMP_OFF, -8 # Bump seed.
# -------------------------------------------------------------------------

# Assorted register market constants.
# -------------------------------------------------------------------------
# From input buffer to base mint duplicate flag.
.equ RM_MISC_BASE_MINT_DUPLICATE_OFF, 20680
# From input buffer to base mint data length.
.equ RM_MISC_BASE_DATA_LEN_OFF, 20760
.equ RM_MISC_BASE_ADDR_OFF, 20688 # From input buffer to base mint address.
# From input buffer to quote mint duplicate flag.
.equ RM_MISC_QUOTE_MINT_DUPLICATE_OFF, 31016
# From input buffer to quote mint address.
.equ RM_MISC_QUOTE_ADDR_OFF, 31024
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
    # if user.data_len != data.DATA_LEN_ZERO
    #     return ErrorCode::UserHasData
    ldxdw r9, [r1 + IB_USER_DATA_LEN_OFF]
    jne r9, DATA_DATA_LEN_ZERO, e_user_has_data
    # if market.duplicate != input_buffer.NON_DUP_MARKER
    #     return ErrorCode::MarketAccountIsDuplicate
    ldxb r9, [r1 + IB_MARKET_DUPLICATE_OFF]
    jne r9, IB_NON_DUP_MARKER, e_market_account_is_duplicate
    # if market.data_len != DATA_DATA_LEN_ZERO
    #     return ErrorCode::MarketHasData
    ldxdw r9, [r1 + IB_MARKET_DATA_LEN_OFF]
    jne r9, DATA_DATA_LEN_ZERO, e_market_has_data
    # if base_mint.duplicate != input_buffer.NON_DUP_MARKER
    #     return ErrorCode::BaseMintIsDuplicate
    ldxb r9, [r1 + RM_MISC_BASE_MINT_DUPLICATE_OFF]
    jne r9, IB_NON_DUP_MARKER, e_base_mint_is_duplicate
    # pda_seeds.base.addr = base_mint.pubkey
    mov64 r9, r1
    add64 r9, RM_MISC_BASE_ADDR_OFF
    stxdw [r10 + RM_PDA_SEEDS_BASE_ADDR_OFF], r9
    # pda_seeds.base.len = Address.size
    mov64 r9, SIZE_OF_ADDRESS
    stxdw [r10 + RM_PDA_SEEDS_BASE_LEN_OFF], r9
    # input_shifted = input + base_mint.padded_data_len
    ldxdw r9, [r1 + RM_MISC_BASE_DATA_LEN_OFF]
    add64 r9, DATA_MAX_DATA_PAD
    and64 r9, DATA_DATA_LEN_AND_MASK
    add64 r9, r1
    # if quote_mint.duplicate != input_buffer.NON_DUP_MARKER
    #     return ErrorCode::QuoteMintIsDuplicate
    ldxb r8, [r9 + RM_MISC_QUOTE_MINT_DUPLICATE_OFF]
    jne r8, IB_NON_DUP_MARKER, e_quote_mint_is_duplicate
    # pda_seeds.quote.addr = quote_mint.pubkey
    mov64 r8, r9
    add64 r8, RM_MISC_QUOTE_ADDR_OFF
    stxdw [r10 + RM_PDA_SEEDS_QUOTE_ADDR_OFF], r8
    # pda_seeds.quote.len = Address.size
    mov64 r8, SIZE_OF_ADDRESS
    stxdw [r10 + RM_PDA_SEEDS_QUOTE_LEN_OFF], r8
    # syscall.program_id = program_id
    mov64 r3, r2
    add64 r3, REGISTER_MARKET_DATA_LEN
    # syscall.seeds = pda_seeds
    mov64 r1, r10
    add64 r1, RM_PDA_SEEDS_OFF
    # syscall.seeds_len = register_misc.TRY_FIND_PDA_SEEDS_LEN
    mov64 r2, RM_MISC_TRY_FIND_PDA_SEEDS_LEN
    # syscall.program_address = RegisterMarketFrame.pda
    mov64 r4, r10
    add64 r4, RM_PDA_OFF
    # syscall.bump_seed = RegisterMarketFrame.bump
    mov64 r5, r10
    add64 r5, RM_BUMP_OFF
    call sol_try_find_program_address
    exit
