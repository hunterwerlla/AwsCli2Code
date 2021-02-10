mod java;
mod javascript;

use crate::generator::java::JavaSdkGenerator;
use crate::generator::javascript::JavascriptSdkGenerator;
use crate::parser::Command;
use std::collections::HashSet;

trait SdkGenerator {
    fn generate(&self, clients: HashSet<String>, sdk: &str, commands: Vec<Command>);
}

pub fn generate(sdk: &str, language: Option<&str>, commands: Vec<Command>) {
    let clients: HashSet<String> =
        HashSet::from(commands.iter().map(|i| i.service.to_string()).collect());
    let generator = match sdk {
        "javav2" => match language {
            None => Box::new(JavaSdkGenerator {}) as Box<dyn SdkGenerator>,
            Some(s) => match s {
                "java" => Box::new(JavaSdkGenerator {}) as Box<dyn SdkGenerator>,
                _ => panic!("Unknown javasdkv2 language {}", s),
            },
        },
        "javascriptv2" => match language {
            None => Box::new(JavascriptSdkGenerator {}) as Box<dyn SdkGenerator>,
            Some(s) => match s {
                "javascript" => Box::new(JavascriptSdkGenerator {}) as Box<dyn SdkGenerator>,
                _ => panic!("Unknown javascriptv2 language {}", s),
            },
        },
        _ => panic!("Unknown AWS SDK {}", sdk),
    };
    generator.generate(clients, sdk, commands)
}
