# RegisterMarketData instruction data length.
.equ REGISTER_MARKET_DATA_LEN, 1

# RegisterMarketAccounts number of accounts.
.equ REGISTER_MARKET_ACCOUNTS_LEN, 10

# Stack frame for REGISTER-MARKET.
# -------------------------------------------------------------------------
.equ RM_PDA_SEEDS_BASE_ADDR_OFF, -88 # Base signer seed address.
.equ RM_PDA_SEEDS_BASE_LEN_OFF, -80 # Base signer seed length.
.equ RM_PDA_SEEDS_QUOTE_ADDR_OFF, -72 # Quote signer seed address.
.equ RM_PDA_SEEDS_QUOTE_LEN_OFF, -64 # Quote signer seed length.
.equ RM_PDA_SEEDS_BUMP_ADDR_OFF, -56 # Bump signer seed address.
.equ RM_PDA_SEEDS_BUMP_LEN_OFF, -48 # Bump signer seed length.
.equ RM_PDA_SEEDS_N_SEEDS, 3 # Number of signer seeds.
.equ RM_PDA_OFF, -40 # PDA address.
.equ RM_BUMP_OFF, -8 # Bump seed.
# -------------------------------------------------------------------------

# Assorted register market constants.
# -------------------------------------------------------------------------
# From input buffer to base mint duplicate flag.
.equ RM_IB_BASE_MINT_DUPLICATE_OFF, 20680
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
    ldxdw r4, [r1 + IB_USER_DATA_LEN_OFF]
    jne r4, DATA_DATA_LEN_ZERO, e_user_has_data
    # if market.duplicate != input_buffer.NON_DUP_MARKER
    #     return ErrorCode::MarketAccountIsDuplicate
    ldxb r4, [r1 + IB_MARKET_DUPLICATE_OFF]
    jne r4, IB_NON_DUP_MARKER, e_market_account_is_duplicate
    # if market.data_len != DATA_DATA_LEN_ZERO
    #     return ErrorCode::MarketHasData
    ldxdw r4, [r1 + IB_MARKET_DATA_LEN_OFF]
    jne r4, DATA_DATA_LEN_ZERO, e_market_has_data
    # if base_mint.duplicate != input_buffer.NON_DUP_MARKER
    #     return ErrorCode::BaseMintIsDuplicate
    ldxb r4, [r1 + RM_IB_BASE_MINT_DUPLICATE_OFF]
    jne r4, IB_NON_DUP_MARKER, e_base_mint_is_duplicate
    exit
