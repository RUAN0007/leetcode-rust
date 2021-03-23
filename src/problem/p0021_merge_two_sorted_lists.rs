/**
 * [21] Merge Two Sorted Lists
 *
 * Merge two sorted linked lists and return it as a sorted list. The list should be made by splicing together the nodes of the first two lists.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/merge_ex1.jpg" style="width: 662px; height: 302px;" />
 * Input: l1 = [1,2,4], l2 = [1,3,4]
 * Output: [1,1,2,3,4,4]
 * 
 * Example 2:
 * 
 * Input: l1 = [], l2 = []
 * Output: []
 * 
 * Example 3:
 * 
 * Input: l1 = [], l2 = [0]
 * Output: [0]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in both lists is in the range [0, 50].
 * 	-100 <= Node.val <= 100
 * 	Both l1 and l2 are sorted in non-decreasing order.
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/merge-two-sorted-lists/
// discuss: https://leetcode.com/problems/merge-two-sorted-lists/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(mut l1_node), Some(mut l2_node)) =>{
                if (l1_node.val <= l2_node.val) {
                    l1_node.next = Self::merge_two_lists(l1_node.next, Some(l2_node));
                    Some(l1_node)
                } else {
                    l2_node.next = Self::merge_two_lists(Some(l1_node), l2_node.next);
                    Some(l2_node)
                }
            },
            (Some(l1_node), None) => Some(l1_node),
            (None, Some(l2_node)) => Some(l2_node),
            (None, None) => None
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_21() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
    }
}
