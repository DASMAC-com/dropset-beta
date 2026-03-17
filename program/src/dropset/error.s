# The instruction's discriminant does not match any known variant.
.equ E_INVALID_DISCRIMINANT, 1
# The instruction data length is invalid.
.equ E_INVALID_INSTRUCTION_LENGTH, 2

e_invalid_instruction_length:
    mov32 r0, E_INVALID_INSTRUCTION_LENGTH
    exit
