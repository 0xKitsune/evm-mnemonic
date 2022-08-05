use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "evmm.pest"]
pub struct EVMMParser;

fn evmm_parse(unparsed_file: &str) {
    let parsed_file = parse_file(unparsed_file);

    for instruction in parsed_file.into_inner() {
        println!("{:?}", instruction);
        match instruction.as_rule() {
            Rule::number => {}
            Rule::hex_number => {}
            Rule::name => {}
            Rule::stop => {}
            Rule::add => {}
            Rule::mul => {}
            Rule::sub => {}
            Rule::div => {}
            Rule::sdiv => {}
            Rule::evmMod => {}
            Rule::smod => {}
            Rule::addmod => {}
            Rule::mulmod => {}
            Rule::exp => {}
            Rule::signextend => {}
            Rule::lt => {}
            Rule::gt => {}
            Rule::slt => {}
            Rule::sgt => {}
            Rule::eq => {}
            Rule::iszero => {}
            Rule::and => {}
            Rule::or => {}
            Rule::xor => {}
            Rule::not => {}
            Rule::byte => {}
            Rule::shl => {}
            Rule::shr => {}
            Rule::sar => {}
            Rule::sha3 => {}
            Rule::address => {}
            Rule::balance => {}
            Rule::origin => {}
            Rule::caller => {}
            Rule::callvalue => {}
            Rule::calldataload => {}
            Rule::calldatasize => {}
            Rule::calldatacopy => {}
            Rule::codesize => {}
            Rule::codecopy => {}
            Rule::gasprice => {}
            Rule::extcodesize => {}
            Rule::extcodecopy => {}
            Rule::returndatasize => {}
            Rule::returndatacopy => {}
            Rule::extcodehash => {}
            Rule::blockhash => {}
            Rule::coinbase => {}
            Rule::timestamp => {}
            Rule::blockNumber => {}
            Rule::difficulty => {}
            Rule::gaslimit => {}
            Rule::chainid => {}
            Rule::selfbalance => {}
            Rule::basefee => {}
            Rule::pop => {}
            Rule::mload => {}
            Rule::mstore => {}
            Rule::mstore8 => {}
            Rule::sload => {}
            Rule::sstore => {}
            Rule::jump => {}
            Rule::jumpi => {}
            Rule::pc => {}
            Rule::msize => {}
            Rule::gas => {}
            Rule::jumpdest => {}
            Rule::push1 => {}
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
            Rule::invalid => {}
            Rule::selfdestruct => {}
            _ => {}
        }
    }
}

fn parse_file(unparsed_file: &str) -> Pair<Rule> {
    EVMMParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap() // get and unwrap the `file` rule; never fails
}

#[test]
fn test_parse_file() {
    let file = r#"
    PUSH1 0x01 //[0x01]
    push5 0x0102030405 //[0x0102030405 0x01]
    CALLER //[CALLER 0x0102030405 0x01]
    "#;

    let parsed_file = parse_file(file);

    // println!("{:?}", parsed_file);

    evmm_parse(file);
}
