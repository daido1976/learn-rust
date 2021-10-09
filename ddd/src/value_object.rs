#[derive(Clone, Debug, PartialEq)]
pub struct FullName {
    first_name: String,
    last_name: String,
}

impl FullName {
    pub fn new(first_name: &str, last_name: &str) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }
}

#[test]
fn test_equality_of_value_object() {
    let taro_tanaka_1 = FullName::new("taro", "tanaka");
    let taro_tanaka_2 = FullName::new("taro", "tanaka");
    let jiro_suzuki = FullName::new("jiro", "suzuki");

    assert_eq!(taro_tanaka_1, taro_tanaka_2);
    assert_ne!(taro_tanaka_1, jiro_suzuki);
}
