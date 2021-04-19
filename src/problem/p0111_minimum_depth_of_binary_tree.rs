/**
 * [111] Minimum Depth of Binary Tree
 *
 * Given a binary tree, find its minimum depth.
 * The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
 * Note: A leaf is a node with no children.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/12/ex_depth.jpg" style="width: 432px; height: 302px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: 2
 * 
 * Example 2:
 * 
 * Input: root = [2,null,3,null,4,null,5,null,6]
 * Output: 5
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 10^5].
 * 	-1000 <= Node.val <= 1000
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/minimum-depth-of-binary-tree/
// discuss: https://leetcode.com/problems/minimum-depth-of-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut last_level = 0;
        if let Some(ref root_node) = root {
            type NodeWithLevel = (Rc<RefCell<TreeNode>>, usize);
            let mut queue : VecDeque<NodeWithLevel> = VecDeque::new();
            queue.push_back((Rc::clone(root_node), 1));

            while let Some(head_entry) = queue.pop_front() { 
                let cur_node : Rc<RefCell<TreeNode>> = head_entry.0;
                let cur_level : usize = head_entry.1;
                let mut has_children = false;
                // left_node typed with &Rc<RefCell<TreeNode>>
                if let Some(left_node) = cur_node.borrow().left.as_ref() {
                    queue.push_back((Rc::clone(left_node), cur_level+1));
                    has_children = true;
                };

                // right_node typed with &Rc<RefCell<TreeNode>>
                if let Some(right_node) = cur_node.borrow().right.as_ref() {
                    queue.push_back((Rc::clone(right_node), cur_level+1));
                    has_children = true;
                };

                if !has_children {return cur_level as i32}
                last_level = cur_level
            }
        }
        last_level as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_111() {
        assert_eq!(Solution::min_depth(tree![3, 9, 20, null, null, 15, 7]), 2);
    }
}
