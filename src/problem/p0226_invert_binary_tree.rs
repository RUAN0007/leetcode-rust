/**
 * [226] Invert Binary Tree
 *
 * Given the root of a binary tree, invert the tree, and return its root.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/invert1-tree.jpg" style="width: 500px; height: 165px;" />
 * Input: root = [4,2,7,1,3,6,9]
 * Output: [4,7,2,9,6,3,1]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/invert2-tree.jpg" style="width: 500px; height: 120px;" />
 * Input: root = [2,1,3]
 * Output: [2,3,1]
 * 
 * Example 3:
 * 
 * Input: root = []
 * Output: []
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 100].
 * 	-100 <= Node.val <= 100
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/invert-binary-tree/
// discuss: https://leetcode.com/problems/invert-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::cell::{RefCell, RefMut};

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            Solution::invert_tree(node.borrow().right.clone());
            Solution::invert_tree(node.borrow().left.clone());
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
        }
        root
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_226() {
        assert_eq!(
            Solution::invert_tree(tree![4, 2, 7, 1, 3, 6, 9]),
            tree![4, 7, 2, 9, 6, 3, 1]
        );
    }
}
