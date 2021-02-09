use crate::awssdk::awssdk::{ResolvedInput, Shape};
use crate::awssdk::load_and_parse_service;
use crate::text::kebab_to_pascal_case;

static AWS_COMMAND: [&str; 2] = ["aws", "aws2"];

pub struct Command {
    pub service: String,
    pub endpoint: String,
    pub arguments: Vec<String>,
}

fn parse_flags(service: &str, endpoint: &str, input: &[String]) -> Vec<ResolvedInput> {
    let service = load_and_parse_service(&format!("resources/{}", command.service)).unwrap();
    let endpoint = service.get(endpoint).unwrap();
    let 
    let mut iter = input.iter();
    while !iter.is_empty() {
        let next = iter.next().unwrap();
        // TODO proper bool support
        let input = kebab_to_pascal_case(next.strip_prefix("--no-").strip_prefix("--"));
        let input_type = endpoint.inputs.get(&input).unwrap();
        let bla = match input_type.shape {
            Shape::String => ResolvedInput::String {
                api: input,
                value: iter.next().unwrap().to_string(),
            },
            _ => panic!("TODO"),
        };
    }
    ()
}

pub fn parse_sdk_input(input: Vec<String>) -> Command {
    // TODO mess with pipes and other separators later
    let command = match input.split(|i| i == "|").next() {
        Some(segment) => segment,
        None => input.as_slice(),
    };
    let mut split_command = command.split(|i| AWS_COMMAND.iter().any(|command| command == i));

    // TODO mess with environment variables later
    let beginning = split_command.next();
    if beginning == None {
        panic!("Command string does not contain `aws` or `aws2`!")
    }
    let end = match split_command.next() {
        None => panic!("Command string has nothing after `aws` or `aws2`!"),
        Some(command) => command,
    };
    let service = match end.get(0) {
        None => panic!("No service specified!"),
        Some(service) => service,
    };
    let endpoint = match end.get(1) {
        None => panic!("No service call specified!"),
        Some(api) => kebab_to_pascal_case(api),
    };
    let flags = end.split_at(2).1;
    parse_flags(service, endpoint, flags);
    Command {
        service: service.to_string(),
        endpoint: endpoint,
        arguments: flags.to_vec(),
    }
}
