// aws s3api get-bucket-acl --bucket jetbrainss3testbucketa

mod awssdk;
mod cli;
mod generator;

use crate::awssdk::definition_parser::Service2;
use crate::awssdk::load_and_parse_service;
use crate::cli::parse_command_line;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let arguments = parse_command_line();
    arguments
        .aws_cli_input
        .iter()
        .for_each(|f| println!("{}", f));
    let parsed = load_and_parse_service("resources/ecr")?;
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
