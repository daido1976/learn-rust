use std::collections::HashMap;

#[allow(dead_code, clippy::box_collection)]
#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_json() {
        println!("{:?}", Json::String("Hello".to_string()))
    }
}
