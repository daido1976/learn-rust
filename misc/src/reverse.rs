// See. https://x.com/j5ik2o/status/1705028271241642247
#[test]
fn test_reverse_string() {
    assert_eq!(reverse_string("bar"), "rab");
    assert_eq!(reverse_string("hoge"), "egoh");
    assert_eq!(reverse_string("h"), "h");
    assert_eq!(reverse_string(""), "");
}

// O(n) だが、実際の計算量は O(n/2)
fn reverse_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    for i in 0..(len / 2) {
        let last_index = len - 1;
        chars.swap(i, last_index - i);
    }
    chars.into_iter().collect()
}
