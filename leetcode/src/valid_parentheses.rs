use crate::Solution;

// See. https://leetcode.com/problems/valid-parentheses/
//
// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
// An input string is valid if:
// 1. Open brackets must be closed by the same type of brackets.
// 1. Open brackets must be closed in the correct order.
#[test]
fn test_is_valid() {
    assert!(Solution::is_valid("()".to_string()));
    assert!(Solution::is_valid("()[]{}".to_string()));
    assert!(!Solution::is_valid("(]".to_string()));
    assert!(!Solution::is_valid("([)]".to_string()));
    assert!(Solution::is_valid("{[]}".to_string()));
    assert!(!Solution::is_valid("((".to_string()));
}

// LeetCode に提出するのは以下のみ
impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' if stack.pop() != Some('(') => return false,
                ']' if stack.pop() != Some('[') => return false,
                '}' if stack.pop() != Some('{') => return false,
                _ => {}
            }
        }

        stack.is_empty()
    }
}
