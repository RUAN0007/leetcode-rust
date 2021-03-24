/**
 * [230] Kth Smallest Element in a BST
 *
 * Given the root of a binary search tree, and an integer k, return the k^th (1-indexed) smallest element in the tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/28/kthtree1.jpg" style="width: 212px; height: 301px;" />
 * Input: root = [3,1,4,null,2], k = 1
 * Output: 1
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/28/kthtree2.jpg" style="width: 382px; height: 302px;" />
 * Input: root = [5,3,6,2,4,null,null,1], k = 3
 * Output: 3
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is n.
 * 	1 <= k <= n <= 10^4
 * 	0 <= Node.val <= 10^4
 * 
 *  
 * Follow up: If the BST is modified often (i.e., we can do insert and delete operations) and you need to find the kth smallest frequently, how would you optimize?
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/kth-smallest-element-in-a-bst/
// discuss: https://leetcode.com/problems/kth-smallest-element-in-a-bst/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn helper(root: Option<Rc<RefCell<TreeNode>>>, k: usize) -> (Option<i32>, usize) {
        match(root) {
            None =>{(None, 0)}
            Some(node)=> {
                let (found, left_node_count) = Self::helper(node.borrow_mut().left.take(), k);
                if found.is_some() {
                    (found, 0)
                } else if left_node_count == k - 1 {
                    (Some(node.borrow().val), 0)
                } else {
                    let (found, right_node_count) = Self::helper(node.borrow_mut().right.take(), k-1-left_node_count);
                    // assume such value must be found. 
                    (found, left_node_count + 1 + right_node_count)
                }
            }
        } 
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let (found, _) = Self::helper(root, k as usize);
        found.unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_230() {
        assert_eq!(Solution::kth_smallest(tree![3, 1, 4, null, 2], 1), 1);
        assert_eq!(Solution::kth_smallest(tree![3, 1, 4, null, 2], 2), 2);
        assert_eq!(Solution::kth_smallest(tree![3, 1, 4, null, 2], 3), 3);
    }
}
