use crate::Solution;

// See. https://leetcode.com/problems/search-insert-position/
//
// Given a sorted array of distinct integers and a target value,return the index if the target is found.
// If not, return the index where it would be if it were inserted in order.
// You must write an algorithm with O(log n) runtime complexity.
#[test]
fn test_two_sum() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
}

// LeetCode に提出するのは以下のみ
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(n) => n as i32,
            Err(n) => n as i32,
        }
    }
}
