use dropset_macros::constant_group;
use pinocchio_token::state::TokenAccount;
// pinocchio-token does not export GetAccountDataSize yet.
use pinocchio_token_2022::instructions::GetAccountDataSize;

constant_group! {
    #[prefix("TOKEN")]
    #[inject("common/token")]
    /// SPL Token constants.
    token {
        /// Size of a token account (SPL Token and Token 2022 base).
        ACCOUNT_SIZE = immediate!(TokenAccount::LEN),
        /// GetAccountDataSize instruction discriminator (Token 2022).
        GET_ACCOUNT_DATA_SIZE_DISC = immediate!(GetAccountDataSize::DISCRIMINATOR),
    }
}
