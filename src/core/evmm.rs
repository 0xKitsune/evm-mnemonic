use crate::compiler::compile::compile_instructions;
use crate::evmm_error::evmm_error::EVMMError;
use crate::parser::parse::parse_file;
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

struct EVMASMFile {
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

pub fn evmm_parse_and_compile(
    deployment_bytecode: bool,
    contract_path: &str,
    directory_to_compile: &str,
    output_directory: &str,
) -> Result<(), EVMMError> {
    let evmm_files = get_contract_contents(contract_path, directory_to_compile);

    let compiled_bytecode_vec = parse_and_compile_bytecode(evmm_files, deployment_bytecode)?;

    //output the deployment bytecode
    output_contracts(compiled_bytecode_vec, deployment_bytecode, output_directory);

    Ok(())
}

fn get_contract_contents(contract_path: &str, directory_to_compile: &str) -> Vec<EVMMFile> {
    let contract_contents: Vec<EVMMFile> = vec![];

    if directory_to_compile != "" {}

    contract_contents
}

fn parse_and_compile_bytecode(
    evmm_files: Vec<EVMMFile>,
    deployment_bytecode: bool,
) -> Result<Vec<EVMASMFile>, EVMMError> {
    let mut compiled_evmasm_files: Vec<EVMASMFile> = vec![];

    for evmm_file in evmm_files {
        let parsed_file = parse_file(evmm_file.file_name.clone(), &evmm_file.file_contents);

        let compiled_bytecode =
            compile_instructions(parsed_file.into_inner().peekable(), "".to_owned())?;

        //If the contract should compile to deployment bytecode
        if deployment_bytecode {
            //add deployment bytecode
        }

        compiled_evmasm_files.push(EVMASMFile::new(evmm_file.file_name, compiled_bytecode));
    }

    Ok(compiled_evmasm_files)
}

fn output_contracts(
    compiled_bytecode_vec: Vec<EVMASMFile>,
    deployment_bytecode: bool,
    output_directory: &str,
) {
    for compiled_bytecode in compiled_bytecode_vec {
        if output_directory != "" {
            //if the compile contract is deployement bytecode, name the file `deploy_<contract_name>.evmasm`
            if deployment_bytecode {
            } else {
                //otherwise, name the file `<contract_name>.evmasm`
            }
        } else {
            //otherwise, just log the output in the terminal
        }
    }
}
