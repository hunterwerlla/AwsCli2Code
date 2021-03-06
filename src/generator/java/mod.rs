use crate::awssdk::awssdk::ResolvedInput;
use crate::generator::SdkGenerator;
use crate::parser::Command;
use crate::text::{capitalize, pascal_case_to_camel_case};
use std::collections::HashSet;

fn build_client(service_name: &str, aws_profile: &Option<String>, aws_region: &Option<String>) {
    let client = capitalize(service_name);
    let client_name = format!("{}Client", client);
    println!(
        "{} {} = {}.builder()",
        client_name, service_name, client_name
    );
    match aws_profile {
        Some(profile) => println!(
            ".credentialsProvider(ProfileCredentialsProvider.builder().profileName(\"{}\").build())",
             profile
        ),
       None => {}
    };
    match aws_region {
        Some(region) => println!(
            ".region(Region.{})",
            region.to_uppercase().replace("-", "_")
        ),
        None => {}
    }
    println!(".build();");
}

fn build_request(command: &Command) {
    let java_endpoint = pascal_case_to_camel_case(&command.endpoint);
    println!(
        "{}.{}{}(",
        command.service,
        java_endpoint,
        if command.paginates { "Paginator" } else { "" }
    );
    println!("{}Request.builder()", command.endpoint);
    command.arguments.iter().for_each(|(key, v)| match v {
        ResolvedInput::String { value } => {
            println!(".{}(\"{}\")", key, value)
        }
        ResolvedInput::Timestamp { value } => {
            println!(".{}(Instance.parse({}))", key, value)
        }
        ResolvedInput::Boolean { value } => {
            println!(".{}({})", key, value)
        }
        ResolvedInput::Integer { value } => {
            println!(".{}({})", key, value)
        }
        ResolvedInput::Long { value } => {
            println!(".{}({})", key, value)
        }
        // TODO make recursive
        ResolvedInput::List { value } => {
            println!(
                ".{}({})",
                key,
                value
                    .iter()
                    .map(|i| match i {
                        ResolvedInput::String { value } => {
                            format!("\"{}\"", value)
                        }
                        _ => panic!("unable to handle list type yet"),
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    });
    println!(".build());")
}

pub struct JavaSdkGenerator {}

impl SdkGenerator for JavaSdkGenerator {
    fn generate(&self, clients: HashSet<String>, sdk: &str, commands: Vec<Command>) {
        let first_command = &commands.first().unwrap();
        clients
            .iter()
            .for_each(|i| build_client(i, &first_command.aws_profile, &first_command.aws_region));
        commands.iter().for_each(|i| build_request(i));
    }
}
