mod awssdk;
mod cli;
mod generator;
mod parser;
mod text;

use crate::cli::parse_command_line;
use crate::generator::generate;
use crate::parser::parse_sdk_input;

fn main() -> Result<(), std::io::Error> {
    let arguments = parse_command_line();
    let commands = parse_sdk_input(arguments.aws_cli_input);
    generate(&arguments.sdk, arguments.language.as_deref(), commands);
    Ok(())
}
