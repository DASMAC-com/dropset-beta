.equ INSN_LEN_REGISTER_MARKET, 0 # Register market instruction data length.

register_market:
    # if insn_len != InstructionLength::RegisterMarket
    #     return ErrorCode::InvalidInstructionLength
    jne r3, INSN_LEN_REGISTER_MARKET, e_invalid_instruction_length
    exit
