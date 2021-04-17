/**
 * [234] Palindrome Linked List
 *
 * Given the head of a singly linked list, return true if it is a palindrome.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/03/pal1linked-list.jpg" style="width: 422px; height: 62px;" />
 * Input: head = [1,2,2,1]
 * Output: true
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/03/pal2linked-list.jpg" style="width: 182px; height: 62px;" />
 * Input: head = [1,2]
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is in the range [1, 10^5].
 * 	0 <= Node.val <= 9
 * 
 *  
 * Follow up: Could you do it in O(n) time and O(1) space?
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/palindrome-linked-list/
// discuss: https://leetcode.com/problems/palindrome-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn reverse(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reversed : Option<Box<ListNode>> = None;
        let mut unreversed : &Option<Box<ListNode>> = head;
        while let Some(ref node) = unreversed {
            unreversed = &(node.next);
            let mut new_node : Box<ListNode> = Box::new(ListNode::new(node.val));
            new_node.next = reversed;
            reversed = Some(new_node);
        }
        reversed
    }

    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let reversed : Option<Box<ListNode>> = Self::reverse(&head);
        let mut node : &Option<Box<ListNode>> = &head;
        let mut rev_node : &Option<Box<ListNode>> = &reversed;

        while node.is_some() {
            if node.as_ref().unwrap().val != rev_node.as_ref().unwrap().val {return false;}
            node = &node.as_ref().unwrap().next;
            rev_node = &rev_node.as_ref().unwrap().next;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_234() {
        assert!(Solution::is_palindrome(linked![1, 2, 2,1]));
        assert!(!Solution::is_palindrome(linked![1, 2]));
    }
}
