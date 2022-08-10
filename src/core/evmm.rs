use crate::compiler::compile::compile_instructions;
use crate::evmm_error::evmm_error::EVMMError;
use crate::parser::parse::{self, parse_file};
use std::io::Error;
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
    output_contracts(evmasm_files, deployment_bytecode, output_directory);

    Ok(())
}

fn get_contract_contents(
    contract_path: &str,
    directory_to_compile: &str,
) -> Result<Vec<EVMMFile>, Error> {
    let mut evmm_files: Vec<EVMMFile> = vec![];

    if directory_to_compile != "" {
        let paths = fs::read_dir(contract_path)?;

        for path in paths {
            let file_path = path.unwrap().path();
            //Sheild your eyes, please disregard this line
            let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();

            let unparsed_file = fs::read_to_string(file_path).unwrap();

            let file_contents = parse::parse_file(&unparsed_file);

            evmm_files.push(EVMMFile::new(file_name, file_contents.to_string()))
        }
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
        }

        evmm_file.file_name.push_str("_deploy");

        compiled_evmasm_files.push(EVMASMFile::new(evmm_file.file_name, compiled_bytecode));
    }

    Ok(compiled_evmasm_files)
}

fn output_contracts(
    evmasm_files: Vec<EVMASMFile>,
    deployment_bytecode: bool,
    output_directory: &str,
) {
    for evmasm_file in evmasm_files {
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
