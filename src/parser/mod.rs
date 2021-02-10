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
    let service_definition = load_and_parse_service(&format!("resources/{}", service)).unwrap();
    let endpoint = service_definition.get(endpoint).unwrap();
    let mut iter = input.iter();
    let mut resolved_input = Vec::new();
    loop {
        let next = match iter.next() {
            Some(i) => i,
            None => break,
        };
        // TODO proper bool support
        let input = kebab_to_camel_case(next.trim_start_matches("--no-").trim_start_matches("--"));
        let input_type = endpoint.inputs.get(&input).unwrap();
        resolved_input.push(match input_type.shape {
            Shape::String => ResolvedInput::String {
                api: input,
                value: iter.next().unwrap().to_string(),
            },
            Shape::Boolean => ResolvedInput::Boolean {
                value: !input.starts_with("--no-"),
                api: input,
            },
            _ => panic!("TODO"),
        });
    }
    resolved_input
}

fn command(command: &[String]) -> Command {
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
    let resolved_input = parse_flags(service, &endpoint, flags);
    Command {
        service: service.to_string(),
        endpoint,
        arguments: resolved_input,
    }
}

pub fn parse_sdk_input(input: Vec<String>) -> Vec<Command> {
    // TODO mess with pipes and other separators later
    input.split(|i| i == "|").map(|c| command(c)).collect()
}
