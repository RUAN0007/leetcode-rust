/**
 * [32] Longest Valid Parentheses
 *
 * Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.
 *  
 * Example 1:
 * 
 * Input: s = "(()"
 * Output: 2
 * Explanation: The longest valid parentheses substring is "()".
 * 
 * Example 2:
 * 
 * Input: s = ")()())"
 * Output: 4
 * Explanation: The longest valid parentheses substring is "()()".
 * 
 * Example 3:
 * 
 * Input: s = ""
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	0 <= s.length <= 3 * 10^4
 * 	s[i] is '(', or ')'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-valid-parentheses/
// discuss: https://leetcode.com/problems/longest-valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut longest_ending : Vec<usize> = vec![0;s.len()];
        let mut stack : Vec<usize> = vec![];
        let mut max_len : usize = 0;
        for (i, c) in s.chars().enumerate() {
            if c == ')' {
                if let Some(left_idx) = stack.pop() {
                    let mut cur_len = i - left_idx + 1;
                    if left_idx > 0 && longest_ending[left_idx-1] > 0 {
                        cur_len += longest_ending[left_idx-1];
                    }
                    longest_ending[i] = cur_len;
                    max_len = std::cmp::max(max_len, cur_len);
                }

            } else {
                stack.push(i);
            }
        }
        max_len as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_32() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses(")(".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(
            Solution::longest_valid_parentheses("(((((()()".to_string()),
            4
        );
        assert_eq!(
            Solution::longest_valid_parentheses("((((((((()))".to_string()),
            6
        );
        assert_eq!(Solution::longest_valid_parentheses("()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
        assert_eq!(
            Solution::longest_valid_parentheses(")()(((())))(".to_string()),
            10
        );
        assert_eq!(
            Solution::longest_valid_parentheses("(()(((()".to_string()),
            2
        );
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    }
}
