pub mod awssdk;

use crate::awssdk::awssdk::{Api, Input, Paginators1, Service2Shape, Shape};
use awssdk::Service2;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

fn resolve_final_type(
    shapes: &HashMap<String, Service2Shape>,
    key: &str,
    field_required: bool,
) -> Input {
    let shape = match shapes.get(key) {
        Some(s) => s,
        None => panic!("TODO shape resolved to not a shape"),
    };
    match shape {
        Service2Shape::String => Input {
            required: field_required,
            shape: Shape::String,
        },
        Service2Shape::Timestamp => Input {
            required: field_required,
            shape: Shape::Timestamp,
        },
        Service2Shape::Boolean => Input {
            required: field_required,
            shape: Shape::Boolean,
        },
        Service2Shape::Integer => Input {
            required: field_required,
            shape: Shape::Integer,
        },
        Service2Shape::Long => Input {
            required: field_required,
            shape: Shape::Long,
        },
        Service2Shape::Blob => Input {
            required: field_required,
            shape: Shape::Blob,
        },
        Service2Shape::Structure { required, members } => {
            let mut m: HashMap<String, Input> = HashMap::new();
            for (k, v) in members {
                let r = match required {
                    Some(s) => s.contains(k),
                    None => false,
                };
                m.insert(k.to_string(), resolve_final_type(shapes, &v.shape, r));
            }
            Input {
                required: field_required,
                shape: Shape::Structure { members: m },
            }
        }
        // TODO list and map
        Service2Shape::List { member } => Input {
            required: field_required,
            shape: Shape::List {
                shape: Box::from(resolve_final_type(shapes, &member.shape, false).shape),
            },
        },
        Service2Shape::Map { .. } => Input {
            required: field_required,
            shape: Shape::String,
        },
    }
}

fn resolve(service: Service2, paginators: Option<Paginators1>) -> HashMap<String, Api> {
    let mut model = HashMap::new();
    let shapes = service.shapes;
    for (key, value) in &service.operations {
        let paginates = match &paginators {
            None => false,
            Some(p) => p.pagination.contains_key(key),
        };
        let inputs = match &value.input {
            Some(s) => {
                let input = &s.shape;
                let t = resolve_final_type(&shapes, input, true);
                match t {
                    Input {
                        shape: Shape::Structure { members },
                        ..
                    } => members,
                    _ => panic!("top level is not a structure"),
                }
            }
            None => HashMap::new(),
        };

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
    let paginators = if !path.exists() {
        return None;
    } else {
        match File::open(path) {
            Ok(file) => file,
            Err(_) => return None,
        }
    };
    match serde_json::from_reader(paginators) {
        Ok(p) => p,
        Err(_) => return None,
    }
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
