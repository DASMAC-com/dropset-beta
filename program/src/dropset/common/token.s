# SPL Token constants.
# -------------------------------------------------------------------------
# Size of a token account (SPL Token and Token 2022 base).
.equ TOKEN_ACCOUNT_SIZE, 165
# spl_token_2022::GetAccountDataSize instruction discriminant.
.equ TOKEN_GET_ACCOUNT_DATA_SIZE_DISC, 21
# spl_token_2022::GetAccountDataSize number of accounts.
.equ TOKEN_GET_ACCOUNT_DATA_SIZE_N_ACCOUNTS, 1
# spl_token_2022::GetAccountDataSize number of seeds.
.equ TOKEN_GET_ACCOUNT_DATA_SIZE_N_SEEDS, 0
# spl_token::InitializeAccount2 instruction discriminant.
.equ TOKEN_INITIALIZE_ACCOUNT_2_DISC, 16
# spl_token::InitializeAccount2 number of accounts.
.equ TOKEN_INITIALIZE_ACCOUNT_2_N_ACCOUNTS, 3
# spl_token::InitializeAccount2 number of seeds.
.equ TOKEN_INITIALIZE_ACCOUNT_2_N_SEEDS, 0
# -------------------------------------------------------------------------
