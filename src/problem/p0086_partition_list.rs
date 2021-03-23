/**
 * [86] Partition List
 *
 * Given the head of a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.
 * You should preserve the original relative order of the nodes in each of the two partitions.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/partition.jpg" style="width: 662px; height: 222px;" />
 * Input: head = [1,4,3,2,5,2], x = 3
 * Output: [1,2,2,4,3,5]
 * 
 * Example 2:
 * 
 * Input: head = [2,1], x = 2
 * Output: [1,2]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is in the range [0, 200].
 * 	-100 <= Node.val <= 100
 * 	-200 <= x <= 200
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/partition-list/
// discuss: https://leetcode.com/problems/partition-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut lt_list = Some(Box::new(ListNode::new(0)));
        let mut lt_tail = lt_list.as_mut().unwrap();
        let mut ge_list = Some(Box::new(ListNode::new(0)));
        let mut ge_tail = ge_list.as_mut().unwrap();

        let mut cur = head.as_ref();
        while let Some(cur_node) = cur {
            if cur_node.val < x {
                lt_tail.next = Some(Box::new(ListNode::new(cur_node.val)));
                lt_tail = lt_tail.next.as_mut().unwrap();
            } else {
                ge_tail.next = Some(Box::new(ListNode::new(cur_node.val)));
                ge_tail = ge_tail.next.as_mut().unwrap();
            }
            cur = cur_node.next.as_ref();
        }
        lt_tail.next = ge_list.unwrap().next;
        lt_list.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_86() {
        assert_eq!(
            Solution::partition(linked![1, 4, 3, 2, 5, 2], 3),
            linked![1, 2, 2, 4, 3, 5]
        );
        assert_eq!(
            Solution::partition(linked![1, 4, 3, 2, 5, 2], 8),
            linked![1, 4, 3, 2, 5, 2]
        );
        assert_eq!(Solution::partition(linked![], 0), linked![]);
    }
}
