register_market:
    # if insn_len != InstructionLength::RegisterMarket \
    # return Error::InvalidInstructionLength
    jne r3, 0, e_invalid_instruction_length
    exit
