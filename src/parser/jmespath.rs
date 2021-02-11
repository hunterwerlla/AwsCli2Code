// an actual implementation of this would need a jmespath parser and ast traverser
// but I'm just going to cover the easy case in an easy way instead

// Only supports x[*].y format
pub struct JmespathQuery {
    front: String,
    back: String,
}

pub fn parse_jmespath_query(input: &str) -> JmespathQuery {
    let mut expression = input.split("[*].");
    let front = match expression.next() {
        None => panic!("Only supports x[*].y format!"),
        Some(s) => s,
    };
    let back = match expression.next() {
        None => panic!("Only supports x[*].y format!"),
        Some(s) => s,
    };
    match expression.next() {
        None => (),
        Some(s) => panic!("Only supports x[*].y format!"),
    }
    JmespathQuery {
        front: front.to_string(),
        back: back.to_string(),
    }
}
