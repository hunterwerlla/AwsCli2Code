use crate::parser::Command;
use crate::text::{capitalize, pascal_case_to_camel_case};

// TODO make generic or whatever
fn buildClient(serviceName: &str) {
    let client = capitalize(serviceName);
    let clientName = format!("{}Client", client);
    println!("{} {} = new {}()", clientName, serviceName, clientName);
}

fn buildRequest(serviceName: &str, endpoint: &str) {
    let javaEndpoint = pascal_case_to_camel_case(endpoint);
    println!("{}.{}()", serviceName, javaEndpoint)
}

pub fn generate(command: &Command) {
    // make client
    buildClient(&command.service);
    // make request
    buildRequest(&command.service, &command.endpoint);
    // execute request
    command
        .arguments
        .iter()
        .for_each(|item| println!("{:?}", item))
}
