fn main() {
    fizzbuzz()
}

fn fizzbuzz() {
    let fizzes = std::iter::repeat("")
        .take(2)
        .chain(std::iter::once("Fizz"))
        .cycle();
    let buzzes = std::iter::repeat("")
        .take(4)
        .chain(std::iter::once("Buzz"))
        .cycle();
    let fizzbuzzes = fizzes.zip(buzzes);
    let fizzbuzz = (1..=100).zip(fizzbuzzes).map(|tupple| match tupple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });

    for line in fizzbuzz {
        println!("{}", line)
    }
}
