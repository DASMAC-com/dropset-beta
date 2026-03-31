use dropset_interface::market::{MarketHeader, RegisterMarketAccounts};
use dropset_interface::pubkey::pubkey::{CHUNK_0_OFF, CHUNK_1_OFF, CHUNK_2_OFF, CHUNK_3_OFF};
use dropset_interface::pubkey::{TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID};
use dropset_interface::{Discriminant, ErrorCode};
use dropset_tests::{
    CaseResult, TestCase, TestSetup, check, check_custom, check_with_accounts, find_pda_seed_pair,
    test_cases,
};
use mollusk_svm::program;
use mollusk_svm::result::ProgramResult as MolluskResult;
use solana_account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
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
        InvalidMarketPubkeyChunk0,
        InvalidMarketPubkeyChunk1,
        InvalidMarketPubkeyChunk2,
        InvalidMarketPubkeyChunk3,
        SystemProgramIsDuplicate,
        InvalidSystemProgramPubkeyChunk0,
        InvalidSystemProgramPubkeyChunk1,
        InvalidSystemProgramPubkeyChunk2,
        InvalidSystemProgramPubkeyChunk3,
        RentSysvarIsDuplicate,
        InvalidRentSysvarPubkeyChunk0,
        InvalidRentSysvarPubkeyChunk1,
        InvalidRentSysvarPubkeyChunk2,
        InvalidRentSysvarPubkeyChunk3,
        InvalidRentSysvarPubkeyChunk3Hi,
        BaseTokenProgramIsDuplicate,
        BaseTokenProgramNotBaseMintOwnerChunk0,
        BaseTokenProgramNotBaseMintOwnerChunk1,
        BaseTokenProgramNotBaseMintOwnerChunk2,
        BaseTokenProgramNotBaseMintOwnerChunk3,
        BaseTokenProgramNotTokenProgramChunk0,
        BaseTokenProgramNotTokenProgramChunk1,
        BaseTokenProgramNotTokenProgramChunk2,
        BaseTokenProgramNotTokenProgramChunk3,
        NonDupQuoteTokenProgramNotQuoteMintOwnerChunk0,
        NonDupQuoteTokenProgramNotQuoteMintOwnerChunk1,
        NonDupQuoteTokenProgramNotQuoteMintOwnerChunk2,
        NonDupQuoteTokenProgramNotQuoteMintOwnerChunk3,
        QuoteTokenProgramNotTokenProgramChunk0,
        QuoteTokenProgramNotTokenProgramChunk1,
        QuoteTokenProgramNotTokenProgramChunk2,
        QuoteTokenProgramNotTokenProgramChunk3,
        InvalidQuoteTokenProgramDuplicateChunk0,
        InvalidQuoteTokenProgramDuplicateChunk1,
        InvalidQuoteTokenProgramDuplicateChunk2,
        InvalidQuoteTokenProgramDuplicateChunk3,
        DupQuoteTokenProgramNotQuoteMintOwnerChunk0,
        DupQuoteTokenProgramNotQuoteMintOwnerChunk1,
        DupQuoteTokenProgramNotQuoteMintOwnerChunk2,
        DupQuoteTokenProgramNotQuoteMintOwnerChunk3,
        CreateAccountHappyPathQuoteDup,
        CreateAccountHappyPathQuoteNonDup,
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

const USER_LAMPORTS: u64 = 1_000_000;
const MARKET_HEADER_SIZE: usize = size_of::<MarketHeader>();

