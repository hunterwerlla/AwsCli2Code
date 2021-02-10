pub fn kebab_to_camel_case(input: &str) -> String {
    lowercase(&kebab_to_pascal_case(input))
}

pub fn kebab_to_pascal_case(input: &str) -> String {
    let inputs = input.split("-");
    inputs.map(|x| capitalize(x)).collect()
}

pub fn pascal_case_to_camel_case(input: &str) -> String {
    lowercase(input)
}

pub fn lowercase(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}

pub fn capitalize(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
