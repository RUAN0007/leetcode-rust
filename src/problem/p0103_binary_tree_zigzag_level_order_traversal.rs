/**
 * [103] Binary Tree Zigzag Level Order Traversal
 *
 * Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg" style="width: 277px; height: 302px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: [[3],[20,9],[15,7]]
 * 
 * Example 2:
 * 
 * Input: root = [1]
 * Output: [[1]]
 * 
 * Example 3:
 * 
 * Input: root = []
 * Output: []
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 2000].
 * 	-100 <= Node.val <= 100
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
// discuss: https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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

use std::collections::LinkedList;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result : Vec<Vec<i32>> = vec![];
        if root.is_none() {
            return vec![];
        }

        let mut odd_stack: LinkedList<Rc<RefCell<TreeNode>>> = LinkedList::new();
        let mut even_stack: LinkedList<Rc<RefCell<TreeNode>>> = LinkedList::new();

        odd_stack.push_back(Rc::clone(root.as_ref().unwrap()));

        loop {
            if odd_stack.is_empty() {
                break;
            }
            let mut level : Vec<i32> = vec![];
            while let Some(node) = odd_stack.pop_back() {
                level.push(node.borrow().val);
                if node.borrow().left.is_some() {
                    even_stack.push_back(Rc::clone(node.borrow().left.as_ref().unwrap()));
                }
                if node.borrow().right.is_some() {
                    even_stack.push_back(Rc::clone(node.borrow().right.as_ref().unwrap()));
                }
            }
            result.push(level);
            let mut level : Vec<i32> = vec![];

            if even_stack.is_empty() {
                break
            }
            while let Some(node) = even_stack.pop_back() {
                level.push(node.borrow().val);
                if node.borrow().right.is_some() {
                    odd_stack.push_back(Rc::clone(node.borrow().right.as_ref().unwrap()));
                }
                if node.borrow().left.is_some() {
                    odd_stack.push_back(Rc::clone(node.borrow().left.as_ref().unwrap()));
                }
            }
            result.push(level);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_103() {
        assert_eq!(
            Solution::zigzag_level_order(tree![3, 9, 20, null, null, 15, 7]),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }
}
