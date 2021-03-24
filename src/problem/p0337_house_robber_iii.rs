/**
 * [337] House Robber III
 *
 * The thief has found himself a new place for his thievery again. There is only one entrance to this area, called root.
 * Besides the root, each house has one and only one parent house. After a tour, the smart thief realized that all houses in this place form a binary tree. It will automatically contact the police if two directly-linked houses were broken into on the same night.
 * Given the root of the binary tree, return the maximum amount of money the thief can rob without alerting the police.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/10/rob1-tree.jpg" style="width: 277px; height: 293px;" />
 * Input: root = [3,2,3,null,3,null,1]
 * Output: 7
 * Explanation: Maximum amount of money the thief can rob = 3 + 3 + 1 = 7.
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/10/rob2-tree.jpg" style="width: 357px; height: 293px;" />
 * Input: root = [3,4,5,1,3,null,1]
 * Output: 9
 * Explanation: Maximum amount of money the thief can rob = 4 + 5 = 9.
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	0 <= Node.val <= 10^4
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/house-robber-iii/
// discuss: https://leetcode.com/problems/house-robber-iii/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::{borrow::Borrow, rc::Rc};
use std::cell::RefCell;
impl Solution {
    // return the max for both cases: this node is robbed or not. 
    pub fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32,i32) {
        let result = match(root) {
            None =>{(0, 0)}
            Some(node) => {
                let (left_on, left_off) = Self::helper(node.borrow_mut().left.take());
                let (right_on, right_off) = Self::helper(node.borrow_mut().right.take());
                let my_on = (*node).borrow().val + left_off + right_off;

                // Assuming this node is not robbed, it is valid that the child not is either robbed or not. 
                // So we simply sum up the max of each option in both children nodes. 

                let max_left = std::cmp::max(left_on, left_off);
                let max_right = std::cmp::max(right_on, right_off);
                let my_off = max_left + max_right;
                // println!("node({}) = (on: {}, off: {}, left_on: {}, left_off: {}, right_on: {}, right_off: {})", (*node).borrow().val, my_on, my_off, left_on, left_off, right_on, right_off);
                (my_on, my_off)
            }
        };
        result
    }

    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (my_on, my_off) = Self::helper(root);
        std::cmp::max(my_on, my_off)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_337() {
        assert_eq!(
            Solution::rob(tree![3,2,3,null,3,null,1]), 7
        );
    }
}
