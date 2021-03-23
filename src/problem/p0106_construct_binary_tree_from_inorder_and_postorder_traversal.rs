/**
 * [106] Construct Binary Tree from Inorder and Postorder Traversal
 *
 * Given two integer arrays inorder and postorder where inorder is the inorder traversal of a binary tree and postorder is the postorder traversal of the same tree, construct and return the binary tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree.jpg" style="width: 277px; height: 302px;" />
 * Input: inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
 * Output: [3,9,20,null,null,15,7]
 * 
 * Example 2:
 * 
 * Input: inorder = [-1], postorder = [-1]
 * Output: [-1]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= inorder.length <= 3000
 * 	postorder.length == inorder.length
 * 	-3000 <= inorder[i], postorder[i] <= 3000
 * 	inorder and postorder consist of unique values.
 * 	Each value of postorder also appears in inorder.
 * 	inorder is guaranteed to be the inorder traversal of the tree.
 * 	postorder is guaranteed to be the postorder traversal of the tree.
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
// discuss: https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::{collections::HashMap, rc::Rc};
use std::cell::RefCell;
impl Solution {
    pub fn helper(in_order_start: usize, in_order_end: usize, inorder: &Vec<i32>, postorder_pos: &HashMap<i32, usize>) -> Option<Rc<RefCell<TreeNode>>> {
        if in_order_start == in_order_end {
            None
        } else if in_order_start == in_order_end-1 {
            Some(Rc::new(RefCell::new(TreeNode::new(*inorder.get(in_order_start).unwrap()))))
        } else if in_order_start < in_order_end-1  {
            // locate the element within [in_order_start, in_order_end) 
            //   which appears at the last of postporder_pos
            // Record down the element pos in inorder list
            let mut lastpost_inorder_pos = in_order_start;
            let inorder_num = inorder[in_order_start];
            let mut cur_lastpost_pos = postorder_pos[&inorder_num];

            for i in in_order_start..in_order_end {
                let inorder_num = inorder[i];
                let num_post_pos = postorder_pos[&inorder_num];
                if cur_lastpost_pos < num_post_pos {
                    cur_lastpost_pos = num_post_pos;
                    lastpost_inorder_pos = i;
                }
            }
            // println!("in_order_start = {}, in_order_end = {}, lastpost_inorder_pos = {}", in_order_start, in_order_end, lastpost_inorder_pos);
            // None
            let left = Self::helper(in_order_start, lastpost_inorder_pos, inorder, postorder_pos);
            let right = Self::helper(lastpost_inorder_pos+1, in_order_end,
                inorder, postorder_pos);
            let mut node = TreeNode::new(*inorder.get(lastpost_inorder_pos).unwrap());
            node.left = left;
            node.right = right;
                
            Some(Rc::new(RefCell::new(node)))

        } else {
            panic!("Not here");
            None
        }
    }
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut postorder_pos = HashMap::new();
        for (i, num) in postorder.iter().enumerate() {
            postorder_pos.insert(*num, i);
        }
        Self::helper(0, inorder.len(), &inorder, &postorder_pos)
        // Some(Rc::new(RefCell::new(TreeNode::new(0))))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_106() {
        assert_eq!(
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
            tree![3, 9, 20, null, null, 15, 7]
        );
        // assert_eq!(
        //     Solution::build_tree(vec![3, 20, 7], vec![7, 20, 3]),
        //     tree![3, null, 20, null, 7]
        // );
        // assert_eq!(Solution::build_tree(vec![], vec![]), tree![]);
    }
}
