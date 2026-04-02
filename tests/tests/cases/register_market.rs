use dropset_interface::market::register_misc::{VAULT_INDEX_BASE, VAULT_INDEX_QUOTE};
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
        InvalidMarketPubkeyChunk0,
        InvalidMarketPubkeyChunk1,
        InvalidMarketPubkeyChunk2,
        InvalidMarketPubkeyChunk3,
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
        BaseVaultIsDuplicate,
        BaseVaultHasData,
        QuoteVaultIsDuplicateDup,
        QuoteVaultIsDuplicateNonDup,
        QuoteVaultHasDataDup,
        QuoteVaultHasDataNonDup,
        InvalidBaseVaultPubkeyChunk0,
        InvalidBaseVaultPubkeyChunk1,
        InvalidBaseVaultPubkeyChunk2,
        InvalidBaseVaultPubkeyChunk3,
        InvalidQuoteVaultPubkeyDupChunk0,
        InvalidQuoteVaultPubkeyDupChunk1,
        InvalidQuoteVaultPubkeyDupChunk2,
        InvalidQuoteVaultPubkeyDupChunk3,
        InvalidQuoteVaultPubkeyNonDupChunk0,
        InvalidQuoteVaultPubkeyNonDupChunk1,
        InvalidQuoteVaultPubkeyNonDupChunk2,
        InvalidQuoteVaultPubkeyNonDupChunk3,
        CreateAccountHappyPathQuoteDup,
        CreateAccountHappyPathQuoteNonDup,
        CreateAccountHappyPathToken2022QuoteDup,
        CreateAccountHappyPathToken2022QuoteNonDup,
    }
}

const N_ACCOUNTS: usize = RegisterMarketAccounts::LEN as usize;

