use crate::compiler::compile::compile_instructions;
use crate::evmm_error::evmm_error::EVMMError;
use crate::parser::parse::parse_file;
use std::fs;
use std::path::Path;

pub fn evmm_parse_and_compile(
    bytecode: bool,
    deployment_bytecode: bool,
    contract_path: &str,
    directory_to_compile: &str,
    output_directory: &str,
) -> Result<(), EVMMError> {
    let file_contents = get_contract_contents(contract_path, directory_to_compile);

    //If bytecode is true or deployment bytecode is false
    if bytecode == true || deployment_bytecode == false {
        let compiled_bytecode_vec = parse_and_compile_bytecode();

        //output the deployment bytecode
        output_contracts(compiled_bytecode_vec, deployment_bytecode, output_directory);
    } else {
        //otherwise, compile deployment bytecode
        let compiled_deployment_bytecode_vec = parse_and_compile_deployment_bytecode();

        //output the deployment bytecode
        output_contracts(
            compiled_deployment_bytecode_vec,
            deployment_bytecode,
            output_directory,
        );
    }

    Ok(())
}

fn get_contract_contents(contract_path: &str, directory_to_compile: &str) -> Vec<String> {
    vec![]
}

fn parse_and_compile_bytecode() -> Vec<String> {
    vec![]
}

fn parse_and_compile_deployment_bytecode() -> Vec<String> {
    vec![]
}

fn output_contracts(
    compiled_bytecode_vec: Vec<String>,
    deployment_bytecode: bool,
    output_directory: &str,
) {
}
