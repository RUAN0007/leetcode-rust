/**
 * [331] Verify Preorder Serialization of a Binary Tree
 *
 * One way to serialize a binary tree is to use preorder traversal. When we encounter a non-null node, we record the node's value. If it is a null node, we record using a sentinel value such as '#'.
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/12/pre-tree.jpg" style="width: 362px; height: 293px;" />
 * For example, the above binary tree can be serialized to the string "9,3,4,#,#,1,#,#,2,#,6,#,#", where '#' represents a null node.
 * Given a string of comma-separated values preorder, return true if it is a correct preorder traversal serialization of a binary tree.
 * It is guaranteed that each comma-separated value in the string must be either an integer or a character '#' representing null pointer.
 * You may assume that the input format is always valid.
 * 
 * 	For example, it could never contain two consecutive commas, such as "1,,3".
 * 
 * Note: You are not allowed to reconstruct the tree.
 *  
 * Example 1:
 * Input: preorder = "9,3,4,#,#,1,#,#,2,#,6,#,#"
 * Output: true
 * Example 2:
 * Input: preorder = "1,#"
 * Output: false
 * Example 3:
 * Input: preorder = "9,#,#,1"
 * Output: false
 *  
 * Constraints:
 * 
 * 	1 <= preorder.length <= 10^4
 * 	preoder consist of integers in the range [0, 100] and '#' separated by commas ','.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/verify-preorder-serialization-of-a-binary-tree/
// discuss: https://leetcode.com/problems/verify-preorder-serialization-of-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
const LEFT : i32 = 0;
const RIGHT : i32 = 1;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut preorder = preorder;
        preorder.push(',');

        let mut prev_str : String = String::from("");
        let mut stack : Vec<(i32, i32)> = vec![];
        let len : usize = preorder.len();

        for (i, c) in preorder.chars().enumerate() {
            if c == ',' {
                if let Ok(num) = prev_str.parse::<i32>() {
                    stack.push((num, LEFT));
                }  else {
                    // println!("==============================");
                    // println!("i={}, c={}, prev_str={}", i, c, prev_str);
                    // println!("Before: {:?}", stack);
                    while let Some(&(last_num, state)) = stack.last() {
                        if state == RIGHT {
                            stack.pop();
                        } else {
                            break;
                        }
                    }

                    if let Some((last_num, state)) = stack.pop() {
                        if state == LEFT {
                            stack.push((last_num, RIGHT));
                        }
                        // println!("Post: {:?}", stack);
                        // println!("=============================")
                    } else {
                        if i == len - 1 {
                            return true;
                        } else {
                            return false;
                        }
                    }

                }


                prev_str = "".to_owned();
            } else {
                // digits or #
                prev_str.push(c);
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_331() {
        assert!(Solution::is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".to_owned()));
        assert!(!Solution::is_valid_serialization("1,#".to_owned()));
        assert!(!Solution::is_valid_serialization("9,#,#,1".to_owned()));
    }
}
