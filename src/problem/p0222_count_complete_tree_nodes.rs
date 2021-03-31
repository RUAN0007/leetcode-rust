/**
 * [222] Count Complete Tree Nodes
 *
 * Given the root of a complete binary tree, return the number of the nodes in the tree.
 * According to <a href="http://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees" target="_blank">Wikipedia</a>, every level, except possibly the last, is completely filled in a complete binary tree, and all nodes in the last level are as far left as possible. It can have between 1 and 2^h nodes inclusive at the last level h.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/14/complete.jpg" style="width: 372px; height: 302px;" />
 * Input: root = [1,2,3,4,5,6]
 * Output: 6
 * 
 * Example 2:
 * 
 * Input: root = []
 * Output: 0
 * 
 * Example 3:
 * 
 * Input: root = [1]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 5 * 10^4].
 * 	0 <= Node.val <= 5 * 10^4
 * 	The tree is guaranteed to be complete.
 * 
 *  
 * Follow up: Traversing the tree to count the number of nodes in the tree is an easy solution but with O(n) complexity. Could you find a faster algorithm?
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/count-complete-tree-nodes/
// discuss: https://leetcode.com/problems/count-complete-tree-nodes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

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
    pub fn left_depth(root : &Option<Rc<RefCell<TreeNode>>>) -> u32 {
        if let Some(node) = root {
            return 1 + Self::left_depth(&node.borrow().left)
        } else {
            return 0;
        }
    }

    pub fn right_depth(root : &Option<Rc<RefCell<TreeNode>>>) -> u32 {
        if let Some(node) = root {
            return 1 + Self::right_depth(&node.borrow().right)
        } else {
            return 0;
        }
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_level = Self::left_depth(&node.borrow().left);
            let right_level = Self::right_depth(&node.borrow().right);
            // println!("val={}, left_level={}, right_level={}", node.borrow().val, left_level, right_level);
            if left_level == right_level {
                return 1 + (2i32.pow(left_level) - 1) + (2i32.pow(right_level) - 1);
            } else {
                let mut node_borrow = node.borrow_mut();
                return 1 + Self::count_nodes(node_borrow.left.take()) + Self::count_nodes(node_borrow.right.take());
            }
        } else {
            return 0;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_222() {
        assert_eq!(Solution::count_nodes(tree![1, 2, 3, 4, 5, 6]), 6);
        assert_eq!(Solution::count_nodes(tree![1, 1, 1, 1, 1, 1, 1]), 7);
        assert_eq!(Solution::count_nodes(tree![]), 0);
        assert_eq!(Solution::count_nodes(tree![1, 1]), 2);
        assert_eq!(Solution::count_nodes(tree![1]), 1);
        assert_eq!(Solution::count_nodes(tree![1, 1, 1]), 3);
        assert_eq!(Solution::count_nodes(tree![1, 2, 3, 4]), 4);
    }
}
