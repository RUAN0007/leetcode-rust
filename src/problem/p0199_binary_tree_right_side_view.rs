/**
 * [199] Binary Tree Right Side View
 *
 * Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/14/tree.jpg" style="width: 401px; height: 301px;" />
 * Input: root = [1,2,3,null,5,null,4]
 * Output: [1,3,4]
 * 
 * Example 2:
 * 
 * Input: root = [1,null,3]
 * Output: [1,3]
 * 
 * Example 3:
 * 
 * Input: root = []
 * Output: []
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 100].
 * 	-100 <= Node.val <= 100
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-right-side-view/
// discuss: https://leetcode.com/problems/binary-tree-right-side-view/discuss/?currentPage=1&orderBy=most_votes&query=

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
struct NodeInfo(Rc<RefCell<TreeNode>>, usize);
use std::collections::VecDeque;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result =vec![];
        if root.is_none() {return result}

        let mut last_level = 0usize;
        let mut last_node : Rc<RefCell<TreeNode>> = Rc::clone(root.as_ref().unwrap());

        let mut queue: VecDeque<NodeInfo> = VecDeque::new();
        queue.push_back(NodeInfo(root.unwrap(), 1usize));

        while let Some(node_info) = queue.pop_front() {
            let NodeInfo(node, level) = node_info;
            // println!("last_level = {}, level = {}, node val = {}", last_level, level, (*node).borrow().val);
            if last_level != 0 && last_level  < level {
                result.push((*last_node).borrow().val);
            }

            if let Some(left_node) = node.borrow_mut().left.take() {
                queue.push_back(NodeInfo(left_node, level+1));
            };

            if let Some(right_node) = node.borrow_mut().right.take() {
                queue.push_back(NodeInfo(right_node, level+1));
            };
            last_level = level; // traversed to a deeper level. 
            last_node = Rc::clone(&node);
        }
        result.push((*last_node).borrow().val);

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_199() {
        assert_eq!(
            Solution::right_side_view(tree![1, 2, 3, null, 5, null, 4]),
            vec![1, 3, 4]
        );
    }
}
