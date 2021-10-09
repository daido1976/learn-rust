#[derive(Clone, Debug, Eq)]
pub struct User {
    id: UserId,
    name: Name,
}

impl User {
    pub fn new(id: UserId, name: Name) -> Self {
        Self { id, name }
    }

    pub fn update_name(&mut self, name: Name) {
        self.name = name;
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserId(String);

impl UserId {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
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
fn test_user_eq() {
    let before = User::new(UserId::new("user_id"), Name::new("before_name").unwrap());
    let mut after = before.clone();
    after.update_name(Name::new("after_name").unwrap());
    // Only UserId is compared to see if the users are the same, Name is not compared.
    assert_eq!(before, after);
}
