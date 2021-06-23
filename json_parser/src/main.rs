fn main() {
    println!("Hello, world!");
}
// レスポンスの型は token 型みたいな独自型のベクタにする？
// https://github.com/Byron/json-tools/blob/master/src/lexer.rs
// fn lex(str: String) -> () {
//     str
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lex() {
        // let s = "{\"foo\": [1, 2, {\"bar\": 2}]}".to_string();
        // assert_eq!(
        //     (
        //         "{".to_string(),
        //         "foo".to_string(),
        //         ":".to_string(),
        //         "[".to_string(),
        //         1,
        //         ",".to_string(),
        //         2,
        //         ",".to_string(),
        //         "{".to_string(),
        //         "bar".to_string(),
        //         ":".to_string(),
        //         2,
        //         "}".to_string(),
        //         "]".to_string(),
        //         "}".to_string()
        //     ),
        //     lex(&s)
        // );
        // let s = "{\"foo\": 1".to_string();
        // assert_eq!(
        //     (
        //         "{".to_string(),
        //         "foo".to_string(),
        //         ":".to_string(),
        //         1,
        //         "}".to_string()
        //     ),
        //     lex(&s)
        // );
    }
}
