# RegisterMarketData instruction data length.
.equ REGISTER_MARKET_DATA_LEN, 1
# RegisterMarketAccounts number of accounts.
.equ REGISTER_MARKET_ACCOUNTS_LEN, 6

register_market:
    # if insn_len != RegisterMarketData.LEN
    #     return ErrorCode::InvalidInstructionLength
    jne r3, REGISTER_MARKET_DATA_LEN, e_invalid_instruction_length
    # if input.n_accounts != RegisterMarketAccounts.LEN
    #     return ErrorCode::InvalidNumberOfAccounts
    ldxdw r3, [r1 + 0]
    jne r3, REGISTER_MARKET_ACCOUNTS_LEN, e_invalid_number_of_accounts
    exit
