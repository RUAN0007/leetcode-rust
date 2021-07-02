/**
 * [109] Convert Sorted List to Binary Search Tree
 *
 * Given the head of a singly linked list where elements are sorted in ascending order, convert it to a height balanced BST.
 * For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of every node never differ by more than 1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/linked.jpg" style="width: 600px; height: 466px;" />
 * Input: head = [-10,-3,0,5,9]
 * Output: [0,-3,9,-10,null,5]
 * Explanation: One possible answer is [0,-3,9,-10,null,5], which represents the shown height balanced BST.
 * 
 * Example 2:
 * 
 * Input: head = []
 * Output: []
 * 
 * Example 3:
 * 
 * Input: head = [0]
 * Output: [0]
 * 
 * Example 4:
 * 
 * Input: head = [1,3]
 * Output: [3,1]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in head is in the range [0, 2 * 10^4].
 * 	-10^5 <= Node.val <= 10^5
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/
// discuss: https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if head.is_none() {
            return None;
        }
        
        let mut l : usize = 1;
        let mut node_ptr : &ListNode = head.as_ref().unwrap();
        while let Some(ref next) = node_ptr.next {
            node_ptr = next;
            l+=1;
        }

        if l == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(node_ptr.val))));
        }
        
        let mid : usize = l / 2;
        let mut first_half_tail : &mut ListNode = head.as_mut().unwrap();
        let mut i : usize = 0;
        while i < mid - 1 {
            first_half_tail = first_half_tail.next.as_mut().unwrap();
            i+=1;
        }

        let mut mid_node : Box<ListNode> = first_half_tail.next.take().unwrap();
        let sec_half : Option<Box<ListNode>> = mid_node.next.take(); 

        let mut this_node : TreeNode = TreeNode::new(mid_node.val);
        this_node.left = Self::sorted_list_to_bst(head);
        this_node.right = Self::sorted_list_to_bst(sec_half);
        Some(Rc::new(RefCell::new(this_node)))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_109() {
        assert_eq!(
            Solution::sorted_list_to_bst(linked![-10, -3, 0, 5, 9]),
            tree![0, -3, 9, -10, null, 5]
        );
    }
}
