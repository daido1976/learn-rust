#[test]
fn test_mutable() {
    let a = "abc".to_string();
    mut_f(a.to_owned());
    assert_eq!(a, "abc");

    let mut a = "abc".to_string();
    mutref_f(&mut a);
    assert_eq!(a, "abcd");
}

// 「mut v: T」は可変な変数を引数に取る
fn mut_f(mut s: String) {
    s.push('d');
}

// 「v: &mut T」は可変参照を引数に取る（元の値を書き換え可能）
fn mutref_f(s: &mut String) {
    s.push('d');
}
