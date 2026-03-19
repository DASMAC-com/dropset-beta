use dropset_interface::{Discriminant, ErrorCode};
use dropset_tests::{CaseResult, TestCase, TestSetup, check};

#[derive(Clone, Copy)]
pub enum Case {
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
            // Verifies: REGISTER-MARKET
            Self::InvalidLength => check(
                setup,
                &[Discriminant::RegisterMarket.into(), 0x01],
                Some(ErrorCode::InvalidInstructionLength),
            ),
        }
    }
}
