use crate::Solution;

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
                if c.is_ascii_digit() {
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
