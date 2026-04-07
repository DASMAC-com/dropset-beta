use dropset_macros::{constant_group, svm_data};
use pinocchio::Address as Pubkey;
use pinocchio_token::state::TokenAccount;
// pinocchio-token does not export GetAccountDataSize yet.
use pinocchio_token_2022::instructions::GetAccountDataSize;

#[svm_data]
/// CPI instruction data for InitializeAccount2.
pub struct InitializeAccount2 {
    pub discriminant: u8,
    /// In this implementation, the market PDA.
    pub proprietor: Pubkey,
}

constant_group! {
    #[prefix("TOKEN")]
    #[inject("svm/token")]
    /// SPL Token constants.
    constants {
        /// Size of a token account (SPL Token and Token 2022 base).
        ACCOUNT_SIZE = immediate!(TokenAccount::LEN),
        /// GetAccountDataSize instruction discriminant (Token 2022).
        GET_ACCOUNT_DATA_SIZE_DISC = immediate!(GetAccountDataSize::DISCRIMINATOR),
        /// GetAccountDataSize number of accounts.
        GET_ACCOUNT_DATA_SIZE_N_ACCOUNTS = immediate!(1),
        /// GetAccountDataSize number of seeds.
        GET_ACCOUNT_DATA_SIZE_N_SEEDS = immediate!(0),
        /// InitializeAccount2 instruction discriminant.
        // Not exported by token program: https://github.com/anza-xyz/pinocchio/issues/384
        INITIALIZE_ACCOUNT_2_DISC = immediate!(16),
        /// InitializeAccount2 number of accounts.
        INITIALIZE_ACCOUNT_2_N_ACCOUNTS = immediate!(3),
        /// InitializeAccount2 number of seeds.
        INITIALIZE_ACCOUNT_2_N_SEEDS = immediate!(0),
    }
}
