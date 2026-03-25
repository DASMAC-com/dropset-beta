# The instruction's discriminant does not match any known variant.
.equ E_INVALID_DISCRIMINANT, 1
# The instruction data length is invalid.
.equ E_INVALID_INSTRUCTION_LENGTH, 2
# The number of accounts provided is invalid for the given instruction.
.equ E_INVALID_NUMBER_OF_ACCOUNTS, 3
.equ E_USER_HAS_DATA, 4 # The user account already has data.
.equ E_MARKET_ACCOUNT_IS_DUPLICATE, 5 # The market account is a duplicate.
.equ E_MARKET_HAS_DATA, 6 # The market account already has data.
.equ E_BASE_MINT_IS_DUPLICATE, 7 # The base mint account is a duplicate.
.equ E_QUOTE_MINT_IS_DUPLICATE, 8 # The quote mint account is a duplicate.
.equ E_INVALID_MARKET_PUBKEY, 9 # The market account pubkey is invalid.

e_invalid_instruction_length:
    mov32 r0, E_INVALID_INSTRUCTION_LENGTH
    exit

e_invalid_number_of_accounts:
    mov32 r0, E_INVALID_NUMBER_OF_ACCOUNTS
    exit

e_user_has_data:
    mov32 r0, E_USER_HAS_DATA
    exit

e_market_account_is_duplicate:
    mov32 r0, E_MARKET_ACCOUNT_IS_DUPLICATE
    exit

e_market_has_data:
    mov32 r0, E_MARKET_HAS_DATA
    exit

e_base_mint_is_duplicate:
    mov32 r0, E_BASE_MINT_IS_DUPLICATE
    exit

e_quote_mint_is_duplicate:
    mov32 r0, E_QUOTE_MINT_IS_DUPLICATE
    exit

e_invalid_market_pubkey:
    mov32 r0, E_INVALID_MARKET_PUBKEY
    exit
