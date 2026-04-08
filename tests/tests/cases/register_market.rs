use dropset_interface::common::pubkey::constants::{
    CHUNK_0_OFF, CHUNK_1_OFF, CHUNK_2_OFF, CHUNK_3_OFF,
};
use dropset_interface::common::pubkey::{TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID};
use dropset_interface::entrypoint::input_buffer::MARKET_DATA_BYTES_OFF;
use dropset_interface::market::MarketHeader;
use dropset_interface::market::constants::{VAULT_INDEX_BASE, VAULT_INDEX_QUOTE};
use dropset_interface::market::register::Accounts;
use dropset_interface::{Discriminant, ErrorCode};
use dropset_tests::{
    CHUNK_0, CHUNK_1, CHUNK_2, CHUNK_3, CHUNK_OFFSETS, CORRUPT_BYTE_MASK, CaseResult,
    NON_EMPTY_DATA_LEN, TestCase, TestSetup, check, check_chunk_error, check_custom,
    check_with_accounts, find_pda_seed_pair, test_cases,
};
use mollusk_svm::program;
use mollusk_svm::result::ProgramResult as MolluskResult;
use solana_account::Account;
use solana_program_pack::Pack;
use solana_sbpf::ebpf::MM_INPUT_START;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use spl_token_2022_interface::extension::transfer_fee::{TransferFeeAmount, TransferFeeConfig};
use spl_token_2022_interface::extension::transfer_hook::{TransferHook, TransferHookAccount};
use spl_token_2022_interface::extension::{AccountType, ExtensionType, Length};
use spl_token_interface::state::Account as TokenAccount;
use spl_token_interface::state::AccountState;
use spl_token_interface::state::Mint;

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
        BaseVaultIsDuplicate,
        BaseVaultHasData,
        InvalidBaseVaultPubkeyChunk0,
        InvalidBaseVaultPubkeyChunk1,
        InvalidBaseVaultPubkeyChunk2,
        InvalidBaseVaultPubkeyChunk3,
        InvalidQuoteTokenProgramDuplicateChunk0,
        InvalidQuoteTokenProgramDuplicateChunk1,
        InvalidQuoteTokenProgramDuplicateChunk2,
        InvalidQuoteTokenProgramDuplicateChunk3,
        NonDupQuoteTokenProgramNotQuoteMintOwnerChunk0,
        DupQuoteTokenProgramNotQuoteMintOwnerChunk0,
        NonDupQuoteTokenProgramNotQuoteMintOwnerChunk1,
        DupQuoteTokenProgramNotQuoteMintOwnerChunk1,
        NonDupQuoteTokenProgramNotQuoteMintOwnerChunk2,
        DupQuoteTokenProgramNotQuoteMintOwnerChunk2,
        NonDupQuoteTokenProgramNotQuoteMintOwnerChunk3,
        DupQuoteTokenProgramNotQuoteMintOwnerChunk3,
        QuoteVaultIsDuplicateDup,
        QuoteVaultHasDataDup,
        QuoteTokenProgramNotTokenProgramChunk0,
        QuoteTokenProgramNotTokenProgramChunk1,
        QuoteTokenProgramNotTokenProgramChunk2,
        QuoteTokenProgramNotTokenProgramChunk3,
        QuoteVaultIsDuplicateNonDup,
        QuoteVaultHasDataNonDup,
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
        CreateAccountHappyPathToken2022QuoteNonDup,
        CreateAccountHappyPathToken2022QuoteDup,
    }
}

const N_ACCOUNTS: usize = Accounts::COUNT as usize;

