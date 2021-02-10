use crate::parser::Command;
use crate::text::{capitalize, pascal_case_to_camel_case};

// TODO make generic or whatever
fn build_client(service_name: &str) {
    let client = capitalize(service_name);
    let client_name = format!("{}Client", client);
    println!("{} {} = new {}()", client_name, service_name, client_name);
}

fn build_request(service_name: &str, endpoint: &str) {
    let java_endpoint = pascal_case_to_camel_case(endpoint);
    println!("{}.{}()", service_name, java_endpoint)
}

pub fn generate(commands: Vec<Command>) {
    commands.iter().for_each(|command| {
        // make client
        build_client(&command.service);
        // make request
        build_request(&command.service, &command.endpoint);
        // execute request
        command
            .arguments
            .iter()
            .for_each(|item| println!("{:?}", item))
    })
}
