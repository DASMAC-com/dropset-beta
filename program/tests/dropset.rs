//! LiteSVM tests for the dropset program — minimal port of the mollusk
//! entrypoint + register_market test cases for anchor debugger profiling.
//!
//! These tests exercise the same instruction paths as the mollusk suite
//! but use `anchor_v2_testing::svm()` so `anchor debugger` can record
//! SBF traces.

use {
    anchor_v2_testing::{Keypair, LiteSVM, Message, Signer, VersionedMessage, VersionedTransaction},
    litesvm::types::FailedTransactionMetadata,
    solana_account::Account,
    solana_instruction::{AccountMeta, Instruction},
    solana_pubkey::Pubkey,
};

const PROGRAM_ID: &str = "F8ZgJy3ma939WWCFx1aU6WJdF9LRykGbQE7KqyUrM2iG";

fn setup() -> (LiteSVM, Pubkey, Keypair) {
    let mut svm = anchor_v2_testing::svm();
    let program_id: Pubkey = PROGRAM_ID.parse().unwrap();
    let bytes = include_bytes!("../target/deploy/dropset.so");
    svm.add_program(program_id, bytes).unwrap();

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    (svm, program_id, payer)
}

fn send(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ix: Instruction,
) -> Result<u64, FailedTransactionMetadata> {
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();
    svm.send_transaction(tx)
        .map(|meta| meta.compute_units_consumed)
}

fn send_ix(svm: &mut LiteSVM, payer: &Keypair, program_id: Pubkey, data: &[u8]) -> Result<u64, FailedTransactionMetadata> {
    let ix = Instruction::new_with_bytes(program_id, data, vec![]);
    send(svm, payer, ix)
}

// --- Entrypoint tests ---

#[test]
fn entrypoint_invalid_discriminant() {
    let (mut svm, program_id, payer) = setup();
    let result = send_ix(&mut svm, &payer, program_id, &[0xFF]);
    assert!(result.is_err(), "invalid discriminant should fail");
}

#[test]
fn entrypoint_empty_instruction_data() {
    let (mut svm, program_id, payer) = setup();
    let result = send_ix(&mut svm, &payer, program_id, &[]);
    assert!(result.is_err(), "empty data should fail");
}

#[test]
fn entrypoint_invalid_instruction_length() {
    let (mut svm, program_id, payer) = setup();
    // Discriminant 0 (RegisterMarket) but wrong data length
    let result = send_ix(&mut svm, &payer, program_id, &[0x00, 0x00]);
    assert!(result.is_err(), "wrong instruction length should fail");
}

#[test]
fn entrypoint_invalid_number_of_accounts() {
    let (mut svm, program_id, payer) = setup();
    // Discriminant 0, correct length (1 byte), but no accounts
    let result = send_ix(&mut svm, &payer, program_id, &[0x00]);
    assert!(result.is_err(), "no accounts should fail");
}

// --- RegisterMarket tests ---

#[test]
fn register_market_insufficient_accounts() {
    let (mut svm, program_id, payer) = setup();
    let ix = Instruction::new_with_bytes(
        program_id,
        &[0x00],
        vec![solana_instruction::AccountMeta::new_readonly(
            payer.pubkey(),
            false,
        )],
    );
    let result = send(&mut svm, &payer, ix);
    assert!(result.is_err());
}

/// Exercises the full RegisterMarket validation path — PDA derivation,
/// system program check, rent sysvar check, token program validation,
/// and market PDA pubkey comparison. Fails at the market PDA check
/// (the accounts aren't real PDAs) but exercises ~1600 CU of validation
/// before that — enough to produce a useful trace in the debugger.
#[test]
fn register_market_deep_validation() {
    use solana_instruction::AccountMeta;

    let (mut svm, program_id, payer) = setup();

    // RegisterMarket expects 10 accounts in this order:
    //   0: user (signer, writable)
    //   1: market
    //   2: base_mint
    //   3: quote_mint
    //   4: system_program
    //   5: rent_sysvar
    //   6: base_token_program
    //   7: base_vault
    //   8: quote_token_program
    //   9: quote_vault
    let system_program = solana_pubkey::Pubkey::from_str_const(
        "11111111111111111111111111111111"
    );
    let rent_sysvar = solana_pubkey::Pubkey::from_str_const(
        "SysvarRent111111111111111111111111111111111"
    );
    // SPL Token program
    let token_program = solana_pubkey::Pubkey::from_str_const(
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
    );

    let base_mint = Keypair::new();
    let quote_mint = Keypair::new();

    // Derive market PDA: seeds = [base_mint, quote_mint]
    let (market_pda, _bump) = Pubkey::find_program_address(
        &[base_mint.pubkey().as_ref(), quote_mint.pubkey().as_ref()],
        &program_id,
    );

    // Derive base vault PDA: seeds = [market, 0]
    let (base_vault_pda, _) = Pubkey::find_program_address(
        &[market_pda.as_ref(), &[0u8]],
        &program_id,
    );

    // Derive quote vault PDA: seeds = [market, 1]
    let (quote_vault_pda, _) = Pubkey::find_program_address(
        &[market_pda.as_ref(), &[1u8]],
        &program_id,
    );

    let metas = vec![
        AccountMeta::new(payer.pubkey(), true),       // user
        AccountMeta::new(market_pda, false),          // market
        AccountMeta::new_readonly(base_mint.pubkey(), false),   // base_mint
        AccountMeta::new_readonly(quote_mint.pubkey(), false),  // quote_mint
        AccountMeta::new_readonly(system_program, false),       // system_program
        AccountMeta::new_readonly(rent_sysvar, false),          // rent_sysvar
        AccountMeta::new_readonly(token_program, false),        // base_token_program
        AccountMeta::new(base_vault_pda, false),      // base_vault
        AccountMeta::new_readonly(token_program, false),        // quote_token_program
        AccountMeta::new(quote_vault_pda, false),     // quote_vault
    ];

    let ix = Instruction::new_with_bytes(program_id, &[0x00], metas);
    let result = send(&mut svm, &payer, ix);

    // Fails at token program validation (mints don't have proper data)
    // but exercises ~1600 CU of validation. The trace is what matters.
    match result {
        Ok(cu) => println!("register_market_deep_validation: {cu} CU (success)"),
        Err(e) => println!(
            "register_market_deep_validation: {} CU (expected failure)",
            e.meta.compute_units_consumed
        ),
    }
}

