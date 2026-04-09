use mollusk_svm::Mollusk;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signer::Signer;
use std::path::Path;

/// Define a test case enum with an auto-generated `ALL` slice and a
/// [`CaseName`] impl that derives snake_case names from the variant
/// identifiers.
#[macro_export]
macro_rules! test_cases {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name { $($variant),* }

        impl $name {
            pub const ALL: &[Self] = &[$(Self::$variant),*];
        }

        impl $crate::CaseName for $name {
            fn name(&self) -> String {
                use ::heck::ToSnakeCase;
                match self {
                    $(Self::$variant => stringify!($variant).to_snake_case(),)*
                }
            }
        }
    }
}

const DEFAULT_PROGRAM: &str = "dropset";

fn deploy_dir() -> String {
    std::env::var("DROPSET_DEPLOY_DIR").expect("DROPSET_DEPLOY_DIR must be set")
}

pub struct TestSetup {
    pub program_id: Pubkey,
    pub mollusk: Mollusk,
}

/// Creates a test environment for the default `dropset` program.
///
/// Sets `exemption_threshold = 1.0` (SIMD-0194) so the rent calculation
/// in the program (`acct_size * lamports_per_byte`) matches the sysvar.
pub fn setup() -> TestSetup {
    let mut setup = setup_program(DEFAULT_PROGRAM);
    #[allow(deprecated)]
    {
        setup.mollusk.sysvars.rent.exemption_threshold = 1.0;
    }
    mollusk_svm_programs_token::token::add_program(&mut setup.mollusk);
    mollusk_svm_programs_token::token2022::add_program(&mut setup.mollusk);
    setup
}

/// Creates a test environment for a named program binary under `target/asm/`.
///
/// Use this for standalone subroutine harnesses assembled as separate binaries.
pub fn setup_program(name: &str) -> TestSetup {
    let dir = deploy_dir();
    let keypair_path = format!("{dir}/{name}-keypair.json");
    let default_keypair_path = format!("{dir}/program-keypair.json");
    let binary_path = format!("{dir}/{name}");

    assert!(
        Path::new(&format!("{binary_path}.so")).exists(),
        "Program binary not found: {binary_path}.so",
    );

    let kp_path = if Path::new(&keypair_path).exists() {
        &keypair_path
    } else {
        &default_keypair_path
    };
    let program_id = read_keypair_file(kp_path)
        .expect("Failed to read program keypair file")
        .pubkey();

    let mollusk = Mollusk::new(&program_id, &binary_path);

    TestSetup {
        program_id,
        mollusk,
    }
}

/// Result of running a single test case.
pub struct CaseResult {
    /// Compute units consumed.
    pub cu: u64,
    /// `None` if the case passed, `Some(message)` if it failed.
    pub error: Option<String>,
}

/// Compare a Mollusk execution result against an expected error code.
fn evaluate(
    result: &mollusk_svm::result::InstructionResult,
    expected: Option<dropset_interface::ErrorCode>,
) -> CaseResult {
    use mollusk_svm::result::ProgramResult as MolluskResult;
    use solana_sdk::program_error::ProgramError;

    let expected_result: Result<(), ProgramError> = match expected {
        None => Ok(()),
        Some(e) => Err(ProgramError::Custom(e.into())),
    };

    let pass = match (&expected_result, &result.program_result) {
        (Ok(()), MolluskResult::Success) => true,
        (Err(e), MolluskResult::Failure(actual)) => actual == e,
        _ => false,
    };

    CaseResult {
        cu: result.compute_units_consumed,
        error: if pass {
            None
        } else {
            Some(format!(
                "expected {:?}, got {:?}",
                expected_result, result.program_result
            ))
        },
    }
}

/// Sends an instruction with the given data (no accounts) and compares against
/// an expected error code. Pass `None` for success, or `Some(ErrorCode::Variant)`
/// for a `ProgramError::Custom` failure.
pub fn check(
    setup: &TestSetup,
    data: &[u8],
    expected: Option<dropset_interface::ErrorCode>,
) -> CaseResult {
    check_with_accounts(setup, data, 0, expected)
}

/// Like [`check`], but populates the input buffer with `n_accounts` dummy
/// accounts so the program sees the requested account count.
pub fn check_with_accounts(
    setup: &TestSetup,
    data: &[u8],
    n_accounts: usize,
    expected: Option<dropset_interface::ErrorCode>,
) -> CaseResult {
    use solana_account::Account;
    use solana_sdk::instruction::{AccountMeta, Instruction};

    let keys: Vec<Pubkey> = (0..n_accounts).map(|_| Pubkey::new_unique()).collect();
    let account_metas: Vec<AccountMeta> = keys
        .iter()
        .map(|k| AccountMeta::new_readonly(*k, false))
        .collect();
    let accounts: Vec<(Pubkey, Account)> = keys.iter().map(|k| (*k, Account::default())).collect();

    let instruction = Instruction::new_with_bytes(setup.program_id, data, account_metas);
    let result = setup.mollusk.process_instruction(&instruction, &accounts);

    evaluate(&result, expected)
}