/// Build valid accounts that pass all checks for a successful CreateAccount CPI.
/// When `base_token_program` and `quote_token_program` share the same key,
/// the runtime serializes the quote token program as a duplicate account.
fn happy_path_accounts(
    setup: &TestSetup,
    base_token_program: Pubkey,
    quote_token_program: Pubkey,
) -> (Vec<AccountMeta>, Vec<(Pubkey, Account)>) {
    let (mut keys, mut accounts) = default_accounts();
    let (base_key, quote_key) = find_pda_seed_pair(&setup.program_id);
    keys[RegisterMarketAccounts::BaseMint as usize] = base_key;
    keys[RegisterMarketAccounts::QuoteMint as usize] = quote_key;
    let (pda, _bump) =
        Pubkey::find_program_address(&[base_key.as_ref(), quote_key.as_ref()], &setup.program_id);
    keys[RegisterMarketAccounts::Market as usize] = pda;

    let (system_program_pubkey, system_program_account) =
        program::keyed_account_for_system_program();
    keys[RegisterMarketAccounts::SystemProgram as usize] = system_program_pubkey;
    accounts[RegisterMarketAccounts::SystemProgram as usize] = system_program_account;

    let (rent_sysvar_pubkey, rent_sysvar_account) =
        setup.mollusk.sysvars.keyed_account_for_rent_sysvar();
    keys[RegisterMarketAccounts::RentSysvar as usize] = rent_sysvar_pubkey;
    accounts[RegisterMarketAccounts::RentSysvar as usize] = rent_sysvar_account;

    // Set mint account owners to their respective token programs.
    accounts[RegisterMarketAccounts::BaseMint as usize].owner = base_token_program;
    accounts[RegisterMarketAccounts::QuoteMint as usize].owner = quote_token_program;

    // Set up token program accounts.
    keys[RegisterMarketAccounts::BaseTokenProgram as usize] = base_token_program;
    keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = quote_token_program;

    // Fund the user account so it can pay for the CreateAccount CPI.
    accounts[RegisterMarketAccounts::User as usize] =
        Account::new(USER_LAMPORTS, 0, &system_program_pubkey);

    let metas: Vec<AccountMeta> = keys
        .iter()
        .enumerate()
        .map(|(i, k)| {
            let writable = matches!(
                i,
                i if i == RegisterMarketAccounts::User as usize
                    || i == RegisterMarketAccounts::Market as usize
            );
            let signer = i == RegisterMarketAccounts::User as usize;
            if writable {
                AccountMeta::new(*k, signer)
            } else {
                AccountMeta::new_readonly(*k, signer)
            }
        })
        .collect();
    let paired = keys.into_iter().zip(accounts).collect();
    (metas, paired)
}

/// Build accounts where the market key is the correct PDA with one
/// 8-byte chunk flipped, so the comparison fails at exactly that chunk.
fn pda_mismatch_accounts(
    setup: &TestSetup,
    corrupt_chunk: usize,
) -> (
    Vec<solana_sdk::instruction::AccountMeta>,
    Vec<(Pubkey, Account)>,
) {
    let (mut keys, accounts) = default_accounts();
    let (base_key, quote_key) = find_pda_seed_pair(&setup.program_id);
    keys[RegisterMarketAccounts::BaseMint as usize] = base_key;
    keys[RegisterMarketAccounts::QuoteMint as usize] = quote_key;
    let (mut pda, _bump) =
        Pubkey::find_program_address(&[base_key.as_ref(), quote_key.as_ref()], &setup.program_id);
    // Flip a byte in the target chunk so only that comparison fails.
    let offset = corrupt_chunk * 8;
    pda.as_mut()[offset] ^= 0xFF;
    keys[RegisterMarketAccounts::Market as usize] = pda;
    into_metas_and_accounts(keys, accounts)
}

/// Build accounts where the market key is the correct PDA but the
/// System Program account has a non-zero pubkey with one 8-byte chunk
/// flipped. The System Program ID is `Pubkey::default()` (all zeroes),
/// so any nonzero byte in a chunk causes a mismatch.
fn system_program_mismatch_accounts(
    setup: &TestSetup,
    corrupt_chunk: usize,
) -> (
    Vec<solana_sdk::instruction::AccountMeta>,
    Vec<(Pubkey, Account)>,
) {
    let (mut keys, accounts) = default_accounts();
    let (base_key, quote_key) = find_pda_seed_pair(&setup.program_id);
    keys[RegisterMarketAccounts::BaseMint as usize] = base_key;
    keys[RegisterMarketAccounts::QuoteMint as usize] = quote_key;
    let (pda, _bump) =
        Pubkey::find_program_address(&[base_key.as_ref(), quote_key.as_ref()], &setup.program_id);
    keys[RegisterMarketAccounts::Market as usize] = pda;
    // System Program ID is Pubkey::default() (all zeroes).
    // Flip a byte in the target chunk so only that comparison fails.
    let mut system_program_key = Pubkey::default();
    let offset = corrupt_chunk * 8;
    system_program_key.as_mut()[offset] ^= 0xFF;
    keys[RegisterMarketAccounts::SystemProgram as usize] = system_program_key;
    into_metas_and_accounts(keys, accounts)
}

