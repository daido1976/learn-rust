use crate::Solution;

// See. https://leetcode.com/problems/roman-to-integer/
//
// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII.
// Instead, the number four is written as IV. Because the one is before the five we subtract it making four.
// The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
//
// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.
#[test]
fn test_roman_to_int() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}

// LeetCode に提出するのは以下のみ
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars().rfold(0, |acc, c| {
            acc + match c {
                // when next char V or X
                'I' if acc >= 5 => -1,
                'I' => 1,
                'V' => 5,
                // when next char L or C
                'X' if acc >= 50 => -10,
                'X' => 10,
                'L' => 50,
                // when next char D or M
                'C' if acc >= 500 => -100,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        })
    }
}
