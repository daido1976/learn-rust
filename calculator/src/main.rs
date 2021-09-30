fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        return 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_calculate() {
        assert_eq!(Solution::calculate("1".to_string()), 1);
    }
}
