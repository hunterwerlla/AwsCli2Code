pub mod definition_parser;

use crate::awssdk::definition_parser::{Api, Input, Paginators1, Service2Shape, Shape};
use definition_parser::Service2;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

fn resolve(service: Service2, paginators: Option<Paginators1>) -> HashMap<String, Api> {
    let mut model = HashMap::new();
    let shapes = service.shapes;
    for (key, value) in &service.operations {
        let mut inputs = HashMap::new();
        let paginates = match &paginators {
            None => false,
            Some(p) => p.pagination.contains_key(key),
        };
        let input = &value.input.shape;
        let (members, required) = match shapes.get(input) {
            None => panic!("Invalid top level shape {} specified!", input),
            Some(s) => match s {
                Service2Shape::Structure { required, members } => (members, required),
                _ => panic!("Top level shape {} is not a structure!", input),
            },
        };
        for (k, v) in members {
            let r = match &required {
                None => false,
                Some(p) => p.contains(k),
            };
            inputs.insert(
                k.to_string(),
                Input {
                    required: r,
                    shape: Shape::String,
                },
            );
        }

        model.insert(
            key.to_string(),
            Api {
                paginator: paginates,
                inputs,
            },
        );
    }
    return model;
}

fn parse_paginators(path: &Path) -> Option<Paginators1> {
    let paginators1file = if !path.exists() {
        return None;
    } else {
        match File::open(path) {
            Ok(file) => file,
            Err(e) => return None,
        }
    };

    None
}

fn parse_service2(path: &Path) -> Result<Service2, std::io::Error> {
    let service2file = File::open(path)?;
    return Ok(serde_json::from_reader(service2file).unwrap());
}

pub fn load_and_parse_service(path: &str) -> Result<HashMap<String, Api>, std::io::Error> {
    let service2_path = Path::new(path).join("service-2.json");
    let paginators_path = Path::new(path).join("paginators-1.json");
    let service2 = parse_service2(service2_path.as_path())?;
    let paginators = parse_paginators(paginators_path.as_path());
    let s = resolve(service2, paginators);
    Ok(s)
}
