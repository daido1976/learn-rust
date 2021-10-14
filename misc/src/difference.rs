#[test]
fn test_difference() {
    // for string
    assert_eq!(
        difference(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec!["a".to_string()]
        ),
        vec!["b".to_string(), "c".to_string()],
    );

    assert_eq!(
        difference(
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "a".to_string()
            ],
            vec!["a".to_string(), "d".to_string()],
        ),
        vec!["b".to_string(), "c".to_string()],
    );

    let empty: Vec<String> = Vec::new();
    assert_eq!(
        difference(
            vec!["a".to_string()],
            vec!["a".to_string(), "b".to_string()]
        ),
        empty,
    );

    // for integer
    assert_eq!(difference(vec![1, 2, 3], vec![1]), vec![2, 3]);
    assert_eq!(difference(vec![1, 2, 3, 1], vec![1, 4]), vec![2, 3]);
    let empty: Vec<i32> = Vec::new();
    assert_eq!(difference(vec![1], vec![1, 2]), empty);
}

#[test]
fn test_difference2() {
    // for string
    assert_eq!(
        difference2(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec!["a".to_string()]
        ),
        vec!["b".to_string(), "c".to_string()],
    );

    assert_eq!(
        difference2(
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "a".to_string()
            ],
            vec!["a".to_string(), "d".to_string()],
        ),
        vec!["b".to_string(), "c".to_string()],
    );

    let empty: Vec<String> = Vec::new();
    assert_eq!(
        difference2(
            vec!["a".to_string()],
            vec!["a".to_string(), "b".to_string()]
        ),
        empty,
    );

    // for integer
    assert_eq!(difference2(vec![1, 2, 3], vec![1]), vec![2, 3]);
    assert_eq!(difference2(vec![1, 2, 3, 1], vec![1, 4]), vec![2, 3]);
    let empty: Vec<i32> = Vec::new();
    assert_eq!(difference2(vec![1], vec![1, 2]), empty);
}

// O(N^2)なので注意
fn difference<T: std::cmp::PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

// HashSet 使う版
// See. https://users.rust-lang.org/t/idiomatic-way-to-get-difference-between-two-vecs/48396/11
fn difference2<T: std::cmp::PartialEq + std::cmp::Eq + std::hash::Hash>(
    a: Vec<T>,
    b: Vec<T>,
) -> Vec<T> {
    let b_set: std::collections::HashSet<_> = b.iter().collect();
    a.into_iter().filter(|x| !b_set.contains(x)).collect()
}
