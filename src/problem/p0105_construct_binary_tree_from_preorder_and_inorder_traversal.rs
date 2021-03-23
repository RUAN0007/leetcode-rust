/**
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
 *
 * Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree.jpg" style="width: 277px; height: 302px;" />
 * Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
 * Output: [3,9,20,null,null,15,7]
 * 
 * Example 2:
 * 
 * Input: preorder = [-1], inorder = [-1]
 * Output: [-1]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= preorder.length <= 3000
 * 	inorder.length == preorder.length
 * 	-3000 <= preorder[i], inorder[i] <= 3000
 * 	preorder and inorder consist of unique values.
 * 	Each value of inorder also appears in preorder.
 * 	preorder is guaranteed to be the preorder traversal of the tree.
 * 	inorder is guaranteed to be the inorder traversal of the tree.
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
// discuss: https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn within(l: &Vec<i32>, val : i32) -> bool {
        for n in l { 
            if *n == val {
                return true;
            }
        }
        false
    }
    
    pub fn helper(pre_order_start : usize, in_order_start : usize, count : usize, preorder: &Vec<i32>, inorder: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if count == 0 {
            return None;
        }

        let root_val = preorder[pre_order_start];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

        let mut right_inorder_start = 0;
        let mut left_count = 0;
        for i in in_order_start..in_order_start + count {
            if inorder[i] == root_val {
                right_inorder_start = i+1;
                break
            } 
            left_count += 1;
        }
        let right_count = count - left_count - 1;
        root.borrow_mut().left = Self::helper(pre_order_start+1, in_order_start, left_count, preorder, inorder);
        root.borrow_mut().right = Self::helper(pre_order_start+1+left_count, right_inorder_start, right_count, preorder, inorder);

        Some(root)
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(0, 0, preorder.len(), &preorder, &inorder)
        // if preorder.len() == 0 {
        //     return None;
        // }

        // let root_val = preorder[0];
        // let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

        // let mut left_inorder: Vec<i32> = vec![];
        // let mut right_inorder: Vec<i32> = vec![];

        // let mut found = false;
        // for num in inorder {
        //     if num == root_val {
        //         found = true
        //     } else if found {
        //         right_inorder.push(num);
        //     } else {
        //         left_inorder.push(num);
        //     }
        // }

        // let mut left_preorder : Vec<i32> = vec![];
        // let mut right_preorder : Vec<i32>= vec![];
        // for num in preorder {
        //     if Self::within(&left_inorder, num) {
        //         left_preorder.push(num);
        //     }
        //     if Self::within(&right_inorder, num) {
        //         right_preorder.push(num);
        //     }
        // }

        // root.borrow_mut().left = Self::build_tree(left_preorder, left_inorder);
        // root.borrow_mut().right = Self::build_tree(right_preorder, right_inorder);
        // Some(root)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_105() {
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            tree![3, 9, 20, null, null, 15, 7]
        );
        assert_eq!(
            Solution::build_tree(vec![3, 20, 7], vec![3, 20, 7]),
            tree![3, null, 20, null, 7]
        );
        assert_eq!(Solution::build_tree(vec![], vec![]), tree![]);
    }
}
