use dropset_interface::ErrorCode;
use dropset_tests::{CaseResult, TestCase, TestSetup, check};
use solana_sdk::program_error::ProgramError;

#[derive(Clone, Copy)]
pub enum Case {
    /// Verifies: ENTRYPOINT
    InvalidDiscriminant,
    /// Verifies: ENTRYPOINT
    EmptyInstructionData,
}

impl Case {
    pub const ALL: &[Self] = &[Self::InvalidDiscriminant, Self::EmptyInstructionData];
}

impl TestCase for Case {
    fn name(&self) -> &'static str {
        match self {
            Self::InvalidDiscriminant => "invalid_discriminant",
            Self::EmptyInstructionData => "empty_instruction_data",
        }
    }

    fn run(&self, setup: &TestSetup) -> CaseResult {
        match self {
            Self::InvalidDiscriminant => check(
                setup,
                &[0xFF],
                Err(ProgramError::Custom(ErrorCode::InvalidDiscriminant.into())),
            ),
            Self::EmptyInstructionData => check(
                setup,
                &[],
                Err(ProgramError::Custom(ErrorCode::InvalidDiscriminant.into())),
            ),
        }
    }
}