/// Build accounts that pass all checks through System Program, but
/// corrupt a byte at a given offset in the Rent sysvar pubkey. The
/// correct Rent sysvar ID is `solana_sdk::sysvar::rent::ID`.
fn rent_sysvar_mismatch_accounts(
    setup: &TestSetup,
    corrupt_byte: usize,
) -> (
    Vec<solana_sdk::instruction::AccountMeta>,
    Vec<(Pubkey, Account)>,
) {
    let (mut keys, accounts) = default_accounts();
    let (base_key, quote_key) = find_pda_seed_pair(&setup.program_id);
    keys[RegisterMarketAccounts::BaseMint as usize] = base_key;
    keys[RegisterMarketAccounts::QuoteMint as usize] = quote_key;
    let (pda, _bump) =
        Pubkey::find_program_address(&[base_key.as_ref(), quote_key.as_ref()], &setup.program_id);
    keys[RegisterMarketAccounts::Market as usize] = pda;
    keys[RegisterMarketAccounts::SystemProgram as usize] = Pubkey::default();
    let mut rent_key = solana_sdk::sysvar::rent::ID;
    // Flip a byte at the given offset so that comparison fails.
    rent_key.as_mut()[corrupt_byte] ^= 0xFF;
    keys[RegisterMarketAccounts::RentSysvar as usize] = rent_key;
    into_metas_and_accounts(keys, accounts)
}

/// Build accounts that pass all checks through the CPI, with the given
/// token programs as owners of the respective mints. Returns keys and
/// accounts that can be further modified for specific error cases.
fn token_program_base_accounts(
    setup: &TestSetup,
    base_token_program: Pubkey,
    quote_token_program: Pubkey,
) -> (Vec<Pubkey>, Vec<Account>) {
    let (mut keys, mut accounts) = default_accounts();
    let (base_key, quote_key) = find_pda_seed_pair(&setup.program_id);
    keys[RegisterMarketAccounts::BaseMint as usize] = base_key;
    keys[RegisterMarketAccounts::QuoteMint as usize] = quote_key;
    let (pda, _bump) =
        Pubkey::find_program_address(&[base_key.as_ref(), quote_key.as_ref()], &setup.program_id);
    keys[RegisterMarketAccounts::Market as usize] = pda;

    let (system_program_pubkey, system_program_account) =
        program::keyed_account_for_system_program();
    keys[RegisterMarketAccounts::SystemProgram as usize] = system_program_pubkey;
    accounts[RegisterMarketAccounts::SystemProgram as usize] = system_program_account;

    let (rent_sysvar_pubkey, rent_sysvar_account) =
        setup.mollusk.sysvars.keyed_account_for_rent_sysvar();
    keys[RegisterMarketAccounts::RentSysvar as usize] = rent_sysvar_pubkey;
    accounts[RegisterMarketAccounts::RentSysvar as usize] = rent_sysvar_account;

    accounts[RegisterMarketAccounts::BaseMint as usize].owner = base_token_program;
    accounts[RegisterMarketAccounts::QuoteMint as usize].owner = quote_token_program;

    keys[RegisterMarketAccounts::BaseTokenProgram as usize] = base_token_program;
    keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = quote_token_program;

    accounts[RegisterMarketAccounts::User as usize] =
        Account::new(USER_LAMPORTS, 0, &system_program_pubkey);

    (keys, accounts)
}

