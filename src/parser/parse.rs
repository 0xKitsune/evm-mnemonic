use crate::compiler::compile::compile_instruction;
use crate::parser::parser_error::EVMMParserError;
use core::num::ParseIntError;
use core::panic;
use num256::uint256;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "evmm.pest"]
pub struct EVMMParser;

fn evmm_parse(file_name: String, unparsed_file: &str) -> Result<String, EVMMParserError> {
    let parsed_file = parse_file(file_name, unparsed_file);

    let instructions = parsed_file.into_inner().peekable();

    Ok(parse_instructions(instructions, String::from(""))?)
}

fn parse_file(file_name: String, unparsed_file: &str) -> Pair<Rule> {
    EVMMParser::parse(Rule::file, &unparsed_file)
        .expect(&format!("Error when parsing {:?}", file_name))
        .next()
        .unwrap()
}

fn parse_instructions(
    mut peekable_instructions: Peekable<Pairs<Rule>>,
    mut contract_bytecode: String,
) -> Result<String, EVMMParserError> {
    let mut stack_size: usize = 0;

    loop {
        let next_instruction = peekable_instructions.peek();

        if next_instruction.is_some() {
            let instruction = peekable_instructions.next().unwrap();

            let instruction_as_rule = instruction.as_rule();

            match instruction_as_rule {
                //Compile instructions that consume 0 stack values and do not push a value on the stack
                Rule::stop => {
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));
                }

                //Compile instructions that consume 0 stack values and push a value on the stack
                Rule::address
                | Rule::origin
                | Rule::caller
                | Rule::callvalue
                | Rule::calldatasize
                | Rule::codesize
                | Rule::returndatasize
                | Rule::pc
                | Rule::msize
                | Rule::gas
                | Rule::jumpdest
                | Rule::balance
                | Rule::gasprice
                | Rule::blockhash
                | Rule::coinbase
                | Rule::timestamp
                | Rule::codecopy
                | Rule::blockNumber
                | Rule::difficulty
                | Rule::gaslimit
                | Rule::chainid
                | Rule::selfbalance
                | Rule::extcodesize
                | Rule::basefee
                | Rule::extcodehash => {
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));
                    //consume 0 stack values and add a value onto the stack, resulting in a increase of the stack size by 1

                    stack_size += 1;
                }

                //Compile instructions that consume 1 stack value and push 0 values on the stack
                Rule::pop | Rule::jump | Rule::selfdestruct => {
                    if stack_size < 1 {
                        return Err(EVMMParserError::NotEnoughValuesOnStack(
                            instruction.as_str().to_owned(),
                            1,
                            stack_size,
                        ));
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 1 stack values and add nothing onto the stack, resulting in a reduction of the stack size by 1
                    stack_size -= 1;
                }

                //Compile instructions that consume 1 stack values and push a value on the stack
                Rule::iszero | Rule::calldataload | Rule::mload | Rule::sload => {
                    if stack_size < 1 {
                        return Err(EVMMParserError::NotEnoughValuesOnStack(
                            instruction.as_str().to_owned(),
                            1,
                            stack_size,
                        ));
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 1 stack values and add 1 onto the stack, resulting in no change to the stack size
                }

                //Compile instructions that consume 2 stack values and push 0 values on the stack
                Rule::mstore
                | Rule::mstore8
                | Rule::sstore
                | Rule::jumpi
                | Rule::log0
                | Rule::evmReturn
                | Rule::revert => {
                    if stack_size < 2 {
                        return Err(EVMMParserError::NotEnoughValuesOnStack(
                            instruction.as_str().to_owned(),
                            2,
                            stack_size,
                        ));
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    stack_size -= 2;
                }

                //Compile instructions that consume 2 stack values and push a value on the stack
                Rule::add
                | Rule::mul
                | Rule::sub
                | Rule::div
                | Rule::sdiv
                | Rule::evmMod
                | Rule::smod
                | Rule::lt
                | Rule::gt
                | Rule::slt
                | Rule::sgt
                | Rule::eq
                | Rule::and
                | Rule::or
                | Rule::xor
                | Rule::not
                | Rule::byte
                | Rule::shl
                | Rule::shr
                | Rule::sar
                | Rule::keccak256
                | Rule::exp
                | Rule::signextend => {
                    if stack_size < 2 {
                        return Err(EVMMParserError::NotEnoughValuesOnStack(
                            instruction.as_str().to_owned(),
                            2,
                            stack_size,
                        ));
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 2 stack values and add 1 onto the stack, resulting in a reduction of the stack size by 1
                    stack_size -= 1;
                }

                //Compile instructions that consume 3 stack values and push 0 values on the stack
                Rule::returndatacopy | Rule::log1 => {
                    if stack_size < 3 {
                        return Err(EVMMParserError::NotEnoughValuesOnStack(
                            instruction.as_str().to_owned(),
                            3,
                            stack_size,
                        ));
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 3 stack values and add 0 onto the stack, resulting in a reduction of the stack size by 3
                    stack_size -= 3;
                }

                //Compile instructions that consume 3 stack values and push 1 value on the stack
                Rule::addmod | Rule::mulmod | Rule::calldatacopy | Rule::create => {
                    if stack_size < 3 {
                        return Err(EVMMParserError::NotEnoughValuesOnStack(
                            instruction.as_str().to_owned(),
                            3,
                            stack_size,
                        ));
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 3 stack values and add 1 onto the stack, resulting in a reduction of the stack size by 2
                    stack_size -= 2;
                }

                //Compile instructions that consume 4 stack values and push 0 values on the stack
                Rule::log2 => {
                    if stack_size < 4 {
                        return Err(EVMMParserError::NotEnoughValuesOnStack(
                            instruction.as_str().to_owned(),
                            4,
                            stack_size,
                        ));
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 4 stack values and add 0 onto the stack, resulting in a reduction of the stack size by 4
                    stack_size -= 4;
                }

                //Compile instructions that consume 4 stack values and push a value on the stack
                Rule::extcodecopy | Rule::create2 => {
                    if stack_size < 4 {
                        return Err(EVMMParserError::NotEnoughValuesOnStack(
                            instruction.as_str().to_owned(),
                            4,
                            stack_size,
                        ));
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 4 stack values and add 1 onto the stack, resulting in a reduction of the stack size by 3
                    stack_size -= 3;
                }

                //Compile instructions that consume 5 stack values and push 0 values on the stack
                Rule::log3 => {
                    if stack_size < 5 {
                        return Err(EVMMParserError::NotEnoughValuesOnStack(
                            instruction.as_str().to_owned(),
                            5,
                            stack_size,
                        ));
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 4 stack values and add 0 onto the stack, resulting in a reduction of the stack size by 4
                    stack_size -= 5;
                }

                //Compile instructions that consume 6 stack values and push 0 values on the stack
                Rule::log4 => {
                    if stack_size < 6 {
                        return Err(EVMMParserError::NotEnoughValuesOnStack(
                            instruction.as_str().to_owned(),
                            6,
                            stack_size,
                        ));
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 6 stack values and add 0 onto the stack, resulting in a reduction of the stack size by 4
                    stack_size -= 6;
                }

                //Compile instructions that consume 6 stack values and push 1 value on the stack
                Rule::delegatecall | Rule::staticcall => {
                    if stack_size < 6 {
                        return Err(EVMMParserError::NotEnoughValuesOnStack(
                            instruction.as_str().to_owned(),
                            6,
                            stack_size,
                        ));
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 6 stack values and add 1 onto the stack, resulting in a reduction of the stack size by 4
                    stack_size -= 5;
                }

                //Compile instructions that consume 7 stack values and push 1 value on the stack
                Rule::call | Rule::callcode => {
                    if stack_size < 7 {
                        return Err(EVMMParserError::NotEnoughValuesOnStack(
                            instruction.as_str().to_owned(),
                            7,
                            stack_size,
                        ));
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 7 stack values and add 1 onto the stack, resulting in a reduction of the stack size by 6
                    stack_size -= 6;
                }

                //Compile dup instructions that duplicate a stack value at a specific position, pushing the duplicate to the top of the stack
                //These instructions consume 0 values and add a value to the stack
                Rule::dup1
                | Rule::dup2
                | Rule::dup3
                | Rule::dup4
                | Rule::dup5
                | Rule::dup6
                | Rule::dup7
                | Rule::dup8
                | Rule::dup9
                | Rule::dup10
                | Rule::dup11
                | Rule::dup12
                | Rule::dup13
                | Rule::dup14
                | Rule::dup15
                | Rule::dup16 => {
                    let expected_stack_size =
                        instruction.as_str().split_at(3).1.parse::<usize>().unwrap();

                    if stack_size < expected_stack_size {
                        return Err(EVMMParserError::NotEnoughValuesOnStack(
                            instruction.as_str().to_owned(),
                            expected_stack_size,
                            stack_size,
                        ));
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    stack_size += 1;
                }

                //Compile swap instructions that swap the top position with a specified position on the stack
                //No stack values are consumed or added
                Rule::swap1
                | Rule::swap2
                | Rule::swap3
                | Rule::swap4
                | Rule::swap5
                | Rule::swap6
                | Rule::swap7
                | Rule::swap8
                | Rule::swap9
                | Rule::swap10
                | Rule::swap11
                | Rule::swap12
                | Rule::swap13
                | Rule::swap14
                | Rule::swap15
                | Rule::swap16 => {
                    let expected_stack_size =
                        instruction.as_str().split_at(3).1.parse::<usize>().unwrap();

                    if stack_size < expected_stack_size {
                        return Err(EVMMParserError::NotEnoughValuesOnStack(
                            instruction.as_str().to_owned(),
                            expected_stack_size,
                            stack_size,
                        ));
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));
                }

                //Validate size of the value following the instruction, compile instruction and add it to the contract bytecode
                Rule::push1
                | Rule::push2
                | Rule::push3
                | Rule::push4
                | Rule::push5
                | Rule::push6
                | Rule::push7
                | Rule::push8
                | Rule::push9
                | Rule::push10
                | Rule::push11
                | Rule::push12
                | Rule::push13
                | Rule::push14
                | Rule::push15
                | Rule::push16
                | Rule::push17
                | Rule::push18
                | Rule::push19
                | Rule::push20
                | Rule::push21
                | Rule::push22
                | Rule::push23
                | Rule::push24
                | Rule::push25
                | Rule::push26
                | Rule::push27
                | Rule::push28
                | Rule::push29
                | Rule::push30
                | Rule::push31
                | Rule::push32 => {
                    //Split the instruction as a string so the result is ("push", expected_size), then parse the expected size as a usize

                    let expected_size =
                        instruction.as_str().split_at(4).1.parse::<usize>().unwrap();

                    //validate the value to be pushed, apply padding and return the compiled value to be added to the contract bytecode
                    let compiled_push_value = validate_proceeding_push_instruction(
                        &instruction,
                        peekable_instructions.peek(),
                        expected_size,
                    )?;

                    //add the push instruction to the bytecode
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //add the padded value to the  bytecode
                    contract_bytecode.push_str(&compiled_push_value);
                }

                _ => {}
            }
        } else {
            break;
        }
    }

    return Ok(contract_bytecode);
}

///Validate the size of a value proceeding a push instruction, returns the push value to be compiled and the size of padding to apply to the value
fn validate_proceeding_push_instruction(
    push_instruction: &Pair<Rule>,
    optional_next_instruction: Option<&Pair<Rule>>,
    expected_size: usize,
) -> Result<String, EVMMParserError> {
    if optional_next_instruction.is_some() {
        let next_instruction = optional_next_instruction.unwrap();

        match next_instruction.as_rule() {
            Rule::number | Rule::hex_number => {
                let value_byte_size = get_byte_size(next_instruction);

                if value_byte_size > expected_size {
                    return Err(EVMMParserError::ValueTooBigForPushInstruction(
                        push_instruction.as_str().to_owned(),
                        next_instruction.as_str().to_owned(),
                        value_byte_size,
                    ));
                } else if value_byte_size == expected_size {
                    Ok(convert_to_hex_number_and_strip_prefix(next_instruction))
                } else {
                    let compiled_push_value =
                        convert_to_hex_number_and_strip_prefix(next_instruction);

                    //pad the value and return the compiled value to be added to the contract code
                    Ok([
                        "00".repeat(expected_size - value_byte_size),
                        compiled_push_value,
                    ]
                    .join(""))
                }
            }

            _ => {
                return Err(EVMMParserError::UnexpectedInstruction(
                    next_instruction.as_str().to_owned(),
                ));
            }
        }
    } else {
        return Err(EVMMParserError::ExpectedInstruction());
    }
}

fn convert_to_hex_number_and_strip_prefix(value: &Pair<Rule>) -> String {
    match value.as_rule() {
        Rule::number => {
            let value_as_uint256 = uint256::Uint256::from_str(value.as_str()).unwrap();

            format!("{:X}", value_as_uint256)
        }

        Rule::hex_number => value.as_str()[2..].to_string(),
        _ => {
            panic!(
                "Error when converting to hex number, unexpected rule: {:?}",
                value.as_rule()
            );
        }
    }
}

///Gets the size of a number or hex number when represented as bytes
fn get_byte_size(instruction: &Pair<Rule>) -> usize {
    match instruction.as_rule() {
        Rule::number => {
            let number_value = uint256::Uint256::from_str(instruction.as_str()).unwrap();
            let number_value_as_bytes = number_value.to_bytes_be();

            //return the length of bytes
            number_value_as_bytes.len()
        }
        Rule::hex_number => {
            let hex_number_value_as_bytes =
                decode_hex(&convert_to_hex_number_and_strip_prefix(instruction)).unwrap();

            //return the length of bytes
            hex_number_value_as_bytes.len()
        }
        _ => {
            panic!("Something went wrong, a non number or hex number Pair<Rule> was passed into get_byte_size")
        }
    }
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evmm_parse() {
        let file = r#"
        PUSH1 0x01 //[0x01]
        push5 0x0102030405 //[0x0102030405 0x01]
        CALLER //[CALLER 0x0102030405 0x01]
        "#;

        evmm_parse(String::from("test_case"), file).unwrap();
    }

    #[test]
    fn test_parse_file() {
        let file = r#"
    PUSH1 0x01 //[0x01]
    push5 0x0102030405 //[0x0102030405 0x01]
    CALLER //[CALLER 0x0102030405 0x01]
    "#;

        let parsed_file = parse_file(String::from("test_case"), file);

        println!("{:?}", parsed_file);
    }

    #[test]
    fn test_push1() {
        let file = r#"
    PUSH1 0x01 
    "#;

        let parsed_file = parse_file(String::from("test_case"), file);

        println!("{:?}", parsed_file);
    }

    #[test]
    fn test_fail_push1() {
        let file = r#"
    PUSH10 0x0102
    "#;

        let result = evmm_parse(String::from("test_case"), file);

        match result {
            Ok(compiled_bytecode) => {
                println!("{:?}", compiled_bytecode);
            }

            Err(err) => {
                println!("{:?}", err.to_string())
            }
        }
    }
}
