// aws s3api get-bucket-acl --bucket jetbrainss3testbucketa

mod awssdk;
mod cli;
mod generator;
mod parser;
mod text;

use crate::awssdk::awssdk::Api;
use crate::awssdk::load_and_parse_service;
use crate::cli::parse_command_line;
use crate::generator::generate;
use crate::parser::parse_sdk_input;

fn main() -> Result<(), std::io::Error> {
    let arguments = parse_command_line();
    let command = parse_sdk_input(arguments.aws_cli_input);
    let parsed = load_and_parse_service(&*format!("resources/{}", command.service))?;
    let api = match parsed.get(&*command.endpoint) {
        None => panic!("no api found!"),
        Some(a) => a,
    };
    generate(api, &command);
    /*
    parsed.shapes.keys().for_each(|item| {
        println!("{}", item)
    });
    println!("#########################");
    parsed.operations.keys().for_each(|item| {
        println!("{}", item)
    });*/

    Ok(())
}
