/**
 * [20] Valid Parentheses
 *
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
 * An input string is valid if:
 * <ol>
 * 	Open brackets must be closed by the same type of brackets.
 * 	Open brackets must be closed in the correct order.
 * </ol>
 *  
 * Example 1:
 * 
 * Input: s = "()"
 * Output: true
 * 
 * Example 2:
 * 
 * Input: s = "()[]{}"
 * Output: true
 * 
 * Example 3:
 * 
 * Input: s = "(]"
 * Output: false
 * 
 * Example 4:
 * 
 * Input: s = "([)]"
 * Output: false
 * 
 * Example 5:
 * 
 * Input: s = "{[]}"
 * Output: true
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^4
 * 	s consists of parentheses only '()[]{}'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-parentheses/
// discuss: https://leetcode.com/problems/valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            match(c) {
                '(' | '[' | '{' => {stack.push(c)},
                ')' => {
                    if let Some('(') = stack.pop() {
                        // do nothing
                    } else {
                        return false;
                    }
                },
                ']' => {
                    if let Some('[') = stack.pop() {
                        // do nothing
                    } else {
                        return false;
                    }
                },
                '}' => {
                    if let Some('{') = stack.pop() {
                        // do nothing
                    } else {
                        return false;
                    }
                },
                _ => {panic!("Unrecognized char...")}
            };
        }
        return stack.len() == 0;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }
}
