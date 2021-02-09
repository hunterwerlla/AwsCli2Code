use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

// service2.json
#[derive(Deserialize)]
pub struct ShapeReference {
    pub shape: String,
}

#[derive(Deserialize)]
pub struct Operation {
    pub name: String,
    pub input: ShapeReference,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Service2Shape {
    String,
    Structure {
        required: Vec<String>,
        members: HashMap<String, ShapeReference>,
    },
    List {
        member: ShapeReference,
    },
    Timestamp,
    Map {
        key: Value,
        value: Value,
    },
    Boolean,
    Integer,
    Long,
    Blob,
}

#[derive(Deserialize)]
pub struct Service2 {
    pub shapes: HashMap<String, Service2Shape>,
    pub operations: HashMap<String, Operation>,
}

// paginators1.json
#[derive(Deserialize)]
pub struct Paginators1 {
    pub pagination: HashMap<String, Value>,
}

// Final, resolved model
pub enum Shape {
    String,
    Structure,
    List { member: Vec<Shape> },
    Timestamp,
    Map { value: HashMap<String, String> },
    Boolean,
    Integer,
    Long,
    Blob,
}

pub struct Input {
    pub name: String,
    pub required: bool,
    pub shape: Shape,
}

pub struct Api {
    pub paginator: bool,
    pub inputs: Vec<Input>,
}

pub type ServiceModel = HashMap<String, Api>;