/// Build unique accounts with default (empty) data.
fn default_accounts() -> (Vec<Pubkey>, Vec<Account>) {
    let keys: Vec<Pubkey> = (0..N_ACCOUNTS).map(|_| Pubkey::new_unique()).collect();
    let mut accounts: Vec<Account> = (0..N_ACCOUNTS).map(|_| Account::default()).collect();
    accounts[Accounts::User as usize] = Account::new(USER_LAMPORTS, 0, &Pubkey::default());
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
const TOKEN_ACCOUNT_SIZE: usize = TokenAccount::LEN;

/// Token 2022 TLV header size: ExtensionType + Length.
const TLV_HEADER_SIZE: usize = size_of::<ExtensionType>() + size_of::<Length>();

/// Token 2022 base token account + AccountType discriminator.
const BASE_ACCOUNT_AND_TYPE_LENGTH: usize = TokenAccount::LEN + size_of::<AccountType>();

/// Expected token account size for a mint with TransferFeeConfig.
/// GetAccountDataSize returns base + AccountType + one TransferFeeAmount TLV entry.
const TOKEN_2022_ACCOUNT_SIZE_A: usize =
    BASE_ACCOUNT_AND_TYPE_LENGTH + TLV_HEADER_SIZE + size_of::<TransferFeeAmount>();

/// Expected token account size for a mint with TransferHook.
/// GetAccountDataSize returns base + AccountType + one TransferHookAccount TLV entry.
const TOKEN_2022_ACCOUNT_SIZE_B: usize =
    BASE_ACCOUNT_AND_TYPE_LENGTH + TLV_HEADER_SIZE + size_of::<TransferHookAccount>();

/// Build a Token 2022 mint account with a single TLV extension appended.
fn mint_account_2022(ext_type: ExtensionType, ext_data_len: usize) -> Account {
    let mut data = vec![0u8; Mint::LEN];
    Mint::pack(default_mint(), &mut data).unwrap();
    // Zero-pad from Mint::LEN to TokenAccount::LEN, the offset Token 2022
    // uses for the AccountType discriminator.
    data.resize(TokenAccount::LEN, 0);
    data.push(AccountType::Mint as u8);
    // TLV entry: ExtensionType + Length + zeroed extension data.
    data.extend_from_slice(&<[u8; size_of::<ExtensionType>()]>::from(ext_type));
    data.extend_from_slice(&(ext_data_len as u16).to_le_bytes());
    data.extend_from_slice(&vec![0u8; ext_data_len]);

    Account {
        lamports: solana_sdk::rent::Rent::default().minimum_balance(data.len()),
        data,
        owner: Pubkey::from(TOKEN_2022_PROGRAM_ID),
        executable: false,
        rent_epoch: 0,
    }
}

/// Token 2022 mint with TransferFeeConfig extension.
fn mint_account_2022_a() -> Account {
    mint_account_2022(
        ExtensionType::TransferFeeConfig,
        size_of::<TransferFeeConfig>(),
    )
}

/// Token 2022 mint with TransferHook extension.
fn mint_account_2022_b() -> Account {
    mint_account_2022(ExtensionType::TransferHook, size_of::<TransferHook>())
}

macro_rules! check_vault {
    ($errors:expr, $label:expr, $vault:expr, $expected_owner:expr, $rent:expr,
     $expected_mint:expr, $expected_proprietor:expr, $expected_data_len:expr) => {{
        let vault = $vault;
        let expected_owner = $expected_owner;
        let expected_data_len: usize = $expected_data_len;
        if vault.owner != *expected_owner {
            $errors.push(format!(
                "{} owner: expected {:?}, got {:?}",
                $label, expected_owner, vault.owner
            ));
        }
        if vault.data.len() != expected_data_len {
            $errors.push(format!(
                "{} data len: expected {}, got {}",
                $label,
                expected_data_len,
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
        match TokenAccount::unpack_from_slice(&vault.data) {
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
                if token_account.state != AccountState::Initialized {
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

fn check_market_header_bumps(
    errors: &mut Vec<String>,
    data: &[u8],
    program_id: &Pubkey,
    base_mint_key: Pubkey,
    quote_mint_key: Pubkey,
    market_pda: Pubkey,
) {
    let header: &MarketHeader = unsafe { &*(data.as_ptr() as *const MarketHeader) };

    let expected_next = MM_INPUT_START + MARKET_DATA_BYTES_OFF as u64;
    let actual_next = header.next as u64;
    if actual_next != expected_next {
        errors.push(format!(
            "market header next: expected {:#x}, got {:#x}",
            expected_next, actual_next
        ));
    }

    let (_, expected_market_bump) = Pubkey::find_program_address(
        &[base_mint_key.as_ref(), quote_mint_key.as_ref()],
        program_id,
    );
    if header.bump != expected_market_bump {
        errors.push(format!(
            "market header bump: expected {}, got {}",
            expected_market_bump, header.bump
        ));
    }

    let (_, expected_base_vault_bump) = Pubkey::find_program_address(
        &[market_pda.as_ref(), &[VAULT_INDEX_BASE as u8]],
        program_id,
    );
    if header.base_vault_bump != expected_base_vault_bump {
        errors.push(format!(
            "market header base_vault_bump: expected {}, got {}",
            expected_base_vault_bump, header.base_vault_bump
        ));
    }

    let (_, expected_quote_vault_bump) = Pubkey::find_program_address(
        &[market_pda.as_ref(), &[VAULT_INDEX_QUOTE as u8]],
        program_id,
    );
    if header.quote_vault_bump != expected_quote_vault_bump {
        errors.push(format!(
            "market header quote_vault_bump: expected {}, got {}",
            expected_quote_vault_bump, header.quote_vault_bump
        ));
    }
}

fn default_mint() -> Mint {
    Mint {
        is_initialized: true,
        ..Default::default()
    }
}

fn mint_account(owner: Pubkey) -> Account {
    if owner == Pubkey::from(TOKEN_PROGRAM_ID) {
        mollusk_svm_programs_token::token::create_account_for_mint(default_mint())
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
    let pda = keys[Accounts::Market as usize];
    let (quote_vault_pda, _) = Pubkey::find_program_address(
        &[pda.as_ref(), &[VAULT_INDEX_QUOTE as u8]],
        &setup.program_id,
    );
    keys[Accounts::QuoteVault as usize] = quote_vault_pda;

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
    keys[Accounts::BaseMint as usize] = base_key;
    keys[Accounts::QuoteMint as usize] = quote_key;
    let (mut pda, _bump) =
        Pubkey::find_program_address(&[base_key.as_ref(), quote_key.as_ref()], &setup.program_id);
    // Flip a byte in the target chunk so only that comparison fails.
    let offset = corrupt_chunk * 8;
    pda.as_mut()[offset] ^= CORRUPT_BYTE_MASK;
    keys[Accounts::Market as usize] = pda;
    let (system_program_pubkey, system_program_account) =
        program::keyed_account_for_system_program();
    keys[Accounts::SystemProgram as usize] = system_program_pubkey;
    accounts[Accounts::SystemProgram as usize] = system_program_account;
    let (rent_sysvar_pubkey, rent_sysvar_account) =
        setup.mollusk.sysvars.keyed_account_for_rent_sysvar();
    keys[Accounts::RentSysvar as usize] = rent_sysvar_pubkey;
    accounts[Accounts::RentSysvar as usize] = rent_sysvar_account;
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
    keys[Accounts::BaseMint as usize] = base_key;
    keys[Accounts::QuoteMint as usize] = quote_key;
    let (pda, _bump) =
        Pubkey::find_program_address(&[base_key.as_ref(), quote_key.as_ref()], &setup.program_id);
    keys[Accounts::Market as usize] = pda;
    // System Program ID is Pubkey::default() (all zeroes).
    // Flip a byte in the target chunk so only that comparison fails.
    let mut system_program_key = Pubkey::default();
    let offset = corrupt_chunk * 8;
    system_program_key.as_mut()[offset] ^= CORRUPT_BYTE_MASK;
    keys[Accounts::SystemProgram as usize] = system_program_key;
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
    keys[Accounts::BaseMint as usize] = base_key;
    keys[Accounts::QuoteMint as usize] = quote_key;
    let (pda, _bump) =
        Pubkey::find_program_address(&[base_key.as_ref(), quote_key.as_ref()], &setup.program_id);
    keys[Accounts::Market as usize] = pda;
    keys[Accounts::SystemProgram as usize] = Pubkey::default();
    let mut rent_key = solana_sdk::sysvar::rent::ID;
    // Flip a byte at the given offset so that comparison fails.
    rent_key.as_mut()[corrupt_byte] ^= CORRUPT_BYTE_MASK;
    keys[Accounts::RentSysvar as usize] = rent_key;
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
    keys[Accounts::BaseMint as usize] = base_key;
    keys[Accounts::QuoteMint as usize] = quote_key;
    keys[Accounts::Market as usize] = pda;

    let (system_program_pubkey, system_program_account) =
        program::keyed_account_for_system_program();
    keys[Accounts::SystemProgram as usize] = system_program_pubkey;
    accounts[Accounts::SystemProgram as usize] = system_program_account;

    let (rent_sysvar_pubkey, rent_sysvar_account) =
        setup.mollusk.sysvars.keyed_account_for_rent_sysvar();
    keys[Accounts::RentSysvar as usize] = rent_sysvar_pubkey;
    accounts[Accounts::RentSysvar as usize] = rent_sysvar_account;

    accounts[Accounts::BaseMint as usize] =
        if base_token_program == Pubkey::from(TOKEN_2022_PROGRAM_ID) {
            mint_account_2022_a()
        } else {
            mint_account(base_token_program)
        };
    accounts[Accounts::QuoteMint as usize] =
        if quote_token_program == Pubkey::from(TOKEN_2022_PROGRAM_ID) {
            mint_account_2022_b()
        } else {
            mint_account(quote_token_program)
        };

    keys[Accounts::BaseTokenProgram as usize] = base_token_program;
    accounts[Accounts::BaseTokenProgram as usize] = token_program_account(base_token_program);
    keys[Accounts::QuoteTokenProgram as usize] = quote_token_program;
    accounts[Accounts::QuoteTokenProgram as usize] = token_program_account(quote_token_program);

    // Derive base vault PDA from market address and vault index.
    let (base_vault_pda, _) = Pubkey::find_program_address(
        &[pda.as_ref(), &[VAULT_INDEX_BASE as u8]],
        &setup.program_id,
    );
    keys[Accounts::BaseVault as usize] = base_vault_pda;

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
            let writable = i == Accounts::User as usize
                || i == Accounts::Market as usize
                || i == Accounts::BaseVault as usize
                || i == Accounts::QuoteVault as usize;
            let signer = i == Accounts::User as usize
                || i == Accounts::BaseVault as usize
                || i == Accounts::QuoteVault as usize;
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
    keys[Accounts::BaseVault as usize].as_mut()[corrupt_byte] ^= CORRUPT_BYTE_MASK;
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

    let pda = keys[Accounts::Market as usize];
    let (mut quote_vault_pda, _) = Pubkey::find_program_address(
        &[pda.as_ref(), &[VAULT_INDEX_QUOTE as u8]],
        &setup.program_id,
    );
    quote_vault_pda.as_mut()[corrupt_byte] ^= CORRUPT_BYTE_MASK;
    keys[Accounts::QuoteVault as usize] = quote_vault_pda;

    writable_metas_and_accounts(keys, accounts)
}

fn check_happy_path(
    setup: &TestSetup,
    insn: &[u8],
    base_token_program: Pubkey,
    quote_token_program: Pubkey,
    expected_base_vault_size: usize,
    expected_quote_vault_size: usize,
) -> CaseResult {
    let (metas, accounts) = happy_path_accounts(setup, base_token_program, quote_token_program);
    let instruction = Instruction::new_with_bytes(setup.program_id, insn, metas);
    let result = setup.mollusk.process_instruction(&instruction, &accounts);

    let mut errors = Vec::new();
    match &result.program_result {
        MolluskResult::Success => {
            let market = &result.resulting_accounts[Accounts::Market as usize].1;

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

            let market_pda = result.resulting_accounts[Accounts::Market as usize].0;
            let base_mint_key = result.resulting_accounts[Accounts::BaseMint as usize].0;
            let quote_mint_key = result.resulting_accounts[Accounts::QuoteMint as usize].0;

            let base_vault = &result.resulting_accounts[Accounts::BaseVault as usize].1;
            check_vault!(
                errors,
                "base vault",
                base_vault,
                &base_token_program,
                rent,
                base_mint_key,
                market_pda,
                expected_base_vault_size
            );

            let quote_vault = &result.resulting_accounts[Accounts::QuoteVault as usize].1;
            check_vault!(
                errors,
                "quote vault",
                quote_vault,
                &quote_token_program,
                rent,
                quote_mint_key,
                market_pda,
                expected_quote_vault_size
            );

            check_market_header_bumps(
                &mut errors,
                &market.data,
                &setup.program_id,
                base_mint_key,
                quote_mint_key,
                market_pda,
            );
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

fn base_owner_mismatch_accounts(
    setup: &TestSetup,
    chunk: usize,
) -> (Vec<AccountMeta>, Vec<(Pubkey, Account)>) {
    let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
    let (mut keys, accounts) =
        token_program_base_accounts(setup, token_program_id, token_program_id, false);
    let mut bad_key = token_program_id;
    bad_key.as_mut()[CHUNK_OFFSETS[chunk]] ^= CORRUPT_BYTE_MASK;
    keys[Accounts::BaseTokenProgram as usize] = bad_key;
    writable_metas_and_accounts(keys, accounts)
}

fn base_program_mismatch_accounts(
    setup: &TestSetup,
    chunk: usize,
) -> (Vec<AccountMeta>, Vec<(Pubkey, Account)>) {
    let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
    bad_program.as_mut()[CHUNK_OFFSETS[chunk]] ^= CORRUPT_BYTE_MASK;
    let (keys, accounts) = token_program_base_accounts(setup, bad_program, bad_program, false);
    writable_metas_and_accounts(keys, accounts)
}

fn quote_program_mismatch_accounts(
    setup: &TestSetup,
    chunk: usize,
) -> (Vec<AccountMeta>, Vec<(Pubkey, Account)>) {
    let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
    let mut bad_program = Pubkey::from(TOKEN_PROGRAM_ID);
    bad_program.as_mut()[CHUNK_OFFSETS[chunk]] ^= CORRUPT_BYTE_MASK;
    let (keys, accounts) = token_program_base_accounts(setup, token_program_id, bad_program, false);
    writable_metas_and_accounts(keys, accounts)
}

fn non_dup_quote_owner_mismatch_accounts(
    setup: &TestSetup,
    chunk: usize,
) -> (Vec<AccountMeta>, Vec<(Pubkey, Account)>) {
    let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
    let token_2022_id = Pubkey::from(TOKEN_2022_PROGRAM_ID);
    let (mut keys, accounts) =
        token_program_base_accounts(setup, token_program_id, token_2022_id, false);
    let mut bad_key = token_2022_id;
    bad_key.as_mut()[CHUNK_OFFSETS[chunk]] ^= CORRUPT_BYTE_MASK;
    keys[Accounts::QuoteTokenProgram as usize] = bad_key;
    writable_metas_and_accounts(keys, accounts)
}

fn dup_quote_owner_mismatch_accounts(
    setup: &TestSetup,
    chunk: usize,
) -> (Vec<AccountMeta>, Vec<(Pubkey, Account)>) {
    let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
    let (mut keys, mut accounts) =
        token_program_base_accounts(setup, token_program_id, token_program_id, false);
    if chunk == 0 {
        accounts[Accounts::QuoteMint as usize].owner = Pubkey::from(TOKEN_2022_PROGRAM_ID);
    } else {
        let mut bad_owner = token_program_id;
        bad_owner.as_mut()[CHUNK_OFFSETS[chunk]] ^= CORRUPT_BYTE_MASK;
        accounts[Accounts::QuoteMint as usize].owner = bad_owner;
    }
    keys[Accounts::QuoteTokenProgram as usize] = token_program_id;
    writable_metas_and_accounts(keys, accounts)
}

impl TestCase for Case {
    fn run(&self, setup: &TestSetup) -> CaseResult {
        let insn = &[Discriminant::RegisterMarket.into()];
        match self {
            // Verifies: MARKET-PDA-PRELUDE
            Self::InvalidNumberOfAccounts => {
                check(setup, insn, Some(ErrorCode::InvalidNumberOfAccounts))
            }
            // Verifies: MARKET-PDA-PRELUDE
            Self::InvalidInstructionLength => check_with_accounts(
                setup,
                &[Discriminant::RegisterMarket.into(), 0x00],
                N_ACCOUNTS,
                Some(ErrorCode::InvalidInstructionLength),
            ),
            // Verifies: MARKET-PDA-PRELUDE
            Self::UserHasData => {
                let (keys, mut accounts) = default_accounts();
                accounts[Accounts::User as usize].data = vec![0u8; NON_EMPTY_DATA_LEN];
                let (metas, accounts) = into_metas_and_accounts(keys, accounts);
                check_custom(setup, insn, metas, accounts, Some(ErrorCode::UserHasData))
            }
            // Verifies: MARKET-PDA-PRELUDE
            Self::MarketAccountIsDuplicate => {
                let (mut keys, accounts) = default_accounts();
                // Market shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[Accounts::Market as usize] = keys[Accounts::User as usize];
                let (metas, accounts) = into_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::MarketAccountIsDuplicate),
                )
            }
            // Verifies: MARKET-PDA-PRELUDE
            Self::MarketHasData => {
                let (keys, mut accounts) = default_accounts();
                accounts[Accounts::Market as usize].data = vec![0u8; NON_EMPTY_DATA_LEN];
                let (metas, accounts) = into_metas_and_accounts(keys, accounts);
                check_custom(setup, insn, metas, accounts, Some(ErrorCode::MarketHasData))
            }
            // Verifies: MARKET-PDA-PRELUDE
            Self::BaseMintIsDuplicate => {
                let (mut keys, accounts) = default_accounts();
                // BaseMint shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[Accounts::BaseMint as usize] = keys[Accounts::User as usize];
                let (metas, accounts) = into_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseMintIsDuplicate),
                )
            }
            // Verifies: MARKET-PDA-PRELUDE
            Self::QuoteMintIsDuplicate => {
                let (mut keys, accounts) = default_accounts();
                // QuoteMint shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[Accounts::QuoteMint as usize] = keys[Accounts::User as usize];
                let (metas, accounts) = into_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteMintIsDuplicate),
                )
            }
            // Verifies: MARKET-PDA-PRELUDE
            Self::SystemProgramIsDuplicate => {
                let (mut keys, accounts) = default_accounts();
                let (base_key, quote_key) = find_pda_seed_pair(&setup.program_id);
                keys[Accounts::BaseMint as usize] = base_key;
                keys[Accounts::QuoteMint as usize] = quote_key;
                let (pda, _bump) = Pubkey::find_program_address(
                    &[base_key.as_ref(), quote_key.as_ref()],
                    &setup.program_id,
                );
                keys[Accounts::Market as usize] = pda;
                // SystemProgram shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[Accounts::SystemProgram as usize] = keys[Accounts::User as usize];
                let (metas, accounts) = into_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::SystemProgramIsDuplicate),
                )
            }
            // Verifies: MARKET-PDA-PRELUDE
            Self::InvalidSystemProgramPubkeyChunk0
            | Self::InvalidSystemProgramPubkeyChunk1
            | Self::InvalidSystemProgramPubkeyChunk2
            | Self::InvalidSystemProgramPubkeyChunk3 => check_chunk_error(
                setup,
                insn,
                *self as usize - Case::InvalidSystemProgramPubkeyChunk0 as usize,
                system_program_mismatch_accounts,
                ErrorCode::InvalidSystemProgramPubkey,
            ),
            // Verifies: MARKET-PDA-PRELUDE
            Self::RentSysvarIsDuplicate => {
                let (mut keys, accounts) = default_accounts();
                let (base_key, quote_key) = find_pda_seed_pair(&setup.program_id);
                keys[Accounts::BaseMint as usize] = base_key;
                keys[Accounts::QuoteMint as usize] = quote_key;
                let (pda, _bump) = Pubkey::find_program_address(
                    &[base_key.as_ref(), quote_key.as_ref()],
                    &setup.program_id,
                );
                keys[Accounts::Market as usize] = pda;
                keys[Accounts::SystemProgram as usize] = Pubkey::default();
                // RentSysvar shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[Accounts::RentSysvar as usize] = keys[Accounts::User as usize];
                let (metas, accounts) = into_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::RentSysvarIsDuplicate),
                )
            }
            // Verifies: MARKET-PDA-PRELUDE
            Self::InvalidRentSysvarPubkeyChunk0
            | Self::InvalidRentSysvarPubkeyChunk1
            | Self::InvalidRentSysvarPubkeyChunk2
            | Self::InvalidRentSysvarPubkeyChunk3 => {
                let chunk = *self as usize - Case::InvalidRentSysvarPubkeyChunk0 as usize;
                check_chunk_error(
                    setup,
                    insn,
                    chunk,
                    |s, c| rent_sysvar_mismatch_accounts(s, CHUNK_OFFSETS[c]),
                    ErrorCode::InvalidRentSysvarPubkey,
                )
            }
            // Verifies: MARKET-PDA-PRELUDE (mov32 optimization: chunk 3 hi
            // bits are zero, so mov32 zero-extends and implicitly checks
            // the upper 32 bits)
            Self::InvalidRentSysvarPubkeyChunk3Hi => {
                let (metas, accounts) =
                    rent_sysvar_mismatch_accounts(setup, CHUNK_3_OFF as usize + size_of::<u32>());
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidRentSysvarPubkey),
                )
            }
            // Verifies: INIT-MARKET-PDA
            Self::InvalidMarketPubkeyChunk0
            | Self::InvalidMarketPubkeyChunk1
            | Self::InvalidMarketPubkeyChunk2
            | Self::InvalidMarketPubkeyChunk3 => check_chunk_error(
                setup,
                insn,
                *self as usize - Case::InvalidMarketPubkeyChunk0 as usize,
                pda_mismatch_accounts,
                ErrorCode::InvalidMarketPubkey,
            ),
            // Verifies: INIT-BASE-VAULT
            Self::BaseTokenProgramIsDuplicate => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                // BaseTokenProgram shares key with User, causing duplicate.
                keys[Accounts::BaseTokenProgram as usize] = keys[Accounts::User as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseTokenProgramIsDuplicate),
                )
            }
            // Verifies: INIT-BASE-VAULT
            Self::BaseTokenProgramNotBaseMintOwnerChunk0
            | Self::BaseTokenProgramNotBaseMintOwnerChunk1
            | Self::BaseTokenProgramNotBaseMintOwnerChunk2
            | Self::BaseTokenProgramNotBaseMintOwnerChunk3 => check_chunk_error(
                setup,
                insn,
                *self as usize - Case::BaseTokenProgramNotBaseMintOwnerChunk0 as usize,
                base_owner_mismatch_accounts,
                ErrorCode::BaseTokenProgramNotBaseMintOwner,
            ),
            // Verifies: INIT-BASE-VAULT
            Self::BaseTokenProgramNotTokenProgramChunk0
            | Self::BaseTokenProgramNotTokenProgramChunk1
            | Self::BaseTokenProgramNotTokenProgramChunk2
            | Self::BaseTokenProgramNotTokenProgramChunk3 => check_chunk_error(
                setup,
                insn,
                *self as usize - Case::BaseTokenProgramNotTokenProgramChunk0 as usize,
                base_program_mismatch_accounts,
                ErrorCode::BaseTokenProgramNotTokenProgram,
            ),
            // Verifies: INIT-BASE-VAULT
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
                keys[Accounts::BaseVault as usize] = keys[Accounts::User as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseVaultIsDuplicate),
                )
            }
            // Verifies: INIT-BASE-VAULT
            Self::BaseVaultHasData => {
                let base_token_program = Pubkey::from(TOKEN_PROGRAM_ID);
                let (keys, mut accounts) = token_program_base_accounts(
                    setup,
                    base_token_program,
                    base_token_program,
                    false,
                );
                accounts[Accounts::BaseVault as usize].data = vec![0u8; NON_EMPTY_DATA_LEN];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::BaseVaultHasData),
                )
            }
            // Verifies: INIT-VAULT
            Self::InvalidBaseVaultPubkeyChunk0
            | Self::InvalidBaseVaultPubkeyChunk1
            | Self::InvalidBaseVaultPubkeyChunk2
            | Self::InvalidBaseVaultPubkeyChunk3 => check_chunk_error(
                setup,
                insn,
                *self as usize - Case::InvalidBaseVaultPubkeyChunk0 as usize,
                |s, c| base_vault_mismatch_accounts(s, CHUNK_OFFSETS[c]),
                ErrorCode::InvalidBaseVaultPubkey,
            ),
            // Verifies: INIT-QUOTE-VAULT
            Self::InvalidQuoteTokenProgramDuplicateChunk0 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                keys[Accounts::QuoteTokenProgram as usize] = keys[Accounts::User as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteTokenProgramDuplicate),
                )
            }
            // Verifies: INIT-QUOTE-VAULT
            Self::InvalidQuoteTokenProgramDuplicateChunk1 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                keys[Accounts::QuoteTokenProgram as usize] = keys[Accounts::Market as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteTokenProgramDuplicate),
                )
            }
            // Verifies: INIT-QUOTE-VAULT
            Self::InvalidQuoteTokenProgramDuplicateChunk2 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                keys[Accounts::QuoteTokenProgram as usize] = keys[Accounts::BaseMint as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteTokenProgramDuplicate),
                )
            }
            // Verifies: INIT-QUOTE-VAULT
            Self::InvalidQuoteTokenProgramDuplicateChunk3 => {
                let token_program_id = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) =
                    token_program_base_accounts(setup, token_program_id, token_program_id, false);
                keys[Accounts::QuoteTokenProgram as usize] = keys[Accounts::QuoteMint as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::InvalidQuoteTokenProgramDuplicate),
                )
            }
            // Verifies: INIT-QUOTE-VAULT
            Self::NonDupQuoteTokenProgramNotQuoteMintOwnerChunk0 => check_chunk_error(
                setup,
                insn,
                CHUNK_0,
                non_dup_quote_owner_mismatch_accounts,
                ErrorCode::NonDupQuoteTokenProgramNotQuoteMintOwner,
            ),
            // Verifies: INIT-QUOTE-VAULT
            Self::DupQuoteTokenProgramNotQuoteMintOwnerChunk0 => check_chunk_error(
                setup,
                insn,
                CHUNK_0,
                dup_quote_owner_mismatch_accounts,
                ErrorCode::DupQuoteTokenProgramNotQuoteMintOwner,
            ),
            // Verifies: INIT-QUOTE-VAULT
            Self::NonDupQuoteTokenProgramNotQuoteMintOwnerChunk1 => check_chunk_error(
                setup,
                insn,
                CHUNK_1,
                non_dup_quote_owner_mismatch_accounts,
                ErrorCode::NonDupQuoteTokenProgramNotQuoteMintOwner,
            ),
            // Verifies: INIT-QUOTE-VAULT
            Self::DupQuoteTokenProgramNotQuoteMintOwnerChunk1 => check_chunk_error(
                setup,
                insn,
                CHUNK_1,
                dup_quote_owner_mismatch_accounts,
                ErrorCode::DupQuoteTokenProgramNotQuoteMintOwner,
            ),
            // Verifies: INIT-QUOTE-VAULT
            Self::NonDupQuoteTokenProgramNotQuoteMintOwnerChunk2 => check_chunk_error(
                setup,
                insn,
                CHUNK_2,
                non_dup_quote_owner_mismatch_accounts,
                ErrorCode::NonDupQuoteTokenProgramNotQuoteMintOwner,
            ),
            // Verifies: INIT-QUOTE-VAULT
            Self::DupQuoteTokenProgramNotQuoteMintOwnerChunk2 => check_chunk_error(
                setup,
                insn,
                CHUNK_2,
                dup_quote_owner_mismatch_accounts,
                ErrorCode::DupQuoteTokenProgramNotQuoteMintOwner,
            ),
            // Verifies: INIT-QUOTE-VAULT
            Self::NonDupQuoteTokenProgramNotQuoteMintOwnerChunk3 => check_chunk_error(
                setup,
                insn,
                CHUNK_3,
                non_dup_quote_owner_mismatch_accounts,
                ErrorCode::NonDupQuoteTokenProgramNotQuoteMintOwner,
            ),
            // Verifies: INIT-QUOTE-VAULT
            Self::DupQuoteTokenProgramNotQuoteMintOwnerChunk3 => check_chunk_error(
                setup,
                insn,
                CHUNK_3,
                dup_quote_owner_mismatch_accounts,
                ErrorCode::DupQuoteTokenProgramNotQuoteMintOwner,
            ),
            // Verifies: INIT-QUOTE-VAULT
            Self::QuoteVaultIsDuplicateDup => {
                let base_token_program = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, accounts) = token_program_base_accounts(
                    setup,
                    base_token_program,
                    base_token_program,
                    true,
                );
                let pda = keys[Accounts::Market as usize];
                let (quote_vault_pda, _) = Pubkey::find_program_address(
                    &[pda.as_ref(), &[VAULT_INDEX_QUOTE as u8]],
                    &setup.program_id,
                );
                keys[Accounts::QuoteVault as usize] = quote_vault_pda;
                // Quote vault shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[Accounts::QuoteVault as usize] = keys[Accounts::User as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteVaultIsDuplicate),
                )
            }
            // Verifies: INIT-QUOTE-VAULT
            Self::QuoteVaultHasDataDup => {
                let base_token_program = Pubkey::from(TOKEN_PROGRAM_ID);
                let (mut keys, mut accounts) = token_program_base_accounts(
                    setup,
                    base_token_program,
                    base_token_program,
                    true,
                );
                let pda = keys[Accounts::Market as usize];
                let (quote_vault_pda, _) = Pubkey::find_program_address(
                    &[pda.as_ref(), &[VAULT_INDEX_QUOTE as u8]],
                    &setup.program_id,
                );
                keys[Accounts::QuoteVault as usize] = quote_vault_pda;
                accounts[Accounts::QuoteVault as usize].data = vec![0u8; NON_EMPTY_DATA_LEN];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteVaultHasData),
                )
            }
            // Verifies: INIT-QUOTE-VAULT
            Self::QuoteTokenProgramNotTokenProgramChunk0
            | Self::QuoteTokenProgramNotTokenProgramChunk1
            | Self::QuoteTokenProgramNotTokenProgramChunk2
            | Self::QuoteTokenProgramNotTokenProgramChunk3 => check_chunk_error(
                setup,
                insn,
                *self as usize - Case::QuoteTokenProgramNotTokenProgramChunk0 as usize,
                quote_program_mismatch_accounts,
                ErrorCode::QuoteTokenProgramNotTokenProgram,
            ),
            // Verifies: INIT-QUOTE-VAULT
            Self::QuoteVaultIsDuplicateNonDup => {
                let base_token_program = Pubkey::from(TOKEN_PROGRAM_ID);
                let quote_token_program = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (mut keys, accounts) = token_program_base_accounts(
                    setup,
                    base_token_program,
                    quote_token_program,
                    true,
                );
                let pda = keys[Accounts::Market as usize];
                let (quote_vault_pda, _) = Pubkey::find_program_address(
                    &[pda.as_ref(), &[VAULT_INDEX_QUOTE as u8]],
                    &setup.program_id,
                );
                keys[Accounts::QuoteVault as usize] = quote_vault_pda;
                // Quote vault shares key with User, causing the runtime
                // to serialize it as a duplicate.
                keys[Accounts::QuoteVault as usize] = keys[Accounts::User as usize];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteVaultIsDuplicate),
                )
            }
            // Verifies: INIT-QUOTE-VAULT
            Self::QuoteVaultHasDataNonDup => {
                let base_token_program = Pubkey::from(TOKEN_PROGRAM_ID);
                let quote_token_program = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                let (mut keys, mut accounts) = token_program_base_accounts(
                    setup,
                    base_token_program,
                    quote_token_program,
                    true,
                );
                let pda = keys[Accounts::Market as usize];
                let (quote_vault_pda, _) = Pubkey::find_program_address(
                    &[pda.as_ref(), &[VAULT_INDEX_QUOTE as u8]],
                    &setup.program_id,
                );
                keys[Accounts::QuoteVault as usize] = quote_vault_pda;
                accounts[Accounts::QuoteVault as usize].data = vec![0u8; NON_EMPTY_DATA_LEN];
                let (metas, accounts) = writable_metas_and_accounts(keys, accounts);
                check_custom(
                    setup,
                    insn,
                    metas,
                    accounts,
                    Some(ErrorCode::QuoteVaultHasData),
                )
            }
            // Verifies: INIT-VAULT
            Self::InvalidQuoteVaultPubkeyDupChunk0
            | Self::InvalidQuoteVaultPubkeyDupChunk1
            | Self::InvalidQuoteVaultPubkeyDupChunk2
            | Self::InvalidQuoteVaultPubkeyDupChunk3 => check_chunk_error(
                setup,
                insn,
                *self as usize - Case::InvalidQuoteVaultPubkeyDupChunk0 as usize,
                |s, c| quote_vault_mismatch_accounts(s, CHUNK_OFFSETS[c], true),
                ErrorCode::InvalidQuoteVaultPubkey,
            ),
            // Verifies: INIT-VAULT
            Self::InvalidQuoteVaultPubkeyNonDupChunk0
            | Self::InvalidQuoteVaultPubkeyNonDupChunk1
            | Self::InvalidQuoteVaultPubkeyNonDupChunk2
            | Self::InvalidQuoteVaultPubkeyNonDupChunk3 => check_chunk_error(
                setup,
                insn,
                *self as usize - Case::InvalidQuoteVaultPubkeyNonDupChunk0 as usize,
                |s, c| quote_vault_mismatch_accounts(s, CHUNK_OFFSETS[c], false),
                ErrorCode::InvalidQuoteVaultPubkey,
            ),
            // Verifies: REGISTER-MARKET
            // Verifies: MARKET-PDA-PRELUDE
            // Verifies: INIT-MARKET-PDA
            // Verifies: CREATE-MARKET-ACCOUNT
            // Verifies: INIT-BASE-VAULT
            // Verifies: INIT-QUOTE-VAULT
            // Verifies: INIT-VAULT
            // Verifies: GET-VAULT-SIZE
            // Verifies: CREATE-VAULT-ACCOUNT
            // Verifies: INIT-VAULT-TOKEN-ACCOUNT
            Self::CreateAccountHappyPathQuoteDup => {
                let tp = Pubkey::from(TOKEN_PROGRAM_ID);
                check_happy_path(setup, insn, tp, tp, TOKEN_ACCOUNT_SIZE, TOKEN_ACCOUNT_SIZE)
            }
            // Verifies: REGISTER-MARKET
            // Verifies: MARKET-PDA-PRELUDE
            // Verifies: INIT-MARKET-PDA
            // Verifies: CREATE-MARKET-ACCOUNT
            // Verifies: INIT-BASE-VAULT
            // Verifies: INIT-QUOTE-VAULT
            // Verifies: INIT-VAULT
            // Verifies: GET-VAULT-SIZE
            // Verifies: CREATE-VAULT-ACCOUNT
            // Verifies: INIT-VAULT-TOKEN-ACCOUNT
            Self::CreateAccountHappyPathQuoteNonDup => check_happy_path(
                setup,
                insn,
                Pubkey::from(TOKEN_PROGRAM_ID),
                Pubkey::from(TOKEN_2022_PROGRAM_ID),
                TOKEN_ACCOUNT_SIZE,
                TOKEN_2022_ACCOUNT_SIZE_B,
            ),
            // Verifies: REGISTER-MARKET
            // Verifies: MARKET-PDA-PRELUDE
            // Verifies: INIT-MARKET-PDA
            // Verifies: CREATE-MARKET-ACCOUNT
            // Verifies: INIT-BASE-VAULT
            // Verifies: INIT-QUOTE-VAULT
            // Verifies: INIT-VAULT
            // Verifies: GET-VAULT-SIZE
            // Verifies: CREATE-VAULT-ACCOUNT
            // Verifies: INIT-VAULT-TOKEN-ACCOUNT
            Self::CreateAccountHappyPathToken2022QuoteNonDup => check_happy_path(
                setup,
                insn,
                Pubkey::from(TOKEN_2022_PROGRAM_ID),
                Pubkey::from(TOKEN_PROGRAM_ID),
                TOKEN_2022_ACCOUNT_SIZE_A,
                TOKEN_ACCOUNT_SIZE,
            ),
            // Verifies: REGISTER-MARKET
            // Verifies: MARKET-PDA-PRELUDE
            // Verifies: INIT-MARKET-PDA
            // Verifies: CREATE-MARKET-ACCOUNT
            // Verifies: INIT-BASE-VAULT
            // Verifies: INIT-QUOTE-VAULT
            // Verifies: INIT-VAULT
            // Verifies: GET-VAULT-SIZE
            // Verifies: CREATE-VAULT-ACCOUNT
            // Verifies: INIT-VAULT-TOKEN-ACCOUNT
            Self::CreateAccountHappyPathToken2022QuoteDup => {
                let t22 = Pubkey::from(TOKEN_2022_PROGRAM_ID);
                check_happy_path(
                    setup,
                    insn,
                    t22,
                    t22,
                    TOKEN_2022_ACCOUNT_SIZE_A,
                    TOKEN_2022_ACCOUNT_SIZE_B,
                )
            }
        }
    }
}
