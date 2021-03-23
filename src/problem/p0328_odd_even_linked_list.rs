/**
 * [328] Odd Even Linked List
 *
 * Given the head of a singly linked list, group all the nodes with odd indices together followed by the nodes with even indices, and return the reordered list.
 * The first node is considered odd, and the second node is even, and so on.
 * Note that the relative order inside both the even and odd groups should remain as it was in the input.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/10/oddeven-linked-list.jpg" style="width: 300px; height: 123px;" />
 * Input: head = [1,2,3,4,5]
 * Output: [1,3,5,2,4]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/10/oddeven2-linked-list.jpg" style="width: 500px; height: 142px;" />
 * Input: head = [2,1,3,5,6,4,7]
 * Output: [2,3,6,7,1,5,4]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the linked list is in the range [0, 10^4].
 * 	-10^6 <= Node.val <= 10^6
 * 
 *  
 * Follow up: Could you solve it in O(1) space complexity and O(nodes) time complexity?
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/odd-even-linked-list/
// discuss: https://leetcode.com/problems/odd-even-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn separate(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        if let None = head {
            return (None, None);
        } else if let None = head.as_ref().unwrap().next {
            return (head, None);
        } else {
            // node -> node1 -> node2 (node2 can be NONE. )
            let mut node:Box<ListNode> = head.unwrap();
            let mut node1:Box<ListNode> = node.next.unwrap();
            let (odd, even) = Self::separate(node1.next);
            node.next = odd;
            node1.next = even;
            return (Some(node), Some(node1));
        }
    }

    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut odd, mut even) = Self::separate(head);
        if let None = odd {
            return None;
        }
        let mut odd_node = &mut odd;
        while odd_node.as_ref().unwrap().next.is_some() {
            odd_node = &mut odd_node.as_mut().unwrap().next;
        }
        odd_node.as_mut().unwrap().next = even;
        return odd;
        // Some(Box::new(ListNode::new(0)))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_328() {
    }
}
