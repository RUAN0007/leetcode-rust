/**
 * [23] Merge k Sorted Lists
 *
 * You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
 * Merge all the linked-lists into one sorted linked-list and return it.
 *  
 * Example 1:
 * 
 * Input: lists = [[1,4,5],[1,3,4],[2,6]]
 * Output: [1,1,2,3,4,4,5,6]
 * Explanation: The linked-lists are:
 * [
 *   1->4->5,
 *   1->3->4,
 *   2->6
 * ]
 * merging them into one sorted list:
 * 1->1->2->3->4->4->5->6
 * 
 * Example 2:
 * 
 * Input: lists = []
 * Output: []
 * 
 * Example 3:
 * 
 * Input: lists = [[]]
 * Output: []
 * 
 *  
 * Constraints:
 * 
 * 	k == lists.length
 * 	0 <= k <= 10^4
 * 	0 <= lists[i].length <= 500
 * 	-10^4 <= lists[i][j] <= 10^4
 * 	lists[i] is sorted in ascending order.
 * 	The sum of lists[i].length won't exceed 10^4.
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/merge-k-sorted-lists/
// discuss: https://leetcode.com/problems/merge-k-sorted-lists/discuss/?currentPage=1&orderBy=most_votes&query=

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

    pub fn merge_two(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>)-> Option<Box<ListNode>> {
        match(l1,l2) {
            (None, None)=>{None},
            (None, Some(l2_node))=>{Some(l2_node)},
            (Some(l1_node), None)=>{Some(l1_node)},
            (Some(mut l1_node), Some(mut l2_node))=>{
                if l1_node.val < l2_node.val {
                    l1_node.next = Self::merge_two(l1_node.next, Some(l2_node));
                    Some(l1_node)
                } else {
                    l2_node.next = Self::merge_two(Some(l1_node), l2_node.next);
                    Some(l2_node)
                }
            }
        }
    }

    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        
        if lists.len() == 0 {
            None
        } else if lists.len() == 1 {
            lists[0].take()
        } else if lists.len() == 2 {
            Self::merge_two(lists[0].take(), lists[1].take())
        } else {
            let mid = lists.len() / 2;
            let mut first_half = vec![];
            let mut sec_half = vec![];
            for i in 0..lists.len() {
                if i < mid {
                    first_half.push(lists[i].take());
                } else {
                    sec_half.push(lists[i].take());
                }
            }

            let mut first_merged = Self::merge_k_lists(first_half);
            let mut sec_merged = Self::merge_k_lists(sec_half);
            Self::merge_two(first_merged, sec_merged)
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_23() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6]),
            ]),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(Solution::merge_k_lists(vec![]), None);
    }
}