/// Build unique accounts with default (empty) data.
fn default_accounts() -> (Vec<Pubkey>, Vec<Account>) {
    let keys: Vec<Pubkey> = (0..N_ACCOUNTS).map(|_| Pubkey::new_unique()).collect();
    let mut accounts: Vec<Account> = (0..N_ACCOUNTS).map(|_| Account::default()).collect();
    accounts[RegisterMarketAccounts::User as usize] =
        Account::new(USER_LAMPORTS, 0, &Pubkey::default());
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

const USER_LAMPORTS: u64 = 10_000_000;
const MARKET_HEADER_SIZE: usize = size_of::<MarketHeader>();
const TOKEN_ACCOUNT_SIZE: usize = 165;

macro_rules! check_vault {
    ($errors:expr, $label:expr, $vault:expr, $expected_owner:expr, $rent:expr,
     $expected_mint:expr, $expected_proprietor:expr) => {{
        let vault = $vault;
        let expected_owner = $expected_owner;
        if vault.owner != *expected_owner {
            $errors.push(format!(
                "{} owner: expected {:?}, got {:?}",
                $label, expected_owner, vault.owner
            ));
        }
        if vault.data.len() != TOKEN_ACCOUNT_SIZE {
            $errors.push(format!(
                "{} data len: expected {}, got {}",
                $label,
                TOKEN_ACCOUNT_SIZE,
                vault.data.len()
            ));
        }
        if !$rent.is_exempt(vault.lamports, vault.data.len()) {
            $errors.push(format!(
                "{} not rent exempt: {} lamports for {} bytes",
                $label,
                vault.lamports,
                vault.data.len()
            ));
        }
        use solana_program_pack::Pack;
        match spl_token_interface::state::Account::unpack_from_slice(&vault.data) {
            Ok(token_account) => {
                if token_account.mint != $expected_mint {
                    $errors.push(format!(
                        "{} mint: expected {:?}, got {:?}",
                        $label, $expected_mint, token_account.mint
                    ));
                }
                if token_account.owner != $expected_proprietor {
                    $errors.push(format!(
                        "{} proprietor: expected {:?}, got {:?}",
                        $label, $expected_proprietor, token_account.owner
                    ));
                }
                if token_account.amount != 0 {
                    $errors.push(format!(
                        "{} amount: expected 0, got {}",
                        $label, token_account.amount
                    ));
                }
                if token_account.state
                    != spl_token_interface::state::AccountState::Initialized
                {
                    $errors.push(format!(
                        "{} state: expected Initialized, got {:?}",
                        $label, token_account.state
                    ));
                }
            }
            Err(e) => {
                $errors.push(format!(
                    "{} failed to unpack token account: {:?}",
                    $label, e
                ));
            }
        }
    }};
}

fn default_mint() -> spl_token_interface::state::Mint {
    spl_token_interface::state::Mint {
        is_initialized: true,
        ..Default::default()
    }
}

fn mint_account(owner: Pubkey) -> Account {
    if owner == Pubkey::from(TOKEN_PROGRAM_ID) {
        mollusk_svm_programs_token::token::create_account_for_mint(default_mint())
    } else if owner == Pubkey::from(TOKEN_2022_PROGRAM_ID) {
        mollusk_svm_programs_token::token2022::create_account_for_mint(default_mint())
    } else {
        let mut acct = Account::default();
        acct.owner = owner;
        acct
    }
}

fn token_program_account(id: Pubkey) -> Account {
    if id == Pubkey::from(TOKEN_PROGRAM_ID) {
        mollusk_svm_programs_token::token::account()
    } else if id == Pubkey::from(TOKEN_2022_PROGRAM_ID) {
        mollusk_svm_programs_token::token2022::account()
    } else {
        Account::default()
    }
}

/// Build valid accounts that pass all checks for a successful CreateAccount CPI.
/// When `base_token_program` and `quote_token_program` share the same key,
/// the runtime serializes the quote token program as a duplicate account.
fn happy_path_accounts(
    setup: &TestSetup,
    base_token_program: Pubkey,
    quote_token_program: Pubkey,
) -> (Vec<AccountMeta>, Vec<(Pubkey, Account)>) {
    let (mut keys, accounts) =
        token_program_base_accounts(setup, base_token_program, quote_token_program, true);

    // Derive quote vault PDA from market address and vault index.
    let pda = keys[RegisterMarketAccounts::Market as usize];
    let (quote_vault_pda, _) = Pubkey::find_program_address(
        &[pda.as_ref(), &[VAULT_INDEX_QUOTE as u8]],
        &setup.program_id,
    );
    keys[RegisterMarketAccounts::QuoteVault as usize] = quote_vault_pda;

    writable_metas_and_accounts(keys, accounts)
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
    let (mut keys, mut accounts) = default_accounts();
    let (base_key, quote_key) = find_pda_seed_pair(&setup.program_id);
    keys[RegisterMarketAccounts::BaseMint as usize] = base_key;
    keys[RegisterMarketAccounts::QuoteMint as usize] = quote_key;
    let (mut pda, _bump) =
        Pubkey::find_program_address(&[base_key.as_ref(), quote_key.as_ref()], &setup.program_id);
    // Flip a byte in the target chunk so only that comparison fails.
    let offset = corrupt_chunk * 8;
    pda.as_mut()[offset] ^= 0xFF;
    keys[RegisterMarketAccounts::Market as usize] = pda;
    let (system_program_pubkey, system_program_account) =
        program::keyed_account_for_system_program();
    keys[RegisterMarketAccounts::SystemProgram as usize] = system_program_pubkey;
    accounts[RegisterMarketAccounts::SystemProgram as usize] = system_program_account;
    let (rent_sysvar_pubkey, rent_sysvar_account) =
        setup.mollusk.sysvars.keyed_account_for_rent_sysvar();
    keys[RegisterMarketAccounts::RentSysvar as usize] = rent_sysvar_pubkey;
    accounts[RegisterMarketAccounts::RentSysvar as usize] = rent_sysvar_account;
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
/// When `require_quote_vault_bump` is true, the seed search also requires
/// the quote vault PDA to derive on the first bump (255).
fn token_program_base_accounts(
    setup: &TestSetup,
    base_token_program: Pubkey,
    quote_token_program: Pubkey,
    require_quote_vault_bump: bool,
) -> (Vec<Pubkey>, Vec<Account>) {
    let (base_key, quote_key, pda) = loop {
        let (base_key, quote_key) = find_pda_seed_pair(&setup.program_id);
        let (pda, _) = Pubkey::find_program_address(
            &[base_key.as_ref(), quote_key.as_ref()],
            &setup.program_id,
        );
        let (_vault_pda, vault_bump) = Pubkey::find_program_address(
            &[pda.as_ref(), &[VAULT_INDEX_BASE as u8]],
            &setup.program_id,
        );
        if vault_bump != u8::MAX {
            continue;
        }
        if require_quote_vault_bump {
            let (_quote_vault, quote_bump) = Pubkey::find_program_address(
                &[pda.as_ref(), &[VAULT_INDEX_QUOTE as u8]],
                &setup.program_id,
            );
            if quote_bump != u8::MAX {
                continue;
            }
        }
        break (base_key, quote_key, pda);
    };

    let (mut keys, mut accounts) = default_accounts();
    keys[RegisterMarketAccounts::BaseMint as usize] = base_key;
    keys[RegisterMarketAccounts::QuoteMint as usize] = quote_key;
    keys[RegisterMarketAccounts::Market as usize] = pda;

    let (system_program_pubkey, system_program_account) =
        program::keyed_account_for_system_program();
    keys[RegisterMarketAccounts::SystemProgram as usize] = system_program_pubkey;
    accounts[RegisterMarketAccounts::SystemProgram as usize] = system_program_account;

    let (rent_sysvar_pubkey, rent_sysvar_account) =
        setup.mollusk.sysvars.keyed_account_for_rent_sysvar();
    keys[RegisterMarketAccounts::RentSysvar as usize] = rent_sysvar_pubkey;
    accounts[RegisterMarketAccounts::RentSysvar as usize] = rent_sysvar_account;

    accounts[RegisterMarketAccounts::BaseMint as usize] = mint_account(base_token_program);
    accounts[RegisterMarketAccounts::QuoteMint as usize] = mint_account(quote_token_program);

    keys[RegisterMarketAccounts::BaseTokenProgram as usize] = base_token_program;
    accounts[RegisterMarketAccounts::BaseTokenProgram as usize] =
        token_program_account(base_token_program);
    keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = quote_token_program;
    accounts[RegisterMarketAccounts::QuoteTokenProgram as usize] =
        token_program_account(quote_token_program);

    // Derive base vault PDA from market address and vault index.
    let (base_vault_pda, _) = Pubkey::find_program_address(
        &[pda.as_ref(), &[VAULT_INDEX_BASE as u8]],
        &setup.program_id,
    );
    keys[RegisterMarketAccounts::BaseVault as usize] = base_vault_pda;

    (keys, accounts)
}

fn writable_metas_and_accounts(
    keys: Vec<Pubkey>,
    accounts: Vec<Account>,
) -> (Vec<AccountMeta>, Vec<(Pubkey, Account)>) {
    let metas: Vec<AccountMeta> = keys
        .iter()
        .enumerate()
        .map(|(i, k)| {
            let writable = i == RegisterMarketAccounts::User as usize
                || i == RegisterMarketAccounts::Market as usize
                || i == RegisterMarketAccounts::BaseVault as usize
                || i == RegisterMarketAccounts::QuoteVault as usize;
            let signer = i == RegisterMarketAccounts::User as usize
                || i == RegisterMarketAccounts::BaseVault as usize
                || i == RegisterMarketAccounts::QuoteVault as usize;
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

/// Build accounts that pass all checks through the base token program,
/// then corrupt one 8-byte chunk of the base vault PDA so the comparison
/// fails at exactly that chunk. Both the market PDA and the vault PDA
/// use the first bump (255) to minimize CU overhead.
fn base_vault_mismatch_accounts(
    setup: &TestSetup,
    corrupt_byte: usize,
) -> (Vec<AccountMeta>, Vec<(Pubkey, Account)>) {
    let base_token_program = Pubkey::from(TOKEN_PROGRAM_ID);
    let (mut keys, accounts) =
        token_program_base_accounts(setup, base_token_program, base_token_program, false);
    keys[RegisterMarketAccounts::BaseVault as usize].as_mut()[corrupt_byte] ^= 0xFF;
    writable_metas_and_accounts(keys, accounts)
}

/// Build accounts that pass all checks through the quote token program,
/// then corrupt one 8-byte chunk of the quote vault PDA so the comparison
/// fails at exactly that chunk. When `dup` is true, both token programs
/// share the same key (duplicate path). The market PDA, base vault PDA,
/// and quote vault PDA all use the first bump (255) to minimize CU overhead.
fn quote_vault_mismatch_accounts(
    setup: &TestSetup,
    corrupt_byte: usize,
    dup: bool,
) -> (Vec<AccountMeta>, Vec<(Pubkey, Account)>) {
    let base_token_program = Pubkey::from(TOKEN_PROGRAM_ID);
    let quote_token_program = if dup {
        base_token_program
    } else {
        Pubkey::from(TOKEN_2022_PROGRAM_ID)
    };
    let (mut keys, accounts) =
        token_program_base_accounts(setup, base_token_program, quote_token_program, true);

    let pda = keys[RegisterMarketAccounts::Market as usize];
    let (mut quote_vault_pda, _) = Pubkey::find_program_address(
        &[pda.as_ref(), &[VAULT_INDEX_QUOTE as u8]],
        &setup.program_id,
    );
    quote_vault_pda.as_mut()[corrupt_byte] ^= 0xFF;
    keys[RegisterMarketAccounts::QuoteVault as usize] = quote_vault_pda;

    writable_metas_and_accounts(keys, accounts)
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
            // Verifies: INIT-MARKET-PDA
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
            // Verifies: INIT-MARKET-PDA
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
            // Verifies: INIT-MARKET-PDA
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
            // Verifies: INIT-MARKET-PDA
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
            // Verifies: INIT-MARKET-PDA
            Self::BaseTokenProgramIsDuplicate => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                // BaseTokenProgram shares key with User, causing duplicate.
                keys[RegisterMarketAccounts::BaseTokenProgram as usize] =
                    keys[RegisterMarketAccounts::User as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramIsDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::BaseTokenProgramNotBaseMintOwnerChunk0 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                let mut bad_key = token_program_id;
                bad_key.as_mut()[CHUNK_0_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::BaseTokenProgram as usize] = bad_key;
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotBaseMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::BaseTokenProgramNotBaseMintOwnerChunk1 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                let mut bad_key = token_program_id;
                bad_key.as_mut()[CHUNK_1_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::BaseTokenProgram as usize] = bad_key;
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotBaseMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::BaseTokenProgramNotBaseMintOwnerChunk2 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                let mut bad_key = token_program_id;
                bad_key.as_mut()[CHUNK_2_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::BaseTokenProgram as usize] = bad_key;
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotBaseMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::BaseTokenProgramNotBaseMintOwnerChunk3 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                let mut bad_key = token_program_id;
                bad_key.as_mut()[CHUNK_3_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::BaseTokenProgram as usize] = bad_key;
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotBaseMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::BaseTokenProgramNotTokenProgramChunk0 => {
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_0_OFF as usize] ^= 0xFF;
                let (keys, accounts) =
                    token_program_base_accounts(setup, bad_program, bad_program, false);
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::BaseTokenProgramNotTokenProgramChunk1 => {
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_1_OFF as usize] ^= 0xFF;
                let (keys, accounts) =
                    token_program_base_accounts(setup, bad_program, bad_program, false);
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::BaseTokenProgramNotTokenProgramChunk2 => {
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_2_OFF as usize] ^= 0xFF;
                let (keys, accounts) =
                    token_program_base_accounts(setup, bad_program, bad_program, false);
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::BaseTokenProgramNotTokenProgramChunk3 => {
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_3_OFF as usize] ^= 0xFF;
                let (keys, accounts) =
                    token_program_base_accounts(setup, bad_program, bad_program, false);
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::NonDupQuoteTokenProgramNotQuoteMintOwnerChunk0 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_2022_id, false);
                // Quote token program key doesn't match quote mint owner.
                let mut bad_key = token_2022_id;
                bad_key.as_mut()[CHUNK_0_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = bad_key;
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::NonDupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::NonDupQuoteTokenProgramNotQuoteMintOwnerChunk1 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_2022_id, false);
                let mut bad_key = token_2022_id;
                bad_key.as_mut()[CHUNK_1_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = bad_key;
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::NonDupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::NonDupQuoteTokenProgramNotQuoteMintOwnerChunk2 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_2022_id, false);
                let mut bad_key = token_2022_id;
                bad_key.as_mut()[CHUNK_2_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = bad_key;
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::NonDupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::NonDupQuoteTokenProgramNotQuoteMintOwnerChunk3 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_2022_id, false);
                let mut bad_key = token_2022_id;
                bad_key.as_mut()[CHUNK_3_OFF as usize] ^= 0xFF;
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = bad_key;
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::NonDupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::QuoteTokenProgramNotTokenProgramChunk0 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_0_OFF as usize] ^= 0xFF;
                let (keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, bad_program, false);
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::QuoteTokenProgramNotTokenProgramChunk1 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_1_OFF as usize] ^= 0xFF;
                let (keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, bad_program, false);
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::QuoteTokenProgramNotTokenProgramChunk2 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_2_OFF as usize] ^= 0xFF;
                let (keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, bad_program, false);
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::QuoteTokenProgramNotTokenProgramChunk3 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
                bad_program.as_mut()[CHUNK_3_OFF as usize] ^= 0xFF;
                let (keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, bad_program, false);
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteTokenProgramNotTokenProgram),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::InvalidQuoteTokenProgramDuplicateChunk0 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] =
                    keys[RegisterMarketAccounts::User as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteTokenProgramDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::InvalidQuoteTokenProgramDuplicateChunk1 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] =
                    keys[RegisterMarketAccounts::Market as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteTokenProgramDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::InvalidQuoteTokenProgramDuplicateChunk2 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] =
                    keys[RegisterMarketAccounts::BaseMint as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteTokenProgramDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::InvalidQuoteTokenProgramDuplicateChunk3 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] =
                    keys[RegisterMarketAccounts::QuoteMint as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteTokenProgramDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::DupQuoteTokenProgramNotQuoteMintOwnerChunk0 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                // Base uses Token Program, quote uses Token 2022 (different owners),
                // but quote key duplicates base key (Token Program).
                let (mut keys, mut accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                accounts[RegisterMarketAccounts::QuoteMint as usize].owner = token_2022_id;
                // Force duplicate by sharing key.
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = token_program_id;
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::DupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::DupQuoteTokenProgramNotQuoteMintOwnerChunk1 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let mut bad_owner = token_program_id;
                bad_owner.as_mut()[CHUNK_1_OFF as usize] ^= 0xFF;
                let (mut keys, mut accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                accounts[RegisterMarketAccounts::QuoteMint as usize].owner = bad_owner;
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = token_program_id;
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::DupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::DupQuoteTokenProgramNotQuoteMintOwnerChunk2 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let mut bad_owner = token_program_id;
                bad_owner.as_mut()[CHUNK_2_OFF as usize] ^= 0xFF;
                let (mut keys, mut accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                accounts[RegisterMarketAccounts::QuoteMint as usize].owner = bad_owner;
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = token_program_id;
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::DupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            Self::DupQuoteTokenProgramNotQuoteMintOwnerChunk3 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let mut bad_owner = token_program_id;
                bad_owner.as_mut()[CHUNK_3_OFF as usize] ^= 0xFF;
                let (mut keys, mut accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                accounts[RegisterMarketAccounts::QuoteMint as usize].owner = bad_owner;
                keys[RegisterMarketAccounts::QuoteTokenProgram as usize] = token_program_id;
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::DupQuoteTokenProgramNotQuoteMintOwner),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::BaseVaultIsDuplicate => {
                let base_token_program = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) = token_program_base_accounts(
                    setup,
                    base_token_program,
                    base_token_program,
                    false,
                );
                // Base vault shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[RegisterMarketAccounts::BaseVault as usize] =
                    keys[RegisterMarketAccounts::User as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseVaultIsDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::BaseVaultHasData => {
                let base_token_program = Pubkey::from(TOKEN_PROGRAM_ID);
                let (keys, mut accounts) = token_program_base_accounts(
                    setup,
                    base_token_program,
                    base_token_program,
                    false,
                );
                accounts[RegisterMarketAccounts::BaseVault as usize].data = vec![0u8; 32];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseVaultHasData),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::QuoteVaultIsDuplicateDup => {
                let base_token_program = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) = token_program_base_accounts(
                    setup,
                    base_token_program,
                    base_token_program,
                    true,
                );
                let pda = keys[RegisterMarketAccounts::Market as usize];
                let (quote_vault_pda, _) = Pubkey::find_program_address(
                    &[pda.as_ref(), &[VAULT_INDEX_QUOTE as u8]],
                    &setup.program_id,
                );
                keys[RegisterMarketAccounts::QuoteVault as usize] = quote_vault_pda;
                // Quote vault shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[RegisterMarketAccounts::QuoteVault as usize] =
                    keys[RegisterMarketAccounts::User as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteVaultIsDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::QuoteVaultIsDuplicateNonDup => {
                let base_token_program = Pubkey::from(TOKEN_PROGRAM_ID);
                let quote_token_program = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (mut keys, accounts) = token_program_base_accounts(
                    setup,
                    base_token_program,
                    quote_token_program,
                    true,
                );
                let pda = keys[RegisterMarketAccounts::Market as usize];
                let (quote_vault_pda, _) = Pubkey::find_program_address(
                    &[pda.as_ref(), &[VAULT_INDEX_QUOTE as u8]],
                    &setup.program_id,
                );
                keys[RegisterMarketAccounts::QuoteVault as usize] = quote_vault_pda;
                // Quote vault shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[RegisterMarketAccounts::QuoteVault as usize] =
                    keys[RegisterMarketAccounts::User as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteVaultIsDuplicate),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::QuoteVaultHasDataDup => {
                let base_token_program = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, mut accounts) = token_program_base_accounts(
                    setup,
                    base_token_program,
                    base_token_program,
                    true,
                );
                let pda = keys[RegisterMarketAccounts::Market as usize];
                let (quote_vault_pda, _) = Pubkey::find_program_address(
                    &[pda.as_ref(), &[VAULT_INDEX_QUOTE as u8]],
                    &setup.program_id,
                );
                keys[RegisterMarketAccounts::QuoteVault as usize] = quote_vault_pda;
                accounts[RegisterMarketAccounts::QuoteVault as usize].data = vec![0u8; 32];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteVaultHasData),
                )
            }
            // Verifies: REGISTER-MARKET
            Self::QuoteVaultHasDataNonDup => {
                let base_token_program = Pubkey::from(TOKEN_PROGRAM_ID);
                let quote_token_program = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (mut keys, mut accounts) = token_program_base_accounts(
                    setup,
                    base_token_program,
                    quote_token_program,
                    true,
                );
                let pda = keys[RegisterMarketAccounts::Market as usize];
                let (quote_vault_pda, _) = Pubkey::find_program_address(
                    &[pda.as_ref(), &[VAULT_INDEX_QUOTE as u8]],
                    &setup.program_id,
                );
                keys[RegisterMarketAccounts::QuoteVault as usize] = quote_vault_pda;
                accounts[RegisterMarketAccounts::QuoteVault as usize].data = vec![0u8; 32];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteVaultHasData),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::InvalidBaseVaultPubkeyChunk0 => {
                let (metas, accounts) = base_vault_mismatch_accounts(setup, CHUNK_0_OFF as usize);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidBaseVaultPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::InvalidBaseVaultPubkeyChunk1 => {
                let (metas, accounts) = base_vault_mismatch_accounts(setup, CHUNK_1_OFF as usize);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidBaseVaultPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::InvalidBaseVaultPubkeyChunk2 => {
                let (metas, accounts) = base_vault_mismatch_accounts(setup, CHUNK_2_OFF as usize);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidBaseVaultPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::InvalidBaseVaultPubkeyChunk3 => {
                let (metas, accounts) = base_vault_mismatch_accounts(setup, CHUNK_3_OFF as usize);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidBaseVaultPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::InvalidQuoteVaultPubkeyDupChunk0 => {
                let (metas, accounts) =
                    quote_vault_mismatch_accounts(setup, CHUNK_0_OFF as usize, true);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteVaultPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::InvalidQuoteVaultPubkeyDupChunk1 => {
                let (metas, accounts) =
                    quote_vault_mismatch_accounts(setup, CHUNK_1_OFF as usize, true);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteVaultPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::InvalidQuoteVaultPubkeyDupChunk2 => {
                let (metas, accounts) =
                    quote_vault_mismatch_accounts(setup, CHUNK_2_OFF as usize, true);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteVaultPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::InvalidQuoteVaultPubkeyDupChunk3 => {
                let (metas, accounts) =
                    quote_vault_mismatch_accounts(setup, CHUNK_3_OFF as usize, true);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteVaultPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::InvalidQuoteVaultPubkeyNonDupChunk0 => {
                let (metas, accounts) =
                    quote_vault_mismatch_accounts(setup, CHUNK_0_OFF as usize, false);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteVaultPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::InvalidQuoteVaultPubkeyNonDupChunk1 => {
                let (metas, accounts) =
                    quote_vault_mismatch_accounts(setup, CHUNK_1_OFF as usize, false);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteVaultPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::InvalidQuoteVaultPubkeyNonDupChunk2 => {
                let (metas, accounts) =
                    quote_vault_mismatch_accounts(setup, CHUNK_2_OFF as usize, false);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteVaultPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::InvalidQuoteVaultPubkeyNonDupChunk3 => {
                let (metas, accounts) =
                    quote_vault_mismatch_accounts(setup, CHUNK_3_OFF as usize, false);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteVaultPubkey),
                )
            }
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
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

                        let market_pda =
                            result.resulting_accounts[RegisterMarketAccounts::Market as usize].0;
                        let base_mint_key =
                            result.resulting_accounts[RegisterMarketAccounts::BaseMint as usize].0;
                        let quote_mint_key =
                            result.resulting_accounts[RegisterMarketAccounts::QuoteMint as usize].0;

                        let base_vault = &result.resulting_accounts
                            [RegisterMarketAccounts::BaseVault as usize]
                            .1;
                        check_vault!(errors, "base vault", base_vault, &token_program_id, rent,
                            base_mint_key, market_pda);

                        let quote_vault = &result.resulting_accounts
                            [RegisterMarketAccounts::QuoteVault as usize]
                            .1;
                        check_vault!(errors, "quote vault", quote_vault, &token_program_id, rent,
                            quote_mint_key, market_pda);
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
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::CreateAccountHappyPathQuoteNonDup => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (metas, accounts) = happy_path_accounts(setup, token_program_id, token_2022_id);
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

                        let market_pda =
                            result.resulting_accounts[RegisterMarketAccounts::Market as usize].0;
                        let base_mint_key =
                            result.resulting_accounts[RegisterMarketAccounts::BaseMint as usize].0;
                        let quote_mint_key =
                            result.resulting_accounts[RegisterMarketAccounts::QuoteMint as usize].0;

                        let base_vault = &result.resulting_accounts
                            [RegisterMarketAccounts::BaseVault as usize]
                            .1;
                        check_vault!(errors, "base vault", base_vault, &token_program_id, rent,
                            base_mint_key, market_pda);

                        let quote_vault = &result.resulting_accounts
                            [RegisterMarketAccounts::QuoteVault as usize]
                            .1;
                        check_vault!(errors, "quote vault", quote_vault, &token_2022_id, rent,
                            quote_mint_key, market_pda);
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
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::CreateAccountHappyPathToken2022QuoteDup => {
                let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (metas, accounts) = happy_path_accounts(setup, token_2022_id, token_2022_id);
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

                        let market_pda =
                            result.resulting_accounts[RegisterMarketAccounts::Market as usize].0;
                        let base_mint_key =
                            result.resulting_accounts[RegisterMarketAccounts::BaseMint as usize].0;
                        let quote_mint_key =
                            result.resulting_accounts[RegisterMarketAccounts::QuoteMint as usize].0;

                        let base_vault = &result.resulting_accounts
                            [RegisterMarketAccounts::BaseVault as usize]
                            .1;
                        check_vault!(errors, "base vault", base_vault, &token_2022_id, rent,
                            base_mint_key, market_pda);

                        let quote_vault = &result.resulting_accounts
                            [RegisterMarketAccounts::QuoteVault as usize]
                            .1;
                        check_vault!(errors, "quote vault", quote_vault, &token_2022_id, rent,
                            quote_mint_key, market_pda);
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
            // Verifies: REGISTER-MARKET
            // Verifies: INIT-MARKET-PDA
            // Verifies: INIT-VAULT
            Self::CreateAccountHappyPathToken2022QuoteNonDup => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (metas, accounts) = happy_path_accounts(setup, token_2022_id, token_program_id);
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

                        let market_pda =
                            result.resulting_accounts[RegisterMarketAccounts::Market as usize].0;
                        let base_mint_key =
                            result.resulting_accounts[RegisterMarketAccounts::BaseMint as usize].0;
                        let quote_mint_key =
                            result.resulting_accounts[RegisterMarketAccounts::QuoteMint as usize].0;

                        let base_vault = &result.resulting_accounts
                            [RegisterMarketAccounts::BaseVault as usize]
                            .1;
                        check_vault!(errors, "base vault", base_vault, &token_2022_id, rent,
                            base_mint_key, market_pda);

                        let quote_vault = &result.resulting_accounts
                            [RegisterMarketAccounts::QuoteVault as usize]
                            .1;
                        check_vault!(errors, "quote vault", quote_vault, &token_program_id, rent,
                            quote_mint_key, market_pda);
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
