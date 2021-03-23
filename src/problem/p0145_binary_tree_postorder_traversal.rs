/**
 * [145] Binary Tree Postorder Traversal
 *
 * Given the root of a binary tree, return the postorder traversal of its nodes' values.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/28/pre1.jpg" style="width: 202px; height: 317px;" />
 * Input: root = [1,null,2,3]
 * Output: [3,2,1]
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
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/28/pre3.jpg" style="width: 202px; height: 197px;" />
 * Input: root = [1,2]
 * Output: [2,1]
 * 
 * Example 5:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/28/pre2.jpg" style="width: 202px; height: 197px;" />
 * Input: root = [1,null,2]
 * Output: [2,1]
 * 
 *  
 * Constraints:
 * 
 * 	The number of the nodes in the tree is in the range [0, 100].
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

// problem: https://leetcode.com/problems/binary-tree-postorder-traversal/
// discuss: https://leetcode.com/problems/binary-tree-postorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match(root) {
            None=>{vec![]},
            Some(node)=>{
                let mut left_vec = Self::postorder_traversal(node.borrow_mut().left.take());
                let mut right_vec = Self::postorder_traversal(node.borrow_mut().right.take());
                left_vec.extend(right_vec);
                left_vec.extend(vec![node.borrow().val]);
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
    fn test_145() {
        assert_eq!(
            Solution::postorder_traversal(tree![1, null, 2, 3]),
            vec![3, 2, 1]
        );
    }
}
