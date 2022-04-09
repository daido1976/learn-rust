use crate::Solution;

// See. https://leetcode.com/problems/roman-to-integer/
#[test]
fn test_roman_to_int() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}

// LeetCode に提出するのは以下のみ
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        const END_OF_CHARS: char = '0';
        let mut result = 0;
        let mut two_chars = false;
        let next_char = |i| s.char_indices().nth(i + 1).map_or(END_OF_CHARS, |n| n.1);

        for (i, c) in s.char_indices() {
            if two_chars {
                two_chars = false;
                continue;
            }

            let n = match c {
                'I' => {
                    let next = next_char(i);
                    if next == 'V' {
                        two_chars = true;
                        4
                    } else if next == 'X' {
                        two_chars = true;
                        9
                    } else {
                        1
                    }
                }
                'V' => 5,
                'X' => {
                    let next = next_char(i);
                    if next == 'L' {
                        two_chars = true;
                        40
                    } else if next == 'C' {
                        two_chars = true;
                        90
                    } else {
                        10
                    }
                }
                'L' => 50,
                'C' => {
                    let next = next_char(i);
                    if next == 'D' {
                        two_chars = true;
                        400
                    } else if next == 'M' {
                        two_chars = true;
                        900
                    } else {
                        100
                    }
                }
                'D' => 500,
                'M' => 1000,
                _ => panic!("Constraint violation."),
            };
            result += n;
        }
        result
    }
}
