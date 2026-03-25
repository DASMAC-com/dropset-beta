use dropset_interface::market::RegisterMarketAccounts;
use dropset_interface::{Discriminant, ErrorCode};
use dropset_tests::{
    CaseResult, TestCase, TestSetup, check, check_custom, check_with_accounts, test_cases,
};
use solana_account::Account;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

test_cases! {
    #[derive(Clone, Copy)]
    pub enum Case {
        InvalidNumberOfAccounts,
        InvalidInstructionLength,
        UserHasData,
        MarketAccountIsDuplicate,
        MarketHasData,
        BaseMintIsDuplicate,
        QuoteMintIsDuplicate,
    }
}

const N_ACCOUNTS: usize = RegisterMarketAccounts::LEN as usize;

/// Build unique accounts with default (empty) data.
fn default_accounts() -> (Vec<Pubkey>, Vec<Account>) {
    let keys: Vec<Pubkey> = (0..N_ACCOUNTS).map(|_| Pubkey::new_unique()).collect();
    let accounts: Vec<Account> = (0..N_ACCOUNTS).map(|_| Account::default()).collect();
    (keys, accounts)
}

fn into_metas_and_accounts(
    keys: Vec<Pubkey>,
    accounts: Vec<Account>,
) -> (Vec<AccountMeta>, Vec<(Pubkey, Account)>) {
    let metas = keys
        .iter()
        .map(|k| AccountMeta::new_readonly(*k, false))
        .collect();
    let paired = keys.into_iter().zip(accounts).collect();
    (metas, paired)
}

impl TestCase for Case {
    fn run(&self, setup: &TestSetup) -> CaseResult {
        let insn = &[Discriminant::RegisterMarket.into()];
        match self {
            // Verifies: REGISTER-MARKET
            Self::InvalidNumberOfAccounts => {
                check(setup, insn, Some(ErrorCode::InvalidNumberOfAccounts))
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidInstructionLength => check_with_accounts(
                setup,
                &[Discriminant::RegisterMarket.into(), 0x00],
                N_ACCOUNTS,
                Some(ErrorCode::InvalidInstructionLength),
            ),
            // Verifies: REGISTER-MARKET
            Self::UserHasData => {
                let (keys, mut accounts) = default_accounts();
                accounts[RegisterMarketAccounts::User as usize].data = vec![0u8; 32];
                let (metas, accounts) = into_metas_and_accounts(keys, accounts);
                check_custom(setup, insn, metas, accounts, Some(ErrorCode::UserHasData))
            }
            // Verifies: REGISTER-MARKET
            Self::MarketAccountIsDuplicate => {
                let (mut keys, accounts) = default_accounts();
                // Market shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[RegisterMarketAccounts::Market as usize] =
                    keys[RegisterMarketAccounts::User as usize];
                let (metas, accounts) = into_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::MarketAccountIsDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::MarketHasData => {
                let (keys, mut accounts) = default_accounts();
                accounts[RegisterMarketAccounts::Market as usize].data = vec![0u8; 32];
                let (metas, accounts) = into_metas_and_accounts(keys, accounts);
                check_custom(setup, insn, metas, accounts, Some(ErrorCode::MarketHasData))
            }
            // Verifies: REGISTER-MARKET
            Self::BaseMintIsDuplicate => {
                let (mut keys, accounts) = default_accounts();
                // BaseMint shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[RegisterMarketAccounts::BaseMint as usize] =
                    keys[RegisterMarketAccounts::User as usize];
                let (metas, accounts) = into_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseMintIsDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::QuoteMintIsDuplicate => {
                let (mut keys, accounts) = default_accounts();
                // QuoteMint shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[RegisterMarketAccounts::QuoteMint as usize] =
                    keys[RegisterMarketAccounts::User as usize];
                let (metas, accounts) = into_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteMintIsDuplicate),
                )
            }
        }
    }
}
