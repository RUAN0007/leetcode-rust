/**
 * [112] Path Sum
 *
 * Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.
 * A leaf is a node with no children.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/pathsum1.jpg" style="width: 500px; height: 356px;" />
 * Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
 * Output: true
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/pathsum2.jpg" />
 * Input: root = [1,2,3], targetSum = 5
 * Output: false
 * 
 * Example 3:
 * 
 * Input: root = [1,2], targetSum = 0
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 5000].
 * 	-1000 <= Node.val <= 1000
 * 	-1000 <= targetSum <= 1000
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/path-sum/
// discuss: https://leetcode.com/problems/path-sum/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn helper(root: Rc<RefCell<TreeNode>>, target_sum: i32) -> bool {
        let this_val = root.borrow().val;
        let next_sum = target_sum - this_val;

        let mut is_leaf = true;
        if let Some(left_node) = root.borrow().left.clone() {
            is_leaf = false;
            if Self::helper(left_node, next_sum) {
                return true;
            }
        }

        if let Some(right_node) = root.borrow().right.clone() {
            is_leaf = false;
            if Self::helper(right_node, next_sum) {
                return true;
            }
        }

        if is_leaf && next_sum == 0 {
            return true;
        } else {
            return false;
        }
    }
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {false} else {
            Self::helper(root.unwrap(), target_sum)
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_112() {
        assert_eq!(
            Solution::has_path_sum(
                tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
                22
            ),
            true
        );
    }
}
