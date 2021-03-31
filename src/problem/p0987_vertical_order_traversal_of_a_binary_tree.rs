/**
 * [987] Vertical Order Traversal of a Binary Tree
 *
 * Given the root of a binary tree, calculate the vertical order traversal of the binary tree.
 * For each node at position (row, col), its left and right children will be at positions (row + 1, col - 1) and (row + 1, col + 1) respectively. The root of the tree is at (0, 0).
 * The vertical order traversal of a binary tree is a list of top-to-bottom orderings for each column index starting from the leftmost column and ending on the rightmost column. There may be multiple nodes in the same row and same column. In such a case, sort these nodes by their values.
 * Return the vertical order traversal of the binary tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/29/vtree1.jpg" style="width: 431px; height: 304px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: [[9],[3,15],[20],[7]]
 * Explanation:
 * Column -1: Only node 9 is in this column.
 * Column 0: Nodes 3 and 15 are in this column in that order from top to bottom.
 * Column 1: Only node 20 is in this column.
 * Column 2: Only node 7 is in this column.
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/29/vtree2.jpg" style="width: 512px; height: 304px;" />
 * Input: root = [1,2,3,4,5,6,7]
 * Output: [[4],[2],[1,5,6],[3],[7]]
 * Explanation:
 * Column -2: Only node 4 is in this column.
 * Column -1: Only node 2 is in this column.
 * Column 0: Nodes 1, 5, and 6 are in this column.
 *           1 is at the top, so it comes first.
 *           5 and 6 are at the same position (2, 0), so we order them by their value, 5 before 6.
 * Column 1: Only node 3 is in this column.
 * Column 2: Only node 7 is in this column.
 * 
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/29/vtree3.jpg" style="width: 512px; height: 304px;" />
 * Input: root = [1,2,3,4,6,5,7]
 * Output: [[4],[2],[1,5,6],[3],[7]]
 * Explanation:
 * This case is the exact same as example 2, but with nodes 5 and 6 swapped.
 * Note that the solution remains the same since 5 and 6 are in the same location and should be ordered by their values.
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 1000].
 * 	0 <= Node.val <= 1000
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/
// discuss: https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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

use std::cmp::Ordering;
#[derive(Eq)]
struct NodeInfo{node: Rc<RefCell<TreeNode>>, row: i32, col: i32}

impl Ord for NodeInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.col, self.row, (*self.node).borrow().val).cmp(&(other.col, other.row, (*other.node).borrow().val))
    }
}

impl PartialOrd for NodeInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for NodeInfo {
    fn eq(&self, other: &Self) -> bool {
        (self.col, self.row, (*self.node).borrow().val) ==  (other.col, other.row, (*other.node).borrow().val)
    }
}


impl Solution {
    fn traverse(node_infos : &mut Vec<NodeInfo>, node: Rc<RefCell<TreeNode>>, row: i32, col: i32) {
        node_infos.push(NodeInfo{node: Rc::clone(&node), row: row, col: col});
        if let Some(left) = node.borrow_mut().left.take() {
            Self::traverse(node_infos, left, row + 1, col - 1);
        }

        if let Some(right) = node.borrow_mut().right.take() {
            Self::traverse(node_infos, right, row + 1, col + 1);
        }
    }

    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let Some(node) = root {
            let mut node_infos = vec![];
            Self::traverse(&mut node_infos, node, 0, 0);
            node_infos.sort();
            let mut last_col_num = node_infos[0].col;
            let mut last_col = vec![(*node_infos[0].node).borrow().val];
            let mut result = vec![];

            for i in 1..node_infos.len() {
                let this_col_num = node_infos[i].col;
                if this_col_num == last_col_num {
                    last_col.push((*node_infos[i].node).borrow().val);
                } else {
                    result.push(last_col);
                    last_col_num = this_col_num;
                    last_col = vec![(*node_infos[i].node).borrow().val];
                }
            }

            result.push(last_col);
            result
        } else {
            vec![]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_987() {
    }
}
