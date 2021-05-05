/**
 * [124] Binary Tree Maximum Path Sum
 *
 * A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence at most once. Note that the path does not need to pass through the root.
 * The path sum of a path is the sum of the node's values in the path.
 * Given the root of a binary tree, return the maximum path sum of any path.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/13/exx1.jpg" style="width: 322px; height: 182px;" />
 * Input: root = [1,2,3]
 * Output: 6
 * Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/13/exx2.jpg" />
 * Input: root = [-10,9,20,null,null,15,7]
 * Output: 42
 * Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 3 * 10^4].
 * 	-1000 <= Node.val <= 1000
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-maximum-path-sum/
// discuss: https://leetcode.com/problems/binary-tree-maximum-path-sum/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, cur_max : &mut i32) -> i32 {
        if node.is_none() { return 0; }
        let node_val : i32 = node.as_ref().unwrap().borrow().val;
        let left_max : i32 = Self::traverse(&node.as_ref().unwrap().borrow().left, cur_max);
        let right_max : i32 = Self::traverse(&node.as_ref().unwrap().borrow().right, cur_max);

        // conditioned that the path must include this node. 
        *cur_max = std::cmp::max(*cur_max, node_val + left_max + right_max);
        *[0, node_val, left_max + node_val, right_max + node_val].iter().max().unwrap()

    }
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = - 1 << 30;
        Self::traverse(&root, &mut max);
        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_124() {
        assert_eq!(Solution::max_path_sum(tree![1, 2, 3]), 6);
        assert_eq!(
            Solution::max_path_sum(tree![-10, 9, 20, null, null, 15, 7]),
            42
        );
        assert_eq!(
            Solution::max_path_sum(tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1]),
            48
        );
        assert_eq!(Solution::max_path_sum(tree![-3]), -3);
    }
}
