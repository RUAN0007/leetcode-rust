/**
 * [99] Recover Binary Search Tree
 *
 * You are given the root of a binary search tree (BST), where exactly two nodes of the tree were swapped by mistake. Recover the tree without changing its structure.
 * Follow up: A solution using O(n) space is pretty straight forward. Could you devise a constant space solution?
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/28/recover1.jpg" style="width: 422px; height: 302px;" />
 * Input: root = [1,3,null,null,2]
 * Output: [3,1,null,null,2]
 * Explanation: 3 cannot be a left child of 1 because 3 > 1. Swapping 1 and 3 makes the BST valid.
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/28/recover2.jpg" style="width: 581px; height: 302px;" />
 * Input: root = [3,1,4,null,null,2]
 * Output: [2,1,4,null,null,3]
 * Explanation: 2 cannot be in the right subtree of 3 because 2 < 3. Swapping 2 and 3 makes the BST valid.
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [2, 1000].
 * 	-2^31 <= Node.val <= 2^31 - 1
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/recover-binary-search-tree/
// discuss: https://leetcode.com/problems/recover-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn inorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Rc<RefCell<TreeNode>>> {
        let mut result = vec![];
        if root.is_some() {
            result.extend(Self::inorder(&root.as_ref().unwrap().borrow().left));
            result.push(Rc::clone(root.as_ref().unwrap()));
            result.extend(Self::inorder(&root.as_ref().unwrap().borrow().right));
        }
        result
    }

    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut nodes = Self::inorder(&root);
        let mut first_higher : Option<Rc<RefCell<TreeNode>>> = None;
        let mut last_lower = Rc::clone(nodes.get(0).unwrap());
        let n = nodes.len();
        for i in 0..n {
            if i == 0 {
                if nodes[i].borrow().val > nodes[i+1].borrow().val && first_higher.is_none() {
                    first_higher = Some(Rc::clone(nodes.get(i).unwrap()));
                }
            } else if i == n - 1 {
                if nodes[i-1].borrow().val > nodes[i].borrow().val {
                    last_lower = Rc::clone(nodes.get(i).unwrap());
                }

            } else {
                if nodes[i-1].borrow().val > nodes[i].borrow().val &&  nodes[i].borrow().val < nodes[i+1].borrow().val {
                    last_lower = Rc::clone(nodes.get(i).unwrap());
                }

                if nodes[i-1].borrow().val < nodes[i].borrow().val &&  nodes[i].borrow().val > nodes[i+1].borrow().val && first_higher.is_none()  {
                    first_higher = Some(Rc::clone(nodes.get(i).unwrap()));
                }
            }
            // println!("nodes[{}]={}", i, nodes[i].borrow().val);
        }

        let first_higher = first_higher.unwrap();
        // println!("higher={}", first_higher.borrow().val);
        // println!("lower={}", last_lower.borrow().val);
        let tmp = first_higher.borrow().val;
        first_higher.borrow_mut().val = last_lower.borrow().val;
        last_lower.borrow_mut().val = tmp;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_99() {
        // let mut tree = tree![3, 1, 4, null, null, 2];
        // Solution::recover_tree(&mut tree);
        // assert_eq!(tree, tree![2, 1, 4, null, null, 3]);

        let mut tree = tree![2, 6, 5, null, null, 3, 1, null, 4];
        Solution::recover_tree(&mut tree);
        assert_eq!(tree, tree![2, 1, 5, null, null, 3, 6, null, 4]);
    }
}
