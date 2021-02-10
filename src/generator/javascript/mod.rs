use crate::generator::SdkGenerator;
use crate::parser::Command;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;

pub struct JavascriptSdkGenerator {}

impl SdkGenerator for JavascriptSdkGenerator {
    fn generate(&self, clients: HashSet<String, RandomState>, sdk: &str, commands: Vec<Command>) {
        unimplemented!()
    }
}
