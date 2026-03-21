# RegisterMarketData instruction data length.
.equ REGISTER_MARKET_DATA_LEN, 1
# RegisterMarketAccounts number of accounts.
.equ REGISTER_MARKET_ACCOUNTS_LEN, 6

register_market:
    # if input.n_accounts < RegisterMarketAccounts.LEN
    #     return ErrorCode::InvalidNumberOfAccounts
    jlt r3, REGISTER_MARKET_ACCOUNTS_LEN, e_invalid_number_of_accounts
    # if insn_len != RegisterMarketData.LEN
    #     return ErrorCode::InvalidInstructionLength
    jne r4, REGISTER_MARKET_DATA_LEN, e_invalid_instruction_length
    # if user.data_len != DATA_DATA_LEN_ZERO
    #     return ErrorCode::UserHasData
    ldxdw r4, [r1 + IB_USER_DATA_LEN_OFF]
    jne r4, DATA_DATA_LEN_ZERO, e_user_has_data
    # if market.duplicate != IB_NON_DUP_MARKER
    #     return ErrorCode::MarketAccountIsDuplicate
    ldxb r4, [r1 + IB_MARKET_DUPLICATE_OFF]
    jne r4, IB_NON_DUP_MARKER, e_market_account_is_duplicate
    # if market.data_len != DATA_DATA_LEN_ZERO
    #     return ErrorCode::MarketHasData
    ldxdw r4, [r1 + IB_MARKET_DATA_LEN_OFF]
    jne r4, DATA_DATA_LEN_ZERO, e_market_has_data
    exit
