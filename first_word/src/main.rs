fn main() {
    let s = String::from("hello world");
    let word1 = first_word1(&s);
    let word2 = first_word2(&s);
    println!("{:?}", word1);
    println!("{:?}", word2);
}

fn first_word1(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word2(s: &str) -> &str {
    let result = s.split_whitespace().nth(0);
    result.unwrap()
}
