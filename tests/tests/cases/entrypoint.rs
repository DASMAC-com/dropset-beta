use dropset_interface::ErrorCode;
use dropset_tests::{CaseResult, TestCase, TestSetup, check};

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
            Self::InvalidDiscriminant => {
                check(setup, &[0xFF], Some(ErrorCode::InvalidDiscriminant))
            }
            Self::EmptyInstructionData => check(setup, &[], Some(ErrorCode::InvalidDiscriminant)),
        }
    }
}