/// Like [`check_with_accounts`], but accepts pre-built account and meta lists
/// for full control over account contents and keys.
pub fn check_custom(
    setup: &TestSetup,
    data: &[u8],
    account_metas: Vec<solana_sdk::instruction::AccountMeta>,
    accounts: Vec<(Pubkey, solana_account::Account)>,
    expected: Option<dropset_interface::ErrorCode>,
) -> CaseResult {
    use solana_sdk::instruction::Instruction;

    let instruction = Instruction::new_with_bytes(setup.program_id, data, account_metas);
    let result = setup.mollusk.process_instruction(&instruction, &accounts);

    evaluate(&result, expected)
}

// region: test_case
/// Auto-derived by [`test_cases!`]: returns a snake_case name for each variant.
pub trait CaseName {
    fn name(&self) -> String;
}

/// A named, runnable test case that can be executed for correctness or CU measurement.
pub trait TestCase: Copy + CaseName {
    fn run(&self, setup: &TestSetup) -> CaseResult;
}
// endregion: test_case

/// Returns a pair of unique pubkeys whose PDA derivation against
/// `program_id` succeeds on the first bump (255), avoiding iteration
/// overhead in `sol_try_find_program_address`.
pub fn find_pda_seed_pair(program_id: &Pubkey) -> (Pubkey, Pubkey) {
    loop {
        let a = Pubkey::new_unique();
        let b = Pubkey::new_unique();
        let (_pda, bump) = Pubkey::find_program_address(&[a.as_ref(), b.as_ref()], program_id);
        if bump == u8::MAX {
            return (a, b);
        }
    }
}

/// XOR mask applied to a single byte to corrupt a pubkey chunk.
pub const CORRUPT_BYTE_MASK: u8 = 0xFF;

/// Arbitrary non-empty data length used to make an account "have data"
/// so that data_len != 0 checks trigger.
pub const NON_EMPTY_DATA_LEN: usize = 1;

/// Chunk indices for 8-byte pubkey comparisons.
pub const CHUNK_0: usize = 0;
pub const CHUNK_1: usize = 1;
pub const CHUNK_2: usize = 2;
pub const CHUNK_3: usize = 3;

/// Maps a chunk index to the byte offset within a 32-byte pubkey.
pub const CHUNK_OFFSETS: [usize; 4] = [
    dropset_interface::common::pubkey::constants::CHUNK_0_OFF as usize,
    dropset_interface::common::pubkey::constants::CHUNK_1_OFF as usize,
    dropset_interface::common::pubkey::constants::CHUNK_2_OFF as usize,
    dropset_interface::common::pubkey::constants::CHUNK_3_OFF as usize,
];

/// Build accounts with a chunk-corrupted pubkey, then check against an
/// expected error code. Used to verify 8-byte pubkey chunk comparisons.
pub fn check_chunk_error(
    setup: &TestSetup,
    insn: &[u8],
    chunk: usize,
    build: impl Fn(
        &TestSetup,
        usize,
    ) -> (
        Vec<solana_sdk::instruction::AccountMeta>,
        Vec<(Pubkey, solana_account::Account)>,
    ),
    error: dropset_interface::ErrorCode,
) -> CaseResult {
    let (metas, accounts) = build(setup, chunk);
    check_custom(setup, insn, metas, accounts, Some(error))
}

/// Runs all cases, prints a CU table, and panics if any case failed.
pub fn run_and_report<T: TestCase>(heading: &str, cases: &[T], setup: &TestSetup) {
    let mut failures = Vec::new();
    let col = cases.iter().map(|c| c.name().len()).max().unwrap_or(4);

    println!();
    println!("  {heading}");
    println!("  {}", "-".repeat(heading.len()));
    println!("  {:<col$} {:>8}", "Case", "CUs");
    println!("  {:<col$} {:>8}", "----", "---");

    for case in cases {
        let name = case.name();
        let result = case.run(setup);
        let status = if result.error.is_some() { "FAIL" } else { "ok" };
        println!("  {:<col$} {:>8}  {status}", name, result.cu);
        if let Some(msg) = result.error {
            failures.push((name, msg));
        }
    }

    println!();

    if !failures.is_empty() {
        for (name, msg) in &failures {
            eprintln!("  FAILED: {name}: {msg}");
        }
        panic!("{} case(s) failed in '{heading}'", failures.len());
    }
}
