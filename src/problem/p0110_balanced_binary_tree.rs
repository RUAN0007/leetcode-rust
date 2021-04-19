/**
 * [110] Balanced Binary Tree
 *
 * Given a binary tree, determine if it is height-balanced.
 * For this problem, a height-balanced binary tree is defined as:
 * <blockquote>
 * a binary tree in which the left and right subtrees of every node differ in height by no more than 1.
 * </blockquote>
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/06/balance_1.jpg" style="width: 342px; height: 221px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: true
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/06/balance_2.jpg" style="width: 452px; height: 301px;" />
 * Input: root = [1,2,2,3,3,null,null,4,4]
 * Output: false
 * 
 * Example 3:
 * 
 * Input: root = []
 * Output: true
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 5000].
 * 	-10^4 <= Node.val <= 10^4
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/balanced-binary-tree/
// discuss: https://leetcode.com/problems/balanced-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn helper(node: &Option<Rc<RefCell<TreeNode>>>) -> (bool, usize) {
        if node.is_none() {
            (true, 0)
        } else {
            let (is_left_balanced, left_depth) = Self::helper(&node.as_ref().unwrap().borrow().left);
            let (is_right_balanced, right_depth) = Self::helper(&node.as_ref().unwrap().borrow().right);

            let (smaller, larger) = if left_depth < right_depth {
                (left_depth, right_depth)
            } else {
                (right_depth, left_depth)
            };
            (is_left_balanced && is_right_balanced && larger - smaller <= 1, larger+1)
        }
    }
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (balanced, _) = Self::helper(&root);
        balanced
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_110() {
        assert_eq!(Solution::is_balanced(tree![]), true);
        assert_eq!(
            Solution::is_balanced(tree![3, 9, 20, null, null, 15, 7]),
            true
        );
        assert_eq!(
            Solution::is_balanced(tree![1, 2, 2, 3, 3, null, null, 4, 4]),
            false
        );
    }
}
