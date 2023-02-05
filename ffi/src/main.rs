fn main() {
    let connection = sqlite::open(":memory:").unwrap();

    let query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
    ";
    connection.execute(query).unwrap();

    connection
        .iterate("SELECT * FROM users WHERE age > 50", |pairs| {
            println!("{:?}", pairs);
            true
        })
        .unwrap();
}
