/**
 * [22] Generate Parentheses
 *
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 *  
 * Example 1:
 * Input: n = 3
 * Output: ["((()))","(()())","(())()","()(())","()()()"]
 * Example 2:
 * Input: n = 1
 * Output: ["()"]
 *  
 * Constraints:
 * 
 * 	1 <= n <= 8
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/generate-parentheses/
// discuss: https://leetcode.com/problems/generate-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
impl Solution {
    pub fn helper(result : &mut Vec<String>, tmp : &mut Vec<char>, unclosed_count : i32, total_count : i32) {
        let len : i32 = tmp.len() as i32;
        if len == 2 * total_count {
            result.push(tmp.iter().cloned().collect());
            return;
        }
        if unclosed_count < 2 * total_count - len {
            tmp.push('(');
            Self::helper(result, tmp, unclosed_count + 1, total_count);
            tmp.pop();
        }

        if unclosed_count > 0 {
            tmp.push(')');
            Self::helper(result, tmp, unclosed_count - 1, total_count);
            tmp.pop();
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result : Vec<String> = vec![];
        let mut tmp : Vec<char> = vec![];
        Self::helper(&mut result, &mut tmp, 0, n);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
        assert_eq!(Solution::generate_parenthesis(2), vec!["(())", "()()"]);
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
