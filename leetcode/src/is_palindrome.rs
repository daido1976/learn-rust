use crate::Solution;

// See. https://leetcode.com/problems/palindrome-number/
//
// Given an integer x, return true if x is palindrome integer.
// An integer is a palindrome when it reads the same backward as forward. For example, 121 is palindrome while 123 is not.
#[test]
fn test_is_palindrome() {
    assert!(Solution::is_palindrome(121));
    assert!(!Solution::is_palindrome(-121));
    assert!(!Solution::is_palindrome(10));
}

// LeetCode に提出するのは以下のみ
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }
        if x.to_string().chars().rev().collect::<String>() == x.to_string() {
            return true;
        }
        false
    }
}
