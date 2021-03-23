/**
 * [113] Path Sum II
 *
 * Given the root of a binary tree and an integer targetSum, return all root-to-leaf paths where each path's sum equals targetSum.
 * A leaf is a node with no children.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/pathsumii1.jpg" style="width: 500px; height: 356px;" />
 * Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
 * Output: [[5,4,11,2],[5,8,4,5]]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/pathsum2.jpg" style="width: 212px; height: 181px;" />
 * Input: root = [1,2,3], targetSum = 5
 * Output: []
 * 
 * Example 3:
 * 
 * Input: root = [1,2], targetSum = 0
 * Output: []
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

// problem: https://leetcode.com/problems/path-sum-ii/
// discuss: https://leetcode.com/problems/path-sum-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::{borrow::BorrowMut, rc::Rc};
use std::cell::RefCell;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        match(root) {
            Some(node)=>{
                let val = node.borrow().val;
                let next_sum = target_sum - val;

                let left = (*node).borrow_mut().left.take();
                let right = (*node).borrow_mut().right.take();
                match (left, right) {
                    (None, None)=>{
                        if next_sum == 0 {
                            result = vec![vec![val]];
                        }                     
                    },
                    (Some(left_node), None)=>{
                        // if target_sum == val {
                        //     result = vec![vec![val]];
                        // }                     

                        let left_result = Self::path_sum(Some(left_node), next_sum);
                        for mut left in left_result {
                            let mut new_left = vec![val];
                            new_left.append(&mut left);
                            result.push(new_left);
                        }
                    },
                    (None, Some(right_node))=>{
                        // if next_sum == 0 {
                        //     result = vec![vec![val]];
                        // }                     
                        let right_result = Self::path_sum(Some(right_node), next_sum);
                        for mut right in right_result {
                            let mut new_right = vec![val];
                            new_right.append(&mut right);
                            result.push(new_right);
                        }
                    },
                    (Some(left_node), Some(right_node))=>{
                        let left_result = Self::path_sum(Some(left_node), next_sum);
                        let right_result = Self::path_sum(Some(right_node), next_sum);
                        for mut left in left_result {
                            let mut new_left = vec![val];
                            new_left.append(&mut left);
                            result.push(new_left);
                        }

                        for mut right in right_result {
                            let mut new_right = vec![val];
                            new_right.append(&mut right);
                            result.push(new_right);
                        }

                    },
                }

            }
            None=>{
                // do nothing
            }
        }; 
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_113() {
        assert_eq!(
            Solution::path_sum(tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1], 22),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        )
    }
}