fn token_program_metas_and_accounts(
    keys: Vec<Pubkey>,
    accounts: Vec<Account>,
) -> (Vec<AccountMeta>, Vec<(Pubkey, Account)>) {
    let metas: Vec<AccountMeta> = keys
        .iter()
        .enumerate()
        .map(|(i, k)| {
            let writable = i == RegisterMarketAccounts::User as usize
                || i == RegisterMarketAccounts::Market as usize;
            let signer = i == RegisterMarketAccounts::User as usize;
            if writable {
                AccountMeta::new(*k, signer)
            } else {
                AccountMeta::new_readonly(*k, signer)
            }
        })
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
            // Verifies: REGISTER-MARKET
            Self::InvalidMarketPubkeyChunk0 => {
                let (metas, accounts) = pda_mismatch_accounts(setup, 0);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidMarketPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidMarketPubkeyChunk1 => {
                let (metas, accounts) = pda_mismatch_accounts(setup, 1);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidMarketPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidMarketPubkeyChunk2 => {
                let (metas, accounts) = pda_mismatch_accounts(setup, 2);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidMarketPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidMarketPubkeyChunk3 => {
                let (metas, accounts) = pda_mismatch_accounts(setup, 3);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidMarketPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::SystemProgramIsDuplicate => {
                let (mut keys, accounts) = default_accounts();
                let (base_key, quote_key) = find_pda_seed_pair(&setup.program_id);
                keys[RegisterMarketAccounts::BaseMint as usize] = base_key;
                keys[RegisterMarketAccounts::QuoteMint as usize] = quote_key;
                let (pda, _bump) = Pubkey::find_program_address(
                    &[base_key.as_ref(), quote_key.as_ref()],
                    &setup.program_id,
                );
                keys[RegisterMarketAccounts::Market as usize] = pda;
                // SystemProgram shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[RegisterMarketAccounts::SystemProgram as usize] =
                    keys[RegisterMarketAccounts::User as usize];
                let (metas, accounts) = into_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::SystemProgramIsDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidSystemProgramPubkeyChunk0 => {
                let (metas, accounts) = system_program_mismatch_accounts(setup, 0);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidSystemProgramPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidSystemProgramPubkeyChunk1 => {
                let (metas, accounts) = system_program_mismatch_accounts(setup, 1);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidSystemProgramPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidSystemProgramPubkeyChunk2 => {
                let (metas, accounts) = system_program_mismatch_accounts(setup, 2);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidSystemProgramPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidSystemProgramPubkeyChunk3 => {
                let (metas, accounts) = system_program_mismatch_accounts(setup, 3);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidSystemProgramPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::RentSysvarIsDuplicate => {
                let (mut keys, accounts) = default_accounts();
                let (base_key, quote_key) = find_pda_seed_pair(&setup.program_id);
                keys[RegisterMarketAccounts::BaseMint as usize] = base_key;
                keys[RegisterMarketAccounts::QuoteMint as usize] = quote_key;
                let (pda, _bump) = Pubkey::find_program_address(
                    &[base_key.as_ref(), quote_key.as_ref()],
                    &setup.program_id,
                );
                keys[RegisterMarketAccounts::Market as usize] = pda;
                keys[RegisterMarketAccounts::SystemProgram as usize] = Pubkey::default();
                // RentSysvar shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[RegisterMarketAccounts::RentSysvar as usize] =
                    keys[RegisterMarketAccounts::User as usize];
                let (metas, accounts) = into_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::RentSysvarIsDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidRentSysvarPubkeyChunk0 => {
                let (metas, accounts) = rent_sysvar_mismatch_accounts(setup, 0);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidRentSysvarPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidRentSysvarPubkeyChunk1 => {
                let (metas, accounts) = rent_sysvar_mismatch_accounts(setup, 8);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidRentSysvarPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidRentSysvarPubkeyChunk2 => {
                let (metas, accounts) = rent_sysvar_mismatch_accounts(setup, 16);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidRentSysvarPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidRentSysvarPubkeyChunk3 => {
                let (metas, accounts) = rent_sysvar_mismatch_accounts(setup, 24);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidRentSysvarPubkey),
                )
            }
            // Verifies: REGISTER-MARKET (mov32 optimization: chunk 3 hi
            // bits are zero, so mov32 zero-extends and implicitly checks
            // the upper 32 bits)
            Self::InvalidRentSysvarPubkeyChunk3Hi => {
                let (metas, accounts) = rent_sysvar_mismatch_accounts(setup, 28);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidRentSysvarPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::BaseTokenProgramIsDuplicate => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id);
                // BaseTokenProgram shares key with User, causing duplicate.
                keys[RegisterMarketAccounts::BaseTokenProgram as usize] =
                    keys[RegisterMarketAccounts::User as usize];
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramIsDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::BaseTokenProgramNotBaseMintOwnerChunk0 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id);
                let mut bad_key = token_program_id;
                bad_key.as_mut()[CHUNK_0_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::BaseTokenProgram as usize] = bad_key;
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotBaseMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::BaseTokenProgramNotBaseMintOwnerChunk1 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id);
                let mut bad_key = token_program_id;
                bad_key.as_mut()[CHUNK_1_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::BaseTokenProgram as usize] = bad_key;
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotBaseMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::BaseTokenProgramNotBaseMintOwnerChunk2 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id);
                let mut bad_key = token_program_id;
                bad_key.as_mut()[CHUNK_2_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::BaseTokenProgram as usize] = bad_key;
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotBaseMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::BaseTokenProgramNotBaseMintOwnerChunk3 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id);
                let mut bad_key = token_program_id;
                bad_key.as_mut()[CHUNK_3_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::BaseTokenProgram as usize] = bad_key;
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotBaseMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::BaseTokenProgramNotTokenProgramChunk0 => {
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_0_OFF as usize] ^= 0xFF;
                let (keys, accounts) = token_program_base_accounts(setup, bad_program, bad_program);
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::BaseTokenProgramNotTokenProgramChunk1 => {
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_1_OFF as usize] ^= 0xFF;
                let (keys, accounts) = token_program_base_accounts(setup, bad_program, bad_program);
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::BaseTokenProgramNotTokenProgramChunk2 => {
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_2_OFF as usize] ^= 0xFF;
                let (keys, accounts) = token_program_base_accounts(setup, bad_program, bad_program);
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::BaseTokenProgramNotTokenProgramChunk3 => {
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_3_OFF as usize] ^= 0xFF;
                let (keys, accounts) = token_program_base_accounts(setup, bad_program, bad_program);
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::NonDupQuoteTokenProgramNotQuoteMintOwnerChunk0 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_2022_id);
                // Quote token program key doesn't match quote mint owner.
                let mut bad_key = token_2022_id;
                bad_key.as_mut()[CHUNK_0_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = bad_key;
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::NonDupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::NonDupQuoteTokenProgramNotQuoteMintOwnerChunk1 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_2022_id);
                let mut bad_key = token_2022_id;
                bad_key.as_mut()[CHUNK_1_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = bad_key;
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::NonDupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::NonDupQuoteTokenProgramNotQuoteMintOwnerChunk2 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_2022_id);
                let mut bad_key = token_2022_id;
                bad_key.as_mut()[CHUNK_2_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = bad_key;
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::NonDupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::NonDupQuoteTokenProgramNotQuoteMintOwnerChunk3 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_2022_id);
                let mut bad_key = token_2022_id;
                bad_key.as_mut()[CHUNK_3_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = bad_key;
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::NonDupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::QuoteTokenProgramNotTokenProgramChunk0 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_0_OFF as usize] ^= 0xFF;
                let (keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, bad_program);
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::QuoteTokenProgramNotTokenProgramChunk1 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_1_OFF as usize] ^= 0xFF;
                let (keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, bad_program);
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::QuoteTokenProgramNotTokenProgramChunk2 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_2_OFF as usize] ^= 0xFF;
                let (keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, bad_program);
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::QuoteTokenProgramNotTokenProgramChunk3 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_3_OFF as usize] ^= 0xFF;
                let (keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, bad_program);
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidQuoteTokenProgramDuplicateChunk0 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id);
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] =
                    keys[RegisterMarketAccounts::User as usize];
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteTokenProgramDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidQuoteTokenProgramDuplicateChunk1 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id);
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] =
                    keys[RegisterMarketAccounts::Market as usize];
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteTokenProgramDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidQuoteTokenProgramDuplicateChunk2 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id);
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] =
                    keys[RegisterMarketAccounts::BaseMint as usize];
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteTokenProgramDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::InvalidQuoteTokenProgramDuplicateChunk3 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id);
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] =
                    keys[RegisterMarketAccounts::QuoteMint as usize];
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteTokenProgramDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::DupQuoteTokenProgramNotQuoteMintOwnerChunk0 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                // Base uses Token Program, quote uses Token 2022 (different owners),
                // but quote key duplicates base key (Token Program).
                let (mut keys, mut accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id);
                accounts[RegisterMarketAccounts::QuoteMint as usize].owner = token_2022_id;
                // Force duplicate by sharing key.
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = token_program_id;
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::DupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::DupQuoteTokenProgramNotQuoteMintOwnerChunk1 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let mut bad_owner = token_program_id;
                bad_owner.as_mut()[CHUNK_1_OFF as usize] ^= 0xFF;
                let (mut keys, mut accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id);
                accounts[RegisterMarketAccounts::QuoteMint as usize].owner = bad_owner;
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = token_program_id;
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::DupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::DupQuoteTokenProgramNotQuoteMintOwnerChunk2 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let mut bad_owner = token_program_id;
                bad_owner.as_mut()[CHUNK_2_OFF as usize] ^= 0xFF;
                let (mut keys, mut accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id);
                accounts[RegisterMarketAccounts::QuoteMint as usize].owner = bad_owner;
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = token_program_id;
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::DupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::DupQuoteTokenProgramNotQuoteMintOwnerChunk3 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let mut bad_owner = token_program_id;
                bad_owner.as_mut()[CHUNK_3_OFF as usize] ^= 0xFF;
                let (mut keys, mut accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id);
                accounts[RegisterMarketAccounts::QuoteMint as usize].owner = bad_owner;
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = token_program_id;
                let (metas, accounts) = token_program_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::DupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET (happy path, quote token program is duplicate)
            Self::CreateAccountHappyPathQuoteDup => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (metas, accounts) =
                    happy_path_accounts(setup, token_program_id, token_program_id);
                let instruction = Instruction::new_with_bytes(setup.program_id, insn, metas);
                let result = setup.mollusk.process_instruction(&instruction, &accounts);

                let mut errors = Vec::new();
                match &result.program_result {
                    MolluskResult::Success => {
                        let market =
                            &result.resulting_accounts[RegisterMarketAccounts::Market as usize].1;

                        if market.owner != setup.program_id {
                            errors.push(format!(
                                "owner: expected {:?}, got {:?}",
                                setup.program_id, market.owner
                            ));
                        }
                        if market.data.len() != MARKET_HEADER_SIZE {
                            errors.push(format!(
                                "data len: expected {}, got {}",
                                MARKET_HEADER_SIZE,
                                market.data.len()
                            ));
                        }
                        let rent = &setup.mollusk.sysvars.rent;
                        if !rent.is_exempt(market.lamports, market.data.len()) {
                            errors.push(format!(
                                "market not rent exempt: {} lamports for {} bytes",
                                market.lamports,
                                market.data.len()
                            ));
                        }
                    }
                    other => {
                        errors.push(format!("expected success, got {:?}", other));
                    }
                }

                CaseResult {
                    cu: result.compute_units_consumed,
                    error: if errors.is_empty() {
                        None
                    } else {
                        Some(errors.join("; "))
                    },
                }
            }
            // Verifies: REGISTER-MARKET (happy path, quote token program is non-duplicate)
            Self::CreateAccountHappyPathQuoteNonDup => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (metas, accounts) =
                    happy_path_accounts(setup, token_program_id, token_2022_id);
                let instruction = Instruction::new_with_bytes(setup.program_id, insn, metas);
                let result = setup.mollusk.process_instruction(&instruction, &accounts);

                let mut errors = Vec::new();
                match &result.program_result {
                    MolluskResult::Success => {
                        let market =
                            &result.resulting_accounts[RegisterMarketAccounts::Market as usize].1;

                        if market.owner != setup.program_id {
                            errors.push(format!(
                                "owner: expected {:?}, got {:?}",
                                setup.program_id, market.owner
                            ));
                        }
                        if market.data.len() != MARKET_HEADER_SIZE {
                            errors.push(format!(
                                "data len: expected {}, got {}",
                                MARKET_HEADER_SIZE,
                                market.data.len()
                            ));
                        }
                        let rent = &setup.mollusk.sysvars.rent;
                        if !rent.is_exempt(market.lamports, market.data.len()) {
                            errors.push(format!(
                                "market not rent exempt: {} lamports for {} bytes",
                                market.lamports,
                                market.data.len()
                            ));
                        }
                    }
                    other => {
                        errors.push(format!("expected success, got {:?}", other));
                    }
                }

                CaseResult {
                    cu: result.compute_units_consumed,
                    error: if errors.is_empty() {
                        None
                    } else {
                        Some(errors.join("; "))
                    },
                }
            }
        }
    }
}
