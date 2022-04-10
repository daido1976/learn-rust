use crate::Solution;

// See. https://leetcode.com/problems/longest-common-prefix/
//
// Write a function to find the longest common prefix string amongst an array of strings.
// If there is no common prefix, return an empty string "".
#[test]
fn test_two_sum() {
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "flower".to_owned(),
            "flow".to_owned(),
            "flight".to_owned()
        ]),
        "fl".to_owned()
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "dog".to_owned(),
            "racecar".to_owned(),
            "car".to_owned()
        ]),
        "".to_owned()
    );
    assert_eq!(
        Solution::longest_common_prefix(vec!["".to_owned(), "b".to_owned(),]),
        "".to_owned()
    );
    assert_eq!(
        Solution::longest_common_prefix(vec!["cir".to_owned(), "car".to_owned(),]),
        "c".to_owned()
    );
}

// LeetCode に提出するのは以下のみ
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = "".to_owned();
        let mut strs = strs;
        let target_word = strs.pop();
        if let Some(target_word) = target_word {
            for (i, c) in target_word.char_indices() {
                let contains = strs
                    .iter()
                    .all(|s| s.chars().nth(i).unwrap_or_default() == c);
                if contains {
                    result.push(c);
                } else {
                    return result;
                }
            }
        } else {
            return "".to_owned();
        }
        result
    }

    pub fn elegant_longest_common_prefix(strs: Vec<String>) -> String {
        match strs.is_empty() {
            true => "".to_string(),
            _ => strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
                acc.chars()
                    .zip(x.chars())
                    .take_while(|(x, y)| x == y)
                    .map(|(x, _)| x)
                    .collect()
            }),
        }
    }
}
