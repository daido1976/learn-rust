use crate::Solution;

// See. https://leetcode.com/problems/merge-two-sorted-lists/
//
// Merge two sorted linked lists and return it as a sorted list. The list should be made by splicing together the nodes of the first two lists.
#[test]
fn test_merge_two_lists() {
    let input1 = Some(Box::new(ListNode::new(1)));
    let input2 = Some(Box::new(ListNode::new(2)));
    let node = ListNode {
        next: Some(Box::new(ListNode::new(2))),
        val: 1,
    };
    let expected = Some(Box::new(node));
    assert_eq!(Solution::merge_two_lists(input1, input2), expected);
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// LeetCode に提出するのは以下のみ
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (Some(l), Some(r)) => {
                if l.val <= r.val {
                    Some(Box::new(ListNode {
                        next: Self::merge_two_lists(l.next, Some(r)),
                        val: l.val,
                    }))
                } else {
                    Some(Box::new(ListNode {
                        next: Self::merge_two_lists(Some(l), r.next),
                        val: r.val,
                    }))
                }
            }
        }
    }
}
