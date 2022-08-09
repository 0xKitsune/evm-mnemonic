use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub enum EVMMError {
    NotEnoughValuesOnStack(String, usize, usize),
    ValueTooBigForPushInstruction(String, String, usize),
    UnexpectedInstruction(String),
    ExpectedInstruction(),
}

impl std::error::Error for EVMMError {}

impl fmt::Display for EVMMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EVMMError::NotEnoughValuesOnStack(
                instruction,
                stack_size_required,
                current_stack_size,
            ) => {
                write!(f, "Not enough values on the stack. Instruction: {:?}. Stack size: {:?}. Stack Size Needed: {:?}", instruction, current_stack_size, stack_size_required)
            }

            EVMMError::ValueTooBigForPushInstruction(instruction, value, value_size) => {
                write!(
                    f,
                    "Value is too large for PUSH instruction. Instruction: {:?}. Value: {:?}. Value byte size: {:?}",
                    instruction, value, value_size
                )
            }

            EVMMError::UnexpectedInstruction(instruction) => {
                write!(f, "Unexpected instruction: {:?}", instruction)
            }

            EVMMError::ExpectedInstruction() => {
                write!(f, "Expected instruction but none was found")
            }
        }
    }
}
