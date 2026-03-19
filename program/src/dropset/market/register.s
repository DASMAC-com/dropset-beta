.equ REGISTER_MARKET_INSN_LEN, 0 # RegisterMarket instruction data length.

register_market:
    # if insn_len != RegisterMarket::INSN_LEN
    #     return ErrorCode::InvalidInstructionLength
    jne r3, REGISTER_MARKET_INSN_LEN, e_invalid_instruction_length
    exit
