#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    name: Name,
}

impl User {
    pub fn new(name: Name) -> Self {
        Self { name }
    }

    pub fn change_name(&mut self, name: Name) {
        self.name = name;
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(String);
type MyError = String;

impl Name {
    pub fn new(s: &str) -> Result<Self, MyError> {
        if s.chars().count() < 3 {
            return Err("ユーザー名は3文字以上です".to_string());
        }
        Ok(Name(s.to_string()))
    }
}
#[test]
fn test_change_name_success() {
    let name = Name::new("daido1976").unwrap();
    let mut user = User::new(name);
    assert_eq!(user.name().0.to_string(), "daido1976".to_string());

    let name = Name::new("updated daido1976").unwrap();
    user.change_name(name);
    assert_eq!(user.name().0.to_string(), "updated daido1976".to_string());
}
