use std::env;
use std::fs;

use parser::parser_error::EVMMParserError;

mod compiler;
mod parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

fn main() -> Result<(), EVMMParserError> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        //TODO: print flag help messages
    } else {
        if args.contains(&"compile".to_string()) {

            //TODO: search for default path to compile
        }
    }

    let unparsed_file =
        fs::read_to_string("example_contract.evmm").expect("Something went wrong reading the file");

    let contract_bytecode =
        parser::parse::evmm_parse("example_contract".to_owned(), &unparsed_file)?;

    println!("{:?}", contract_bytecode);

    Ok(())
}
