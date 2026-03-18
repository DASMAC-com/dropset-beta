use crate::{CaseResult, TestCase, TestSetup};
use mollusk_svm::result::ProgramResult as MolluskResult;
use solana_sdk::instruction::Instruction;
use solana_sdk::program_error::ProgramError;

/// Discriminant byte for RegisterMarket (from interface).
const DISC_REGISTER_MARKET: u8 = 0;

/// Error code for InvalidDiscriminant (from interface).
const E_INVALID_DISCRIMINANT: u32 = 1;

/// Error code for InvalidInstructionLength (from interface).
const E_INVALID_INSTRUCTION_LENGTH: u32 = 2;

/// Test cases covering the program entrypoint dispatch and basic instruction validation.
///
/// These cases exercise the paths in `entrypoint.s` and the instruction-length check at the
/// top of each handler.
#[derive(Clone, Copy)]
pub enum EntrypointCase {
    /// Discriminant byte does not match any known variant.
    /// Verifies: ENTRYPOINT
    InvalidDiscriminant,

    /// RegisterMarket discriminant with trailing bytes (wrong instruction length).
    /// Verifies: ENTRYPOINT
    /// Verifies: REGISTER-MARKET
    RegisterMarketInvalidLength,

    /// Completely empty instruction data (zero bytes).
    /// Verifies: ENTRYPOINT
    EmptyInstructionData,
}

impl EntrypointCase {
    pub const ALL: &[Self] = &[
        Self::InvalidDiscriminant,
        Self::RegisterMarketInvalidLength,
        Self::EmptyInstructionData,
    ];
}

impl TestCase for EntrypointCase {
    fn name(&self) -> &'static str {
        match self {
            Self::InvalidDiscriminant => "invalid_discriminant",
            Self::RegisterMarketInvalidLength => "register_market_invalid_length",
            Self::EmptyInstructionData => "empty_instruction_data",
        }
    }

    fn run(&self, setup: &TestSetup) -> CaseResult {
        let (data, expected) = match self {
            // Unknown discriminant.
            Self::InvalidDiscriminant => (
                vec![0xFF],
                Err(ProgramError::Custom(E_INVALID_DISCRIMINANT)),
            ),

            // RegisterMarket discriminant + 1 extra byte.
            Self::RegisterMarketInvalidLength => (
                vec![DISC_REGISTER_MARKET, 0x00],
                Err(ProgramError::Custom(E_INVALID_INSTRUCTION_LENGTH)),
            ),

            // Empty instruction data.
            Self::EmptyInstructionData => (
                vec![],
                Err(ProgramError::Custom(E_INVALID_DISCRIMINANT)),
            ),
        };

        let instruction = Instruction::new_with_bytes(setup.program_id, &data, vec![]);
        let result = setup.mollusk.process_instruction(&instruction, &[]);

        match (&expected, &result.program_result) {
            (Ok(()), MolluskResult::Success) => CaseResult {
                cu: result.compute_units_consumed,
                error: None,
            },
            (Err(expected_err), MolluskResult::Failure(actual_err))
                if actual_err == expected_err =>
            {
                CaseResult {
                    cu: result.compute_units_consumed,
                    error: None,
                }
            }
            _ => CaseResult {
                cu: result.compute_units_consumed,
                error: Some(format!(
                    "expected {:?}, got {:?}",
                    expected, result.program_result
                )),
            },
        }
    }
}
