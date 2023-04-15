use crate::Solution;

// See. https://leetcode.com/problems/maximum-subarray/
//
// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
// A subarray is a contiguous part of an array.
#[test]
fn test_max_sub_array() {
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
    assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
}

// LeetCode に提出するのは以下のみ
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut last = 0;
        for i in nums.iter() {
            last = std::cmp::max(*i, last + *i);
            result = std::cmp::max(result, last);
        }
        result
    }
}
