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
# The System Program account is a duplicate.
.equ E_SYSTEM_PROGRAM_IS_DUPLICATE, 10
# The System Program account pubkey is invalid.
.equ E_INVALID_SYSTEM_PROGRAM_PUBKEY, 11
# The Rent sysvar account is a duplicate.
.equ E_RENT_SYSVAR_IS_DUPLICATE, 12
# The Rent sysvar account pubkey is invalid.
.equ E_INVALID_RENT_SYSVAR_PUBKEY, 13
# The base token program account is a duplicate.
.equ E_BASE_TOKEN_PROGRAM_IS_DUPLICATE, 14
# The base token program does not own the base mint.
.equ E_BASE_TOKEN_PROGRAM_NOT_BASE_MINT_OWNER, 15
# The base token program is not Token Program or Token 2022.
.equ E_BASE_TOKEN_PROGRAM_NOT_TOKEN_PROGRAM, 16
# The quote token program duplicate position is invalid.
.equ E_INVALID_QUOTE_TOKEN_PROGRAM_DUPLICATE, 17
# The duplicate quote token program does not own the quote mint.
.equ E_DUP_QUOTE_TOKEN_PROGRAM_NOT_QUOTE_MINT_OWNER, 18
# The non-duplicate quote token program does not own the quote mint.
.equ E_NON_DUP_QUOTE_TOKEN_PROGRAM_NOT_QUOTE_MINT_OWNER, 19
# The quote token program is not Token Program or Token 2022.
.equ E_QUOTE_TOKEN_PROGRAM_NOT_TOKEN_PROGRAM, 20
# The base vault account pubkey is invalid.
.equ E_INVALID_BASE_VAULT_PUBKEY, 21

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

e_system_program_is_duplicate:
    mov32 r0, E_SYSTEM_PROGRAM_IS_DUPLICATE
    exit

e_invalid_system_program_pubkey:
    mov32 r0, E_INVALID_SYSTEM_PROGRAM_PUBKEY
    exit

e_rent_sysvar_is_duplicate:
    mov32 r0, E_RENT_SYSVAR_IS_DUPLICATE
    exit

e_invalid_rent_sysvar_pubkey:
    mov32 r0, E_INVALID_RENT_SYSVAR_PUBKEY
    exit

e_base_token_program_is_duplicate:
    mov32 r0, E_BASE_TOKEN_PROGRAM_IS_DUPLICATE
    exit

e_base_token_program_not_base_mint_owner:
    mov32 r0, E_BASE_TOKEN_PROGRAM_NOT_BASE_MINT_OWNER
    exit

e_base_token_program_not_token_program:
    mov32 r0, E_BASE_TOKEN_PROGRAM_NOT_TOKEN_PROGRAM
    exit

e_invalid_quote_token_program_duplicate:
    mov32 r0, E_INVALID_QUOTE_TOKEN_PROGRAM_DUPLICATE
    exit

e_dup_quote_token_program_not_quote_mint_owner:
    mov32 r0, E_DUP_QUOTE_TOKEN_PROGRAM_NOT_QUOTE_MINT_OWNER
    exit

e_non_dup_quote_token_program_not_quote_mint_owner:
    mov32 r0, E_NON_DUP_QUOTE_TOKEN_PROGRAM_NOT_QUOTE_MINT_OWNER
    exit

e_quote_token_program_not_token_program:
    mov32 r0, E_QUOTE_TOKEN_PROGRAM_NOT_TOKEN_PROGRAM
    exit

e_invalid_base_vault_pubkey:
    mov32 r0, E_INVALID_BASE_VAULT_PUBKEY
    exit
