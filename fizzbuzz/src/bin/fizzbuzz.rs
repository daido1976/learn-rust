fn main() {
    for n in 1..=100 {
        println!("{}", fizzbuzz(n))
    }
}

fn fizzbuzz(n: usize) -> String {
    assert!(n > 0);

    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        (_, _) => n.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fizzbuzz() {
        assert_eq!("1".to_string(), fizzbuzz(1));
        assert_eq!("Fizz".to_string(), fizzbuzz(3));
        assert_eq!("Buzz".to_string(), fizzbuzz(5));
        assert_eq!("FizzBuzz".to_string(), fizzbuzz(15));
    }

    #[test]
    #[should_panic]
    fn test_fizzbuzz_given_zero() {
        fizzbuzz(0);
    }
}
