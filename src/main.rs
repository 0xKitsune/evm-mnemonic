use std::env;
use std::fs;
mod compiler;
mod evmm_error;
mod parser;
use crate::evmm_error::evmm_error::EVMMError;
use clap::{Arg, ArgAction, Command};
extern crate pest;
#[macro_use]
extern crate pest_derive;

fn main() -> Result<(), EVMMError> {
    let matches = Command::new("evmm")
        .about("A barebones framework to write hand tuned smart contracts in pure opcodes.")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        //
        // compile subcommand
        .subcommand(
            Command::new("compile")
            .about("Compile .evmm contracts to bytecode. If a specific contract is provided, the compile command will only compile that contract. If no contract is provided, the compile command will compile everything in `./src/contracts/` by default. You can set the target directory to compile with the `-d` option. See `compile --help` for full usage.")
                .arg(
                    Arg::new("directory")
                        .long("directory")
                        .short('d')
                        .help("Specify a directory to target for compilation. Ex: compile -d ./my_contracts")
                        .conflicts_with("bytecode")
                        .action(ArgAction::Set)
                        .number_of_values(1),
                )
                .arg(
                    Arg::new("bytecode")
                        .long("bytecode")
                        .short('b')
                        .conflicts_with("directory")
                        .help("Prints the compiled bytecode for a specific contract into the terminal.")
                        .action(ArgAction::Set)
                ) .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .help("Outputs the compiled bytecode for any contract that was compiled. If an output directory is not specified, the output will be written to `evmasm/` by default.")
                        .action(ArgAction::Set)
                        .number_of_values(1),
                ),
        )
        //
        // init subcommand
        .subcommand(
            Command::new("init")
                .about("Creates a new Foundry project with all the moving parts to test EVMM contracts")
        )
        .get_matches();

    Ok(())
}
