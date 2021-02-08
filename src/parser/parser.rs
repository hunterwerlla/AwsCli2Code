use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

// service2.json
#[derive(Deserialize)]
pub struct ShapeReference {
    shape: String,
}

#[derive(Deserialize)]
pub struct Operation {
    name: String,
    input: ShapeReference,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Shape {
    String,
    Structure,
    List { member: ShapeReference },
    Timestamp,
    Map { key: Value, value: Value },
    Boolean,
    Integer,
    Long,
    Blob,
}

#[derive(Deserialize)]
pub struct Service2 {
    pub shapes: HashMap<String, Shape>,
    pub operations: HashMap<String, Operation>,
}

// paginators1.json
#[derive(Deserialize)]
pub struct Paginators1 {
    pagination: HashMap<String, Value>,
}

// Final, resolved model
pub struct ServiceModel {}
