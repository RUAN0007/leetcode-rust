/**
 * [148] Sort List
 *
 * Given the head of a linked list, return the list after sorting it in ascending order.
 * Follow up: Can you sort the linked list in O(n logn) time and O(1) memory (i.e. constant space)?
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/14/sort_list_1.jpg" style="width: 450px; height: 194px;" />
 * Input: head = [4,2,1,3]
 * Output: [1,2,3,4]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/14/sort_list_2.jpg" style="width: 550px; height: 184px;" />
 * Input: head = [-1,5,3,4,0]
 * Output: [-1,0,3,4,5]
 * 
 * Example 3:
 * 
 * Input: head = []
 * Output: []
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is in the range [0, 5 * 10^4].
 * 	-10^5 <= Node.val <= 10^5
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/sort-list/
// discuss: https://leetcode.com/problems/sort-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn merge_sorted(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match(l1,l2) {
            (None, None)=>None,
            (Some(l1_node), None) => {Some(l1_node)},
            (None, Some(l2_node)) => {Some(l2_node)},
            (Some(mut l1_node), Some(mut l2_node)) => {
                if l1_node.val < l2_node.val {
                    l1_node.next = Self::merge_sorted(l1_node.next, Some(l2_node));
                    Some(l1_node)
                } else {
                    l2_node.next = Self::merge_sorted(Some(l1_node), l2_node.next);
                    Some(l2_node)
                }
            },
        }
    }

    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {return None;}
        if head.as_ref().unwrap().next.is_none() {return head;}
        let mut list_len = 0;
        let mut cur_head = head.as_ref();
        while let Some(cur_node) = cur_head {
            cur_head = cur_node.next.as_ref();
            list_len += 1;
        }

        let mut step_to_mid = (list_len - 1) / 2;
        let mut first_half_tail = head.as_mut();
        while 0 < step_to_mid {
            first_half_tail = first_half_tail.unwrap().next.as_mut();
            step_to_mid-=1;
        }

        let sec_half = first_half_tail.unwrap().next.take();
        let sorted_sec_half = Self::sort_list(sec_half);
        let sorted_first_half = Self::sort_list(head);
        Self::merge_sorted(sorted_first_half, sorted_sec_half)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_148() {
        assert_eq!(
            Solution::sort_list(linked![4, 2, 1, 3]),
            linked![1, 2, 3, 4]
        );
        // assert_eq!(
        //     Solution::sort_list(linked![-1, 5, 3, 4, 0]),
        //     linked![-1, 0, 3, 4, 5]
        // );
        // assert_eq!(Solution::sort_list(linked![]), linked![]);
    }
}
