use dropset_tests::{CaseResult, TestCase, TestSetup, check};
use solana_sdk::program_error::ProgramError;

const DISC_REGISTER_MARKET: u8 = 0;
const E_INVALID_INSTRUCTION_LENGTH: u32 = 2;

#[derive(Clone, Copy)]
pub enum Case {
    /// Verifies: REGISTER-MARKET
    InvalidLength,
}

impl Case {
    pub const ALL: &[Self] = &[Self::InvalidLength];
}

impl TestCase for Case {
    fn name(&self) -> &'static str {
        match self {
            Self::InvalidLength => "invalid_length",
        }
    }

    fn run(&self, setup: &TestSetup) -> CaseResult {
        match self {
            Self::InvalidLength => check(
                setup,
                &[DISC_REGISTER_MARKET, 0x00],
                Err(ProgramError::Custom(E_INVALID_INSTRUCTION_LENGTH)),
            ),
        }
    }
}
