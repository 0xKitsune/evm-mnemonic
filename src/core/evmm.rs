use crate::compiler::compile::compile_instructions;
use crate::core::evmm;
use crate::evmm_error::evmm_error::EVMMError;
use crate::parser::parse::{self, parse_file};
use std::fs::{File, ReadDir};
use std::io::Write;
use std::io::{Error, Read};
use std::path::Path;
use std::{fs, vec};

struct EVMMFile {
    file_name: String,
    file_contents: String,
}

impl EVMMFile {
    pub fn new(file_name: String, file_contents: String) -> EVMMFile {
        EVMMFile {
            file_name,
            file_contents,
        }
    }
}

pub struct EVMASMFile {
    file_name: String,
    compiled_bytecode: String,
}

impl EVMASMFile {
    pub fn new(file_name: String, compiled_bytecode: String) -> EVMASMFile {
        EVMASMFile {
            file_name,
            compiled_bytecode,
        }
    }
}

const DEFAULT_CONTRACTS_DIR: &str = "./evmm_contracts";
pub const DEFAULT_COMPILATION_DIR: &str = "./evm_asm";

pub fn evmm_parse_and_compile(
    deployment_bytecode: bool,
    contract_path: &str,
    directory_to_compile: &str,
    output_directory: &str,
) -> Result<(), EVMMError> {
    //TODO: need to handle errors gracefully
    let evmm_files = get_contract_contents(contract_path, directory_to_compile).unwrap();

    let evmasm_files = parse_and_compile_bytecode(evmm_files, deployment_bytecode)?;

    //output the deployment bytecode
    output_contracts(evmasm_files, output_directory).unwrap();

    Ok(())
}

fn get_contract_contents(
    contract_path: &str,
    directory_to_compile: &str,
) -> Result<Vec<EVMMFile>, Error> {
    let mut evmm_files: Vec<EVMMFile> = vec![];

    let paths: ReadDir;
    if directory_to_compile != "" {
        paths = fs::read_dir(contract_path)?;
    } else {
        paths = fs::read_dir(DEFAULT_CONTRACTS_DIR)?;
    }

    for path in paths {
        let file_path = path.unwrap().path();
        //Sheild your eyes, please disregard this line
        let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();

        let file_contents = fs::read_to_string(file_path).unwrap();

        evmm_files.push(EVMMFile::new(file_name, file_contents.to_string()))
    }

    Ok(evmm_files)
}

fn parse_and_compile_bytecode(
    evmm_files: Vec<EVMMFile>,
    deployment_bytecode: bool,
) -> Result<Vec<EVMASMFile>, EVMMError> {
    let mut compiled_evmasm_files: Vec<EVMASMFile> = vec![];

    for mut evmm_file in evmm_files {
        let parsed_file = parse_file(&evmm_file.file_contents);

        let compiled_bytecode =
            compile_instructions(parsed_file.into_inner().peekable(), "".to_owned())?;

        //If the contract should compile to deployment bytecode
        if deployment_bytecode {
            //add deployment bytecode

            //add _deploy to the filename to indicate that it is deployment bytecode
            //otherwise, just add the evmasm extension
            let mut evmasm_file_name = evmm_file
                .file_name
                .strip_suffix(".evmm")
                .unwrap()
                .to_owned();

            evmasm_file_name.push_str("_deploy.evmasm");

            compiled_evmasm_files.push(EVMASMFile::new(evmasm_file_name, compiled_bytecode));
        } else {
            //otherwise, just add the evmasm extension
            let mut evmasm_file_name = evmm_file
                .file_name
                .strip_suffix(".evmm")
                .unwrap()
                .to_owned();

            evmasm_file_name.push_str(".evmasm");

            compiled_evmasm_files.push(EVMASMFile::new(evmasm_file_name, compiled_bytecode));
        }
    }

    Ok(compiled_evmasm_files)
}

fn output_contracts(evmasm_files: Vec<EVMASMFile>, output_directory: &str) -> Result<(), Error> {
    for evmasm_file in evmasm_files {
        //If an output directory is specified, write to a file
        if output_directory != "" {
            if fs::metadata(output_directory).is_ok() {
                let mut new_evmasm_file =
                    File::create(format!("{}/{}", output_directory, evmasm_file.file_name))
                        .unwrap();
                new_evmasm_file.write_all(evmasm_file.compiled_bytecode.as_bytes())?;
            } else {
                std::fs::create_dir(output_directory).unwrap();

                let mut new_evmasm_file =
                    File::create(format!("{}/{}", output_directory, evmasm_file.file_name))
                        .unwrap();
                new_evmasm_file.write_all(evmasm_file.compiled_bytecode.as_bytes())?;
            }
        } else {
            //otherwise, just log the output in the terminal
        }
    }

    Ok(())
}
