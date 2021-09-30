use std::usize;

fn main() {
    for n in 1..=100 {
        println!("{}", fizzbuzz(n))
    }
}

fn fizzbuzz(n: usize) -> String {
    assert!(n > 0);

    if n % 15 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        n.to_string()
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
