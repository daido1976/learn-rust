use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FullName {
    first_name: Name,
    last_name: Name,
}

impl FullName {
    pub fn new(first_name: Name, last_name: Name) -> Self {
        Self {
            first_name,
            last_name,
        }
    }

    pub fn first_name(&self) -> String {
        self.first_name.0.clone()
    }

    pub fn last_name(&self) -> String {
        self.last_name.0.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(String);

impl FromStr for Name {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r#"^[a-zA-Z]+$"#).unwrap();
        if regex.is_match(s) {
            Ok(Name(s.to_string()))
        } else {
            Err("許可されていない文字が使われています".to_string())
        }
    }
}

#[test]
fn show_names() {
    let first_name = "taro".parse().unwrap();
    let last_name = "tanaka".parse().unwrap();
    let full_name = FullName::new(first_name, last_name);

    println!("full_name: {:?}", full_name);
    println!("first_name: {:?}", full_name.first_name());
    println!("last_name: {:?}", full_name.last_name());
}

#[test]
fn test_parse_name() {
    let valid_name = "taro".parse::<Name>();
    assert!(valid_name.is_ok());

    let invalid_name = "taro123".parse::<Name>();
    assert!(invalid_name.is_err());

    let invalid_name = "太郎".parse::<Name>();
    assert!(invalid_name.is_err());
}

use std::marker::PhantomData;
use std::ops::Add;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Money<T> {
    amount: u32,
    currency: PhantomData<T>,
}

impl<T> Money<T> {
    fn new(amount: u32) -> Self {
        Self {
            amount,
            currency: PhantomData::<T>,
        }
    }
}

impl<T> Add for Money<T> {
    type Output = Money<T>;

    fn add(self, other: Money<T>) -> Self::Output {
        Self::new(self.amount + other.amount)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Jpy;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Usd;

#[test]
fn test_phantom_money() {
    let jpy_1 = Money::<Jpy>::new(100);
    let jpy_2 = Money::<Jpy>::new(200);

    let _usd = Money::<Usd>::new(300);

    let result = jpy_1 + jpy_2; // コンパイルOk
    assert_eq!(result, Money::<Jpy>::new(300));
    // let invalid_result = jpy_1 + _usd; //コンパイルエラー
}
