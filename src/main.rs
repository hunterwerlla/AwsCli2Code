// aws s3api get-bucket-acl --bucket jetbrainss3testbucketa

mod parser;

use crate::parser::load_and_parse_service;
use crate::parser::parser::Service2;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
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
