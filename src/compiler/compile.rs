use crate::parser::parse::Rule;
use core::panic;

pub fn compile_instruction(instruction: Rule) -> String {
    match instruction {
        Rule::stop => return String::from("00"),
        Rule::add => return String::from("01"),
        Rule::mul => return String::from("02"),
        Rule::sub => return String::from("03"),
        Rule::div => return String::from("04"),
        Rule::sdiv => return String::from("05"),
        Rule::evmMod => return String::from("06"),
        Rule::smod => return String::from("07"),
        Rule::addmod => return String::from("08"),
        Rule::mulmod => return String::from("09"),
        Rule::exp => return String::from("0A"),
        Rule::signextend => return String::from("0B"),
        Rule::lt => return String::from("10"),
        Rule::gt => return String::from("11"),
        Rule::slt => return String::from("12"),
        Rule::sgt => return String::from("13"),
        Rule::eq => return String::from("14"),
        Rule::iszero => return String::from("15"),
        Rule::and => return String::from("16"),
        Rule::or => return String::from("17"),
        Rule::xor => return String::from("18"),
        Rule::not => return String::from("19"),
        Rule::byte => return String::from("1A"),
        Rule::shl => return String::from("1B"),
        Rule::shr => return String::from("1C"),
        Rule::sar => return String::from("1D"),
        Rule::keccak256 => return String::from("20"),
        Rule::address => return String::from("30"),
        Rule::balance => return String::from("31"),
        Rule::origin => return String::from("32"),
        Rule::caller => return String::from("33"),
        Rule::callvalue => return String::from("34"),
        Rule::calldataload => return String::from("35"),
        Rule::calldatasize => return String::from("36"),
        Rule::calldatacopy => return String::from("37"),
        Rule::codesize => return String::from("38"),
        Rule::codecopy => return String::from("39"),
        Rule::gasprice => return String::from("3A"),
        Rule::extcodesize => return String::from("3B"),
        Rule::extcodecopy => return String::from("3C"),
        Rule::returndatasize => return String::from("3D"),
        Rule::returndatacopy => return String::from("3E"),
        Rule::extcodehash => return String::from("3F"),
        Rule::blockhash => return String::from("40"),
        Rule::coinbase => return String::from("41"),
        Rule::timestamp => return String::from("42"),
        Rule::blockNumber => return String::from("43"),
        Rule::difficulty => return String::from("44"),
        Rule::gaslimit => return String::from("45"),
        Rule::chainid => return String::from("46"),
        Rule::selfbalance => return String::from("67"),
        Rule::basefee => return String::from("48"),
        Rule::pop => return String::from("49"),
        Rule::mload => return String::from("50"),
        Rule::mstore => return String::from("51"),
        Rule::mstore8 => return String::from("52"),
        Rule::sload => return String::from("53"),
        Rule::sstore => return String::from("54"),
        Rule::jump => return String::from("55"),
        Rule::jumpi => return String::from("56"),
        Rule::pc => return String::from("57"),
        Rule::msize => return String::from("58"),
        Rule::gas => return String::from("59"),
        Rule::jumpdest => return String::from("5A"),
        Rule::push1 => return String::from("5B"),
        Rule::push2 => return String::from("60"),
        Rule::push3 => return String::from("61"),
        Rule::push4 => return String::from("62"),
        Rule::push5 => return String::from("63"),
        Rule::push6 => return String::from("64"),
        Rule::push7 => return String::from("65"),
        Rule::push8 => return String::from("66"),
        Rule::push9 => return String::from("67"),
        Rule::push10 => return String::from("68"),
        Rule::push11 => return String::from("69"),
        Rule::push12 => return String::from("6A"),
        Rule::push13 => return String::from("6B"),
        Rule::push14 => return String::from("6C"),
        Rule::push15 => return String::from("6D"),
        Rule::push16 => return String::from("6E"),
        Rule::push17 => return String::from("6F"),
        Rule::push18 => return String::from("70"),
        Rule::push19 => return String::from("71"),
        Rule::push20 => return String::from("72"),
        Rule::push21 => return String::from("73"),
        Rule::push22 => return String::from("74"),
        Rule::push23 => return String::from("75"),
        Rule::push24 => return String::from("76"),
        Rule::push25 => return String::from("77"),
        Rule::push26 => return String::from("78"),
        Rule::push27 => return String::from("79"),
        Rule::push28 => return String::from("7A"),
        Rule::push29 => return String::from("7B"),
        Rule::push30 => return String::from("7D"),
        Rule::push31 => return String::from("7E"),
        Rule::push32 => return String::from("7F"),
        Rule::dup1 => return String::from("80"),
        Rule::dup2 => return String::from("81"),
        Rule::dup3 => return String::from("82"),
        Rule::dup4 => return String::from("83"),
        Rule::dup5 => return String::from("84"),
        Rule::dup6 => return String::from("85"),
        Rule::dup7 => return String::from("86"),
        Rule::dup8 => return String::from("87"),
        Rule::dup9 => return String::from("88"),
        Rule::dup10 => return String::from("89"),
        Rule::dup11 => return String::from("8A"),
        Rule::dup12 => return String::from("8B"),
        Rule::dup13 => return String::from("8C"),
        Rule::dup14 => return String::from("8D"),
        Rule::dup15 => return String::from("8E"),
        Rule::dup16 => return String::from("8F"),
        Rule::swap1 => return String::from("90"),
        Rule::swap2 => return String::from("91"),
        Rule::swap3 => return String::from("92"),
        Rule::swap4 => return String::from("93"),
        Rule::swap5 => return String::from("94"),
        Rule::swap6 => return String::from("95"),
        Rule::swap7 => return String::from("96"),
        Rule::swap8 => return String::from("97"),
        Rule::swap9 => return String::from("98"),
        Rule::swap10 => return String::from("99"),
        Rule::swap11 => return String::from("9A"),
        Rule::swap12 => return String::from("9B"),
        Rule::swap13 => return String::from("9C"),
        Rule::swap14 => return String::from("9D"),
        Rule::swap15 => return String::from("9E"),
        Rule::swap16 => return String::from("9F"),
        Rule::log0 => return String::from("A0"),
        Rule::log1 => return String::from("A1"),
        Rule::log2 => return String::from("A2"),
        Rule::log3 => return String::from("A3"),
        Rule::log4 => return String::from("A4"),
        Rule::create => return String::from("F0"),
        Rule::call => return String::from("F1"),
        Rule::callcode => return String::from("F2"),
        Rule::evmReturn => return String::from("F3"),
        Rule::delegatecall => return String::from("F4"),
        Rule::create2 => return String::from("F5"),
        Rule::staticcall => return String::from("FA"),
        Rule::revert => return String::from("FD"),
        Rule::selfdestruct => return String::from("FF"),

        _ => {
            panic!("Something went wrong when compiling instruction, unexpected instruction");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_stop() {
        let bytecode = compile_instruction(Rule::stop);
        assert_eq!(bytecode, "00");
    }

    #[test]
    fn test_compile_add() {
        let bytecode = compile_instruction(Rule::add);
        assert_eq!(bytecode, "01");
    }

    #[test]
    fn test_compile_mul() {
        let bytecode = compile_instruction(Rule::mul);
        assert_eq!(bytecode, "02");
    }

    #[test]
    fn test_compile_sub() {
        let bytecode = compile_instruction(Rule::sub);
        assert_eq!(bytecode, "03");
    }

    #[test]
    fn test_compile_div() {
        let bytecode = compile_instruction(Rule::div);
        assert_eq!(bytecode, "04");
    }

    #[test]
    fn test_compile_sdiv() {
        let bytecode = compile_instruction(Rule::sdiv);
        assert_eq!(bytecode, "05");
    }

    #[test]
    fn test_compile_mod() {
        let bytecode = compile_instruction(Rule::evmMod);
        assert_eq!(bytecode, "06");
    }

    #[test]
    fn test_compile_smod() {
        let bytecode = compile_instruction(Rule::smod);
        assert_eq!(bytecode, "07");
    }

    #[test]
    fn test_compile_addmod() {
        let bytecode = compile_instruction(Rule::addmod);
        assert_eq!(bytecode, "08");
    }

    #[test]
    fn test_compile_mulmod() {
        let bytecode = compile_instruction(Rule::mulmod);
        assert_eq!(bytecode, "09");
    }

    #[test]
    fn test_compile_exp() {
        let bytecode = compile_instruction(Rule::exp);
        assert_eq!(bytecode, "0A");
    }

    #[test]
    fn test_compile_signextend() {
        let bytecode = compile_instruction(Rule::signextend);
        assert_eq!(bytecode, "0B");
    }

    #[test]
    fn test_compile_lt() {
        let bytecode = compile_instruction(Rule::lt);
        assert_eq!(bytecode, "10");
    }

    #[test]
    fn test_compile_gt() {
        let bytecode = compile_instruction(Rule::gt);
        assert_eq!(bytecode, "11");
    }

    #[test]
    fn test_compile_slt() {
        let bytecode = compile_instruction(Rule::slt);
        assert_eq!(bytecode, "12");
    }

    #[test]
    fn test_compile_sgt() {
        let bytecode = compile_instruction(Rule::sgt);
        assert_eq!(bytecode, "13");
    }

    #[test]
    fn test_compile_eq() {
        let bytecode = compile_instruction(Rule::eq);
        assert_eq!(bytecode, "14");
    }

    #[test]
    fn test_compile_iszero() {
        let bytecode = compile_instruction(Rule::iszero);
        assert_eq!(bytecode, "15");
    }

    #[test]
    fn test_compile_and() {
        let bytecode = compile_instruction(Rule::and);
        assert_eq!(bytecode, "16");
    }

    #[test]
    fn test_compile_or() {
        let bytecode = compile_instruction(Rule::or);
        assert_eq!(bytecode, "17");
    }

    #[test]
    fn test_compile_xor() {
        let bytecode = compile_instruction(Rule::xor);
        assert_eq!(bytecode, "18");
    }

    #[test]
    fn test_compile_not() {
        let bytecode = compile_instruction(Rule::not);
        assert_eq!(bytecode, "19");
    }

    #[test]
    fn test_compile_byte() {
        let bytecode = compile_instruction(Rule::byte);
        assert_eq!(bytecode, "1A");
    }

    #[test]
    fn test_compile_shl() {
        let bytecode = compile_instruction(Rule::shl);
        assert_eq!(bytecode, "1B");
    }

    #[test]
    fn test_compile_shr() {
        let bytecode = compile_instruction(Rule::shr);
        assert_eq!(bytecode, "1C");
    }

    #[test]
    fn test_compile_sar() {
        let bytecode = compile_instruction(Rule::sar);
        assert_eq!(bytecode, "1D");
    }

    #[test]
    fn test_compile_sha3() {
        let bytecode = compile_instruction(Rule::keccak256);
        assert_eq!(bytecode, "20");
    }

    #[test]
    fn test_compile_address() {
        let bytecode = compile_instruction(Rule::address);
        assert_eq!(bytecode, "30");
    }

    #[test]
    fn test_compile_balance() {
        let bytecode = compile_instruction(Rule::balance);
        assert_eq!(bytecode, "31");
    }

    #[test]
    fn test_compile_origin() {
        let bytecode = compile_instruction(Rule::origin);
        assert_eq!(bytecode, "32");
    }

    #[test]
    fn test_compile_caller() {
        let bytecode = compile_instruction(Rule::caller);
        assert_eq!(bytecode, "33");
    }

    #[test]
    fn test_compile_callvalue() {
        let bytecode = compile_instruction(Rule::callvalue);
        assert_eq!(bytecode, "34");
    }

    #[test]
    fn test_compile_calldataload() {
        let bytecode = compile_instruction(Rule::calldataload);
        assert_eq!(bytecode, "35");
    }

    #[test]
    fn test_compile_calldatasize() {
        let bytecode = compile_instruction(Rule::calldatasize);
        assert_eq!(bytecode, "36");
    }

    #[test]
    fn test_compile_calldatacopy() {
        let bytecode = compile_instruction(Rule::calldatacopy);
        assert_eq!(bytecode, "37");
    }

    #[test]
    fn test_compile_codesize() {
        let bytecode = compile_instruction(Rule::codesize);
        assert_eq!(bytecode, "38");
    }

    #[test]
    fn test_compile_codecopy() {
        let bytecode = compile_instruction(Rule::codecopy);
        assert_eq!(bytecode, "39");
    }

    #[test]
    fn test_compile_gasprice() {
        let bytecode = compile_instruction(Rule::gasprice);
        assert_eq!(bytecode, "3A");
    }

    #[test]
    fn test_compile_extcodesize() {
        let bytecode = compile_instruction(Rule::extcodesize);
        assert_eq!(bytecode, "3B");
    }

    #[test]
    fn test_compile_extcodecopy() {
        let bytecode = compile_instruction(Rule::extcodecopy);
        assert_eq!(bytecode, "3C");
    }

    #[test]
    fn test_compile_returndatasize() {
        let bytecode = compile_instruction(Rule::returndatasize);
        assert_eq!(bytecode, "3D");
    }

    #[test]
    fn test_compile_returndatacopy() {
        let bytecode = compile_instruction(Rule::returndatacopy);
        assert_eq!(bytecode, "3E");
    }

    #[test]
    fn test_compile_extcodehash() {
        let bytecode = compile_instruction(Rule::extcodehash);
        assert_eq!(bytecode, "3F");
    }

    #[test]
    fn test_compile_blockhash() {
        let bytecode = compile_instruction(Rule::blockhash);
        assert_eq!(bytecode, "40");
    }

    #[test]
    fn test_compile_coinbase() {
        let bytecode = compile_instruction(Rule::coinbase);
        assert_eq!(bytecode, "41");
    }

    #[test]
    fn test_compile_timestamp() {
        let bytecode = compile_instruction(Rule::timestamp);
        assert_eq!(bytecode, "42");
    }

    #[test]
    fn test_compile_number() {
        let bytecode = compile_instruction(Rule::blockNumber);
        assert_eq!(bytecode, "43");
    }

    #[test]
    fn test_compile_difficulty() {
        let bytecode = compile_instruction(Rule::difficulty);
        assert_eq!(bytecode, "44");
    }

    #[test]
    fn test_compile_gaslimit() {
        let bytecode = compile_instruction(Rule::gaslimit);
        assert_eq!(bytecode, "45");
    }

    #[test]
    fn test_compile_chainid() {
        let bytecode = compile_instruction(Rule::chainid);
        assert_eq!(bytecode, "46");
    }

    #[test]
    fn test_compile_selfbalance() {
        let bytecode = compile_instruction(Rule::selfbalance);
        assert_eq!(bytecode, "67");
    }

    #[test]
    fn test_compile_basefee() {
        let bytecode = compile_instruction(Rule::basefee);
        assert_eq!(bytecode, "48");
    }

    #[test]
    fn test_compile_pop() {
        let bytecode = compile_instruction(Rule::pop);
        assert_eq!(bytecode, "49");
    }

    #[test]
    fn test_compile_mload() {
        let bytecode = compile_instruction(Rule::mload);
        assert_eq!(bytecode, "50");
    }

    #[test]
    fn test_compile_mstore() {
        let bytecode = compile_instruction(Rule::mstore);
        assert_eq!(bytecode, "51");
    }

    #[test]
    fn test_compile_mstore8() {
        let bytecode = compile_instruction(Rule::mstore8);
        assert_eq!(bytecode, "52");
    }

    #[test]
    fn test_compile_sload() {
        let bytecode = compile_instruction(Rule::sload);
        assert_eq!(bytecode, "53");
    }

    #[test]
    fn test_compile_sstore() {
        let bytecode = compile_instruction(Rule::sstore);
        assert_eq!(bytecode, "54");
    }

    #[test]
    fn test_compile_jump() {
        let bytecode = compile_instruction(Rule::jump);
        assert_eq!(bytecode, "55");
    }

    #[test]
    fn test_compile_jumpi() {
        let bytecode = compile_instruction(Rule::jumpi);
        assert_eq!(bytecode, "56");
    }

    #[test]
    fn test_compile_pc() {
        let bytecode = compile_instruction(Rule::pc);
        assert_eq!(bytecode, "57");
    }

    #[test]
    fn test_compile_msize() {
        let bytecode = compile_instruction(Rule::msize);
        assert_eq!(bytecode, "58");
    }

    #[test]
    fn test_compile_gas() {
        let bytecode = compile_instruction(Rule::gas);
        assert_eq!(bytecode, "59");
    }

    #[test]
    fn test_compile_jumpdest() {
        let bytecode = compile_instruction(Rule::jumpdest);
        assert_eq!(bytecode, "5A");
    }

    #[test]
    fn test_compile_push1() {
        let bytecode = compile_instruction(Rule::push1);
        assert_eq!(bytecode, "5B");
    }

    #[test]
    fn test_compile_push2() {
        let bytecode = compile_instruction(Rule::push2);
        assert_eq!(bytecode, "60");
    }

    #[test]
    fn test_compile_push3() {
        let bytecode = compile_instruction(Rule::push3);
        assert_eq!(bytecode, "61");
    }

    #[test]
    fn test_compile_push4() {
        let bytecode = compile_instruction(Rule::push4);
        assert_eq!(bytecode, "62");
    }

    #[test]
    fn test_compile_push5() {
        let bytecode = compile_instruction(Rule::push5);
        assert_eq!(bytecode, "63");
    }

    #[test]
    fn test_compile_push6() {
        let bytecode = compile_instruction(Rule::push6);
        assert_eq!(bytecode, "64");
    }

    #[test]
    fn test_compile_push7() {
        let bytecode = compile_instruction(Rule::push7);
        assert_eq!(bytecode, "65");
    }

    #[test]
    fn test_compile_push8() {
        let bytecode = compile_instruction(Rule::push8);
        assert_eq!(bytecode, "66");
    }

    #[test]
    fn test_compile_push9() {
        let bytecode = compile_instruction(Rule::push9);
        assert_eq!(bytecode, "67");
    }

    #[test]
    fn test_compile_push10() {
        let bytecode = compile_instruction(Rule::push10);
        assert_eq!(bytecode, "68");
    }

    #[test]
    fn test_compile_push11() {
        let bytecode = compile_instruction(Rule::push11);
        assert_eq!(bytecode, "69");
    }

    #[test]
    fn test_compile_push12() {
        let bytecode = compile_instruction(Rule::push12);
        assert_eq!(bytecode, "6A");
    }

    #[test]
    fn test_compile_push13() {
        let bytecode = compile_instruction(Rule::push13);
        assert_eq!(bytecode, "6B");
    }

    #[test]
    fn test_compile_push14() {
        let bytecode = compile_instruction(Rule::push14);
        assert_eq!(bytecode, "6C");
    }

    #[test]
    fn test_compile_push15() {
        let bytecode = compile_instruction(Rule::push15);
        assert_eq!(bytecode, "6D");
    }

    #[test]
    fn test_compile_push16() {
        let bytecode = compile_instruction(Rule::push16);
        assert_eq!(bytecode, "6E");
    }

    #[test]
    fn test_compile_push17() {
        let bytecode = compile_instruction(Rule::push17);
        assert_eq!(bytecode, "6F");
    }

    #[test]
    fn test_compile_push18() {
        let bytecode = compile_instruction(Rule::push18);
        assert_eq!(bytecode, "70");
    }

    #[test]
    fn test_compile_push19() {
        let bytecode = compile_instruction(Rule::push19);
        assert_eq!(bytecode, "71");
    }

    #[test]
    fn test_compile_push20() {
        let bytecode = compile_instruction(Rule::push20);
        assert_eq!(bytecode, "72");
    }

    #[test]
    fn test_compile_push21() {
        let bytecode = compile_instruction(Rule::push21);
        assert_eq!(bytecode, "73");
    }

    #[test]
    fn test_compile_push22() {
        let bytecode = compile_instruction(Rule::push22);
        assert_eq!(bytecode, "74");
    }

    #[test]
    fn test_compile_push23() {
        let bytecode = compile_instruction(Rule::push23);
        assert_eq!(bytecode, "75");
    }

    #[test]
    fn test_compile_push24() {
        let bytecode = compile_instruction(Rule::push24);
        assert_eq!(bytecode, "76");
    }

    #[test]
    fn test_compile_push25() {
        let bytecode = compile_instruction(Rule::push25);
        assert_eq!(bytecode, "77");
    }

    #[test]
    fn test_compile_push26() {
        let bytecode = compile_instruction(Rule::push26);
        assert_eq!(bytecode, "78");
    }

    #[test]
    fn test_compile_push27() {
        let bytecode = compile_instruction(Rule::push27);
        assert_eq!(bytecode, "79");
    }

    #[test]
    fn test_compile_push28() {
        let bytecode = compile_instruction(Rule::push28);
        assert_eq!(bytecode, "7A");
    }

    #[test]
    fn test_compile_push29() {
        let bytecode = compile_instruction(Rule::push29);
        assert_eq!(bytecode, "7B");
    }

    #[test]
    fn test_compile_push30() {
        let bytecode = compile_instruction(Rule::push30);
        assert_eq!(bytecode, "7D");
    }

    #[test]
    fn test_compile_push31() {
        let bytecode = compile_instruction(Rule::push31);
        assert_eq!(bytecode, "7E");
    }

    #[test]
    fn test_compile_push32() {
        let bytecode = compile_instruction(Rule::push32);
        assert_eq!(bytecode, "7F");
    }

    #[test]
    fn test_compile_dup1() {
        let bytecode = compile_instruction(Rule::dup1);
        assert_eq!(bytecode, "80");
    }

    #[test]
    fn test_compile_dup2() {
        let bytecode = compile_instruction(Rule::dup2);
        assert_eq!(bytecode, "81");
    }

    #[test]
    fn test_compile_dup3() {
        let bytecode = compile_instruction(Rule::dup3);
        assert_eq!(bytecode, "82");
    }

    #[test]
    fn test_compile_dup4() {
        let bytecode = compile_instruction(Rule::dup4);
        assert_eq!(bytecode, "83");
    }

    #[test]
    fn test_compile_dup5() {
        let bytecode = compile_instruction(Rule::dup5);
        assert_eq!(bytecode, "84");
    }

    #[test]
    fn test_compile_dup6() {
        let bytecode = compile_instruction(Rule::dup6);
        assert_eq!(bytecode, "85");
    }

    #[test]
    fn test_compile_dup7() {
        let bytecode = compile_instruction(Rule::dup7);
        assert_eq!(bytecode, "86");
    }

    #[test]
    fn test_compile_dup8() {
        let bytecode = compile_instruction(Rule::dup8);
        assert_eq!(bytecode, "87");
    }

    #[test]
    fn test_compile_dup9() {
        let bytecode = compile_instruction(Rule::dup9);
        assert_eq!(bytecode, "88");
    }

    #[test]
    fn test_compile_dup10() {
        let bytecode = compile_instruction(Rule::dup10);
        assert_eq!(bytecode, "89");
    }

    #[test]
    fn test_compile_dup11() {
        let bytecode = compile_instruction(Rule::dup11);
        assert_eq!(bytecode, "8A");
    }

    #[test]
    fn test_compile_dup12() {
        let bytecode = compile_instruction(Rule::dup12);
        assert_eq!(bytecode, "8B");
    }

    #[test]
    fn test_compile_dup13() {
        let bytecode = compile_instruction(Rule::dup13);
        assert_eq!(bytecode, "8C");
    }

    #[test]
    fn test_compile_dup14() {
        let bytecode = compile_instruction(Rule::dup14);
        assert_eq!(bytecode, "8D");
    }

    #[test]
    fn test_compile_dup15() {
        let bytecode = compile_instruction(Rule::dup15);
        assert_eq!(bytecode, "8E");
    }

    #[test]
    fn test_compile_dup16() {
        let bytecode = compile_instruction(Rule::dup16);
        assert_eq!(bytecode, "8F");
    }

    #[test]
    fn test_compile_swap1() {
        let bytecode = compile_instruction(Rule::swap1);
        assert_eq!(bytecode, "90");
    }

    #[test]
    fn test_compile_swap2() {
        let bytecode = compile_instruction(Rule::swap2);
        assert_eq!(bytecode, "91");
    }

    #[test]
    fn test_compile_swap3() {
        let bytecode = compile_instruction(Rule::swap3);
        assert_eq!(bytecode, "92");
    }

    #[test]
    fn test_compile_swap4() {
        let bytecode = compile_instruction(Rule::swap4);
        assert_eq!(bytecode, "93");
    }

    #[test]
    fn test_compile_swap5() {
        let bytecode = compile_instruction(Rule::swap5);
        assert_eq!(bytecode, "94");
    }

    #[test]
    fn test_compile_swap6() {
        let bytecode = compile_instruction(Rule::swap6);
        assert_eq!(bytecode, "95");
    }

    #[test]
    fn test_compile_swap7() {
        let bytecode = compile_instruction(Rule::swap7);
        assert_eq!(bytecode, "96");
    }

    #[test]
    fn test_compile_swap8() {
        let bytecode = compile_instruction(Rule::swap8);
        assert_eq!(bytecode, "97");
    }

    #[test]
    fn test_compile_swap9() {
        let bytecode = compile_instruction(Rule::swap9);
        assert_eq!(bytecode, "98");
    }

    #[test]
    fn test_compile_swap10() {
        let bytecode = compile_instruction(Rule::swap10);
        assert_eq!(bytecode, "99");
    }

    #[test]
    fn test_compile_swap11() {
        let bytecode = compile_instruction(Rule::swap11);
        assert_eq!(bytecode, "9A");
    }

    #[test]
    fn test_compile_swap12() {
        let bytecode = compile_instruction(Rule::swap12);
        assert_eq!(bytecode, "9B");
    }

    #[test]
    fn test_compile_swap13() {
        let bytecode = compile_instruction(Rule::swap13);
        assert_eq!(bytecode, "9C");
    }

    #[test]
    fn test_compile_swap14() {
        let bytecode = compile_instruction(Rule::swap14);
        assert_eq!(bytecode, "9D");
    }

    #[test]
    fn test_compile_swap15() {
        let bytecode = compile_instruction(Rule::swap15);
        assert_eq!(bytecode, "9E");
    }

    #[test]
    fn test_compile_swap16() {
        let bytecode = compile_instruction(Rule::swap16);
        assert_eq!(bytecode, "9F");
    }

    #[test]
    fn test_compile_log0() {
        let bytecode = compile_instruction(Rule::log0);
        assert_eq!(bytecode, "A0");
    }

    #[test]
    fn test_compile_log1() {
        let bytecode = compile_instruction(Rule::log1);
        assert_eq!(bytecode, "A1");
    }

    #[test]
    fn test_compile_log2() {
        let bytecode = compile_instruction(Rule::log2);
        assert_eq!(bytecode, "A2");
    }

    #[test]
    fn test_compile_log3() {
        let bytecode = compile_instruction(Rule::log3);
        assert_eq!(bytecode, "A3");
    }

    #[test]
    fn test_compile_log4() {
        let bytecode = compile_instruction(Rule::log4);
        assert_eq!(bytecode, "A4");
    }

    #[test]
    fn test_compile_create() {
        let bytecode = compile_instruction(Rule::create);
        assert_eq!(bytecode, "F0");
    }

    #[test]
    fn test_compile_call() {
        let bytecode = compile_instruction(Rule::call);
        assert_eq!(bytecode, "F1");
    }

    #[test]
    fn test_compile_callcode() {
        let bytecode = compile_instruction(Rule::callcode);
        assert_eq!(bytecode, "F2");
    }

    #[test]
    fn test_compile_return() {
        let bytecode = compile_instruction(Rule::evmReturn);
        assert_eq!(bytecode, "F3");
    }

    #[test]
    fn test_compile_delegatecall() {
        let bytecode = compile_instruction(Rule::delegatecall);
        assert_eq!(bytecode, "F4");
    }

    #[test]
    fn test_compile_create2() {
        let bytecode = compile_instruction(Rule::create2);
        assert_eq!(bytecode, "F5");
    }

    #[test]
    fn test_compile_staticcall() {
        let bytecode = compile_instruction(Rule::staticcall);
        assert_eq!(bytecode, "FA");
    }

    #[test]
    fn test_compile_revert() {
        let bytecode = compile_instruction(Rule::revert);
        assert_eq!(bytecode, "FD");
    }

    #[test]
    fn test_compile_selfdestruct() {
        let bytecode = compile_instruction(Rule::selfdestruct);
        assert_eq!(bytecode, "FF");
    }
}
