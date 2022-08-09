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
                        .conflicts_with("deploymentBytecode")
                        .help("Logs the compiled bytecode for all compiled contracts into the terminal.")
                        .action(ArgAction::Set)
                ) 
                .arg(   
                    Arg::new("deploymentBytecode")
                .long("deploymentBytecode")
                .short('e')
                .help("Logs the compiled deployment bytecode for all compiled contracts into the terminal.")
                .conflicts_with("bytecode")
                .action(ArgAction::Set)
                .number_of_values(1),)
                
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .help("Outputs the bytecode for all compiled contracts into an `.evmasm` file. If an output directory is not specified, the output will be written to `evmasm/` by default. This command can be chained with `--bytecode` or `--deploymentBytecode`, but if neither is specified, the `--output` option will default to writing the contract's bytecode to the output file. For example, to write a specific contract's bytecode to a file, you can use `compile <contract_name.evmm> --bytecode --output. To compile all contracts and write the deployment bytecode, you can use `compile --deploymentBytecode --output`")
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
