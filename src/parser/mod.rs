use crate::text::kebab_to_pascal_case;

static AWS_COMMAND: [&str; 2] = ["aws", "aws2"];

pub struct Command {
    pub service: String,
    pub endpoint: String,
    pub arguments: Vec<String>,
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
        Some(api) => api,
    };
    let flags = end.split_at(2).1;
    Command {
        service: service.to_string(),
        endpoint: kebab_to_pascal_case(endpoint),
        arguments: flags.to_vec(),
    }
}
