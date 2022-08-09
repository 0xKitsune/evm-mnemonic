use std::env;
use std::fs;
mod compiler;
mod evmm_error;
mod parser;
use crate::evmm_error::evmm_error::EVMMError;

extern crate pest;
#[macro_use]
extern crate pest_derive;

fn main() -> Result<(), EVMMError> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        //TODO: print flag help messages
    } else {
        if args.contains(&"compile".to_string()) {
            //TODO: implement default compilation path, looks for the current folder ./src or something

            //TODO: flag to check if there is a specified path

            //TODO: flag to check if there is a specified contract to compile

            //TODO: flag to output bytecode to a file

            let unparsed_file = fs::read_to_string("example_contract.evmm")
                .expect("Something went wrong reading the file");

            let contract_bytecode =
                parser::parse::evmm_parse("example_contract".to_owned(), &unparsed_file)?;

            println!("{:?}", contract_bytecode);
        }
    }

    Ok(())
}
