/**
 * [143] Reorder List
 *
 * You are given the head of a singly linked-list. The list can be represented as:
 * 
 * L0 &rarr; L1 &rarr; &hellip; &rarr; Ln - 1 &rarr; Ln
 * 
 * Reorder the list to be on the following form:
 * 
 * L0 &rarr; Ln &rarr; L1 &rarr; Ln - 1 &rarr; L2 &rarr; Ln - 2 &rarr; &hellip;
 * 
 * You may not modify the values in the list's nodes. Only nodes themselves may be changed.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/04/reorder1linked-list.jpg" style="width: 422px; height: 222px;" />
 * Input: head = [1,2,3,4]
 * Output: [1,4,2,3]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/09/reorder2-linked-list.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5]
 * Output: [1,5,2,4,3]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is in the range [1, 5 * 10^4].
 * 	1 <= Node.val <= 1000
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/reorder-list/
// discuss: https://leetcode.com/problems/reorder-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reversed = None;
        let mut unreversed = head;
        while let Some(mut unreversed_node) = unreversed {
            unreversed = unreversed_node.next;
            unreversed_node.next = reversed;
            reversed = Some(unreversed_node);
        }
        reversed
    }

    pub fn merge(mut l1 : Option<Box<ListNode>>, mut l2 : Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // take turn to merge, where l1 is the first. 
        match(l1,l2) {
            (None, None) => None,
            (None, Some(l2_head)) => Some(l2_head),
            (Some(l1_head), None) => Some(l1_head),
            (Some(mut l1_head), Some(l2_head)) => {
                l1_head.next = Self::merge(Some(l2_head), l1_head.next);
                Some(l1_head)
            }
        }
    }

    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // Separate list into two halves. 
        if let None = head {return} 
        let mut cur_node = head.as_ref().unwrap();
        let mut list_size = 1;
        while cur_node.next.is_some() {
            cur_node = cur_node.next.as_ref().unwrap();
            list_size+=1;
        }
        let mut step_to_mid = (list_size-1) / 2;

        let mut first_half_tail = head.as_mut().unwrap();
        while 0 < step_to_mid {
            first_half_tail = first_half_tail.next.as_mut().unwrap();
            step_to_mid-=1;
        }
        let mut sec_half = first_half_tail.next.take();
        let mut sec_half_reversed = Self::reverse(sec_half);

        head.as_mut().unwrap().next = Self::merge(sec_half_reversed, head.as_mut().unwrap().next.take());
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_143() {}
}