// --- Happy path: full RegisterMarket with real mints and CPIs ---

const TOKEN_PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const SYSTEM_PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("11111111111111111111111111111111");
const RENT_SYSVAR_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("SysvarRent111111111111111111111111111111111");

/// Find a (base_mint, quote_mint) pair whose market PDA and base vault
/// PDA both derive on the first bump (255), matching what the asm
/// program expects for minimal CU.
fn find_pda_seed_pair(program_id: &Pubkey) -> (Pubkey, Pubkey, Pubkey) {
    loop {
        let base = Pubkey::new_unique();
        let quote = Pubkey::new_unique();
        let (pda, pda_bump) =
            Pubkey::find_program_address(&[base.as_ref(), quote.as_ref()], program_id);
        if pda_bump != 255 {
            continue;
        }
        let (_vault, vault_bump) =
            Pubkey::find_program_address(&[pda.as_ref(), &[0u8]], program_id);
        if vault_bump != 255 {
            continue;
        }
        let (_qvault, qvault_bump) =
            Pubkey::find_program_address(&[pda.as_ref(), &[1u8]], program_id);
        if qvault_bump != 255 {
            continue;
        }
        return (base, quote, pda);
    }
}

/// Create a packed SPL Token mint account.
fn mint_account() -> Account {
    use spl_token::state::Mint;
    use spl_token::solana_program::program_pack::Pack;
    let mut data = vec![0u8; Mint::LEN];
    let mint = Mint {
        is_initialized: true,
        decimals: 6,
        ..Mint::default()
    };
    Mint::pack(mint, &mut data).unwrap();
    Account {
        lamports: 1_000_000_000,
        data,
        owner: TOKEN_PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    }
}

/// Full RegisterMarket happy path: exercises PDA derivation, system
/// program create_account CPI, and token program InitializeAccount CPI.
/// This is the ~10,000 CU path that the mollusk "happy_path_quote_dup"
/// test exercises.
#[test]
fn register_market_happy_path() {
    let (mut svm, program_id, payer) = setup();

    let (base_mint_key, quote_mint_key, market_pda) = find_pda_seed_pair(&program_id);

    let (base_vault_pda, _) =
        Pubkey::find_program_address(&[market_pda.as_ref(), &[0u8]], &program_id);
    let (quote_vault_pda, _) =
        Pubkey::find_program_address(&[market_pda.as_ref(), &[1u8]], &program_id);

    // Set mint accounts with proper SPL Token data
    svm.set_account(base_mint_key, mint_account()).unwrap();
    svm.set_account(quote_mint_key, mint_account()).unwrap();

    let metas = vec![
        AccountMeta::new(payer.pubkey(), true),               // user
        AccountMeta::new(market_pda, false),                  // market
        AccountMeta::new_readonly(base_mint_key, false),      // base_mint
        AccountMeta::new_readonly(quote_mint_key, false),     // quote_mint
        AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),  // system_program
        AccountMeta::new_readonly(RENT_SYSVAR_ID, false),     // rent_sysvar
        AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),   // base_token_program
        AccountMeta::new(base_vault_pda, false),              // base_vault
        AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),   // quote_token_program (dup)
        AccountMeta::new(quote_vault_pda, false),             // quote_vault
    ];

    let ix = Instruction::new_with_bytes(program_id, &[0x00], metas);
    let result = send(&mut svm, &payer, ix);

    match result {
        Ok(cu) => println!("register_market_happy_path: {cu} CU"),
        Err(e) => {
            println!(
                "register_market_happy_path: {} CU (failed: {:?})",
                e.meta.compute_units_consumed, e.err
            );
            // Print logs for debugging
            for log in &e.meta.logs {
                println!("  {log}");
            }
        }
    }
}
