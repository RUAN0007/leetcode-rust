/**
 * [19] Remove Nth Node From End of List
 *
 * Given the head of a linked list, remove the n^th node from the end of the list and return its head.
 * Follow up: Could you do this in one pass?
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/remove_ex1.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5], n = 2
 * Output: [1,2,3,5]
 * 
 * Example 2:
 * 
 * Input: head = [1], n = 1
 * Output: []
 * 
 * Example 3:
 * 
 * Input: head = [1,2], n = 1
 * Output: [1]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is sz.
 * 	1 <= sz <= 30
 * 	0 <= Node.val <= 100
 * 	1 <= n <= sz
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/remove-nth-node-from-end-of-list/
// discuss: https://leetcode.com/problems/remove-nth-node-from-end-of-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut node : &Option<Box<ListNode>> = &head;
        let mut node_count = 0usize;
        while node.is_some() {
            node_count += 1;
            node = &(node.as_ref().unwrap().next);
        }

        let n = n as usize;
        let mut head = head;
        if node_count == n {
            // we are removing the head. 
            return head.unwrap().next;
        } else {
            let mut idx : usize = node_count - n - 1;
            // Find the preceding of the removed node. 
            let mut node : &mut Option<Box<ListNode>> = &mut head;

            while 0 < idx {
                node = &mut (node.as_mut().unwrap().next);
                idx -= 1;
            }
            // Since we are manipulating via mut ref, must take() to move the content out. 
            let next_node = node.as_mut().unwrap().next.as_mut().unwrap().next.take();
            node.as_mut().unwrap().next = next_node;

            return head;
        }

    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_19() {
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
        assert_eq!(Solution::remove_nth_from_end(to_list(vec![1]), 1), None);
    }
}
