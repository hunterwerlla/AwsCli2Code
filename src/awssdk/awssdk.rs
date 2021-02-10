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
    pub input: Option<ShapeReference>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Service2Shape {
    String,
    Structure {
        required: Option<Vec<String>>,
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
    Structure { members: HashMap<String, Input> },
    List { shape: Box<Shape> },
    Timestamp,
    Map { key: Box<Shape>, value: Box<Shape> },
    Boolean,
    Integer,
    Long,
    Blob,
}

pub struct Input {
    pub required: bool,
    pub shape: Shape,
}

pub struct Api {
    pub paginator: bool,
    pub inputs: HashMap<String, Input>,
}

pub enum ResolvedInput {
    String { value: String },
    Timestamp { value: String },
    Boolean { value: bool },
    Integer { value: i32 },
    Long { value: i64 },
    List { value: Vec<ResolvedInput> },
}
