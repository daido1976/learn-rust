use std::collections::HashMap;

fn main() {
    let s = String::from("hello world");
    let word1 = first_word1(&s);
    let word2 = first_word2(&s);
    println!("{:?}", word1);
    println!("{:?}", word2);
}

fn first_word1(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

fn first_word2(s: &str) -> &str {
    let result = s.split_whitespace().next();
    result.unwrap()
}

fn _count_word(s: &str) -> HashMap<&str, i32> {
    s.split_whitespace()
        .into_iter()
        .fold(HashMap::new(), |mut acc, word| {
            let count = acc.entry(word).or_insert(0);
            *count += 1;
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_word1() {
        let s = "hello world".to_string();
        assert_eq!("hello".to_string(), first_word1(&s));
    }
    #[test]
    fn test_first_word2() {
        let s = "hello world".to_string();
        assert_eq!("hello".to_string(), first_word2(&s));
    }

    #[test]
    fn test_count_word() {
        let s = "no rust no life".to_string();
        let mut expected: HashMap<&str, i32> = HashMap::new();
        expected.insert("no", 2);
        expected.insert("rust", 1);
        expected.insert("life", 1);
        assert_eq!(expected, _count_word(&s));
    }
}
