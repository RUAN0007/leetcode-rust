/**
 * [114] Flatten Binary Tree to Linked List
 *
 * Given the root of a binary tree, flatten the tree into a "linked list":
 * 
 * 	The "linked list" should use the same TreeNode class where the right child pointer points to the next node in the list and the left child pointer is always null.
 * 	The "linked list" should be in the same order as a <a href="https://en.wikipedia.org/wiki/Tree_traversal#Pre-order,_NLR" target="_blank">pre-order traversal</a> of the binary tree.
 * 
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/14/flaten.jpg" style="width: 500px; height: 226px;" />
 * Input: root = [1,2,5,3,4,null,6]
 * Output: [1,null,2,null,3,null,4,null,5,null,6]
 * 
 * Example 2:
 * 
 * Input: root = []
 * Output: []
 * 
 * Example 3:
 * 
 * Input: root = [0]
 * Output: [0]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 2000].
 * 	-100 <= Node.val <= 100
 * 
 *  
 * Follow up: Can you flatten the tree in-place (with O(1) extra space)?
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/flatten-binary-tree-to-linked-list/
// discuss: https://leetcode.com/problems/flatten-binary-tree-to-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn helper (root : Rc<RefCell<TreeNode>>) -> (Rc<RefCell<TreeNode>>, Rc<RefCell<TreeNode>>) {
        let prev_left = root.borrow().left.clone();
        let prev_right = root.borrow().right.clone();
let mut my_tail = Rc::clone(&root);
        my_tail.borrow_mut().left = None;

        if let Some(left_node) = prev_left {
            let (left_head, left_tail) = Self::helper(left_node);
            root.borrow_mut().right = Some(left_head);
            my_tail = left_tail
        }

        if let Some(right_node) = prev_right {
            let (right_head, right_tail) = Self::helper(right_node);
            my_tail.borrow_mut().right = Some(right_head);
            my_tail = right_tail;
        }

        return (root, my_tail);
    }

    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        match(root) {
            None => {},
            Some(node) => {
                Self::helper(Rc::clone(node));
            }
        };
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_114() {
        let mut tree = tree![1, 2, 5, 3, 4, null, 6];
        Solution::flatten(&mut tree);
        assert_eq!(tree, tree![1, null, 2, null, 3, null, 4, null, 5, null, 6]);

        let mut tree = tree![1, 2, null, 3];
        Solution::flatten(&mut tree);
        assert_eq!(tree, tree![1, null, 2, null, 3]);

        let mut tree = tree![1, null, 2, 3];
        Solution::flatten(&mut tree);
        assert_eq!(tree, tree![1, null, 2, null, 3]);
    }
}
