register_market:
    # if insn_len != InstructionLength::RegisterMarket
    #     return ErrorCode::InvalidInstructionLength
    jne r3, INSN_LEN_REGISTER_MARKET, e_invalid_instruction_length
    exit
