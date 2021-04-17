/**
 * [82] Remove Duplicates from Sorted List II
 *
 * Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list. Return the linked list sorted as well.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/linkedlist1.jpg" style="width: 500px; height: 142px;" />
 * Input: head = [1,2,3,3,4,4,5]
 * Output: [1,2,5]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/linkedlist2.jpg" style="width: 500px; height: 205px;" />
 * Input: head = [1,1,1,2,3]
 * Output: [2,3]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is in the range [0, 300].
 * 	-100 <= Node.val <= 100
 * 	The list is guaranteed to be sorted in ascending order.
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/
// discuss: https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {return None;}

        let mut fake_list : Option<Box<ListNode>> =  Some(Box::new(ListNode::new(0)));
        let mut fake_tail : &mut Box<ListNode> = fake_list.as_mut().unwrap();

        let mut prev_val : i32 = head.as_ref().unwrap().val;
        let mut has_dup = false;

        let mut cur_node : &Option<Box<ListNode>> = &(head.as_ref().unwrap().next);
        while cur_node.is_some() {
            let cur_val = cur_node.as_ref().unwrap().val;
            if cur_val == prev_val {
                has_dup = true;
            } else if has_dup {
                prev_val = cur_val;
                has_dup = false;
            } else {
                fake_tail.next = Some(Box::new(ListNode::new(prev_val)));
                fake_tail = fake_tail.next.as_mut().unwrap();
                
                prev_val = cur_val;
                has_dup = false;
            }
            cur_node = &(cur_node.as_ref().unwrap().next);
        }
        if !has_dup {
            fake_tail.next = Some(Box::new(ListNode::new(prev_val)));
        }
        fake_list.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_82() {
        assert_eq!(Solution::delete_duplicates(to_list(vec![1,2,3,3,4,4,5])), to_list(vec![1,2,5]));
    }
}
