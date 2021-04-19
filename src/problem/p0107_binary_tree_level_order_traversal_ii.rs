/**
 * [107] Binary Tree Level Order Traversal II
 *
 * Given the root of a binary tree, return the bottom-up level order traversal of its nodes' values. (i.e., from left to right, level by level from leaf to root).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg" style="width: 277px; height: 302px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: [[15,7],[9,20],[3]]
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
 * 	-1000 <= Node.val <= 1000
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-level-order-traversal-ii/
// discuss: https://leetcode.com/problems/binary-tree-level-order-traversal-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::collections::VecDeque;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {

        let mut traversed : Vec<Vec<i32>> = vec![];
        let mut this_level : Vec<i32> = vec![];
        let mut last_level = 0usize;
        if let Some(ref root_node) = root {
            type NodeWithLevel = (Rc<RefCell<TreeNode>>, usize);
            let mut queue : VecDeque<NodeWithLevel> = VecDeque::new();
            queue.push_back((Rc::clone(root_node), 1));

            while let Some(head_entry) = queue.pop_front() { 
                let cur_node : Rc<RefCell<TreeNode>> = head_entry.0;
                let cur_level : usize = head_entry.1;

                if cur_level != last_level {
                    // can be empty when processing the first node
                    if this_level.len() != 0 {
                        traversed.push(this_level);
                    }
                    this_level = vec![];
                }

                last_level = cur_level;
                this_level.push(cur_node.borrow().val);

                // left_node typed with &Rc<RefCell<TreeNode>>
                if let Some(left_node) = cur_node.borrow().left.as_ref() {
                    queue.push_back((Rc::clone(left_node), cur_level+1));
                };

                // right_node typed with &Rc<RefCell<TreeNode>>
                if let Some(right_node) = cur_node.borrow().right.as_ref() {
                    queue.push_back((Rc::clone(right_node), cur_level+1));
                };
            }
        }

        if this_level.len() != 0 {
            traversed.push(this_level);
        }
        traversed.into_iter().rev().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_107() {
        assert_eq!(
            Solution::level_order_bottom(tree![3, 9, 20, null, null, 15, 7]),
            vec![vec![15, 7], vec![9, 20], vec![3],]
        );
    }
}
