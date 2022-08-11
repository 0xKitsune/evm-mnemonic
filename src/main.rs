use std::env;
use std::fs;
mod compiler;
mod core;
mod evmm_error;
mod parser;
use crate::core::evmm::{evmm_parse_and_compile, DEFAULT_COMPILATION_DIR};

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
            .about("Compile .evmm contracts to bytecode. If a specific contract is provided, the compile command will only compile that contract. If no contract is provided, the compile command will compile everything in `./src/contracts/` by default. You can set the target directory to compile with the `-d` option. The contract bytecode is compiled by default, however you can specify the --deployment-bytecode or -d flag to compile the deployement bytecode See `compile --help` for full usage.")
                //
                //--contract option
                .arg(
                    Arg::new("contract")
                        .long("contract")
                        .short('c')
                        .help("Target a specific contract to compile. This flag takes a path as the argument. If the filename of the contract is only provided, the program will look in the ./evmm_contracts directory by default.")
                        .conflicts_with("target-directory")
                        .action(ArgAction::Set)
                        .number_of_values(1),
                )
                //
                //--directory option 
                .arg(
                    Arg::new("target-directory")
                        .long("target-directory")
                        .short('t')
                        .help("Specify a directory to target for compilation. Ex: `compile -t ./my_contracts`")
                        .conflicts_with("contract")
                        .action(ArgAction::Set)
                        .number_of_values(1),
                )
                //
                //--print option 
                .arg(
                    Arg::new("print")
                        .long("print")
                        .short('p')
                        .conflicts_with("output-directory")
                        .help("Logs the bytecode for all compiled contracts into the terminal.")
                )
                //
                //--deployment-bytecode option 
                .arg(
                    Arg::new("deployment-bytecode")
                        .long("deployment-bytecode")
                        .short('d')
                        .help("Logs the compiled deployment bytecode for all compiled contracts into the terminal.")
                )
                //
                //--output option
                .arg(
                    Arg::new("output-directory")
                        .long("output-directory")
                        .short('o')
                        .conflicts_with("print")
                        .help("Outputs the bytecode for each compiled contract as a `.evmasm` file into the specified output directory. This command can be chained with `--bytecode` or `--deploymentBytecode`, but if neither is specified, the `--output` option will default to writing the contract's bytecode to the output file. For example, to write a specific contract's bytecode to a file, you can use `compile <contract_name.evmm> --bytecode --output. To compile all contracts and write the deployment bytecode, you can use `compile --deploymentBytecode --output`")
                        .action(ArgAction::Set).number_of_values(1),
                ),
        )
        //
        // init subcommand
        .subcommand(
            Command::new("init")
                .about("Creates a new Foundry project with all the moving parts to test EVMM contracts")
        )
        .get_matches();

    //Handle the matched arguments
    match matches.subcommand() {
        Some(("compile", arg_matches)) => {
            //handle the command line args
            let mut deployment_bytecode = false;
            let mut print_in_terminal = false;
            let mut contract = "";
            let mut directory_to_compile = "";
            let mut output_directory = "";

            if arg_matches.contains_id("deployment-bytecode") {
                deployment_bytecode = true;
            }

            if arg_matches.contains_id("contract") {
                //Clap should not let the program get this far if the --directory flag is used
                //unless there was a value set so we can use unwrap
                contract = arg_matches.get_one::<String>("contract").unwrap();
            } else if arg_matches.contains_id("target-directory") {
                //Clap should not let the program get this far if the --directory flag is used
                //unless there was a value set so we can use unwrap
                directory_to_compile = arg_matches.get_one::<String>("directory").unwrap();
            }

            //if the output directory is specified
            if arg_matches.contains_id("output-directory") {
                output_directory = arg_matches.get_one::<String>("output").unwrap();
            } else if arg_matches.contains_id("print") {
                //if the print arg is specified
                print_in_terminal = true;
            } else if !arg_matches.contains_id("output-directory")
                && !arg_matches.contains_id("print")
            {
                //if the output directory is not specified and the print flag is not true, output to the default directory
                output_directory = DEFAULT_COMPILATION_DIR;
            }

            //compile evmm contracts with command line args
            evmm_parse_and_compile(
                deployment_bytecode,
                contract,
                directory_to_compile,
                output_directory,
                print_in_terminal,
            )?;
        }

        Some(("init", _)) => {
            //TODO: initialize new foundry project with all evmm dependencies
        }

        _ => unreachable!("clap should ensure we don't get here"),
    };

    Ok(())
}
