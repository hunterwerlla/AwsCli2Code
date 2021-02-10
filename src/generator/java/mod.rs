use crate::generator::SdkGenerator;
use crate::parser::Command;
use crate::text::{capitalize, pascal_case_to_camel_case};
use std::collections::HashSet;

fn build_client(service_name: &str) {
    let client = capitalize(service_name);
    let client_name = format!("{}Client", client);
    println!("{} {} = new {}()", client_name, service_name, client_name);
}

fn build_request(command: &Command) {
    let java_endpoint = pascal_case_to_camel_case(&command.endpoint);
    println!("{}.{}(", command.service, java_endpoint);
    println!(")")
}

pub struct JavaSdkGenerator {}

impl SdkGenerator for JavaSdkGenerator {
    fn generate(&self, clients: HashSet<String>, sdk: &str, commands: Vec<Command>) {
        clients.iter().for_each(|i| build_client(i));
        commands.iter().for_each(|i| build_request(i));
    }
}
