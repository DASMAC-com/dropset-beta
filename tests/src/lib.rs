pub mod cases;

use mollusk_svm::Mollusk;
use solana_keypair::read_keypair_file;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use std::path::Path;

const DEFAULT_PROGRAM: &str = "dropset";

fn asm_dir() -> String {
    std::env::var("DROPSET_ASM_DIR").unwrap_or_else(|_| "../target/asm".to_string())
}

pub struct TestSetup {
    pub program_id: Pubkey,
    pub mollusk: Mollusk,
}

/// Creates a test environment for the default `dropset` program.
pub fn setup() -> TestSetup {
    setup_program(DEFAULT_PROGRAM)
}

/// Creates a test environment for a named program binary under `target/asm/`.
///
/// Use this for standalone subroutine harnesses assembled as separate binaries.
pub fn setup_program(name: &str) -> TestSetup {
    let dir = asm_dir();
    let keypair_path = format!("{dir}/{name}-keypair.json");
    let default_keypair_path = format!("{dir}/program-keypair.json");
    let binary_path = format!("{dir}/{name}");

    assert!(
        Path::new(&format!("{binary_path}.so")).exists(),
        "Program binary not found: {binary_path}.so. Run `make asm` first.",
    );

    let program_id = if Path::new(&keypair_path).exists() {
        let keypair =
            read_keypair_file(&keypair_path).expect("Failed to read program keypair file");
        keypair.pubkey()
    } else {
        // Fall back to the default keypair for sub-programs that share the main program ID.
        let keypair =
            read_keypair_file(&default_keypair_path).expect("Failed to read default keypair");
        keypair.pubkey()
    };

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

/// A named, runnable test case that can be executed for correctness or CU measurement.
pub trait TestCase: Copy {
    fn name(&self) -> &'static str;
    fn run(&self, setup: &TestSetup) -> CaseResult;
}

/// Runs all cases, prints a CU table, and panics if any case failed.
pub fn run_and_report<T: TestCase>(heading: &str, cases: &[T], setup: &TestSetup) {
    let mut failures = Vec::new();

    println!();
    println!("  {heading}");
    println!("  {}", "-".repeat(heading.len()));
    println!("  {:<40} {:>8}", "Case", "CUs");
    println!("  {:<40} {:>8}", "----", "---");

    for case in cases {
        let result = case.run(setup);
        let status = if result.error.is_some() { "FAIL" } else { "ok" };
        println!("  {:<40} {:>8}  {status}", case.name(), result.cu);
        if let Some(msg) = result.error {
            failures.push((case.name(), msg));
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
