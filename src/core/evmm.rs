use crate::compiler::compile::compile_instructions;
use crate::evmm_error::evmm_error::EVMMError;
use crate::parser::parse::parse_file;
use std::fs;
use std::path::Path;

pub fn evmm_parse_and_compile(
    bytecode: bool,
    deploymentBytecode: bool,
    contract_path: &str,
    directory_to_compile: &str,
    output_directory: &str,
) -> Result<String, EVMMError> {
    //If a specific contract has been targeted for compilation
    if contract_path != "" {
        //if the path to the contract includes a directory
        if contract_path.contains("/") {
            match fs::read_to_string(contract_path) {
                Ok(file_contents) => {
                    let file_name = Path::new(&contract_path)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();

                    let parsed_file = parse_file(file_name, &file_contents);

                    let compiled_bytecode =
                        compile_instructions(parsed_file.into_inner().peekable(), "".to_string())?;

                    //compile the deployment bytecode
                    if deploymentBytecode {
                        //If the output directory was specified, then output to filepath
                        if output_directory != "" {}
                    } else {
                        if output_directory != "" {
                        } else {
                        }
                    }
                }
                Err(_) => return Err(EVMMError::ContractNotFound(contract_path.to_string())),
            };
        } else {
        }

        return Ok("".to_string());
    } else if directory_to_compile != "" {
    } else {
        //compile the ./evmm_contracts directory
    }

    // let parsed_file = parse_file(file_name, unparsed_file);

    // let instructions = parsed_file.into_inner().peekable();

    // Ok(compile_instructions(instructions, String::from(""))?)

    Ok("".to_string())
}
