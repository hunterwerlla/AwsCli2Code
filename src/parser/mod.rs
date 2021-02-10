use crate::awssdk::awssdk::{ResolvedInput, Shape};
use crate::awssdk::load_and_parse_service;
use crate::text::{kebab_to_camel_case, kebab_to_pascal_case};

static AWS_COMMAND: [&str; 2] = ["aws", "aws2"];

pub struct Command {
    pub service: String,
    pub endpoint: String,
    pub arguments: Vec<ResolvedInput>,
}

fn parse_flags(service: &str, endpoint: &str, input: &[String]) -> Vec<ResolvedInput> {
    let service = load_and_parse_service(&format!("resources/{}", service)).unwrap();
    let endpoint = service.get(endpoint).unwrap();
    let mut iter = input.iter();
    let mut resolvedInput = Vec::new();
    loop {
        let next = match iter.next() {
            Some(i) => i,
            None => break,
        };
        // TODO proper bool support
        let input = kebab_to_camel_case(next.trim_start_matches("--no-").trim_start_matches("--"));
        let input_type = endpoint.inputs.get(&input).unwrap();
        let i = match input_type.shape {
            Shape::String => ResolvedInput::String {
                api: input,
                value: iter.next().unwrap().to_string(),
            },
            _ => panic!("TODO"),
        };
        resolvedInput.push(i);
    }
    resolvedInput
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
    let resolvedInput = parse_flags(service, &endpoint, flags);
    Command {
        service: service.to_string(),
        endpoint,
        arguments: resolvedInput,
    }
}
