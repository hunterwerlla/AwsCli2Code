use crate::awssdk::awssdk::{ResolvedInput, Shape};
use crate::awssdk::load_and_parse_service;
use crate::text::{capitalize, kebab_to_camel_case, kebab_to_pascal_case};
use std::collections::HashMap;

static AWS_COMMAND: [&str; 2] = ["aws", "aws2"];
static SEPARATORS: [&str; 3] = ["|", "&&", "||"];
static AWS_PROFILE_VARIABLE: &str = "AWS_PROFILE";
static AWS_REGION_VARIABLE: [&str; 2] = ["AWS_REGION", "AWS_DEFAULT_REGION"];

pub struct Command {
    pub service: String,
    pub endpoint: String,
    pub arguments: HashMap<String, ResolvedInput>,
    pub aws_profile: Option<String>,
    pub aws_region: Option<String>,
}

fn parse_jmespath_query(input: &str) {
    let whatever = jmespath::compile(input).unwrap();
    println!("{}", whatever.as_str())
}

fn parse_flags(service: &str, endpoint: &str, input: &[String]) -> HashMap<String, ResolvedInput> {
    let service_definition = load_and_parse_service(&format!("resources/{}", service)).unwrap();
    let api_endpoint = service_definition.get(endpoint).unwrap();
    let mut iter = input.iter().peekable();
    let mut resolved_input = HashMap::new();
    loop {
        let raw_flag = match iter.next() {
            Some(i) => i,
            None => break,
        };
        let flag = kebab_to_camel_case(
            raw_flag
                .trim_start_matches("--no-")
                .trim_start_matches("--"),
        );
        // Skip over output (for now) later it can be used to deduce jmespath
        if flag == "output" {
            iter.next();
            continue;
        }
        if flag == "query" {
            parse_jmespath_query(iter.next().unwrap());
            continue;
        }
        let input_type = match api_endpoint.inputs.get(&flag) {
            // S3 breaks convention so check PascalCase as well
            None => match api_endpoint.inputs.get(&capitalize(&flag)) {
                Some(s) => s,
                None => panic!("Unknown argument {} for {}", flag, endpoint),
            },
            Some(s) => s,
        };
        resolved_input.insert(
            flag,
            match input_type.shape {
                Shape::String => ResolvedInput::String {
                    value: iter.next().unwrap().to_string(),
                },
                Shape::Boolean => ResolvedInput::Boolean {
                    value: !raw_flag.starts_with("--no-"),
                },
                Shape::Integer => ResolvedInput::Integer {
                    value: iter.next().unwrap().parse::<i32>().unwrap(),
                },
                Shape::Long => ResolvedInput::Long {
                    value: iter.next().unwrap().parse::<i64>().unwrap(),
                },
                // TODO take type into account
                Shape::List { .. } => {
                    let mut container = Vec::new();
                    while iter.peek().is_some() && !iter.peek().unwrap().starts_with("-") {
                        container.push(ResolvedInput::String {
                            value: iter.next().unwrap().to_string(),
                        })
                    }
                    ResolvedInput::List { value: container }
                }
                _ => panic!("TODO"),
            },
        );
    }
    resolved_input
}

fn resolve_service(raw_service: &str) -> &str {
    if raw_service == "s3api" {
        "s3"
    } else {
        raw_service
    }
}

// TODO mess with environment variables later
fn parse_beginning(commands: &[String]) -> (Option<String>, Option<String>) {
    let mut iter = commands.iter();
    let mut profile: Option<String> = None;
    let mut region: Option<String> = None;
    loop {
        let raw_flag = match iter.next() {
            Some(i) => i,
            None => break,
        };
        if raw_flag.starts_with(AWS_PROFILE_VARIABLE) {
            // TODO put "/' in a better spot
            let p = raw_flag
                .trim_start_matches(AWS_PROFILE_VARIABLE)
                .trim_start_matches("=")
                .trim_matches('"')
                .trim_matches('\'');
            if !p.is_empty() {
                profile = Some(p.to_string());
            }
        }
        if AWS_REGION_VARIABLE.iter().any(|f| raw_flag.starts_with(f)) {
            let p = raw_flag
                .trim_start_matches(AWS_REGION_VARIABLE[0])
                .trim_start_matches(AWS_REGION_VARIABLE[1])
                .trim_start_matches("=")
                .trim_matches('"')
                .trim_matches('\'');
            if !p.is_empty() {
                region = Some(p.to_string());
            }
        }
    }
    return (profile, region);
}

fn command(command: &[String]) -> Command {
    let mut split_command = command.split(|i| AWS_COMMAND.iter().any(|command| command == i));

    let beginning = split_command.next();
    if beginning == None {
        panic!("Command string does not contain `aws` or `aws2`!")
    }
    let (profile, region) = parse_beginning(beginning.unwrap());

    let end = match split_command.next() {
        None => panic!("Command string has nothing after `aws` or `aws2`!"),
        Some(command) => command,
    };
    let service = match end.get(0) {
        None => panic!("No service specified!"),
        Some(service) => resolve_service(service),
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
        aws_profile: profile,
        aws_region: region,
    }
}

pub fn parse_sdk_input(input: Vec<String>) -> Vec<Command> {
    // TODO mess with pipes and other separators later
    input
        .split(|i| SEPARATORS.iter().any(|s| s == i))
        .map(|c| command(c))
        .collect()
}
