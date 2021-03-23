/**
 * [445] Add Two Numbers II
 *
 * You are given two non-empty linked lists representing two non-negative integers. The most significant digit comes first and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.
 * 
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 * 
 * Follow up:<br />
 * What if you cannot modify the input lists? In other words, reversing the lists is not allowed.
 * 
 * 
 * 
 * Example:
 * 
 * Input: (7 -> 2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 8 -> 0 -> 7
 * 
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/add-two-numbers-ii/
// discuss: https://leetcode.com/problems/add-two-numbers-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn add_equal_len(mut l1: Box<ListNode>, mut l2: Box<ListNode>) -> (i32, Box<ListNode>) {
        // let head = Box::new(ListNode::new());
        if l1.next.is_some() && l2.next.is_some() {
            let (prev_carry, mut prev_head) = Self::add_equal_len(l1.next.unwrap(), l2.next.unwrap());
            let carry = (l1.val + l2.val + prev_carry) / 10;
            let val = (l1.val + l2.val + prev_carry) % 10;
            let mut this_node = Box::new(ListNode::new(val));
            this_node.next = Some(prev_head);
            return (carry, this_node);
        } else {
            let carry = (l1.val + l2.val) / 10;
            let val = (l1.val + l2.val) % 10;
            let mut this_node = Box::new(ListNode::new(val));
            return (carry, this_node);
        }
    }

    pub fn add_carry_append_tail(l1: Option<Box<ListNode>>, carry: i32, tail_head: Box<ListNode>) -> (i32, Option<Box<ListNode>>) {
        if let Some(l1_node) = l1 {
            if let None = l1_node.next {
                let this_carry = (carry + l1_node.val) / 10;
                let this_val = (carry + l1_node.val) % 10;
                let mut this_node = Box::new(ListNode::new(this_val));
                this_node.next = Some(tail_head);
                return (this_carry, Some(this_node));
            } else {
                let (prev_carry, prev_tail) = Self::add_carry_append_tail(l1_node.next, carry, tail_head);

                let this_carry = (prev_carry + l1_node.val) / 10;
                let this_val = (prev_carry + l1_node.val) % 10;
                let mut this_node = Box::new(ListNode::new(this_val));
                this_node.next = prev_tail;
                return (this_carry, Some(this_node));
            }
        } else if 0 < carry {
            let mut this_node = Some(Box::new(ListNode::new(carry)));
            this_node.as_mut().unwrap().next = Some(tail_head);
            return (0, this_node);
        } else {
            return (0, Some(tail_head));
        }
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Find the size of two lists. 
        let (mut l1_size, mut l2_size) = (0, 0);
        let mut l1_node = l1.as_ref();
        while let Some(node) = l1_node {
            l1_size+=1;
            l1_node = node.next.as_ref();
        }

        let mut l2_node = l2.as_ref();
        while let Some(node) = l2_node {
            l2_size+=1;
            l2_node = node.next.as_ref();
        }

        let (mut shorter, mut longer) : (Option<Box<ListNode>>, Option<Box<ListNode>>);
        let mut diff = 0;
        if l1_size < l2_size {
            shorter = l1;
            longer = l2;
            diff = l2_size - l1_size;
        } else {
            shorter = l2;
            longer = l1;
            diff = l1_size - l2_size;
        }

        // Truncate the longer list into two parts. The second is of equal size of the smaller list. Box<ListNode>
        // let mut first_part_tail = longer.as_mut().unwrap();
        let first_part_head : Option<Box<ListNode>>;
        let sec_part_head : Option<Box<ListNode>>;
        if diff == 0 {
            first_part_head = None;
            sec_part_head = longer;
        } else {
            diff -=1;
            let mut first_part_tail = longer.as_mut().unwrap();
            while 0 < diff {
                // println!("diff = {}, node val = {}", diff, first_part_tail.val);
                first_part_tail = first_part_tail.next.as_mut().unwrap();
                diff -=1;
            }  // end while
            sec_part_head = first_part_tail.next.take();
            first_part_head = longer;
        }

        // Sum two lists of equal length
        let (carry, tail_node) = Self::add_equal_len(sec_part_head.unwrap(),shorter.unwrap());
        // Add carry to the first halve and append the above list to the tail. 
        let (carry, head) = Self::add_carry_append_tail(first_part_head, carry, tail_node);
        if 0 < carry  {
            let mut this_node = Box::new(ListNode::new(carry));
            this_node.next = head;
            return Some(this_node);
        } else {
            return head;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_445() {
        // assert_eq!(
        //     Solution::add_two_numbers(linked![7, 2, 4, 3], linked![5, 6, 4]),
        //     linked![7, 8, 0, 7]
        // );
        assert_eq!(
            Solution::add_two_numbers(linked![9, 1, 6], linked![0]),
            linked![9, 1, 6]
        )

    }
}
