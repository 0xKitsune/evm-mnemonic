use crate::compiler::compile::compile_instructions;
use crate::evmm_error::evmm_error::EVMMError;
use crate::parser::parse::parse_file;

pub fn evmm_parse_and_compile(
    bytecode: bool,
    deploymentBytecode: bool,
    contract: &str,
    directory_to_compile: &str,
    ouput_directory: &str,
) -> Result<String, EVMMError> {
    // let parsed_file = parse_file(file_name, unparsed_file);

    // let instructions = parsed_file.into_inner().peekable();

    // Ok(compile_instructions(instructions, String::from(""))?)

    Ok(String::from("_"))
}
