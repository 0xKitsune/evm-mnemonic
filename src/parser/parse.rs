use core::num::ParseIntError;
use core::panic;
use num256::uint256;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::iter::Peekable;
use std::str::FromStr;

use crate::compiler::compile::compile_instruction;

#[derive(Parser)]
#[grammar = "evmm.pest"]
pub struct EVMMParser;

fn evmm_parse(unparsed_file: &str) {
    let parsed_file = parse_file(unparsed_file);

    let instructions = parsed_file.into_inner().peekable();

    let contract_bytecode = parse_instructions(instructions, String::from(""));

    println!("{:?}", contract_bytecode);
}

fn parse_file(unparsed_file: &str) -> Pair<Rule> {
    EVMMParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap() // get and unwrap the `file` rule; never fails
}

//TODO: validate that pushx pushes the right byte size to the stack
//TODO: check for stack too deep errors
fn parse_instructions(
    mut peekable_instructions: Peekable<Pairs<Rule>>,
    mut contract_bytecode: String,
) -> String {
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
                | Rule::basefee => {
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));
                    stack_size += 1;
                }

                //Compile instructions that consume 1 stack value and push a value on the stack
                Rule::iszero | Rule::calldataload => {
                    if stack_size < 1 {
                        //TODO: error saying not enough values on stack
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 1 stack values and add 1 onto the stack, resulting in no change to the stack size
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
                        //TODO: error saying not enough values on stack
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 2 stack values and add 1 onto the stack, resulting in a reduction of the stack size by 1
                    stack_size -= 1;
                }

                //Compile instructions that consume 3 stack values and push a value on the stack
                Rule::addmod | Rule::mulmod | Rule::calldatacopy => {
                    if stack_size < 2 {
                        //TODO: error saying not enough values on stack
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 3 stack values and add 1 onto the stack, resulting in a reduction of the stack size by 2
                    stack_size -= 2;
                }

                //Compile instructions that consume 3 stack values and push no values on the stack
                Rule::returndatacopy => {
                    if stack_size < 2 {
                        //TODO: error saying not enough values on stack
                    }
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));

                    //consume 3 stack values and add 0 onto the stack, resulting in a reduction of the stack size by 3
                    stack_size -= 3;
                }

                //TODO:
                Rule::extcodecopy => {}
                Rule::extcodehash => {}
                Rule::pop => {}
                Rule::mload => {}
                Rule::mstore => {}
                Rule::mstore8 => {}
                Rule::sload => {}
                Rule::sstore => {}
                Rule::jump => {}
                Rule::jumpi => {}
                Rule::dup1 => {}
                Rule::dup2 => {}
                Rule::dup3 => {}
                Rule::dup4 => {}
                Rule::dup5 => {}
                Rule::dup6 => {}
                Rule::dup7 => {}
                Rule::dup8 => {}
                Rule::dup9 => {}
                Rule::dup10 => {}
                Rule::dup11 => {}
                Rule::dup12 => {}
                Rule::dup13 => {}
                Rule::dup14 => {}
                Rule::dup15 => {}
                Rule::dup16 => {}
                Rule::swap1 => {}
                Rule::swap2 => {}
                Rule::swap3 => {}
                Rule::swap4 => {}
                Rule::swap5 => {}
                Rule::swap6 => {}
                Rule::swap7 => {}
                Rule::swap8 => {}
                Rule::swap9 => {}
                Rule::swap10 => {}
                Rule::swap11 => {}
                Rule::swap12 => {}
                Rule::swap13 => {}
                Rule::swap14 => {}
                Rule::swap15 => {}
                Rule::swap16 => {}
                Rule::log0 => {}
                Rule::log1 => {}
                Rule::log2 => {}
                Rule::log3 => {}
                Rule::log4 => {}
                Rule::create => {}
                Rule::call => {}
                Rule::callcode => {}
                Rule::evmReturn => {}
                Rule::delegatecall => {}
                Rule::create2 => {}
                Rule::staticcall => {}
                Rule::revert => {}
                Rule::selfdestruct => {}

                //Validate size of the value following the instruction, compile instruction and add it to the contract bytecode
                Rule::push1 => {
                    contract_bytecode.push_str(&compile_instruction(instruction_as_rule));
                }
                Rule::push2 => {}
                Rule::push3 => {}
                Rule::push4 => {}
                Rule::push5 => {}
                Rule::push6 => {}
                Rule::push7 => {}
                Rule::push8 => {}
                Rule::push9 => {}
                Rule::push10 => {}
                Rule::push11 => {}
                Rule::push12 => {}
                Rule::push13 => {}
                Rule::push14 => {}
                Rule::push15 => {}
                Rule::push16 => {}
                Rule::push17 => {}
                Rule::push18 => {}
                Rule::push19 => {}
                Rule::push20 => {}
                Rule::push21 => {}
                Rule::push22 => {}
                Rule::push23 => {}
                Rule::push24 => {}
                Rule::push25 => {}
                Rule::push26 => {}
                Rule::push27 => {}
                Rule::push28 => {}
                Rule::push29 => {}
                Rule::push30 => {}
                Rule::push31 => {}
                Rule::push32 => {}

                //Add literals to the contract bytecode as a hex number
                Rule::number | Rule::hex_number => {
                    contract_bytecode
                        .push_str(&convert_to_hex_number_and_strip_prefix(instruction));
                }

                _ => {}
            }
        } else {
            break;
        }
    }

    return contract_bytecode;
}

fn convert_to_hex_number_and_strip_prefix(value: Pair<Rule>) -> String {
    match value.as_rule() {
        Rule::number => {
            let value_as_uint256 = uint256::Uint256::from_str(value.as_str()).unwrap();

            format!("{:X}", value_as_uint256)
        }

        Rule::hex_number => value.as_str()[2..].to_string(),
        _ => {
            panic!("Error when converting to hex number, unexpected rule");
        }
    }
}

///Gets the size of a number or hex number when represented as bytes
fn get_byte_size(instruction: Pair<Rule>) -> usize {
    match instruction.as_rule() {
        Rule::number => {
            let number_value = uint256::Uint256::from_str(instruction.as_str()).unwrap();
            let number_value_as_bytes = number_value.to_bytes_be();

            //return the length of bytes
            number_value_as_bytes.len()
        }
        Rule::hex_number => {
            let hex_number_value_as_bytes = decode_hex(instruction.as_str()).unwrap();

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

#[test]
fn test_evmm_parse() {
    let file = r#"
    PUSH1 0x01 //[0x01]
    push5 0x0102030405 //[0x0102030405 0x01]
    CALLER //[CALLER 0x0102030405 0x01]
    "#;

    evmm_parse(file);
}

#[test]
fn test_parse_file() {
    let file = r#"
    PUSH1 0x01 //[0x01]
    push5 0x0102030405 //[0x0102030405 0x01]
    CALLER //[CALLER 0x0102030405 0x01]
    "#;

    let parsed_file = parse_file(file);
}
