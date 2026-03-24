use dropset_interface::ErrorCode;
use dropset_tests::{CaseResult, TestCase, TestSetup, check, test_cases};

test_cases! {
    #[derive(Clone, Copy)]
    pub enum Case {
        InvalidDiscriminant,
        EmptyInstructionData,
    }
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
            // Verifies: ENTRYPOINT
            Self::InvalidDiscriminant => {
                check(setup, &[0xFF], Some(ErrorCode::InvalidDiscriminant))
            }
            // Verifies: ENTRYPOINT
            Self::EmptyInstructionData => check(setup, &[], Some(ErrorCode::InvalidDiscriminant)),
        }
    }
}
