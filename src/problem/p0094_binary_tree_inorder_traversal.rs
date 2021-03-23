/**
 * [94] Binary Tree Inorder Traversal
 *
 * Given the root of a binary tree, return the inorder traversal of its nodes' values.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/inorder_1.jpg" style="width: 202px; height: 324px;" />
 * Input: root = [1,null,2,3]
 * Output: [1,3,2]
 * 
 * Example 2:
 * 
 * Input: root = []
 * Output: []
 * 
 * Example 3:
 * 
 * Input: root = [1]
 * Output: [1]
 * 
 * Example 4:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/inorder_5.jpg" style="width: 202px; height: 202px;" />
 * Input: root = [1,2]
 * Output: [2,1]
 * 
 * Example 5:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/inorder_4.jpg" style="width: 202px; height: 202px;" />
 * Input: root = [1,null,2]
 * Output: [1,2]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 100].
 * 	-100 <= Node.val <= 100
 * 
 *  
 * Follow up:
 * Recursive solution is trivial, could you do it iteratively?
 *  
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-inorder-traversal/
// discuss: https://leetcode.com/problems/binary-tree-inorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match(root) {
            None=>{vec![]},
            Some(node)=>{
                let mut left_vec = Self::inorder_traversal(node.borrow_mut().left.take());
                let mut right_vec = Self::inorder_traversal(node.borrow_mut().right.take());
                left_vec.extend(vec![node.borrow().val]);
                left_vec.extend(right_vec);
                left_vec
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_94() {
        assert_eq!(
            Solution::inorder_traversal(tree![1, null, 2, 3]),
            vec![1, 3, 2]
        );
        assert_eq!(
            Solution::inorder_traversal(tree![1, 2, 3, 4, 5, 6, 7]),
            vec![4, 2, 5, 1, 6, 3, 7]
        );
    }
}
