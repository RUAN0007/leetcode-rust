/**
 * [61] Rotate List
 *
 * Given the head of a linked list, rotate the list to the right by k places.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/rotate1.jpg" style="width: 600px; height: 254px;" />
 * Input: head = [1,2,3,4,5], k = 2
 * Output: [4,5,1,2,3]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/roate2.jpg" style="width: 472px; height: 542px;" />
 * Input: head = [0,1,2], k = 4
 * Output: [2,0,1]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is in the range [0, 500].
 * 	-100 <= Node.val <= 100
 * 	0 <= k <= 2 * 10^9
 * 
 */
pub struct Solution {}
use std::borrow::BorrowMut;

use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/rotate-list/
// discuss: https://leetcode.com/problems/rotate-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if let None = head {
            return None;
        }
        let mut head = head;
        let mut k_ahead = head.as_ref().unwrap();
        let mut ki = k;
        let mut l = 1;
        // k_ahead points to the k-th node with head as 0-th, possibly rotating. 
        while 0 < ki {
            match(k_ahead.next) {
                None => {
                    break;
                },
                Some(ref node) => {
                    k_ahead = node
                }
            }
            ki-=1;
            l+=1;
        }

        // Count the steps to advance k_ahead until k_ahead is the last
        let mut steps_to_tail = 0; // f
        if ki == 0 {
            while k_ahead.next.is_some() {
                steps_to_tail+=1;
                k_ahead = k_ahead.next.as_ref().unwrap();
            }
        } else {
            steps_to_tail = l - k % l - 1;
        }
        // println!("l = {}, k = {}, steps_to_tail = {}", l, k, steps_to_tail);

        // Advance new_tail to the steps_to_tail-th node. 
        let mut new_tail = head.as_mut().unwrap();
        for i in 0..steps_to_tail {
            new_tail = new_tail.next.as_mut().unwrap();
        }

        // does not rotate at all, due to k = list.len. 
        if new_tail.as_ref().next.is_none() {
            return head;
        }

        let mut new_head = new_tail.next.take();
        let mut old_tail = new_head.as_mut().unwrap();
        while old_tail.next.is_some() {
            old_tail = old_tail.next.as_mut().unwrap();
        }

        old_tail.next = head;
        return new_head;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_61() {
        assert_eq!(
            Solution::rotate_right(linked![0, 1, 2], 4),
            linked![2, 0, 1]
        );

    }
}
