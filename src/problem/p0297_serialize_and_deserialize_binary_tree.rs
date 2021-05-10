/**
 * [297] Serialize and Deserialize Binary Tree
 *
 * Serialization is the process of converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.
 * Design an algorithm to serialize and deserialize a binary tree. There is no restriction on how your serialization/deserialization algorithm should work. You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.
 * Clarification: The input/output format is the same as <a href="/faq/#binary-tree">how LeetCode serializes a binary tree</a>. You do not necessarily need to follow this format, so please be creative and come up with different approaches yourself.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/serdeser.jpg" style="width: 442px; height: 324px;" />
 * Input: root = [1,2,3,null,null,4,5]
 * Output: [1,2,3,null,null,4,5]
 * 
 * Example 2:
 * 
 * Input: root = []
 * Output: []
 * 
 * Example 3:
 * 
 * Input: root = [1]
 * Output: [1]
 * 
 * Example 4:
 * 
 * Input: root = [1,2]
 * Output: [1,2]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [0, 10^4].
 * 	-1000 <= Node.val <= 1000
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/serialize-and-deserialize-binary-tree/
// discuss: https://leetcode.com/problems/serialize-and-deserialize-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
struct Codec {
	
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec{}
    }

    fn preorder_encode(node : &Option<Rc<RefCell<TreeNode>>>, encoded : &mut Vec<String>) {
        if node.is_none() {
            encoded.push("X".to_owned());
            return;
        }
        let val_str : String = node.as_ref().unwrap().borrow().val.to_string();
        encoded.push(val_str);

        Self::preorder_encode(&node.as_ref().unwrap().borrow().left, encoded);
        Self::preorder_encode(&node.as_ref().unwrap().borrow().right, encoded);
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut encoded : Vec<String> = vec![];
        Self::preorder_encode(&root, &mut encoded);
        encoded.join("_")
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let encoded : Vec<String> = data.split("_").map(|x|{x.to_owned()}).collect();
        Self::preorder_parse(&encoded, &mut 0usize)
    }

    fn preorder_parse(encoded : &Vec<String>, pos : &mut usize) ->Option<Rc<RefCell<TreeNode>>> {
        if encoded[*pos].eq("X") {
            *pos+=1;
            return None;
        }
        let val : i32 = encoded[*pos].parse::<i32>().unwrap();
        *pos+=1;
        let left_node : Option<Rc<RefCell<TreeNode>>> = Self::preorder_parse(&encoded, pos);
        let right_node : Option<Rc<RefCell<TreeNode>>> = Self::preorder_parse(&encoded, pos);
        let this_node : Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(val)));
        this_node.borrow_mut().left = left_node;
        this_node.borrow_mut().right = right_node;
        Some(this_node)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_297() {
        let codec = Codec{};
        let expected : String = "1_2_X_X_3_4_X_X_5_X_X".to_owned();
        let tree : Option<Rc<RefCell<TreeNode>>> = tree![1,2,3,null,null,4,5];
        assert_eq!(codec.serialize(tree), expected.to_owned());
        let tree : Option<Rc<RefCell<TreeNode>>> = tree![1,2,3,null,null,4,5];
        assert_eq!(codec.deserialize(expected), tree);

        let expected : String = "X".to_owned();
        let tree : Option<Rc<RefCell<TreeNode>>> = tree![];
        assert_eq!(codec.serialize(tree), expected.to_owned());
        let tree : Option<Rc<RefCell<TreeNode>>> = tree![];
        assert_eq!(codec.deserialize(expected), tree);
    }
}
