use std::collections::HashMap;

/// Extracts a column of values from a vector of hashmaps.
fn array_column<K, V>(array: &[HashMap<K, V>], key: &K) -> Vec<V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    array
        .iter()
        .filter_map(|map| map.get(key).cloned())
        .collect()
}

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Int(i32),
    Str(&'static str),
}

#[test]
fn test_array_column() {
    let mut user1 = HashMap::new();
    user1.insert("id", Value::Int(1));
    user1.insert("name", Value::Str("Alice"));
    user1.insert("email", Value::Str("alice@example.com"));

    let mut user2 = HashMap::new();
    user2.insert("id", Value::Int(2));
    user2.insert("name", Value::Str("Bob"));
    user2.insert("email", Value::Str("bob@example.com"));

    let mut user3 = HashMap::new();
    user3.insert("id", Value::Int(3));
    user3.insert("name", Value::Str("Charlie"));
    user3.insert("email", Value::Str("charlie@example.com"));

    let users = vec![user1, user2, user3];

    // Test extracting "name" column
    let names: Vec<Value> = array_column(&users, &"name");
    assert_eq!(
        names,
        vec![
            Value::Str("Alice"),
            Value::Str("Bob"),
            Value::Str("Charlie")
        ]
    );

    // Test extracting "id" column
    let ids: Vec<Value> = array_column(&users, &"id");
    assert_eq!(ids, vec![Value::Int(1), Value::Int(2), Value::Int(3)]);

    // Test extracting non-existent column
    let non_existent: Vec<Value> = array_column(&users, &"non_existent");
    assert!(non_existent.is_empty());
}
