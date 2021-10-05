use std::collections::HashMap;

#[test]
fn test_count_word() {
    let s = "no rust no life".to_string();
    let mut expected: HashMap<&str, i32> = HashMap::new();
    expected.insert("no", 2);
    expected.insert("rust", 1);
    expected.insert("life", 1);
    assert_eq!(count_word(&s), expected);
}

fn count_word(s: &str) -> HashMap<&str, i32> {
    s.split_whitespace()
        .into_iter()
        .fold(HashMap::new(), |mut acc, word| {
            let count = acc.entry(word).or_insert(0);
            *count += 1;
            acc
        })
}
