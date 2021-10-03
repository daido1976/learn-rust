#![allow(dead_code)]
fn main() {}
struct Solution {}

// See. https://leetcode.com/problems/basic-calculator/description/
// Also see. https://medium.com/@CalvinChankf/solving-basic-calculator-i-ii-iii-on-leetcode-74d926732437
//
// Given a string s representing a valid expression, implement a basic calculator to evaluate it, and return the result of the evaluation.
// Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().
#[test]
fn test_calculate() {
    assert_eq!(Solution::calculate("1 + 1".to_string()), 2);
    assert_eq!(Solution::calculate(" 2-1 + 2 ".to_string()), 3);
    assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
    assert_eq!(Solution::calculate("- (3 + (4 + 5))".to_string()), -12);
}

// LeetCode に提出するのは以下のみ
use std::collections::VecDeque;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut chars = Self::build_chars(s);
        Self::_calculate(&mut chars)
    }

    fn build_chars(s: String) -> VecDeque<char> {
        let mut chars: VecDeque<char> = VecDeque::new();
        for c in s.chars() {
            if c == ' ' {
                // skip whitespace
                continue;
            }
            chars.push_back(c)
        }
        chars
    }

    fn _calculate(chars: &mut VecDeque<char>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut num: i32 = 0;
        let mut sign: i32 = 1;
        loop {
            if let Some(c) = chars.pop_front() {
                if c.is_digit(10) {
                    num = num * 10 + c.to_digit(10).unwrap() as i32;
                } else if c == '+' {
                    stack.push(sign * num);
                    num = 0;
                    sign = 1;
                } else if c == '-' {
                    stack.push(sign * num);
                    num = 0;
                    sign = -1;
                } else if c == '(' {
                    num = Self::_calculate(chars);
                } else if c == ')' {
                    stack.push(sign * num);
                    break;
                }
            } else {
                // when the last char
                stack.push(sign * num);
                break;
            }
        }
        stack.into_iter().sum()
    }
}

// See. https://leetcode.com/problems/two-sum/
//
// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.
#[test]
fn test_two_sum() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}

// LeetCode に提出するのは以下のみ
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut hmap: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            if let Some(j) = hmap.get(&(target - n)) {
                result.push(*j);
                result.push(i as i32);
                break;
            }
            hmap.insert(*n, i as i32);
        }

        result
    }
}
