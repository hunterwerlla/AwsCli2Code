pub mod parser;

use crate::parser::parser::{Paginators1, ServiceModel};
use parser::Service2;
use std::fs::File;
use std::path::Path;

fn resolve(
    service: Service2,
    paginators: Option<Paginators1>,
) -> Result<ServiceModel, std::io::Error> {
    return Ok(ServiceModel {});
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

pub fn load_and_parse_service(path: &str) -> Result<ServiceModel, std::io::Error> {
    let service2_path = Path::new(path).join("service-2.json");
    let paginators_path = Path::new(path).join("paginators-1.json");
    let service2 = parse_service2(service2_path.as_path())?;
    let paginators = parse_paginators(paginators_path.as_path());
    resolve(service2, paginators)
}
