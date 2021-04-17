/**
 * [25] Reverse Nodes in k-Group
 *
 * Given a linked list, reverse the nodes of a linked list k at a time and return its modified list.
 * k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.
 * Follow up:
 * 
 * 	Could you solve the problem in O(1) extra memory space?
 * 	You may not alter the values in the list's nodes, only nodes itself may be changed.
 * 
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/reverse_ex1.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5], k = 2
 * Output: [2,1,4,3,5]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/reverse_ex2.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5], k = 3
 * Output: [3,2,1,4,5]
 * 
 * Example 3:
 * 
 * Input: head = [1,2,3,4,5], k = 1
 * Output: [1,2,3,4,5]
 * 
 * Example 4:
 * 
 * Input: head = [1], k = 1
 * Output: [1]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is in the range sz.
 * 	1 <= sz <= 5000
 * 	0 <= Node.val <= 1000
 * 	1 <= k <= sz
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/reverse-nodes-in-k-group/
// discuss: https://leetcode.com/problems/reverse-nodes-in-k-group/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut reversed = None;
        let mut unreversed = head;
        while let Some(mut unreversed_node) = unreversed {
            unreversed = unreversed_node.next;
            unreversed_node.next = reversed;
            reversed = Some(unreversed_node);
        }

        reversed
    }

    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {return None}

       let mut fake_head = Some(Box::new(ListNode::new(0)));
       let mut processed_tail : &mut Box<ListNode> = fake_head.as_mut().unwrap();

       let mut cur_head : Option<Box<ListNode>> = head;

        // println!("cur_head={}", cur_node.val);
       let mut cur_node : &mut Box<ListNode> = cur_head.as_mut().unwrap();

       let mut count = (k - 1) as usize;
       while cur_node.next.is_some() && count > 0 {
        //    println!("\tcur_node={}", cur_node.val);
           cur_node = cur_node.next.as_mut().unwrap();
           count -= 1;
       }
        // println!("tail_node={}", cur_node.val);
    //    println!("cur_head={}", cur_head.as_ref().unwrap().val);
       if count == 0 {
            // there are more than k elements later.
            let next_head = cur_node.next.take();
            processed_tail.next = Self::reverse_list(cur_head);

            while processed_tail.next.is_some() {
                // println!("\tprocessed_tail={}", processed_tail.val);
                processed_tail = processed_tail.next.as_mut().unwrap();
            }
            processed_tail.next = Self::reverse_k_group(next_head, k);
            fake_head.unwrap().next

       } else {
           cur_head
       }

    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_25() {
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![2, 1, 4, 3, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 3),
            to_list(vec![3, 2, 1, 4, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 5),
            to_list(vec![5, 4, 3, 2, 1])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1]), 1),
            to_list(vec![1])
        );
    }
}
