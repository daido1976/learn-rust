use crate::Solution;

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
        // One-pass Hash Table
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
