/**
 * [95] Unique Binary Search Trees II
 *
 * Given an integer n, return all the structurally unique BST's (binary search trees), which has exactly n nodes of unique values from 1 to n. Return the answer in any order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/uniquebstn3.jpg" style="width: 600px; height: 148px;" />
 * Input: n = 3
 * Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
 * 
 * Example 2:
 * 
 * Input: n = 1
 * Output: [[1]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 8
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/unique-binary-search-trees-ii/
// discuss: https://leetcode.com/problems/unique-binary-search-trees-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let n = n as usize;
        let mut matrix : Vec<Vec<Vec<Option<Rc<RefCell<TreeNode>>>>>> = vec![vec![vec![];n+1];n+1];
        for i in 0..n+1 {
            matrix[i][i].push(None);
        }
        for l in 1..n+1 {
            let mut start = 0;
            while start + l <= n {
                let end : usize = start + l;
                let mut this_result = vec![];
                for mid in (start..end) {

                    for left_tree in matrix[start][mid].iter() {
                        for right_tree in matrix[mid+1][end].iter() {
                            let mut node : TreeNode = TreeNode::new((mid + 1) as i32);
                            node.left = left_tree.clone();
                            node.right = right_tree.clone();
                            this_result.push(Some(Rc::new(RefCell::new(node))));
                        }
                    }
                }

                // println!("start={}, end={}, result.len={}", start, end, this_result.len());
                matrix[start][end].extend(this_result);
                start+=1;
            }
        }
        matrix[0][n].clone()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_95() {
    }
}
