use dropset_interface::{Discriminant, ErrorCode};
use dropset_tests::{CaseResult, TestCase, TestSetup, check};
use solana_sdk::program_error::ProgramError;

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
                &[Discriminant::RegisterMarket.into(), 0x00],
                Err(ProgramError::Custom(
                    ErrorCode::InvalidInstructionLength.into(),
                )),
            ),
        }
    }
}
