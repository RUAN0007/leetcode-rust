/**
 * [92] Reverse Linked List II
 *
 * Given the head of a singly linked list and two integers left and right where left <= right, reverse the nodes of the list from position left to position right, and return the reversed list.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev2ex2.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5], left = 2, right = 4
 * Output: [1,4,3,2,5]
 * 
 * Example 2:
 * 
 * Input: head = [5], left = 1, right = 1
 * Output: [5]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is n.
 * 	1 <= n <= 500
 * 	-500 <= Node.val <= 500
 * 	1 <= left <= right <= n
 * 
 *  
 * Follow up: Could you do it in one pass?
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/reverse-linked-list-ii/
// discuss: https://leetcode.com/problems/reverse-linked-list-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
    fn reverse(mut head : Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reversed : Option<Box<ListNode>> = None;

        while let Some(mut node) = head {
            head = node.next;
            node.next = reversed;
            reversed = Some(node);
        }

        reversed
    }

    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let left : usize = left as usize;
        let right : usize = right as usize;

        let mut fake_head : ListNode = ListNode::new(0);
        fake_head.next = head;

        let mut front_tail : &mut ListNode = &mut fake_head;

        for i in 0..(left-1) {
            front_tail = front_tail.next.as_mut().unwrap();
        }

        let mut to_reversed : Box<ListNode> = front_tail.next.take().unwrap();

        let mut reversed_tail : &mut ListNode = &mut to_reversed;
        for i in 0..(right - left) {
            reversed_tail = reversed_tail.next.as_mut().unwrap();
        }

        let to_append : Option<Box<ListNode>> = reversed_tail.next.take();

        let mut reversed : Box<ListNode> = Self::reverse(Some(to_reversed)).unwrap();

        let mut reversed_tail : &mut ListNode = &mut reversed;
        while let Some(ref mut next_node) = reversed_tail.next {
            reversed_tail = next_node;
        }
        reversed_tail.next = to_append;

        let mut front_tail : &mut ListNode = &mut fake_head;
        while let Some(ref mut next_node) = front_tail.next {
            front_tail = next_node;
        }
        front_tail.next = Some(reversed);

        fake_head.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_92() {
        assert_eq!(Solution::reverse_between(linked![1,2,3,4,5],2,4), linked![1,4,3,2,5]);
        assert_eq!(Solution::reverse_between(linked![5],1,1), linked![5]);
    }
}
