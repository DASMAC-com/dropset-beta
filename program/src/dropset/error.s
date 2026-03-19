# The instruction's discriminant does not match any known variant.
.equ E_INVALID_DISCRIMINANT, 1
# The instruction data length is invalid.
.equ E_INVALID_INSTRUCTION_LENGTH, 2
# The number of accounts provided is invalid for the given instruction.
.equ E_INVALID_NUMBER_OF_ACCOUNTS, 3

e_invalid_instruction_length:
    mov32 r0, E_INVALID_INSTRUCTION_LENGTH
    exit

e_invalid_number_of_accounts:
    mov32 r0, E_INVALID_NUMBER_OF_ACCOUNTS
    exit
