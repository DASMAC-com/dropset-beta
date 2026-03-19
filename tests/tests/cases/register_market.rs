use dropset_interface::{Discriminant, ErrorCode};
use dropset_tests::{CaseResult, TestCase, TestSetup, check};

#[derive(Clone, Copy)]
pub enum Case {
    InvalidLength,
    InvalidNumberOfAccounts,
}

impl Case {
    pub const ALL: &[Self] = &[Self::InvalidLength, Self::InvalidNumberOfAccounts];
}

impl TestCase for Case {
    fn name(&self) -> &'static str {
        match self {
            Self::InvalidLength => "invalid_length",
            Self::InvalidNumberOfAccounts => "invalid_number_of_accounts",
        }
    }

    fn run(&self, setup: &TestSetup) -> CaseResult {
        match self {
            // Verifies: REGISTER-MARKET
            Self::InvalidLength => check(
                setup,
                &[Discriminant::RegisterMarket.into(), 0x00],
                Some(ErrorCode::InvalidInstructionLength),
            ),
            // Verifies: REGISTER-MARKET
            Self::InvalidNumberOfAccounts => check(
                setup,
                &[Discriminant::RegisterMarket.into()],
                Some(ErrorCode::InvalidNumberOfAccounts),
            ),
        }
    }
}
